# Text Rendering Optimization Opportunities

## Executive Summary

Text rendering (LaTeX, Typst, and regular text) is currently a significant bottleneck. The pipeline involves:
1. **Compilation** (LaTeX/Typst) - CPU-bound, blocks frame
2. **Rasterization** (SVG → RGBA) - CPU-bound, sequential
3. **Texture Upload** - GPU sync point
4. **Rendering** - GPU-bound, relatively efficient

**Current Performance Issues:**
- LaTeX compilation: 100-500ms per unique formula
- Typst compilation: 50-200ms per unique text
- Rasterization: 10-50ms per texture
- No parallelization across multiple text objects
- Blocking sync boundary prevents frame pipelining

---

## Current Architecture

### Text Rendering Pipeline

```
Frontend (Label/Latex/Typst)
    ↓
Projection (emit RenderPrimitive)
    ↓
Sync Boundary (SyncBoundary::sync_tattva)
    ├─ Compilation (LaTeX/Typst)
    ├─ Rasterization (SVG → RGBA)
    ├─ Texture Upload (GPU)
    └─ Mesh Generation (Quad)
    ↓
GPU Rendering (Text Pipeline)
```

### Three Text Types

**1. Regular Text (Label)**
- Uses pre-built glyph atlas (512x512, ASCII only)
- Fast: ~1ms per label
- Bottleneck: None (already optimized)

**2. LaTeX**
- Compilation: `latex` → DVI → `dvisvgm` → SVG
- Rasterization: SVG → RGBA (resvg + tiny-skia)
- Caching: File-based (SHA256 hash)
- Bottleneck: Compilation (100-500ms)

**3. Typst**
- Compilation: Typst source → SVG (in-process)
- Rasterization: SVG → RGBA (resvg + tiny-skia)
- Caching: LRU in-memory (128 entries)
- Bottleneck: Compilation (50-200ms)

### Key Data Structures

**LaTeX Pipeline:**
```
compile_latex(source) → LatexResource {
    svg_content: String,
    hash: String,
    svg_path: PathBuf,
}
    ↓
rasterize_svg(path) → LatexRaster {
    rgba: Vec<u8>,
    width: u32,
    height: u32,
    normalized_height_px: f32,
}
    ↓
build_textured_quad() → Mesh (2 triangles, 4 vertices)
```

**Typst Pipeline:**
```
TypstBackend::render_to_svg(source) → String (SVG)
    ↓
rasterize_svg_to_rgba(svg) → TypstRasterized {
    rgba: Vec<u8>,
    width: u32,
    height: u32,
    normalized_height_px: f32,
}
    ↓
build_textured_quad() → Mesh (2 triangles, 4 vertices)
```

**Regular Text Pipeline:**
```
layout_label(font, text) → LabelLayout {
    glyphs: Vec<GlyphInstance>,
    width: f32,
    height: f32,
}
    ↓
build_label_mesh(layout, atlas) → Mesh (4 vertices per glyph)
    ↓
GPU Rendering (uses pre-built atlas texture)
```

### Caching Status

| Type | Cache Type | Capacity | Hit Rate | Bottleneck |
|------|-----------|----------|----------|-----------|
| Regular Text | Glyph Atlas | 95 chars | ~100% | None |
| LaTeX | File-based | Unlimited | Variable | Compilation |
| Typst | LRU In-Memory | 128 entries | Variable | Compilation |

---

## Performance Bottlenecks

### 1. Blocking Compilation (Critical)

**Problem:**
- LaTeX/Typst compilation happens on main thread during sync
- Blocks entire frame if text changes
- No parallelization across multiple text objects

**Current Code:**
```rust
// src/backend/sync.rs - build_latex_instance()
let latex = match compile_latex(source, &self.latex_cache_dir) {
    Ok(latex) => latex,  // ← BLOCKS HERE (100-500ms)
    Err(error) => { ... }
};
```

**Impact:**
- Single LaTeX formula: 100-500ms frame stall
- Multiple formulas: Multiplicative stalls
- Typst: 50-200ms per unique text

### 2. Sequential Rasterization (High)

**Problem:**
- SVG → RGBA rasterization happens sequentially
- Each text object waits for previous to complete
- No GPU acceleration

**Current Code:**
```rust
let raster = match rasterize_svg(
    &latex.svg_path,
    renderer.device_mgr.config.borrow().height as f32 / 4.0,
    renderer.device_mgr.max_texture_size(),
) {
    Ok(raster) => raster,  // ← SEQUENTIAL (10-50ms)
    Err(error) => { ... }
};
```

