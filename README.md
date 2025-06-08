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

## Foreground Text

| Color                                              | Example                  | Output                                                                                                               |
| -----                                              | ------------------       | -------                                                                                                              |
| ![Green](assets/swatch_green_16x16.png)     | `format!("{GREEN}Hello, world!")`   | ![Bright green text color in terminal](assets/text_green_16x16.png)     |
| ![Yellow](assets/swatch_yellow_16x16.png)   | `format!("{YELLOW}Hello, world!")`  | ![Bright yellow text color in terminal](assets/text_yellow_16x16.png)   |
| ![Red](assets/swatch_red_16x16.png)         | `format!("{RED}Hello, world!")`     | ![Bright red text color in terminal](assets/text_red_16x16.png)         |
| ![Magenta](assets/swatch_magenta_16x16.png) | `format!("{MAGENTA}Hello, world!")` | ![Bright magenta text color in terminal](assets/text_magenta_16x16.png) |
| ![Blue](assets/swatch_blue_16x16.png)       | `format!("{BLUE}Hello, world!")`    | ![Bright blue text color in terminal](assets/text_blue_16x16.png)       |
| ![Cyan](assets/swatch_cyan_16x16.png)       | `format!("{CYAN}Hello, world!")`    | ![Bright cyan text color in terminal](assets/text_cyan_16x16.png)       |
| ![White](assets/swatch_white_16x16.png)     | `format!("{WHITE}Hello, world!")`   | ![Bright white text color in terminal](assets/text_white_16x16.png)     |
| ![Black](assets/swatch_black_16x16.png)     | `format!("{BLACK}Hello, world!")`   | ![Bright black text color in terminal](assets/text_black_16x16.png)     |
| ![Dim green](assets/swatch_dim_green_16x16.png)        | `format!("{DIM_GREEN}Hello, world!")`          | ![Green text color in terminal](assets/text_dim_green_16x16.png)                   |
| ![Dim yellow](assets/swatch_dim_yellow_16x16.png)      | `format!("{DIM_YELLOW}Hello, world!")`         | ![Yellow text color in terminal](assets/text_dim_yellow_16x16.png)                 |
| ![Dim red](assets/swatch_dim_red_16x16.png)            | `format!("{DIM_RED}Hello, world!")`            | ![Red text color in terminal](assets/text_dim_red_16x16.png)                       |
| ![Dim magenta](assets/swatch_dim_magenta_16x16.png)    | `format!("{DIM_MAGENTA}Hello, world!")`        | ![Magenta text color in terminal](assets/text_dim_magenta_16x16.png)               |
| ![Dim blue](assets/swatch_dim_blue_16x16.png)          | `format!("{DIM_BLUE}Hello, world!")`           | ![Blue text color in terminal](assets/text_dim_blue_16x16.png)                     |
| ![Dim cyan](assets/swatch_dim_cyan_16x16.png)          | `format!("{DIM_CYAN}Hello, world!")`           | ![Cyan text color in terminal](assets/text_dim_cyan_16x16.png)                     |
| ![Dim white](assets/swatch_dim_white_16x16.png)        | `format!("{DIM_WHITE}Hello, world!")`          | ![White text color in terminal](assets/text_dim_white_16x16.png)                   |
| ![Dim black](assets/swatch_dim_black_16x16.png)        | `format!("{DIM_BLACK}Hello, world!")`          | ![Black text color in terminal](assets/text_dim_black_16x16.png)                   |

## Background

