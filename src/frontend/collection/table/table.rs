use glam::{Vec2, Vec3, Vec4};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use crate::frontend::layout::{Bounded, Bounds};

/// Represents a single cell in a table
#[derive(Clone, Debug)]
pub struct TableCell {
    pub content: String,
    pub width: f32,
    pub height: f32,
}

impl TableCell {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            width: 1.0,
            height: 0.5,
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

/// Configuration for table styling
#[derive(Clone, Debug)]
pub struct TableConfig {
    pub h_buff: f32,           // Horizontal buffer between cells
    pub v_buff: f32,           // Vertical buffer between cells
    pub line_color: Vec4,      // Color of grid lines
    pub line_thickness: f32,   // Thickness of grid lines
    pub text_color: Vec4,      // Color of text
    pub text_height: f32,      // Height of text
    pub include_outer_lines: bool,
    pub background_color: Option<Vec4>,
    pub labels_inside: bool,   // Whether to include row/col labels inside the table grid
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            h_buff: 0.2,
            v_buff: 0.2,
            line_color: Vec4::new(0.8, 0.8, 0.8, 1.0),
            line_thickness: 0.02,
            text_color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            text_height: 0.3,
            include_outer_lines: true,
            background_color: None,
            labels_inside: false,
        }
    }
}

/// A table mobject similar to Manim's Table
/// Displays a 2D grid of cells with configurable styling
#[derive(Clone, Debug)]
pub struct Table {
    pub rows: Vec<Vec<TableCell>>,
    pub row_labels: Option<Vec<String>>,
    pub col_labels: Option<Vec<String>>,
    pub config: TableConfig,
    pub write_progress: f32,  // 0.0 to 1.0 for animation
}

impl Table {
    /// Create a new table from a 2D array of strings
    pub fn new(data: Vec<Vec<impl Into<String>>>) -> Self {
        let rows = data
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|cell| TableCell::new(cell))
                    .collect()
            })
            .collect();

        Self {
            rows,
            row_labels: None,
            col_labels: None,
            config: TableConfig::default(),
            write_progress: 1.0,  // Fully visible by default
        }
    }

    /// Add row labels
    pub fn with_row_labels(mut self, labels: Vec<impl Into<String>>) -> Self {
        self.row_labels = Some(labels.into_iter().map(|l| l.into()).collect());
        self
    }

    /// Add column labels
    pub fn with_col_labels(mut self, labels: Vec<impl Into<String>>) -> Self {
        self.col_labels = Some(labels.into_iter().map(|l| l.into()).collect());
        self
    }

    /// Configure table styling
    pub fn with_config(mut self, config: TableConfig) -> Self {
        self.config = config;
        self
    }

    /// Set horizontal buffer
    pub fn with_h_buff(mut self, h_buff: f32) -> Self {
        self.config.h_buff = h_buff;
        self
    }

    /// Set vertical buffer
    pub fn with_v_buff(mut self, v_buff: f32) -> Self {
        self.config.v_buff = v_buff;
        self
    }

    /// Set line color
    pub fn with_line_color(mut self, color: Vec4) -> Self {
        self.config.line_color = color;
        self
    }

    /// Set line thickness
    pub fn with_line_thickness(mut self, thickness: f32) -> Self {
        self.config.line_thickness = thickness;
        self
    }

    /// Set text color
    pub fn with_text_color(mut self, color: Vec4) -> Self {
        self.config.text_color = color;
        self
    }

    /// Set text height
    pub fn with_text_height(mut self, height: f32) -> Self {
        self.config.text_height = height;
        self
    }

    /// Include outer lines
    pub fn with_outer_lines(mut self, include: bool) -> Self {
        self.config.include_outer_lines = include;
        self
    }

    /// Set background color
    pub fn with_background_color(mut self, color: Vec4) -> Self {
        self.config.background_color = Some(color);
        self
    }

    /// Include row/column labels inside the table grid
    pub fn with_labels_inside(mut self, inside: bool) -> Self {
        self.config.labels_inside = inside;
        self
    }

    /// Set write progress for animation (0.0 to 1.0)
    pub fn with_write_progress(mut self, progress: f32) -> Self {
        self.write_progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Get the number of rows
    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }

    /// Get the number of columns
    pub fn num_cols(&self) -> usize {
        self.rows.first().map(|r| r.len()).unwrap_or(0)
    }

    /// Get cell at (row, col) - 1-indexed like Manim
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&TableCell> {
        if row > 0 && col > 0 {
            self.rows.get(row - 1)?.get(col - 1)
        } else {
            None
        }
    }

    /// Calculate total table dimensions
    fn calculate_dimensions(&self) -> (f32, f32) {
        let num_rows = self.num_rows();
        let num_cols = self.num_cols();

        let mut total_width = 0.0;
        let mut total_height = 0.0;

        // Calculate width - use fixed cell widths for simplicity
        let cell_width = 1.2;
        let cell_height = 0.6;

        total_width = (num_cols as f32) * cell_width + (num_cols as f32 - 1.0) * self.config.h_buff;

        // Add space for row labels if present
        if self.row_labels.is_some() {
            total_width += 1.5 + self.config.h_buff;
        }

        // Calculate height
        total_height = (num_rows as f32) * cell_height + (num_rows as f32 - 1.0) * self.config.v_buff;

        // Add space for column labels if present
        if self.col_labels.is_some() {
            total_height += cell_height + self.config.v_buff;
        }

        (total_width, total_height)
    }

}