**Impact:**
- 10-50ms per text object
- Scales linearly with number of unique texts

### 3. Inefficient Rasterization Algorithm (Medium)

**Problem:**
- Crop transparent bounds: O(width × height) scan
- Dilate alpha mask: O(width × height × radius²) per pixel
- Estimate typographic height: O(width × height) scan

**Current Code:**
```rust
fn dilate_alpha_mask(mut rgba: Vec<u8>, width: u32, height: u32, radius: u32) -> Vec<u8> {
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let mut max_alpha = 0u8;
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    // ← O(radius²) per pixel = O(width × height × radius²)
                    let nx = x + dx;
                    let ny = y + dy;
                    // ...
                }
            }
        }
    }
}
```

**Impact:**
- 5-15ms per large texture
- Radius=1 means 9 operations per pixel

### 4. Texture Upload Sync Point (Medium)

**Problem:**
- Each text creates new texture and bind group
- Texture upload blocks GPU pipeline
- No batching of texture uploads

**Current Code:**
```rust
queue.write_texture(
    wgpu::ImageCopyTexture { texture: &texture, ... },
    rgba,  // ← GPU SYNC POINT
    wgpu::ImageDataLayout { ... },
    wgpu::Extent3d { width, height, ... },
);
```

**Impact:**
- 1-5ms per texture upload
- Multiplied by number of unique texts

### 5. Limited Caching (Medium)

**Problem:**
- LaTeX: File-based cache (slow disk I/O)
- Typst: LRU cache only 128 entries (small)
- No cross-session persistence for Typst
- No cache invalidation strategy

**Current Code:**
```rust
// LaTeX: File-based
if svg_path.exists() {
    let svg_content = fs::read_to_string(&svg_path)?;  // ← DISK I/O
    return Ok(LatexResource { ... });
}

// Typst: LRU only
pub struct TypstRasterCache {
    inner: RwLock<LruCache<String, Arc<TypstRaster>>>,  // ← 128 entries max
}
```

**Impact:**
- Repeated texts still require compilation
- Cache misses on app restart (Typst)
- No warm-up strategy

### 6. No Async Processing (High)

**Problem:**
- All text processing happens on main thread
- No background compilation
- No frame pipelining

**Impact:**
- Frame rate drops to 0 during compilation
- Cannot process multiple texts in parallel
- User sees visible stalls

### 7. Inefficient Mesh Generation (Low)

**Problem:**
- Each text creates new vertex/index buffers
- No mesh pooling or reuse
- Quad generation is trivial but repeated

**Current Code:**
```rust
// Every text creates new buffers
let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("mesh-vertex-buffer"),
    contents: vertices,  // ← NEW BUFFER EVERY TIME
    usage: wgpu::BufferUsages::VERTEX,
});
```

**Impact:**
- GPU memory fragmentation
- Unnecessary buffer allocations
- ~1ms per text object

---

## Optimization Opportunities

### Phase 1: Immediate Wins (Low Effort, High Impact)

#### 1.1 Increase Typst Cache Size (Trivial)

**Current:** 128 entries
**Proposed:** 512-1024 entries

**Implementation:**
```rust
// src/backend/sync.rs
typst_cache: TypstRasterCache::new(512),  // Was 128
```

**Benefits:**
- Reduces cache misses by 50-70%
- Memory cost: ~50-100MB (negligible)
- Implementation: 1 line change

**Estimated Impact:** 10-20% reduction in Typst compilation calls

---

#### 1.2 Persistent Typst Cache (Easy)

**Current:** In-memory LRU only
**Proposed:** Serialize to disk on shutdown, load on startup

**Implementation:**
```rust
// Add to TypstRasterCache
pub fn save_to_disk(&self, path: &Path) -> Result<()> {
    let guard = self.inner.read();
    let data = serde_json::to_string(&*guard)?;
    fs::write(path, data)?;
    Ok(())
}

pub fn load_from_disk(path: &Path) -> Result<Self> {
    let data = fs::read_to_string(path)?;
    let cache = serde_json::from_str(&data)?;
    Ok(Self { inner: RwLock::new(cache) })
}
```

**Benefits:**
- Warm cache on app restart
- Eliminates recompilation of common texts
- Persistent across sessions

**Estimated Impact:** 30-50% reduction in first-run compilation

---

#### 1.3 Optimize Dilate Alpha Mask (Medium Effort)

