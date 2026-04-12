---
sidebar_position: 2
---

# Dirty flags & the sync cycle

Every `Tattva<T>` carries a `DirtyFlags` bitmask. This is how Murali avoids re-uploading GPU data every frame — only tattvas with dirty flags get processed by the sync boundary.

## The flags

```rust
pub const TRANSFORM:    DirtyFlags = Self(1 << 0); // position, rotation, scale changed
pub const GEOMETRY:     DirtyFlags = Self(1 << 1); // shape vertices changed
pub const STYLE:        DirtyFlags = Self(1 << 2); // color changed
pub const TEXT_LAYOUT:  DirtyFlags = Self(1 << 3); // label content/size changed
pub const RASTER:       DirtyFlags = Self(1 << 4); // LaTeX/Typst source changed
pub const BOUNDS:       DirtyFlags = Self(1 << 5); // bounding box changed
pub const VISIBILITY:   DirtyFlags = Self(1 << 6); // visible/opacity changed

// Composite: any of these triggers a full ECS entity rebuild
pub const REBUILD: DirtyFlags = GEOMETRY | TEXT_LAYOUT | RASTER | BOUNDS;
```

## The sync decision

Each frame, `SyncBoundary::sync_tattva` checks the flags:

```
dirty flags empty?  →  skip (no work)
dirty & REBUILD?    →  despawn old entities, re-project, re-upload GPU buffers
otherwise           →  clear TRANSFORM | STYLE | VISIBILITY (handled by model matrix at draw time)
```

The key insight: `TRANSFORM` changes (moving a tattva) don't require re-uploading vertex data. The model matrix is computed from `SharedProps` at draw time and multiplied into the MVP uniform. Only geometry changes require a full rebuild.

## When flags are set

Flags are set by the animation system when it applies a frame:

- `move_to` → sets `TRANSFORM`
- `fade_to` → sets `VISIBILITY`
- `morph_from` → sets `GEOMETRY` (triggers `REBUILD`)
- `create` → sets `GEOMETRY`

They're also set when you mutate props directly:

```rust
if let Some(t) = scene.get_tattva_any_mut(id) {
    let mut props = t.props().write();
    props.visible = false;  // sets VISIBILITY flag internally
}
```

## New tattvas

When a tattva is first added to the scene, it starts with `DirtyFlags::GEOMETRY` set, so it gets materialized on the first frame.
