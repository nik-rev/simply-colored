//! `simply_colored` is an incredibly simple solution for coloring text in the terminal.

#![no_std]

pub mod effects {
    /// Reset styling
    pub const OFF: &str = "\u{1b}[0m";
    /// Following text will be bold
    pub const BOLD: &str = "\u{1b}[1m";
    /// Following text will NOT be bold
    pub const NO_BOLD: &str = "\u{1b}[21m";
    /// Following text will be dim
    pub const DIM: &str = "\u{1b}[2m";
    /// Following text will NOT be dim
    pub const NO_DIM: &str = "\u{1b}[22m";

    /// Following text will be italic
    pub const ITALIC: &str = "\u{1b}[3m";
    /// Following text will NOT be dim
    pub const NO_ITALIC: &str = "\u{1b}[23m";
    /// Following text will be underlined
    pub const UNDERLINE: &str = "\u{1b}[4m";
    /// Following text will NOT be underlined
    pub const NO_UNDERLINE: &str = "\u{1b}[24m";

    /// Following text will be blinking
    pub const BLINK: &str = "\u{1b}[5m";
    /// Following text will NOT be blinking
    pub const NO_BLINK: &str = "\u{1b}[25m";
    /// Foreground and background for the following text will be reversed
    pub const REVERSE: &str = "\u{1b}[7m";
    /// Foreground and background for the following text will NOT be reversed
    pub const NO_REVERSE: &str = "\u{1b}[27m";

    /// Following text will be invisible
    pub const HIDE: &str = "\u{1b}[8m";
    /// Following text will be visible
    pub const NO_HIDE: &str = "\u{1b}[28m";
    /// Following text will be crossed out
    pub const CROSS: &str = "\u{1b}[8m";
    /// Following text will NOT be crossed out
    pub const NO_CROSS: &str = "\u{1b}[28m";
}

/// Set color of text to dim black
pub const DIM_BLACK: &str = "\u{1b}[30m";
/// Set background of text to dim black
pub const BG_DIM_BLACK: &str = "\u{1b}[40m";
/// Set color of text to black
pub const BLACK: &str = "\u{1b}[90m";
/// Set background of text to black
pub const BG_BLACK: &str = "\u{1b}[100m";

/// Set color of text to dim red
pub const DIM_RED: &str = "\u{1b}[31m";
/// Set background of text to dim red
pub const BG_DIM_RED: &str = "\u{1b}[41m";
/// Set color of text to red
pub const RED: &str = "\u{1b}[91m";
/// Set background of text to red
pub const BG_RED: &str = "\u{1b}[101m";

/// Set color of text to dim green
pub const DIM_GREEN: &str = "\u{1b}[32m";
/// Set background of text to dim green
pub const BG_DIM_GREEN: &str = "\u{1b}[42m";
/// Set color of text to green
pub const GREEN: &str = "\u{1b}[92m";
/// Set background of text to green
pub const BG_GREEN: &str = "\u{1b}[102m";

/// Set color of text to dim yellow
pub const DIM_YELLOW: &str = "\u{1b}[33m";
/// Set background of text to dim yellow
pub const BG_DIM_YELLOW: &str = "\u{1b}[43m";
/// Set color of text to yellow
pub const YELLOW: &str = "\u{1b}[93m";
/// Set background of text to yellow
pub const BG_YELLOW: &str = "\u{1b}[103m";

/// Set color of text to dim blue
pub const DIM_BLUE: &str = "\u{1b}[34m";
/// Set background of text to dim blue
pub const BG_DIM_BLUE: &str = "\u{1b}[44m";
/// Set color of text to blue
pub const BLUE: &str = "\u{1b}[94m";
/// Set background of text to blue
pub const BG_BLUE: &str = "\u{1b}[104m";

/// Set color of text to dim magenta
pub const DIM_MAGENTA: &str = "\u{1b}[35m";
/// Set background of text to dim magenta
pub const BG_DIM_MAGENTA: &str = "\u{1b}[45m";
/// Set color of text to magenta
pub const MAGENTA: &str = "\u{1b}[95m";
/// Set background of text to magenta
pub const BG_MAGENTA: &str = "\u{1b}[105m";

/// Set color of text to dim cyan
pub const DIM_CYAN: &str = "\u{1b}[36m";
/// Set background of text to dim cyan
pub const BG_DIM_CYAN: &str = "\u{1b}[46m";
/// Set color of text to cyan
pub const CYAN: &str = "\u{1b}[96m";
/// Set background of text to cyan
pub const BG_CYAN: &str = "\u{1b}[106m";

/// Set color of text to dim white
pub const DIM_WHITE: &str = "\u{1b}[37m";
/// Set background of text to dim white
pub const BG_DIM_WHITE: &str = "\u{1b}[47m";
/// Set color of text to white
pub const WHITE: &str = "\u{1b}[97m";
/// Set background of text to white
pub const BG_WHITE: &str = "\u{1b}[107m";

/// Set color of text to default
pub const DIM_DEFAULT: &str = "\u{1b}[39m";
/// Set background of text to default
pub const BG_DIM_DEFAULT: &str = "\u{1b}[49m";
/// Set color of text to default
pub const DEFAULT: &str = "\u{1b}[99m";
/// Set background of text to default
pub const BG_DEFAULT: &str = "\u{1b}[109m";
