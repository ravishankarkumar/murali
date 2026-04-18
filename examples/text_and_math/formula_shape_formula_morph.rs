use glam::{Vec2, Vec3, Vec4, vec2};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::primitives::path::{Path, PathSegment};
use murali::resource::latex_resource::backend::compile_latex;
use murali::resource::typst_resource::compiler::TypstBackend;
use murali::resource::typst_resource::vector::{VectorSymbol, parse_svg_to_paths};

fn scale_path(path: &mut Path, factor: f32) {
    for seg in &mut path.segments {
        match seg {
            PathSegment::MoveTo(p) => *p *= factor,
            PathSegment::LineTo(p) => *p *= factor,
            PathSegment::QuadTo(c, p) => {
                *c *= factor;
                *p *= factor;
            }
            PathSegment::CubicTo(c1, c2, p) => {
                *c1 *= factor;
                *c2 *= factor;
                *p *= factor;
            }
        }
    }
}

fn translate_path(path: &mut Path, offset: Vec2) {
    for seg in &mut path.segments {
        match seg {
            PathSegment::MoveTo(p) => *p += offset,
            PathSegment::LineTo(p) => *p += offset,
            PathSegment::QuadTo(c, p) => {
                *c += offset;
                *p += offset;
            }
            PathSegment::CubicTo(c1, c2, p) => {
                *c1 += offset;
                *c2 += offset;
                *p += offset;
            }
        }
    }
}

fn merge_symbols_into_path(symbols: Vec<VectorSymbol>, world_height: f32) -> Path {
    let base_size = 32.0;
    let mut merged = Path::new();
    let mut style = None;
    let mut fill_rule = None;

    for symbol in symbols {
        let mut path = symbol.path;
        scale_path(&mut path, world_height / base_size);
        if style.is_none() {
            style = Some(path.style.clone());
        }
        if fill_rule.is_none() {
            fill_rule = Some(path.fill_rule);
        }
        merged.segments.extend(path.segments);
    }

    if let Some(style) = style {
        merged.style = style;
    }
    if let Some(fill_rule) = fill_rule {
        merged.fill_rule = fill_rule;
    }
    merged.closed = true;
    merged
}

fn build_typst_formula_path(content: &str, world_height: f32, color: Vec4) -> anyhow::Result<Path> {
    let backend = TypstBackend::new()?;
    let svg = backend.render_to_svg(content, 32.0)?;
    let symbols = parse_svg_to_paths(&svg, color)?;
    Ok(merge_symbols_into_path(symbols, world_height))
}

fn build_latex_formula_path(content: &str, world_height: f32, color: Vec4) -> anyhow::Result<Path> {
    let cache_dir = std::env::temp_dir().join("murali_latex_cache");
    let latex = compile_latex(content, &cache_dir)?;
    let symbols = parse_svg_to_paths(&latex.svg_content, color)?;
    Ok(merge_symbols_into_path(symbols, world_height))
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let mut source_formula =
        build_typst_formula_path("$(a + b)^2$", 1.55, BLUE_B)?;
    translate_path(&mut source_formula, vec2(0.0, 0.15));
    let source_id = scene.add_tattva(source_formula, Vec3::ZERO);

    let shape_id = scene.add_tattva(
        Circle::new(0.95, 64, TEAL_C),
        0.15 * UP,
    );
    scene.hide_tattva(shape_id);

    let mut target_formula =
        build_latex_formula_path(r"a^2 + 2ab + b^2", 1.45, GOLD_C)?;
    translate_path(&mut target_formula, vec2(0.0, 0.15));
    let target_id = scene.add_tattva(target_formula, Vec3::ZERO);
    scene.hide_tattva(target_id);

    let mut timeline = Timeline::new();
    timeline
        .animate(shape_id)
        .at(1.0)
        .for_duration(2.2)
        .ease(Ease::InOutCubic)
        .morph_from(source_id)
        .spawn();

    timeline
        .animate(target_id)
        .at(4.0)
        .for_duration(2.4)
        .ease(Ease::InOutCubic)
        .morph_from(shape_id)
        .spawn();

    scene.set_timeline("main", timeline);
    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    App::new()?.with_scene(scene).run_app()
}
