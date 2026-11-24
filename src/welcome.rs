use anyhow::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{Alignment, Constraint, Flex, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

const TITLE: &str = "T E R M I N A L - A I";
const TAGLINE: &'static str = "Your AI coding agent, right in the terminal";
const GRADIENT_START: (u8, u8, u8) = (0, 220, 255); // cyan
const GRADIENT_END: (u8, u8, u8) = (200, 90, 255); // purple

/// What the user chose on the welcome screen.
pub enum Outcome {
    Start,
    Quit,
}

/// Show the welcome screen until the user starts or quits.
pub fn run(terminal: &mut DefaultTerminal) -> Result<Outcome> {
    let cwd = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "?".into());

    loop {
        terminal.draw(|f| draw(f, &cwd))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match (key.code, key.modifiers) {
                (KeyCode::Enter, _) => return Ok(Outcome::Start),
                (KeyCode::Char('q') | KeyCode::Esc, _) => return Ok(Outcome::Quit),
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Ok(Outcome::Quit),
                _ => {}
            }
        }
    }
}

fn draw(frame: &mut Frame, cwd: &str) {
    let dim = Style::new().fg(Color::DarkGray);
    let label = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);

    let lines: Vec<Line> = vec![
        Line::raw(""),
        Line::from("✦  W E L C O M E   T O  ✦").style(dim),
        Line::raw(""),
        gradient_line(TITLE),
        Line::raw(""),
        Line::from(TAGLINE).italic().fg(Color::Gray),
        Line::raw(""),
        Line::from("─".repeat(44)).style(dim),
        Line::raw(""),
        Line::from(vec![
            Span::styled("Version  ", label),
            Span::raw(format!("v{}", env!("CARGO_PKG_VERSION"))),
        ]),
        Line::from(vec![Span::styled("Folder   ", label), Span::raw(cwd.to_string())]),
        Line::raw(""),
        Line::from("Tips".bold().fg(Color::Yellow)),
        Line::from("• Describe a task and press Enter").style(dim),
        Line::from("• Use /help to see commands").style(dim),
        Line::from("• Ctrl+C quits at any time").style(dim),
        Line::raw(""),
        Line::from(vec![
            Span::raw("Press "),
            Span::styled(" Enter ", Style::new().black().on_cyan().bold()),
            Span::raw(" to start  ·  "),
            Span::styled(" q ", Style::new().black().on_gray().bold()),
            Span::raw(" to quit"),
        ]),
        Line::raw(""),
    ];

    let height = lines.len() as u16 + 2;
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::horizontal([Constraint::Length(70)])
        .flex(Flex::Center)
        .areas(area);

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::Rgb(120, 140, 255)));

    frame.render_widget(
        Paragraph::new(lines).alignment(Alignment::Center).block(block),
        area,
    );
}

/// Bold text with a per-character horizontal color gradient.
fn gradient_line(text: &str) -> Line<'static> {
    let n = text.chars().count().saturating_sub(1).max(1) as f32;
    let lerp = |a: u8, b: u8, t: f32| (a as f32 + (b as f32 - a as f32) * t) as u8;
    let spans = text.chars().enumerate().map(|(i, ch)| {
        let t = i as f32 / n;
        let color = Color::Rgb(
            lerp(GRADIENT_START.0, GRADIENT_END.0, t),
            lerp(GRADIENT_START.1, GRADIENT_END.1, t),
            lerp(GRADIENT_START.2, GRADIENT_END.2, t),
        );
        Span::styled(ch.to_string(), Style::new().fg(color).add_modifier(Modifier::BOLD))
    });
    Line::from(spans.collect::<Vec<_>>())
}
