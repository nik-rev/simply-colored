//! [![crates.io](https://img.shields.io/crates/v/simply_colored?style=flat-square&logo=rust)](https://crates.io/crates/simply_colored)
//! [![docs.rs](https://img.shields.io/badge/docs.rs-simply_colored-blue?style=flat-square&logo=docs.rs)](https://docs.rs/simply_colored)
//! ![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)
//! ![msrv](https://img.shields.io/badge/msrv-1.17-blue?style=flat-square&logo=rust)
//! [![github](https://img.shields.io/github/stars/nik-rev/simply-colored)](https://github.com/nik-rev/simply-colored)
//!
//! This crate is the simplest yet ergonomic way to add color to your terminal.
//!
//! ```toml
//! [dependencies]
//! simply-colored = "0.1"
//! ```
//!
//! All this crate contains is a few dozen `const`ants corresponding to particular ANSI escape codes.
//!
//! # Usage
//!
//! ```rust
//! use simply_colored::*;
//!
//! println!("{BLUE}{BOLD}Simply colored!")
//! ```
//!
//! ## Foreground
//!
//! | Color                                                                                                                | Type                                   | To get                                                                                                                                                                             |
//! | -----                                                                                                                | ------------------                     | -------                                                                                                                                                                            |
//! | ![Green](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_green_16x16.png)                | `{GREEN}Simply colored!`                 | ![Green text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_green_164x16.png)                                                     |
//! | ![Yellow](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_yellow_16x16.png)              | `{YELLOW}Simply colored!`                | ![Yellow text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_yellow_164x16.png)                                                   |
//! | ![Red](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_red_16x16.png)                    | `{RED}Simply colored!`                   | ![Red text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_red_164x16.png)                                                         |
//! | ![Magenta](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_magenta_16x16.png)            | `{MAGENTA}Simply colored!`               | ![Magenta text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_magenta_164x16.png)                                                 |
//! | ![Blue](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_blue_16x16.png)                  | `{BLUE}Simply colored!`                  | ![Blue text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_blue_164x16.png)                                                       |
//! | ![Cyan](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_cyan_16x16.png)                  | `{CYAN}Simply colored!`                  | ![Cyan text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_cyan_164x16.png)                                                       |
//! | ![White](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_white_16x16.png)                | `{WHITE}Simply colored!`                 | ![White text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_white_164x16.png)                                                     |
//! | ![Black](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_black_16x16.png)                | `{BLACK}Simply colored!`                 | ![Black text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_black_164x16.png)                                                     |
//! | ![Dim green](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_green_16x16.png)        | `{DIM_GREEN}Simply colored!`             | ![Dim green text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_green_164x16.png)                                             |
//! | ![Dim yellow](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_yellow_16x16.png)      | `{DIM_YELLOW}Simply colored!`            | ![Dim yellow text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_yellow_164x16.png)                                           |
//! | ![Dim red](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_red_16x16.png)            | `{DIM_RED}Simply colored!`               | ![Dim red text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_red_164x16.png)                                                 |
//! | ![Dim magenta](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_magenta_16x16.png)    | `{DIM_MAGENTA}Simply colored!`           | ![Dim magenta text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_magenta_164x16.png)                                         |
//! | ![Dim blue](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_blue_16x16.png)          | `{DIM_BLUE}Simply colored!`              | ![Dim blue text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_blue_164x16.png)                                               |
//! | ![Dim cyan](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_cyan_16x16.png)          | `{DIM_CYAN}Simply colored!`              | ![Dim cyan text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_cyan_164x16.png)                                               |
//! | ![Dim white](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_white_16x16.png)        | `{DIM_WHITE}Simply colored!`             | ![Dim white text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_white_164x16.png)                                             |
//! | ![Dim black](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_black_16x16.png)        | `{DIM_BLACK}Simply colored!`             | ![Dim black text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_black_164x16.png)                                             |
//!
//! ## Background
//!
//! | Color                                                                                                                | Type                                   |  To get                                                                                                                                                                            |
//! | -----                                                                                                                | ------------------                     | -------                                                                                                                                                                            |
//! | ![Green](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_green_16x16.png)                | `{BG_GREEN}Simply colored!`              | ![Green text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_green_164x16.png)                                                     |
//! | ![Yellow](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_yellow_16x16.png)              | `{BG_YELLOW}Simply colored!`             | ![Yellow text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_yellow_164x16.png)                                                   |
//! | ![Red](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_red_16x16.png)                    | `{BG_RED}Simply colored!`                | ![Red text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_red_164x16.png)                                                         |
//! | ![Magenta](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_magenta_16x16.png)            | `{BG_MAGENTA}Simply colored!`            | ![Magenta text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_magenta_164x16.png)                                                 |
//! | ![Blue](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_blue_16x16.png)                  | `{BG_BLUE}Simply colored!`               | ![Blue text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_blue_164x16.png)                                                       |
//! | ![Cyan](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_cyan_16x16.png)                  | `{BG_CYAN}Simply colored!`               | ![Cyan text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_cyan_164x16.png)                                                       |
//! | ![White](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_white_16x16.png)                | `{BG_WHITE}Simply colored!`              | ![White text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_white_164x16.png)                                                     |
//! | ![Black](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_black_16x16.png)                | `{BG_BLACK}Simply colored!`              | ![Black text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_black_164x16.png)                                                     |
//! | ![Dim green](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_green_16x16.png)        | `{BG_DIM_GREEN}Simply colored!`          | ![Dim green text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_green_164x16.png)                                             |
//! | ![Dim yellow](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_yellow_16x16.png)      | `{BG_DIM_YELLOW}Simply colored!`         | ![Dim yellow text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_yellow_164x16.png)                                           |
//! | ![Dim red](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_red_16x16.png)            | `{BG_DIM_RED}Simply colored!`            | ![Dim red text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_red_164x16.png)                                                 |
//! | ![Dim magenta](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_magenta_16x16.png)    | `{BG_DIM_MAGENTA}Simply colored!`        | ![Dim magenta text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_magenta_164x16.png)                                         |
//! | ![Dim blue](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_blue_16x16.png)          | `{BG_DIM_BLUE}Simply colored!`           | ![Dim blue text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_blue_164x16.png)                                               |
//! | ![Dim cyan](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_cyan_16x16.png)          | `{BG_DIM_CYAN}Simply colored!`           | ![Dim cyan text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_cyan_164x16.png)                                               |
//! | ![Dim white](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_white_16x16.png)        | `{BG_DIM_WHITE}Simply colored!`          | ![Dim white text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_white_164x16.png)                                             |
//! | ![Dim black](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_black_16x16.png)        | `{BG_DIM_BLACK}Simply colored!`          | ![Dim black text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_black_164x16.png)                                             |
//!
//! ## Effects
//!
//! | Effect                    | Type                               |
//! | ------                    | -----                              |
//! | *Italic*                  | `{ITALIC}Simply colored!`            |
//! | **Bold**                  | `{BOLD}Simply colored!`              |
//! | <u>Underline</u>          | `{UNDERLINE}Simply colored!`         |
//! | Blink                     | `{BLINK}Simply colored!`             |
//! | Reverse                   | `{REVERSE}Simply colored!`           |
//! | <del>Strikethrough</del>  | `{STRIKETHROUGH}Simply colored!`     |
//! | Dim                       | `{DIM}Simply colored!`               |
//! | Hide                      | `{HIDE}Simply colored!`              |
//! | Reset all styles          | `{RESET}Simply colored!`             |
//!
//! All effects can be prefixed with `NO_` to disable e.g. `NO_BOLD`.
//!
//! ## Extra
//!
//! If you want links in the terminal, all you need is:
//!
//! ```rust
//! fn hyperlink(link: impl core::fmt::Display, text: impl core::fmt::Display) -> String {
//!     format!("\x1b]8;;{link}\x1b\\{text}\x1b]8;;\x1b\\")
//! }
//! ```
//!
//! Example usage:
//!
//! ```rust
//! println!(
//!     "Check out simply_colored on {}!",
//!     hyperlink(
//!         "https://github.com/nik-rev/simply-colored",
//!         "GitHub"
//!     )
//! );
//! ```
#![no_std]

