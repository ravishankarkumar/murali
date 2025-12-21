//! src/resources/typst/compiler.rs
//! Embedded Typst backend: Typst source -> SVG string (in-process)

use anyhow::{Result, anyhow};
use typst_kit::fonts::FontSearcher;
// use typst_kit::fonts::Fonts;

// use typst::text::Fonts;

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
            use typst_svg::svg;

            println!("TypstBackend: rendering to SVG with source:\n{}", typst_src);

            // let text = r#"#text("Hello, Murali!")"#;

            // 1. Ensure you use the input typst_src, not a hardcoded variable
            let effective_font_size = if _font_size > 0.0 { _font_size } else { 11.0 };
            let bottom_margin_pt = effective_font_size * 0.3;

            // 2. Use the '@' operator to correctly insert the content string as markup
            // This prevents Typst from misinterpreting the source as pure code.
            let configured_src = format!(
                "#set page(
                width: auto, 
                height: auto, 
                margin: (top: 0pt, bottom: {}pt),
                fill: none)
                #set text(size: {}pt)
                {}",
                bottom_margin_pt, effective_font_size, typst_src
            );

            let mut searcher = FontSearcher::new();
            let fonts_struct = searcher.search();

            let font_datas: Vec<Vec<u8>> = fonts_struct
                .fonts
                .iter()
                .filter_map(|slot| slot.get().map(|f| f.data().to_vec()))
                .collect();

            let font_slices: Vec<&[u8]> = font_datas.iter().map(|v| v.as_slice()).collect();
            let engine = TypstEngine::builder()
                .main_file(&*configured_src)
                .fonts(font_slices)
                .build();

            // ------------------------------------------------------------
            // 1. Build engine with the Typst source as the *main file*
            // ------------------------------------------------------------
            // let engine = TypstEngine::builder().main_file(&*configured_src).build();
            // let engine = TypstEngine::builder().main_file(typst_src).build();

            println!("TypstBackend: engine creation\n");

            // ------------------------------------------------------------
            // 2. Compile → Warned<Result<Document, TypstAsLibError>>
            // ------------------------------------------------------------
            let warned = engine.compile::<PagedDocument>();

            // ------------------------------------------------------------
            // 3. Handle diagnostics / errors
            // ------------------------------------------------------------
            let document = warned
                .output
                .map_err(|e| anyhow!("Typst compilation failed: {e:?}"))?;

            // ------------------------------------------------------------
            // 4. Extract first page
            // ------------------------------------------------------------
            let page = document
                .pages
                .get(0)
                .ok_or_else(|| anyhow!("Typst document produced no pages"))?;

            println!("TypstBackend: page creation\n");

            // ------------------------------------------------------------
            // 5. Export page → SVG
            // ------------------------------------------------------------
            let svg_text = svg(page);

            println!("TypstBackend: svg_text creation\n");
            return Ok(svg_text);
        }

        #[cfg(not(feature = "typst_embedded"))]
        {
            Err(anyhow!(
                "Embedded Typst backend is disabled.\n\
                 Enable it with:\n\n\
                 cargo run --features engine/typst_embedded"
            ))
        }
    }
}