impl Project for Table {
    fn project(&self, ctx: &mut ProjectionCtx) {
        if self.write_progress <= 0.0 {
            return; // Nothing to draw
        }

        let cell_width = 1.2;
        let cell_height = 0.6;
        let num_rows = self.num_rows();
        let num_cols = self.num_cols();

        // Calculate grid dimensions based on labels_inside setting
        let (grid_cols, grid_rows) = if self.config.labels_inside {
            let cols = num_cols + if self.row_labels.is_some() { 1 } else { 0 };
            let rows = num_rows + if self.col_labels.is_some() { 1 } else { 0 };
            (cols, rows)
        } else {
            (num_cols, num_rows)
        };

        // Calculate total grid size
        let grid_width = (grid_cols as f32) * (cell_width + self.config.h_buff);
        let grid_height = (grid_rows as f32) * (cell_height + self.config.v_buff);

        // Calculate starting position (centered at origin)
        let start_x = -grid_width / 2.0;
        let start_y = grid_height / 2.0;

        // Calculate how many elements to show based on write_progress
        // Animation phases: 
        // 0.0 - 0.5: Draw grid lines (horizontal then vertical)
        // 0.5 - 1.0: Write text content
        
        let total_h_lines = grid_rows + 1;
        let total_v_lines = grid_cols + 1;
        let total_lines = total_h_lines + total_v_lines;
        
        let line_progress = (self.write_progress * 2.0).min(1.0); // 0.0 to 0.5 maps to 0.0 to 1.0
        let text_progress = ((self.write_progress - 0.5) * 2.0).max(0.0); // 0.5 to 1.0 maps to 0.0 to 1.0
        
        let lines_to_draw = (line_progress * total_lines as f32) as usize;

        // Draw horizontal lines
        let h_lines_to_draw = lines_to_draw.min(total_h_lines);
        for row in 0..h_lines_to_draw {
            let y = start_y - (row as f32) * (cell_height + self.config.v_buff);
            ctx.emit(RenderPrimitive::Line {
                start: Vec3::new(start_x, y, 0.0),
                end: Vec3::new(start_x + grid_width, y, 0.0),
                thickness: self.config.line_thickness,
                color: self.config.line_color,
                dash_length: 0.0,
                gap_length: 0.0,
                dash_offset: 0.0,
            });
        }

        // Draw vertical lines
        if lines_to_draw > total_h_lines {
            let v_lines_to_draw = (lines_to_draw - total_h_lines).min(total_v_lines);
            for col in 0..v_lines_to_draw {
                let x = start_x + (col as f32) * (cell_width + self.config.h_buff);
                ctx.emit(RenderPrimitive::Line {
                    start: Vec3::new(x, start_y, 0.0),
                    end: Vec3::new(x, start_y - grid_height, 0.0),
                    thickness: self.config.line_thickness,
                    color: self.config.line_color,
                    dash_length: 0.0,
                    gap_length: 0.0,
                    dash_offset: 0.0,
                });
            }
        }

        // Only draw text if we're past the line drawing phase
        if text_progress > 0.0 {
            let total_cells = if self.config.labels_inside {
                let label_cells = (if self.col_labels.is_some() { grid_cols } else { 0 })
                    + (if self.row_labels.is_some() { grid_rows - if self.col_labels.is_some() { 1 } else { 0 } } else { 0 });
                label_cells + (num_rows * num_cols)
            } else {
                let label_cells = (if self.col_labels.is_some() { num_cols } else { 0 })
                    + (if self.row_labels.is_some() { num_rows } else { 0 });
                label_cells + (num_rows * num_cols)
            };

            let cells_to_show = (text_progress * total_cells as f32) as usize;
            let mut cell_count = 0;

            if self.config.labels_inside {
                let row_offset = if self.col_labels.is_some() { 1 } else { 0 };
                let col_offset = if self.row_labels.is_some() { 1 } else { 0 };

                // Draw column labels
                if let Some(labels) = &self.col_labels {
                    for (idx, label) in labels.iter().enumerate() {
                        if cell_count >= cells_to_show { break; }
                        let x = start_x + ((idx + col_offset) as f32) * (cell_width + self.config.h_buff) + (cell_width + self.config.h_buff) / 2.0;
                        let y = start_y - (cell_height + self.config.v_buff) / 2.0;
                        ctx.emit(RenderPrimitive::Text {
                            content: label.clone(),
                            height: self.config.text_height * 0.8,
                            color: self.config.text_color,
                            offset: Vec3::new(x, y, 0.0),
                        });
                        cell_count += 1;
                    }
                }

                // Draw row labels
                if let Some(labels) = &self.row_labels {
                    for (idx, label) in labels.iter().enumerate() {
                        if cell_count >= cells_to_show { break; }
                        let x = start_x + (cell_width + self.config.h_buff) / 2.0;
                        let y = start_y - ((idx + row_offset) as f32) * (cell_height + self.config.v_buff) - (cell_height + self.config.v_buff) / 2.0;
                        ctx.emit(RenderPrimitive::Text {
                            content: label.clone(),
                            height: self.config.text_height * 0.8,
                            color: self.config.text_color,
                            offset: Vec3::new(x, y, 0.0),
                        });
                        cell_count += 1;
                    }
                }

                // Draw cell content
                for (row_idx, row) in self.rows.iter().enumerate() {
                    for (col_idx, cell) in row.iter().enumerate() {
                        if cell_count >= cells_to_show { break; }
                        let x = start_x + ((col_idx + col_offset) as f32) * (cell_width + self.config.h_buff) + (cell_width + self.config.h_buff) / 2.0;
                        let y = start_y - ((row_idx + row_offset) as f32) * (cell_height + self.config.v_buff) - (cell_height + self.config.v_buff) / 2.0;
                        ctx.emit(RenderPrimitive::Text {
                            content: cell.content.clone(),
                            height: self.config.text_height * 0.8,
                            color: self.config.text_color,
                            offset: Vec3::new(x, y, 0.0),
                        });
                        cell_count += 1;
                    }
                }
            } else {
                // Draw column labels (outside grid, above)
                if let Some(labels) = &self.col_labels {
                    for (idx, label) in labels.iter().enumerate() {
                        if cell_count >= cells_to_show { break; }
                        let x = start_x + (idx as f32) * (cell_width + self.config.h_buff) + (cell_width + self.config.h_buff) / 2.0;
                        let y = start_y + (cell_height + self.config.v_buff) / 2.0;
                        ctx.emit(RenderPrimitive::Text {
                            content: label.clone(),
                            height: self.config.text_height * 0.8,
                            color: self.config.text_color,
                            offset: Vec3::new(x, y, 0.0),
                        });
                        cell_count += 1;
                    }
                }

                // Draw row labels (outside grid, left)
                if let Some(labels) = &self.row_labels {
                    for (idx, label) in labels.iter().enumerate() {
                        if cell_count >= cells_to_show { break; }
                        let x = start_x - (cell_width + self.config.h_buff) / 2.0;
                        let y = start_y - (idx as f32) * (cell_height + self.config.v_buff) - (cell_height + self.config.v_buff) / 2.0;
                        ctx.emit(RenderPrimitive::Text {
                            content: label.clone(),
                            height: self.config.text_height * 0.8,
                            color: self.config.text_color,
                            offset: Vec3::new(x, y, 0.0),
                        });
                        cell_count += 1;
                    }
                }

                // Draw cell content
                for (row_idx, row) in self.rows.iter().enumerate() {
                    for (col_idx, cell) in row.iter().enumerate() {
                        if cell_count >= cells_to_show { break; }
                        let x = start_x + (col_idx as f32) * (cell_width + self.config.h_buff) + (cell_width + self.config.h_buff) / 2.0;
                        let y = start_y - (row_idx as f32) * (cell_height + self.config.v_buff) - (cell_height + self.config.v_buff) / 2.0;
                        ctx.emit(RenderPrimitive::Text {
                            content: cell.content.clone(),
                            height: self.config.text_height * 0.8,
                            color: self.config.text_color,
                            offset: Vec3::new(x, y, 0.0),
                        });
                        cell_count += 1;
                    }
                }
            }
        }
    }
}

impl Bounded for Table {
    fn local_bounds(&self) -> Bounds {
        let (width, height) = self.calculate_dimensions();
        Bounds {
            min: Vec2::new(-width / 2.0, -height / 2.0),
            max: Vec2::new(width / 2.0, height / 2.0),
        }
    }
}
