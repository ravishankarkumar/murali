//! src/resources/typst/compiler.rs
//! Embedded Typst backend: Typst source -> SVG string (in-process)

use anyhow::{Result, anyhow};

pub struct TypstBackend;

impl TypstBackend {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn render_to_svg(&self, typst_src: &str, _font_size: f32) -> Result<String> {
        #[cfg(feature = "typst_embedded")]
        {
            use typst::layout::PagedDocument;
            use typst_as_lib::TypstEngine;
            use typst_as_lib::typst_kit_options::TypstKitFontOptions;
            use typst_svg::svg;

            let effective_font_size = if _font_size > 0.0 { _font_size } else { 11.0 };
            let vertical_margin_pt = effective_font_size * 0.3;

            let configured_src = format!(
                "#set page(
                width: auto, 
                height: auto, 
                margin: (top: {}pt, bottom: {}pt),
                fill: none)
                #set text(
                  size: {}pt,
                  font: (
                    \"Libertinus Serif\",
                    \"Libertinus Math\",
                    \"New Computer Modern\",
                    \"New Computer Modern Math\",
                    \"STIX Two Text\",
                    \"STIX Two Math\",
                    \"DejaVu Serif\"
                  ),
                )
                {}",
                vertical_margin_pt, vertical_margin_pt, effective_font_size, typst_src
            );

            let engine = TypstEngine::builder()
                .main_file(&*configured_src)
                .search_fonts_with(
                    TypstKitFontOptions::new()
                        .include_system_fonts(true)
                        .include_embedded_fonts(true),
                )
                .build();

            let warned = engine.compile::<PagedDocument>();

            let document = warned
                .output
                .map_err(|e| anyhow!("Typst compilation failed: {e:?}"))?;

            let page = document
                .pages
                .get(0)
                .ok_or_else(|| anyhow!("Typst document produced no pages"))?;

            return Ok(svg(page));
        }

        #[cfg(not(feature = "typst_embedded"))]
        {
            Err(anyhow!(
                "Embedded Typst backend is disabled.\n\
                 Enable it with:\n\n\
                 cargo run --features typst_embedded"
            ))
        }
    }
}