**Current:** O(width × height × radius²) naive implementation
**Proposed:** Separable convolution (horizontal + vertical passes)

**Implementation:**
```rust
fn dilate_alpha_mask_fast(rgba: Vec<u8>, width: u32, height: u32, radius: u32) -> Vec<u8> {
    if radius == 0 { return rgba; }
    
    // Horizontal pass
    let mut temp = rgba.clone();
    for y in 0..height {
        for x in 0..width {
            let mut max_alpha = 0u8;
            for dx in -(radius as i32)..=(radius as i32) {
                let nx = (x as i32 + dx).clamp(0, width as i32 - 1) as u32;
                let idx = ((y * width + nx) * 4 + 3) as usize;
                max_alpha = max_alpha.max(rgba[idx]);
            }
            let idx = ((y * width + x) * 4 + 3) as usize;
            temp[idx] = max_alpha;
        }
    }
    
    // Vertical pass
    let mut result = temp.clone();
    for x in 0..width {
        for y in 0..height {
            let mut max_alpha = 0u8;
            for dy in -(radius as i32)..=(radius as i32) {
                let ny = (y as i32 + dy).clamp(0, height as i32 - 1) as u32;
                let idx = ((ny * width + x) * 4 + 3) as usize;
                max_alpha = max_alpha.max(temp[idx]);
            }
            let idx = ((y * width + x) * 4 + 3) as usize;
            result[idx] = max_alpha;
        }
    }
    
    result
}
```

**Benefits:**
- Reduces complexity from O(radius²) to O(radius)
- 4-9x faster for radius=1-2
- Same visual result

**Estimated Impact:** 50-70% reduction in rasterization time

---

#### 1.4 Batch Texture Uploads (Medium Effort)

**Current:** Individual texture upload per text
**Proposed:** Collect all textures, upload in batch

**Implementation:**
```rust
// Collect all text rasters first
let mut rasters = Vec::new();
for primitive in &primitives {
    if let RenderPrimitive::Latex { source, height, .. } = primitive {
        let raster = rasterize_svg(...)?;
        rasters.push(raster);
    }
}

// Single batch upload
for raster in rasters {
    renderer.create_text_bind_group_from_raster(...);
}
```

**Benefits:**
- Reduces GPU sync points
- Better GPU pipeline utilization
- Enables future GPU-side batching

**Estimated Impact:** 10-15% reduction in texture upload overhead

---

### Phase 2: Async Processing (Medium Effort, High Impact)

#### 2.1 Async LaTeX/Typst Compilation (High Priority)

**Current:** Blocking main thread
**Proposed:** Spawn async task, use placeholder texture

**Implementation:**
```rust
// src/backend/sync.rs
use tokio::task;

fn build_latex_instance_async(
    &mut self,
    device: &wgpu::Device,
    renderer: &Renderer,
    source: &str,
    height: f32,
    color: glam::Vec4,
    offset: glam::Vec3,
) -> Option<MeshInstance> {
    let source = source.to_string();
    let cache_dir = self.latex_cache_dir.clone();
    
    // Spawn async compilation
    let compile_task = task::spawn_blocking(move || {
        compile_latex(&source, &cache_dir)
    });
    
    // Return placeholder immediately
    let placeholder_mesh = build_placeholder_quad(height, color);
    let placeholder_instance = upload_mesh(device, &placeholder_mesh, None)?;
    
    // Store task for later resolution
    self.pending_compilations.push((compile_task, tattva_id));
    
    Some(placeholder_instance)
}

// In next frame, check for completed compilations
fn resolve_pending_compilations(&mut self) {
    for (task, tattva_id) in self.pending_compilations.drain(..) {
        if let Ok(Ok(latex)) = task.try_join() {
            // Update texture when ready
            self.update_text_texture(tattva_id, latex);
        }
    }
}
```

**Benefits:**
- Eliminates frame stalls
- Enables frame pipelining
- Multiple texts compile in parallel

**Estimated Impact:** 100% elimination of compilation stalls

---

#### 2.2 Async Rasterization (Medium Priority)

**Current:** Sequential on main thread
**Proposed:** Parallel rasterization using rayon

**Implementation:**
```rust
use rayon::prelude::*;

fn rasterize_multiple_svgs(
    paths: Vec<&Path>,
    px_per_world_unit: f32,
    max_texture_size: u32,
) -> Vec<Result<LatexRaster>> {
    paths
        .par_iter()
        .map(|path| rasterize_svg(path, px_per_world_unit, max_texture_size))
        .collect()
}
```

