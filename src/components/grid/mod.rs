use ratatui::{
    layout::{Alignment, Constraint, Layout, Spacing},
    style::Style,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

mod column;

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
            cols: 7,
            rows: 24,
            selected_cell: CellCoordinates {
                x_coordinate: 1,
                y_coordinate: 1,
            },
        }
    }

    fn get_weekday_from_index(&self, weekday_idx: usize) -> String {
        return match weekday_idx {
            0 => String::from("Montag"),
            1 => String::from("Dienstag"),
            2 => String::from("Mittwoch"),
            3 => String::from("Donnnerstag"),
            4 => String::from("Freitag"),
            5 => String::from("Samstag"),
            6 => String::from("Sonntag"),
            _ => String::from(""),
        };
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
            .spacing(Spacing::Overlap(1))
            .split(inner_area);

        let row_constraints = (0..self.rows).map(|_| Constraint::Length(3));
        let vertical = Layout::vertical(row_constraints).spacing(Spacing::Overlap(1));

        for (col_index, chunk) in chunks.iter().enumerate() {
            let column_layout = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Fill(1)])
                .spacing(Spacing::Overlap(1));

            let [title_area, content_area] = column_layout.areas(*chunk);

            let centered_title_area = title_area.centered_vertically(Constraint::Length(1));

            let col_title = self.get_weekday_from_index(col_index);
            Paragraph::new(col_title)
                .block(
                    Block::default()
                        .borders(Borders::NONE)
                        .padding(Padding::horizontal(2)),
                )
                .alignment(Alignment::Right)
                .render(centered_title_area, buf);

            let column_cells = vertical.split(content_area);

            for (row_index, cell) in column_cells.iter().enumerate() {
                let is_selected = self.is_cell_selected(col_index, row_index);

                Block::new()
                    .borders(Borders::ALL)
                    .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact)
                    .render(*cell, buf);
            }
        }
    }
}
