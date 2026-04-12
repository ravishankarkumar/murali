# Agentic Flow Chart Animation Improvements

## Current State

The agentic flow chart currently supports:
- **RevealTo**: Sequential reveal of nodes/edges based on thresholds
- **PropagateSignal**: Animated flow through the graph showing active path
- **Node Arrivals**: Calculates exact times when nodes are visited in the flow

## Suggested Animation Enhancements

### 1. Sequential Drop Animation (Recommended)

**Concept**: Nodes drop from above with a bouncy landing effect, appearing one by one as they're revealed.

**Implementation Approach**:
```rust
// For each node, create a drop animation:
// 1. Start position: node_position + Vec3::new(0, drop_height, 0)
// 2. End position: node_position
// 3. Easing: OutBack (bouncy landing)
// 4. Timing: Stagger based on node reveal order

// Example timing:
// Node 0: drops at t=0.0
// Node 1: drops at t=0.3
// Node 2: drops at t=0.6
// Node 3: drops at t=0.9
// Node 4: drops at t=1.2
```

**Benefits**:
- Creates visual interest and engagement
- Clearly shows the order of node appearance
- Bouncy landing feels satisfying
- Can be combined with scale-in for emphasis

**Easing Recommendations**:
- `OutBack`: Bouncy landing (overshoots then settles)
- `OutElastic`: Springy effect
- `OutBounce`: Multiple bounces

### 2. Staggered Fade-In with Scale

**Concept**: Nodes fade in while scaling up from 0 to 1, with staggered timing.

**Implementation**:
```rust
// For each node:
// 1. Start: opacity=0, scale=0.3
// 2. End: opacity=1, scale=1.0
// 3. Duration: 0.4s per node
// 4. Stagger: 0.2s between nodes
```

**Benefits**:
- Smoother than instant appearance
- Scale-up emphasizes importance
- Less jarring than drop animation
- Works well with reveal_progress

### 3. Pulse/Highlight on Flow Arrival

**Concept**: When the flow reaches a node, it pulses or glows to highlight the current step.

**Current Support**:
- `indicate_scale` already creates a pulsing effect
- `indicate_color` controls the highlight color
- `indicate_window` controls the pulse timing window

**Enhancement Ideas**:
- Add color transition when flow arrives
- Increase pulse intensity during active flow
- Add glow effect (outer ring that expands)

### 4. Edge Animation Improvements

**Concept A - Edge Draw Animation**:
- Edges draw themselves as they're revealed
- Similar to write effect for paths
- Creates a "drawing" effect

**Concept B - Animated Dashes**:
- Active edges have animated dashes moving along them
- Creates sense of flow/energy
- Can pulse or change color

**Concept C - Edge Glow**:
- Edges glow when flow passes through
- Glow intensity follows the flow progress
- Creates visual feedback of active path

### 5. Rotation/Tilt on Appearance

**Concept**: Nodes rotate slightly as they appear, then settle to normal rotation.

**Implementation**:
```rust
// For each node:
// 1. Start: rotation = 15 degrees
// 2. End: rotation = 0 degrees
// 3. Easing: OutQuad
// 4. Duration: 0.3s
```

**Benefits**:
- Adds personality to animations
- Creates sense of motion
- Works well with drop animation

### 6. Sequential Edge Reveal

**Concept**: Edges appear in sequence, following the flow path order.

**Implementation**:
- Use `reveal_at` on edges to control timing
- Edges appear just before their destination node
- Creates a "drawing" effect of the flow path

## Recommended Implementation Strategy

### Phase 1: Basic Drop Animation
1. Modify `RevealTo` animation to support position changes
2. Calculate drop height based on node size
3. Use `OutBack` easing for bouncy effect
4. Stagger timing based on node reveal order

### Phase 2: Enhanced Feedback
1. Add color transitions on flow arrival
2. Enhance pulse effect with glow
3. Add edge animations (dashes or glow)

### Phase 3: Advanced Effects
1. Rotation on appearance
2. Edge draw animation
3. Sound effects (optional)

## Code Examples

### Example 1: Drop Animation Setup
```rust
// In timeline setup:
let drop_height = 3.0;
let stagger_time = 0.3;

for (idx, node) in nodes.iter().enumerate() {
    let drop_start = reveal_start + (idx as f32 * stagger_time);
    let drop_duration = 0.6;
    
    // Create individual node tattva for animation
    let node_id = scene.add_tattva(node.clone(), node_position);
    
    // Animate drop
    timeline
        .animate(node_id)
        .at(drop_start)
        .for_duration(drop_duration)
        .ease(Ease::OutBack)
        .move_to(node_position)  // from above
        .spawn();
}
```

### Example 2: Pulse on Arrival
```rust
// Use node_arrivals to get exact times
let arrivals = chart.node_arrivals(flow_start, flow_duration);

for arrival in arrivals {
    timeline.call_at(arrival.time, move |scene| {
        // Trigger pulse effect
        // Could scale up then down
        // Or change color temporarily
    });
}
```

## Performance Considerations

- **Current**: RevealTo + PropagateSignal = 2 animations
- **With Drops**: Add 5 MoveTo animations (one per node)
- **Total**: ~7 animations running
- **Impact**: Minimal (well within budget)

## Testing Recommendations

1. Test with different node counts (5, 10, 20 nodes)
2. Verify timing accuracy with long flow paths
3. Check visual smoothness at different frame rates
4. Combine with other effects (edges, labels)

## Future Enhancements

- Particle effects on drop landing
- Sound effects synchronized with animations
- Custom animation curves per node
- Interactive animations (click to trigger)
- Reverse animations (nodes disappear)