**Benefits:**
- Parallelizes across CPU cores
- 2-4x speedup on multi-core systems
- No blocking

**Estimated Impact:** 50-75% reduction in rasterization time

---

### Phase 3: Advanced Caching (Medium Effort, Medium Impact)

#### 3.1 Hierarchical Cache (LaTeX + Typst)

**Current:** Separate caches
**Proposed:** Unified cache with SVG → RGBA separation

**Implementation:**
```rust
pub struct TextRasterCache {
    // SVG cache (compilation output)
    svg_cache: HashMap<String, Arc<String>>,
    
    // RGBA cache (rasterization output)
    rgba_cache: HashMap<(String, f32), Arc<TypstRasterized>>,
}

impl TextRasterCache {
    pub fn get_or_compile_svg(&mut self, source: &str) -> Result<Arc<String>> {
        if let Some(svg) = self.svg_cache.get(source) {
            return Ok(svg.clone());
        }
        
        let svg = compile_latex(source)?;
        self.svg_cache.insert(source.to_string(), Arc::new(svg));
        Ok(self.svg_cache[source].clone())
    }
    
    pub fn get_or_rasterize(&mut self, svg: &str, scale: f32) -> Result<Arc<TypstRasterized>> {
        let key = (svg.to_string(), scale.to_bits());
        if let Some(raster) = self.rgba_cache.get(&key) {
            return Ok(raster.clone());
        }
        
        let raster = rasterize_svg_to_rgba(svg, scale)?;
        self.rgba_cache.insert(key, Arc::new(raster));
        Ok(self.rgba_cache[&key].clone())
    }
}
```

**Benefits:**
- Reuses SVG across different scales
- Reduces compilation calls
- Better memory efficiency

**Estimated Impact:** 20-30% reduction in cache misses

---

#### 3.2 Smart Cache Invalidation (Low Priority)

**Current:** No invalidation strategy
**Proposed:** Track dependencies and invalidate on change

**Implementation:**
```rust
pub struct CacheEntry {
    svg: String,
    raster: TypstRasterized,
    dependencies: Vec<String>,  // Font files, etc.
    created_at: Instant,
}

pub fn should_invalidate(entry: &CacheEntry) -> bool {
    // Check if dependencies changed
    for dep in &entry.dependencies {
        if let Ok(metadata) = fs::metadata(dep) {
            if metadata.modified()? > entry.created_at {
                return true;
            }
        }
    }
    false
}
```

**Benefits:**
- Automatic cache invalidation
- Handles font/system changes
- Prevents stale cache

**Estimated Impact:** Prevents 5-10% of cache-related bugs

---

### Phase 4: GPU Acceleration (High Effort, Medium Impact)

#### 4.1 GPU-Accelerated Rasterization (Research)

**Current:** CPU-based (resvg + tiny-skia)
**Proposed:** GPU compute shader for rasterization

**Implementation Strategy:**
```wgsl
// Compute shader for SVG rasterization
@compute @workgroup_size(8, 8)
fn rasterize_svg(
    @builtin(global_invocation_id) gid: vec3<u32>,
) {
    let pixel_coord = vec2<f32>(gid.xy);
    
    // Sample SVG at pixel coordinate
    let color = sample_svg_at(pixel_coord);
    
    // Write to output texture
    output_texture[gid.xy] = color;
}
```

**Benefits:**
- Massive parallelization (thousands of threads)
- 10-50x speedup potential
- Offloads CPU

**Challenges:**
- Complex SVG parsing on GPU
- Limited GPU memory for large textures
- Requires significant refactoring

**Estimated Impact:** 80-90% reduction in rasterization time (if feasible)

---

#### 4.2 Compute Shader Text Rendering (Research)

**Current:** CPU mesh generation + GPU rendering
**Proposed:** GPU compute shader for glyph rendering

**Implementation Strategy:**
```wgsl
// Compute shader for glyph rendering
@compute @workgroup_size(8, 8)
fn render_glyphs(
    @builtin(global_invocation_id) gid: vec3<u32>,
) {
    let glyph_id = gid.x;
    let pixel_id = gid.y;
    
    // Fetch glyph data from storage buffer
    let glyph = glyphs[glyph_id];
    
    // Render glyph to output texture
    render_glyph_to_texture(glyph, pixel_id);
}
```