| Color | Full Text Function | Color Code | Example |
| ----- | ------------------ | ---------- | ------- |
| ![Green](assets/bg_colors/green_16x16.png) | `bg_green("text")` | `BgColor.GREEN` | ![Green background color in terminal](/docs/assets/images/examples/bg_color_map/green_full_text_194x16.png) |
| ![Yellow](assets/bg_colors/yellow_16x16.png) | `bg_yellow("text")` | `BgColor.YELLOW` | ![Yellow background color in terminal](/docs/assets/images/examples/bg_color_map/yellow_full_text_194x16.png) |
| ![Red](assets/bg_colors/red_16x16.png) | `bg_red("text")` | `BgColor.RED` | ![Red background color in terminal](/docs/assets/images/examples/bg_color_map/red_full_text_194x16.png) |
| ![Magenta](assets/bg_colors/magenta_16x16.png) | `bg_magenta("text")` | `BgColor.MAGENTA` | ![Magenta background color in terminal](/docs/assets/images/examples/bg_color_map/magenta_full_text_194x16.png) |
| ![Blue](assets/bg_colors/blue_16x16.png) | `bg_blue("text")` | `BgColor.BLUE` | ![Blue background color in terminal](/docs/assets/images/examples/bg_color_map/blue_full_text_194x16.png) |
| ![Cyan](assets/bg_colors/cyan_16x16.png) | `bg_cyan("text")` | `BgColor.CYAN` | ![Cyan background color in terminal](/docs/assets/images/examples/bg_color_map/cyan_full_text_194x16.png) |
| ![White](assets/bg_colors/white_16x16.png) | `bg_white("text")` | `BgColor.WHITE` | ![White background color in terminal](/docs/assets/images/examples/bg_color_map/white_full_text_194x16.png) |
| ![Black](assets/bg_colors/black_16x16.png) | `bg_black("text")` | `BgColor.BLACK` | ![Black background color in terminal](/docs/assets/images/examples/bg_color_map/black_full_text_194x16.png) |
| - | - | `BgColor.DEFAULT` | - |
| - | - | `BgColor.OFF` | - |
| ![Bright green](assets/bg_colors/bright_green_16x16.png) | `bg_bright_green("text")` | `BgBrightColor.GREEN` | ![Bright green background color in terminal](/docs/assets/images/examples/bg_color_map/bright_green_full_text_194x16.png) |
| ![Bright yellow](assets/bg_colors/bright_yellow_16x16.png) | `bg_bright_yellow("text")` | `BgBrightColor.YELLOW` | ![Bright yellow background color in terminal](/docs/assets/images/examples/bg_color_map/bright_yellow_full_text_194x16.png) |
| ![Bright red](assets/bg_colors/bright_red_16x16.png) | `bg_bright_red("text")` | `BgBrightColor.RED` | ![Bright red background color in terminal](/docs/assets/images/examples/bg_color_map/bright_red_full_text_194x16.png) |
| ![Bright magenta](assets/bg_colors/bright_magenta_16x16.png) | `bg_bright_magenta("text")` | `BgBrightColor.MAGENTA` | ![Bright magenta background color in terminal](/docs/assets/images/examples/bg_color_map/bright_magenta_full_text_194x16.png) |
| ![Bright blue](assets/bg_colors/bright_blue_16x16.png) | `bg_bright_blue("text")` | `BgBrightColor.BLUE` | ![Bright blue background color in terminal](/docs/assets/images/examples/bg_color_map/bright_blue_full_text_194x16.png) |
| ![Bright cyan](assets/bg_colors/bright_cyan_16x16.png) | `bg_bright_cyan("text")` | `BgBrightColor.CYAN` | ![Bright cyan background color in terminal](/docs/assets/images/examples/bg_color_map/bright_cyan_full_text_194x16.png) |
| ![Bright white](assets/bg_colors/bright_white_16x16.png) | `bg_bright_white("text")` | `BgBrightColor.WHITE` | ![Bright white background color in terminal](/docs/assets/images/examples/bg_color_map/bright_white_full_text_194x16.png) |
| ![Bright black](assets/bg_colors/bright_black_16x16.png) | `bg_bright_black("text")` | `BgBrightColor.BLACK` | ![Bright black background color in terminal](/docs/assets/images/examples/bg_color_map/bright_black_full_text_194x16.png) |
| - | - | `BgBrightColor.DEFAULT` | - |
| - | - | `BgBrightColor.OFF` | - |

