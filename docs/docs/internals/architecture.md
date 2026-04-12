---
sidebar_position: 1
---

# Architecture overview

Murali is split into three layers that communicate in one direction: Frontend → Engine → Backend.

```
┌─────────────────────────────────────────┐
│              User code                  │
│   Scene, Timeline, Tattvas, Updaters    │
└────────────────────┬────────────────────┘
                     │
┌────────────────────▼────────────────────┐
│               Frontend                  │
│  Tattva<T>, DirtyFlags, Projection      │
└────────────────────┬────────────────────┘
                     │  SyncBoundary
┌────────────────────▼────────────────────┐
│               Backend                   │
│  ECS World, Renderer, DeviceManager     │
└─────────────────────────────────────────┘
```

## Frontend

The frontend is the user-facing layer. It owns:

- `Scene` — the authoritative list of tattvas and timelines
- `Tattva<T>` — a generic wrapper around any shape/text state, plus `SharedProps` (position, rotation, scale, opacity, visibility)
- `Timeline` — schedules `AnimationSpec` entries against a time axis
- `DirtyFlags` — a bitmask that tracks what changed on a tattva each frame

The frontend never touches the GPU. It only produces `RenderPrimitive` values via the `Project` trait.

## Engine

The `Engine` struct is the heartbeat. Each frame it:

1. Calls `scene.update(dt)` — advances `scene_time`, runs timelines, runs updaters
2. Iterates all tattvas and calls `sync_boundary.sync_tattva(...)` for each dirty one
3. Calls `renderer.render_scene(...)` to draw the current ECS world

## Backend

The backend owns all GPU state:

- `DeviceManager` — wgpu device, queue, surface, and config
- `Renderer` — render pipelines, uniform buffers, depth texture
- `hecs::World` — the ECS world holding GPU-side components

The backend never knows about tattvas, timelines, or animations. It only knows about ECS components and how to draw them.

## The sync boundary

`SyncBoundary` is the bridge between the two worlds. It:

1. Checks `DirtyFlags` on each tattva
2. If dirty, calls `tattva.project()` to get `Vec<RenderPrimitive>`
3. Despawns old ECS entities for that tattva
4. Materializes new ECS entities from the primitives (uploading GPU buffers)
5. Caches the entity IDs for next frame

This means the GPU world is always a derived view of the frontend state — never the source of truth.
