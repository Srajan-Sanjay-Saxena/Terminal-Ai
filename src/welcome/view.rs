use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

use super::constants::{
    BORDER_COLOR, BOX_WIDTH, GRADIENT_END, GRADIENT_START, HEADING, SEPARATOR_WIDTH, TAGLINE, TIPS,
    TITLE,
};
use crate::ui::gradient::gradient_line;

/// Render the welcome screen.
pub fn draw(frame: &mut Frame, cwd: &str) {
    let dim = Style::new().fg(Color::DarkGray);
    let label = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);

    let mut lines: Vec<Line> = vec![
        Line::raw(""),
        Line::from(HEADING).style(dim),
        Line::raw(""),
        gradient_line(TITLE, GRADIENT_START, GRADIENT_END),
        Line::raw(""),
        Line::from(TAGLINE).italic().fg(Color::Gray),
        Line::raw(""),
        Line::from("─".repeat(SEPARATOR_WIDTH)).style(dim),
        Line::raw(""),
        Line::from(vec![
            Span::styled("Version  ", label),
            Span::raw(format!("v{}", env!("CARGO_PKG_VERSION"))),
        ]),
        Line::from(vec![
            Span::styled("Folder   ", label),
            Span::raw(cwd.to_string()),
        ]),
        Line::raw(""),
        Line::from("Tips".bold().fg(Color::Yellow)),
    ];
    lines.extend(TIPS.iter().map(|tip| Line::from(*tip).style(dim)));
    lines.push(Line::raw(""));
    lines.push(Line::from(vec![
        Span::raw("Press "),
        Span::styled(" Enter ", Style::new().black().on_cyan().bold()),
        Span::raw(" to start  ·  "),
        Span::styled(" q ", Style::new().black().on_gray().bold()),
        Span::raw(" to quit"),
    ]));
    lines.push(Line::raw(""));

    let height = lines.len() as u16 + 2; // +2 for the border
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::horizontal([Constraint::Length(BOX_WIDTH)])
        .flex(Flex::Center)
        .areas(area);

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(BORDER_COLOR));

    frame.render_widget(
        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .block(block),
        area,
    );
}
