use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect, Spacing},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

#[derive(Debug)]
struct CellCoordinates {
    x_coordinate: usize,
    y_coordinate: usize,
}

impl CellCoordinates {
    pub fn set_x(&mut self, x: usize) {
        self.x_coordinate = x;
    }

    pub fn set_y(&mut self, y: usize) {
        self.y_coordinate = y;
    }
}

#[derive(Debug)]
pub struct Grid {
    cols: u16,
    rows: u16,
    selected_cell: CellCoordinates,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cols: 5,
            rows: 10,
            selected_cell: CellCoordinates {
                x_coordinate: 0,
                y_coordinate: 0,
            },
        }
    }

    fn is_cell_selected(&self, cell_x: usize, cell_y: usize) -> bool {
        if cell_x == self.selected_cell.x_coordinate && cell_y == self.selected_cell.y_coordinate {
            return true;
        }

        return false;
    }
}

impl Widget for Grid {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default());
        let inner_area = block.inner(area);

        block.render(area, buf);

        let col_constraints = (0..self.cols).map(|_| Constraint::Length(30));

        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(col_constraints.clone())
            .split(inner_area);

        let row_constraints = (0..self.rows).map(|_| Constraint::Length(3));
        let vertical = Layout::vertical(row_constraints);

        let mut selected_cell: Option<Rect> = None;

        for (col_index, chunk) in chunks.iter().enumerate() {
            let column_cells = vertical.split(*chunk);

            for (row_index, cell) in column_cells.iter().enumerate() {
                let is_selected = self.is_cell_selected(col_index, row_index);

                if !is_selected {
                    Block::new()
                        .borders(Borders::ALL)
                        .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact)
                        .render(*cell, buf);
                } else {
                    selected_cell = Some(*cell);
                }
            }
        }

        if let Some(selected_area) = selected_cell {
            Block::new()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Thick)
                .border_style(Style::new().light_blue())
                .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact)
                .bg(Color::DarkGray)
                .render(selected_area, buf);
        }
    }
}
