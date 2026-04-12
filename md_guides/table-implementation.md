# Table Implementation - Manim-style

## Overview

Murali now includes a Manim-style `Table` implementation for displaying structured data in grid format. Tables support rows, columns, labels, and customizable styling.

## Features

- **2D Grid Layout**: Organize data in rows and columns
- **Row & Column Labels**: Optional labels for rows and columns
- **Customizable Styling**: 
  - Line color and thickness
  - Text color and height
  - Horizontal and vertical spacing (buffers)
  - Outer lines toggle
  - Background color support
- **Flexible Cell Content**: String-based content for cells
- **Manim-compatible API**: Similar to Manim's Table class

## Basic Usage

```rust
use murali::frontend::collection::table::Table;
use glam::Vec4;

// Create a simple table
let table = Table::new(vec![
    vec!["Name", "Age", "City"],
    vec!["Alice", "28", "NYC"],
    vec!["Bob", "34", "LA"],
]);

// Add labels
let table = table
    .with_col_labels(vec!["Name", "Age", "City"])
    .with_row_labels(vec!["Person 1", "Person 2"]);

// Customize styling
let table = table
    .with_line_color(Vec4::new(0.44, 0.84, 0.71, 1.0))
    .with_text_color(Vec4::new(0.96, 0.98, 0.99, 1.0))
    .with_text_height(0.25)
    .with_h_buff(0.3)
    .with_v_buff(0.2)
    .with_outer_lines(true);
```

## API Reference

### Constructor

```rust
Table::new(data: Vec<Vec<impl Into<String>>>) -> Self
```

Creates a table from a 2D array of strings.

### Configuration Methods

- `.with_row_labels(labels: Vec<impl Into<String>>) -> Self` - Add row labels
- `.with_col_labels(labels: Vec<impl Into<String>>) -> Self` - Add column labels
- `.with_h_buff(h_buff: f32) -> Self` - Set horizontal buffer between cells
- `.with_v_buff(v_buff: f32) -> Self` - Set vertical buffer between cells
- `.with_line_color(color: Vec4) -> Self` - Set grid line color
- `.with_line_thickness(thickness: f32) -> Self` - Set grid line thickness
- `.with_text_color(color: Vec4) -> Self` - Set text color
- `.with_text_height(height: f32) -> Self` - Set text height
- `.with_outer_lines(include: bool) -> Self` - Toggle outer lines
- `.with_background_color(color: Vec4) -> Self` - Set background color

### Query Methods

- `.num_rows() -> usize` - Get number of rows
- `.num_cols() -> usize` - Get number of columns
- `.get_cell(row: usize, col: usize) -> Option<&TableCell>` - Get cell (1-indexed like Manim)

## Example

See `examples/table_showcase.rs` for a complete example with multiple tables.

## Rendering

Tables are rendered as:
1. Grid lines (horizontal and vertical)
2. Cell content (text)
3. Row labels (if provided)
4. Column labels (if provided)

## Differences from Manim

- Murali tables use 1-indexed cell access (like Manim) via `get_cell(row, col)`
- Currently supports string content only (Manim supports arbitrary Mobjects)
- No background rectangles per cell yet (planned feature)
- No cell highlighting methods yet (planned feature)

## Future Enhancements

- [ ] Per-cell background colors
- [ ] Cell highlighting/selection
- [ ] Arbitrary Mobject content in cells
- [ ] Write animation for table rows/columns
- [ ] Table morphing between different data
- [ ] Gradient fills for cells