/// Reset styling
pub const RESET: &str = "\x1b[0m";

/// Following text will be bold
pub const BOLD: &str = "\x1b[1m";
/// Following text will NOT be bold
pub const NO_BOLD: &str = "\x1b[21m";

/// Following text will be dim
pub const DIM: &str = "\x1b[2m";
/// Following text will NOT be dim
pub const NO_DIM: &str = "\x1b[22m";

/// Following text will be italic
pub const ITALIC: &str = "\x1b[3m";
/// Following text will NOT be italic
pub const NO_ITALIC: &str = "\x1b[23m";

/// Following text will be underlined
pub const UNDERLINE: &str = "\x1b[4m";
/// Following text will NOT be underlined
pub const NO_UNDERLINE: &str = "\x1b[24m";

/// Following text will be blinking
pub const BLINK: &str = "\x1b[5m";
/// Following text will NOT be blinking
pub const NO_BLINK: &str = "\x1b[25m";

/// Foreground and background for the following text will be reversed
pub const REVERSE: &str = "\x1b[7m";
/// Foreground and background for the following text will NOT be reversed
pub const NO_REVERSE: &str = "\x1b[27m";

/// Following text will be invisible
pub const HIDE: &str = "\x1b[8m";
/// Following text will be visible
pub const NO_HIDE: &str = "\x1b[28m";

