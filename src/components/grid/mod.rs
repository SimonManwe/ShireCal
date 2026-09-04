use ratatui::{
    layout::{Alignment, Constraint, Layout, Spacing},
    style::Style,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

mod column;

#[derive(Debug)]
pub struct Grid {
    cols: u16,
    rows: u16,
}

impl Grid {
    pub fn new() -> Self {
        Self { cols: 7, rows: 24 }
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

        for chunk in chunks.iter() {
            let column_layout = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Fill(1)])
                .spacing(Spacing::Overlap(1));

            let [title_area, content_area] = column_layout.areas(*chunk);

            let centered_title_area = title_area.centered_vertically(Constraint::Length(1));

            Paragraph::new(format!("Montag"))
                .block(
                    Block::default()
                        .borders(Borders::NONE)
                        .padding(Padding::horizontal(2)),
                )
                .alignment(Alignment::Right)
                .render(centered_title_area, buf);

            let column_cells = vertical.split(content_area);

            for (i, cell) in column_cells.iter().enumerate() {
                Block::new()
                    .borders(Borders::ALL)
                    .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact)
                    .render(*cell, buf);
            }
        }
    }
}
