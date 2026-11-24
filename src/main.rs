mod cli;
mod welcome;

use anyhow::Result;
use clap::Parser;

use cli::Cli;
use welcome::Outcome;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.no_welcome {
        // ratatui::init() enters the alternate screen and installs a panic hook
        // that restores the terminal.
        let mut terminal = ratatui::init();
        let outcome = welcome::run(&mut terminal);
        ratatui::restore();

        if matches!(outcome?, Outcome::Quit) {
            return Ok(());
        }
    }

    // TODO: start the chat session UI here.
    println!("Starting Terminal-AI session…");
    Ok(())
}