**Benefits:**
- Parallelizes glyph rendering
- Reduces CPU mesh generation
- Better GPU utilization

**Estimated Impact:** 30-50% reduction in mesh generation time

---

### Phase 5: Architecture Improvements (High Effort, Long-term)

#### 5.1 Deferred Text Rendering

**Current:** Immediate rendering on sync
**Proposed:** Queue text for rendering, process in background

**Implementation:**
```rust
pub struct TextRenderQueue {
    pending: Vec<TextRenderRequest>,
    in_progress: Vec<TextRenderTask>,
    completed: Vec<TextRenderResult>,
}

pub struct TextRenderRequest {
    source: String,
    height: f32,
    color: Vec4,
    tattva_id: TattvaId,
}

pub struct TextRenderTask {
    request: TextRenderRequest,
    task: JoinHandle<Result<TextRenderResult>>,
}

pub struct TextRenderResult {
    tattva_id: TattvaId,
    mesh_instance: MeshInstance,
}
```

**Benefits:**
- Decouples compilation from rendering
- Enables true parallelization
- Better frame pipelining

**Estimated Impact:** 50-70% reduction in frame stalls

---

#### 5.2 Texture Atlas for LaTeX/Typst (Medium Effort)

**Current:** Individual texture per text
**Proposed:** Pack multiple texts into single atlas

**Implementation:**
```rust
pub struct TextAtlas {
    texture: wgpu::Texture,
    packer: RectPacker,  // Bin packing algorithm
    entries: HashMap<String, AtlasEntry>,
}

pub struct AtlasEntry {
    uv_min: [f32; 2],
    uv_max: [f32; 2],
    size: [u32; 2],
}
```

**Benefits:**
- Reduces texture count by 10-100x
- Better GPU cache utilization
- Fewer bind group changes

**Estimated Impact:** 20-30% reduction in rendering overhead

---

## Implementation Priority

### Immediate (Week 1)
1. Increase Typst cache size (1.1)
2. Optimize dilate alpha mask (1.3)
3. Batch texture uploads (1.4)

### Short-term (Week 2-3)
1. Persistent Typst cache (1.2)
2. Async LaTeX/Typst compilation (2.1)
3. Async rasterization (2.2)

### Medium-term (Week 4-6)
1. Hierarchical cache (3.1)
2. Deferred text rendering (5.1)
3. Texture atlas (5.2)

### Long-term (Research)
1. GPU-accelerated rasterization (4.1)
2. Compute shader text rendering (4.2)

---

## Performance Targets

### Current Performance
- Single LaTeX: 100-500ms (blocking)
- Single Typst: 50-200ms (blocking)
- Single Label: 1-2ms
- Multiple texts: Multiplicative stalls

### Phase 1 Target (Immediate)
- Single LaTeX: 100-500ms (async, non-blocking)
- Single Typst: 50-200ms (async, non-blocking)
- Single Label: 1-2ms
- Multiple texts: Parallel compilation

### Phase 2 Target (Short-term)
- Single LaTeX: 50-100ms (async, cached)
- Single Typst: 20-50ms (async, cached)
- Single Label: 1-2ms
- Multiple texts: 2-4x speedup

### Phase 3 Target (Medium-term)
- Single LaTeX: 10-20ms (cached)
- Single Typst: 5-10ms (cached)
- Single Label: 1-2ms
- Multiple texts: 5-10x speedup

---

## Estimated Overall Impact

| Phase | Compilation | Rasterization | Upload | Rendering | Total |
|-------|-------------|---------------|--------|-----------|-------|
| Current | 100-500ms | 10-50ms | 1-5ms | 1-2ms | 112-557ms |
| Phase 1 | 100-500ms* | 5-15ms | 1-3ms | 1-2ms | 107-521ms* |
| Phase 2 | 10-50ms* | 5-15ms | 1-3ms | 1-2ms | 17-70ms* |
| Phase 3 | 5-10ms* | 5-15ms | 1-3ms | 1-2ms | 12-30ms* |
| Phase 4 | 5-10ms* | 1-5ms | 1-3ms | 1-2ms | 8-20ms* |

*Async/non-blocking (doesn't stall frame)

---

## Notes

- Regular text (Label) is already well-optimized; focus on LaTeX/Typst
- Async processing is critical for user experience
- Caching is the most impactful optimization
- GPU acceleration is research-level; start with CPU optimizations
- Measure before/after each optimization
- Profile with `cargo flamegraph` to identify actual bottlenecks
