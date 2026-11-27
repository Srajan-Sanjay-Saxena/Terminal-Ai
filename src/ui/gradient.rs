use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub type Rgb = (u8, u8, u8);

/// Bold text with a per-character horizontal color gradient from `start` to `end`.
pub fn gradient_line(text: &str, start: Rgb, end: Rgb) -> Line<'static> {
    let last = text.chars().count().saturating_sub(1).max(1) as f32;
    let lerp = |a: u8, b: u8, t: f32| (a as f32 + (b as f32 - a as f32) * t) as u8;

    let spans: Vec<Span<'static>> = text
        .chars()
        .enumerate()
        .map(|(i, ch)| {
            let t = i as f32 / last;
            let color = Color::Rgb(
                lerp(start.0, end.0, t),
                lerp(start.1, end.1, t),
                lerp(start.2, end.2, t),
            );
            Span::styled(
                ch.to_string(),
                Style::new().fg(color).add_modifier(Modifier::BOLD),
            )
        })
        .collect();

    Line::from(spans)
}
