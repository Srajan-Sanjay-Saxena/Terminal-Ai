use clap::{Parser, Subcommand};

/// Terminal-AI: an AI coding agent in your terminal.
#[derive(Parser, Debug)]
#[command(name = "terminal-ai", version, about)]
pub struct Cli {
    /// Skip the welcome screen and go straight to the session.
    #[arg(long, env = "TERMINAL_AI_NO_WELCOME")]
    pub no_welcome: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start an interactive chat session (default).
    Chat,
}
