mod constants;
mod view;

use anyhow::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
};

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
        terminal.draw(|f| view::draw(f, &cwd))?;

        if let Event::Key(key) = event::read()?
            && let Some(outcome) = handle_key(key)
        {
            return Ok(outcome);
        }
    }
}

fn handle_key(key: KeyEvent) -> Option<Outcome> {
    if key.kind != KeyEventKind::Press {
        return None;
    }
    match (key.code, key.modifiers) {
        (KeyCode::Enter, _) => Some(Outcome::Start),
        (KeyCode::Char('q') | KeyCode::Esc, _) => Some(Outcome::Quit),
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => Some(Outcome::Quit),
        _ => None,
    }
}
