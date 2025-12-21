// src/draw/mod.rs

use std::sync::Arc;

pub mod context;
// Note: ConstructContext is kept if your timelines still use it for state, 
// otherwise it can be removed if strictly used for the old Construct system.
pub use context::ConstructContext;

use crate::{
    config::RenderConfig,
    renderer::{
        mesh::MeshInstance,
        renderer::{DrawableKind, Renderer},
    },
    scene::Scene,
    tattva::mesh_only_tattva::MeshOnlyTattva,
};

use crate::label::label_resources::{LabelResources, ensure_label_resources};

/// Expand semantic objects (Tattvas) into drawable GPU instances.
/// This is the bridge between user-intent and GPU-ready meshes.
pub fn materialize_scene(
    scene: &mut Scene,
    renderer: &mut Renderer,
    render_config: &RenderConfig,
    label_resources: &mut Option<LabelResources>,
) {
    // We collect IDs first to avoid borrow checker issues while mutating the scene
    let tattva_ids: Vec<usize> = scene.tattva_ids().collect();

    for tid in tattva_ids {
        let Some(tattva) = scene.get_tattva(tid) else {
            continue;
        };

        // ---------------------------------------------------------
        // 1. Label Materialization (Text -> Mesh + Atlas)
        // ---------------------------------------------------------
        if let Some(label_t) = tattva
            .as_any()
            .downcast_ref::<crate::label::tattva::LabelTattva>()
        {
            use crate::label::{layout::layout_label, mesh::build_label_mesh};

            let label_res = ensure_label_resources(label_resources, renderer);

            // Turn text into character quads
            let layout = layout_label(&label_res.font, &label_t.text, label_t.world_height);
            let mesh = build_label_mesh(&layout, &label_res.atlas);
            
            let temp = MeshOnlyTattva::new(mesh);
            let mut mesh_instance = renderer
                .create_drawable_for_tattva(&temp)
                .expect("Label must produce a mesh");

            // IMPORTANT: Assign the Font Atlas BindGroup to this instance
            // This is what Path B in your Renderer uses to switch textures
            let mut inst = Arc::make_mut(&mut mesh_instance);
            inst.bind_group = Some(label_res.bind_group.clone());

            scene.override_mesh(tid, mesh_instance);
            continue;
        }

        // ---------------------------------------------------------
        // 2. LaTeX Materialization (Math -> Mesh + SVG Texture)
        // ---------------------------------------------------------
        if let Some(latex_t) = tattva
            .as_any()
            .downcast_ref::<crate::latex::tattva::LatexTattva>()
        {
            use crate::latex::raster::rasterize_svg;
            use crate::renderer::mesh::latex_quad::build_textured_quad;

            let max_tex = renderer.device_mgr.max_texture_size();

            // Render SVG to pixels
            let raster = rasterize_svg(&latex_t.svg_path, latex_t.world_height, max_tex)
                .expect("LaTeX rasterization failed");

            // Create a plane for the texture
            let mesh = build_textured_quad(raster.width, raster.height, latex_t.world_height);
            let temp = MeshOnlyTattva::new(mesh);
            
            let mut mesh_instance = renderer
                .create_drawable_for_tattva(&temp)
                .expect("LaTeX must produce mesh");

            // Upload the SVG texture to the GPU
            let bind_group = Arc::new(renderer.create_text_bind_group_from_raster(
                &raster.rgba,
                raster.width,
                raster.height,
            ));

            // Attach the SVG texture to the mesh instance
            let mut inst = Arc::make_mut(&mut mesh_instance);
            inst.bind_group = Some(bind_group);

            scene.override_mesh(tid, mesh_instance);
            continue;
        }

        // ---------------------------------------------------------
        // 3. Standard Primitive Geometry (Triangles, Cubes, etc.)
        // ---------------------------------------------------------
        if let Some(mesh_instance) = renderer.create_drawable_for_tattva(&**tattva) {
            scene.override_mesh(tid, mesh_instance);
        }
    }
    
    // NOTE: Section 2 (Constructs) has been removed. 
    // High-performance lines/circles are now Sanghs added directly 
    // to scene.sanghs and processed in scene.sync().
}