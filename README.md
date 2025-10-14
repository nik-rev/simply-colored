# `simply-colored`

<!-- cargo-rdme start -->

[![crates.io](https://img.shields.io/crates/v/simply_colored?style=flat-square&logo=rust)](https://crates.io/crates/simply_colored)
[![docs.rs](https://img.shields.io/badge/docs.rs-simply_colored-blue?style=flat-square&logo=docs.rs)](https://docs.rs/simply_colored)
![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)
![msrv](https://img.shields.io/badge/msrv-1.0-blue?style=flat-square&logo=rust)
[![github](https://img.shields.io/github/stars/nik-rev/simply-colored)](https://github.com/nik-rev/simply-colored)

This crate is the simplest yet ergonomic way to add color to your terminal.

```toml
[dependencies]
simply-colored = "0.1"
```

All this crate contains is a few dozen `const`ants corresponding to particular ANSI escape codes.

## Usage

```rust
use simply_colored::*;

println!("{BLUE}{BOLD}Simply colored!")
```

### Foreground

| Color                                                                                                                | Type                                   | To get                                                                                                                                                                             |
| -----                                                                                                                | ------------------                     | -------                                                                                                                                                                            |
| ![Green](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_green_16x16.png)                | `{GREEN}Simply colored!`                 | ![Green text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_green_164x16.png)                                                     |
| ![Yellow](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_yellow_16x16.png)              | `{YELLOW}Simply colored!`                | ![Yellow text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_yellow_164x16.png)                                                   |
| ![Red](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_red_16x16.png)                    | `{RED}Simply colored!`                   | ![Red text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_red_164x16.png)                                                         |
| ![Magenta](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_magenta_16x16.png)            | `{MAGENTA}Simply colored!`               | ![Magenta text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_magenta_164x16.png)                                                 |
| ![Blue](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_blue_16x16.png)                  | `{BLUE}Simply colored!`                  | ![Blue text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_blue_164x16.png)                                                       |
| ![Cyan](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_cyan_16x16.png)                  | `{CYAN}Simply colored!`                  | ![Cyan text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_cyan_164x16.png)                                                       |
| ![White](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_white_16x16.png)                | `{WHITE}Simply colored!`                 | ![White text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_white_164x16.png)                                                     |
| ![Black](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_black_16x16.png)                | `{BLACK}Simply colored!`                 | ![Black text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_black_164x16.png)                                                     |
| ![Dim green](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_green_16x16.png)        | `{DIM_GREEN}Simply colored!`             | ![Dim green text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_green_164x16.png)                                             |
| ![Dim yellow](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_yellow_16x16.png)      | `{DIM_YELLOW}Simply colored!`            | ![Dim yellow text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_yellow_164x16.png)                                           |
| ![Dim red](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_red_16x16.png)            | `{DIM_RED}Simply colored!`               | ![Dim red text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_red_164x16.png)                                                 |
| ![Dim magenta](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_magenta_16x16.png)    | `{DIM_MAGENTA}Simply colored!`           | ![Dim magenta text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_magenta_164x16.png)                                         |
| ![Dim blue](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_blue_16x16.png)          | `{DIM_BLUE}Simply colored!`              | ![Dim blue text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_blue_164x16.png)                                               |
| ![Dim cyan](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_cyan_16x16.png)          | `{DIM_CYAN}Simply colored!`              | ![Dim cyan text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_cyan_164x16.png)                                               |
| ![Dim white](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_white_16x16.png)        | `{DIM_WHITE}Simply colored!`             | ![Dim white text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_white_164x16.png)                                             |
| ![Dim black](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_black_16x16.png)        | `{DIM_BLACK}Simply colored!`             | ![Dim black text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/fg_text_dim_black_164x16.png)                                             |

### Background

| Color                                                                                                                | Type                                   |  To get                                                                                                                                                                            |
| -----                                                                                                                | ------------------                     | -------                                                                                                                                                                            |
| ![Green](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_green_16x16.png)                | `{BG_GREEN}Simply colored!`              | ![Green text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_green_164x16.png)                                                     |
| ![Yellow](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_yellow_16x16.png)              | `{BG_YELLOW}Simply colored!`             | ![Yellow text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_yellow_164x16.png)                                                   |
| ![Red](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_red_16x16.png)                    | `{BG_RED}Simply colored!`                | ![Red text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_red_164x16.png)                                                         |
| ![Magenta](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_magenta_16x16.png)            | `{BG_MAGENTA}Simply colored!`            | ![Magenta text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_magenta_164x16.png)                                                 |
| ![Blue](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_blue_16x16.png)                  | `{BG_BLUE}Simply colored!`               | ![Blue text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_blue_164x16.png)                                                       |
| ![Cyan](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_cyan_16x16.png)                  | `{BG_CYAN}Simply colored!`               | ![Cyan text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_cyan_164x16.png)                                                       |
| ![White](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_white_16x16.png)                | `{BG_WHITE}Simply colored!`              | ![White text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_white_164x16.png)                                                     |
| ![Black](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_black_16x16.png)                | `{BG_BLACK}Simply colored!`              | ![Black text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_black_164x16.png)                                                     |
| ![Dim green](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_green_16x16.png)        | `{BG_DIM_GREEN}Simply colored!`          | ![Dim green text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_green_164x16.png)                                             |
| ![Dim yellow](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_yellow_16x16.png)      | `{BG_DIM_YELLOW}Simply colored!`         | ![Dim yellow text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_yellow_164x16.png)                                           |
| ![Dim red](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_red_16x16.png)            | `{BG_DIM_RED}Simply colored!`            | ![Dim red text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_red_164x16.png)                                                 |
| ![Dim magenta](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_magenta_16x16.png)    | `{BG_DIM_MAGENTA}Simply colored!`        | ![Dim magenta text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_magenta_164x16.png)                                         |
| ![Dim blue](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_blue_16x16.png)          | `{BG_DIM_BLUE}Simply colored!`           | ![Dim blue text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_blue_164x16.png)                                               |
| ![Dim cyan](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_cyan_16x16.png)          | `{BG_DIM_CYAN}Simply colored!`           | ![Dim cyan text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_cyan_164x16.png)                                               |
| ![Dim white](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_white_16x16.png)        | `{BG_DIM_WHITE}Simply colored!`          | ![Dim white text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_white_164x16.png)                                             |
| ![Dim black](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/swatch_dim_black_16x16.png)        | `{BG_DIM_BLACK}Simply colored!`          | ![Dim black text color in terminal](https://raw.githubusercontent.com/nik-rev/simply-colored/main/assets/bg_text_dim_black_164x16.png)                                             |

### Effects

| Effect                    | Type                               |
| ------                    | -----                              |
| *Italic*                  | `{ITALIC}Simply colored!`            |
| **Bold**                  | `{BOLD}Simply colored!`              |
| <u>Underline</u>          | `{UNDERLINE}Simply colored!`         |
| Blink                     | `{BLINK}Simply colored!`             |
| Reverse                   | `{REVERSE}Simply colored!`           |
| <del>Strikethrough</del>  | `{STRIKETHROUGH}Simply colored!`     |
| Dim                       | `{DIM}Simply colored!`               |
| Hide                      | `{HIDE}Simply colored!`              |
| Reset all styles          | `{RESET}Simply colored!`             |

All effects can be prefixed with `NO_` to disable e.g. `NO_BOLD`.

### Extra

If you want links in the terminal, all you need is:

```rust
fn hyperlink(link: impl core::fmt::Display, text: impl core::fmt::Display) -> String {
    format!("\x1b]8;;{link}\x1b\\{text}\x1b]8;;\x1b\\")
}
```

Example usage:

```rust
println!(
    "Check out simply_colored on {}!",
    hyperlink(
        "https://github.com/nik-rev/simply-colored",
        "GitHub"
    )
);
```

<!-- cargo-rdme end -->
