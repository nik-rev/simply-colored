# `simply_colored`

This crate is the simplest yet ergonomic way to add color to your terminal:

```rs
use simply_colored::*;

println!("\
{GREEN}green!
{BLUE}{BOLD}bold blue!
{UNDERLINE}{BOLD}{STRIKETHROUGH}{RED}{ITALIC}and this has a bunch of effects!{OFF}
")
```

The crate's code is essentially 50 lines, containing only 3 helper functions with everything else being a `const`ant that you can easily use in format strings!

## Foreground

| Color                                              | Example                  | Output                                                                                                               |
| -----                                              | ------------------       | -------                                                                                                              |
| ![Green](assets/swatch_green_16x16.png)     | `{GREEN}Hello, world!`   | ![Green text color in terminal](assets/fg_text_green_196x16.png)     |
| ![Yellow](assets/swatch_yellow_16x16.png)   | `{YELLOW}Hello, world!`  | ![Yellow text color in terminal](assets/fg_text_yellow_196x16.png)   |
| ![Red](assets/swatch_red_16x16.png)         | `{RED}Hello, world!`     | ![Red text color in terminal](assets/fg_text_red_196x16.png)         |
| ![Magenta](assets/swatch_magenta_16x16.png) | `{MAGENTA}Hello, world!` | ![Magenta text color in terminal](assets/fg_text_magenta_196x16.png) |
| ![Blue](assets/swatch_blue_16x16.png)       | `{BLUE}Hello, world!`    | ![Blue text color in terminal](assets/fg_text_blue_196x16.png)       |
| ![Cyan](assets/swatch_cyan_16x16.png)       | `{CYAN}Hello, world!`    | ![Cyan text color in terminal](assets/fg_text_cyan_196x16.png)       |
| ![White](assets/swatch_white_16x16.png)     | `{WHITE}Hello, world!`   | ![White text color in terminal](assets/fg_text_white_196x16.png)     |
| ![Black](assets/swatch_black_16x16.png)     | `{BLACK}Hello, world!`   | ![Black text color in terminal](assets/fg_text_black_196x16.png)     |
| ![Dim green](assets/swatch_dim_green_16x16.png)        | `{DIM_GREEN}Hello, world!`          | ![Dim green text color in terminal](assets/fg_text_dim_green_196x16.png)                   |
| ![Dim yellow](assets/swatch_dim_yellow_16x16.png)      | `{DIM_YELLOW}Hello, world!`         | ![Dim yellow text color in terminal](assets/fg_text_dim_yellow_196x16.png)                 |
| ![Dim red](assets/swatch_dim_red_16x16.png)            | `{DIM_RED}Hello, world!`            | ![Dim red text color in terminal](assets/fg_text_dim_red_196x16.png)                       |
| ![Dim magenta](assets/swatch_dim_magenta_16x16.png)    | `{DIM_MAGENTA}Hello, world!`        | ![Dim magenta text color in terminal](assets/fg_text_dim_magenta_196x16.png)               |
| ![Dim blue](assets/swatch_dim_blue_16x16.png)          | `{DIM_BLUE}Hello, world!`           | ![Dim blue text color in terminal](assets/fg_text_dim_blue_196x16.png)                     |
| ![Dim cyan](assets/swatch_dim_cyan_16x16.png)          | `{DIM_CYAN}Hello, world!`           | ![Dim cyan text color in terminal](assets/fg_text_dim_cyan_196x16.png)                     |
| ![Dim white](assets/swatch_dim_white_16x16.png)        | `{DIM_WHITE}Hello, world!`          | ![Dim white text color in terminal](assets/fg_text_dim_white_196x16.png)                   |
| ![Dim black](assets/swatch_dim_black_16x16.png)        | `{DIM_BLACK}Hello, world!`          | ![Dim black text color in terminal](assets/fg_text_dim_black_196x16.png)                   |

## Background Text

| Color                                              | Example                  | Output                                                                                                               |
| -----                                              | ------------------       | -------                                                                                                              |
| ![Green](assets/swatch_green_16x16.png)     | `{BG_GREEN}Hello, world!`   | ![Green text color in terminal](assets/bg_text_green_196x16.png)     |
| ![Yellow](assets/swatch_yellow_16x16.png)   | `{BG_YELLOW}Hello, world!`  | ![Yellow text color in terminal](assets/bg_text_yellow_196x16.png)   |
| ![Red](assets/swatch_red_16x16.png)         | `{BG_RED}Hello, world!`     | ![Red text color in terminal](assets/bg_text_red_196x16.png)         |
| ![Magenta](assets/swatch_magenta_16x16.png) | `{BG_MAGENTA}Hello, world!` | ![Magenta text color in terminal](assets/bg_text_magenta_196x16.png) |
| ![Blue](assets/swatch_blue_16x16.png)       | `{BG_BLUE}Hello, world!`    | ![Blue text color in terminal](assets/bg_text_blue_196x16.png)       |
| ![Cyan](assets/swatch_cyan_16x16.png)       | `{BG_CYAN}Hello, world!`    | ![Cyan text color in terminal](assets/bg_text_cyan_196x16.png)       |
| ![White](assets/swatch_white_16x16.png)     | `{BG_WHITE}Hello, world!`   | ![White text color in terminal](assets/bg_text_white_196x16.png)     |
| ![Black](assets/swatch_black_16x16.png)     | `{BG_BLACK}Hello, world!`   | ![Black text color in terminal](assets/bg_text_black_196x16.png)     |
| ![Dim green](assets/swatch_dim_green_16x16.png)        | `{BG_DIM_GREEN}Hello, world!`          | ![Dim green text color in terminal](assets/bg_text_dim_green_196x16.png)                   |
| ![Dim yellow](assets/swatch_dim_yellow_16x16.png)      | `{BG_DIM_YELLOW}Hello, world!`         | ![Dim yellow text color in terminal](assets/bg_text_dim_yellow_196x16.png)                 |
| ![Dim red](assets/swatch_dim_red_16x16.png)            | `{BG_DIM_RED}Hello, world!`            | ![Dim red text color in terminal](assets/bg_text_dim_red_196x16.png)                       |
| ![Dim magenta](assets/swatch_dim_magenta_16x16.png)    | `{BG_DIM_MAGENTA}Hello, world!`        | ![Dim magenta text color in terminal](assets/bg_text_dim_magenta_196x16.png)               |
| ![Dim blue](assets/swatch_dim_blue_16x16.png)          | `{BG_DIM_BLUE}Hello, world!`           | ![Dim blue text color in terminal](assets/bg_text_dim_blue_196x16.png)                     |
| ![Dim cyan](assets/swatch_dim_cyan_16x16.png)          | `{BG_DIM_CYAN}Hello, world!`           | ![Dim cyan text color in terminal](assets/bg_text_dim_cyan_196x16.png)                     |
| ![Dim white](assets/swatch_dim_white_16x16.png)        | `{BG_DIM_WHITE}Hello, world!`          | ![Dim white text color in terminal](assets/bg_text_dim_white_196x16.png)                   |
| ![Dim black](assets/swatch_dim_black_16x16.png)        | `{BG_DIM_BLACK}Hello, world!`          | ![Dim black text color in terminal](assets/bg_text_dim_black_196x16.png)                   |
