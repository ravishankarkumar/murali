# TracedPath Debugging Guide

## Problem
The traced path for a rolling circle is showing a straight horizontal line instead of a cycloid curve.

## Expected Behavior
When a circle rolls from left to right while rotating, a point on its circumference should trace a cycloid curve - a wavy path that goes up and down as the point rotates around the circle's center.

## Current Behavior
The traced path is a straight horizontal line at y = -0.6 (the bottom of the circle).

## Analysis
The straight line suggests that the Y-coordinate of the traced point is constant, which means:
1. Either the rotation isn't being applied to the local point
2. Or the rotation value in props is always identity (no rotation)
3. Or there's an issue with how quaternion rotation is applied to vectors

## Code Flow
1. `Scene::update()` calls `update_traced_paths()`
2. `update_traced_paths()` reads the circle's position and rotation from props
3. The point function is called with `(circle_pos, circle_rot)`
4. The point function should:
   - Take a local point (0, -radius, 0) - bottom of circle
   - Rotate it by circle_rot
   - Add to circle_pos to get world position

## Potential Issues
- The rotation animation might not be updating props.rotation
- The quaternion multiplication might not be working as expected
- The update might be happening before the animation starts

## Next Steps
- Add logging to verify rotation values are changing
- Test quaternion rotation separately
- Check if the issue is with timing (animation not started yet)