/// Following text will be crossed out
pub const STRIKETHROUGH: &str = "\x1b[9m";
/// Following text will NOT be crossed out
pub const NO_STRIKETHROUGH: &str = "\x1b[29m";

/// Set color of text to black
pub const BLACK: &str = "\x1b[90m";
/// Set background of text to black
pub const BG_BLACK: &str = "\x1b[100m";
/// Set color of text to dim black
pub const DIM_BLACK: &str = "\x1b[30m";
/// Set background of text to dim black
pub const BG_DIM_BLACK: &str = "\x1b[40m";

/// Set color of text to red
pub const RED: &str = "\x1b[91m";
/// Set background of text to red
pub const BG_RED: &str = "\x1b[101m";
/// Set color of text to dim red
pub const DIM_RED: &str = "\x1b[31m";
/// Set background of text to dim red
pub const BG_DIM_RED: &str = "\x1b[41m";

/// Set color of text to green
pub const GREEN: &str = "\x1b[92m";
/// Set background of text to green
pub const BG_GREEN: &str = "\x1b[102m";
/// Set color of text to dim green
pub const DIM_GREEN: &str = "\x1b[32m";
/// Set background of text to dim green
pub const BG_DIM_GREEN: &str = "\x1b[42m";

/// Set color of text to yellow
pub const YELLOW: &str = "\x1b[93m";
/// Set background of text to yellow
pub const BG_YELLOW: &str = "\x1b[103m";
/// Set color of text to dim yellow
pub const DIM_YELLOW: &str = "\x1b[33m";
/// Set background of text to dim yellow
pub const BG_DIM_YELLOW: &str = "\x1b[43m";

/// Set color of text to blue
pub const BLUE: &str = "\x1b[94m";
/// Set background of text to blue
pub const BG_BLUE: &str = "\x1b[104m";
/// Set color of text to dim blue
pub const DIM_BLUE: &str = "\x1b[34m";
/// Set background of text to dim blue
pub const BG_DIM_BLUE: &str = "\x1b[44m";

/// Set color of text to magenta
pub const MAGENTA: &str = "\x1b[95m";
/// Set background of text to magenta
pub const BG_MAGENTA: &str = "\x1b[105m";
/// Set color of text to dim magenta
pub const DIM_MAGENTA: &str = "\x1b[35m";
/// Set background of text to dim magenta
pub const BG_DIM_MAGENTA: &str = "\x1b[45m";

/// Set color of text to cyan
pub const CYAN: &str = "\x1b[96m";
/// Set background of text to cyan
pub const BG_CYAN: &str = "\x1b[106m";
/// Set color of text to dim cyan
pub const DIM_CYAN: &str = "\x1b[36m";
/// Set background of text to dim cyan
pub const BG_DIM_CYAN: &str = "\x1b[46m";

/// Set color of text to white
pub const WHITE: &str = "\x1b[97m";
/// Set background of text to white
pub const BG_WHITE: &str = "\x1b[107m";
/// Set color of text to dim white
pub const DIM_WHITE: &str = "\x1b[37m";
/// Set background of text to dim white
pub const BG_DIM_WHITE: &str = "\x1b[47m";

/// Set color of text to default
pub const DEFAULT: &str = "\x1b[99m";
/// Set background of text to default
pub const BG_DEFAULT: &str = "\x1b[109m";
/// Set color of text to default
pub const DIM_DEFAULT: &str = "\x1b[39m";
/// Set background of text to default
pub const BG_DIM_DEFAULT: &str = "\x1b[49m";
