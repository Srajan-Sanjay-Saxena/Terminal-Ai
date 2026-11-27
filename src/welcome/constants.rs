use crate::ui::gradient::Rgb;
use ratatui::style::Color;

// Text
pub const HEADING: &str = "✦  W E L C O M E   T O  ✦";
pub const TITLE: &str = "T E R M I N A L - A I";
pub const TAGLINE: &str = "Your AI coding agent, right in the terminal";
pub const TIPS: [&str; 3] = [
    "• Describe a task and press Enter",
    "• Use /help to see commands",
    "• Ctrl+C quits at any time",
];

// Layout
pub const BOX_WIDTH: u16 = 70;
pub const SEPARATOR_WIDTH: usize = 44;

// Colors
pub const GRADIENT_START: Rgb = (0, 220, 255); // cyan
pub const GRADIENT_END: Rgb = (200, 90, 255); // purple
pub const BORDER_COLOR: Color = Color::Rgb(120, 140, 255);
