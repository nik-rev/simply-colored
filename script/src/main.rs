use std::{fs, path::Path};

use image::Rgb;
use imageproc::drawing::{draw_text_mut, text_size};
use strum::IntoEnumIterator;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Copy, strum::EnumIter, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    DimBlack,
    DimRed,
    DimGreen,
    DimYellow,
    DimBlue,
    DimMagenta,
    DimCyan,
    DimWhite,
}

impl Color {
    /// `[Red, Green, Blue]` for this color
    fn rgb(self) -> [u8; 3] {
        match self {
            Color::Black => [55, 59, 65],
            Color::Red => [204, 102, 102],
            Color::Green => [181, 189, 104],
            Color::Yellow => [240, 198, 116],
            Color::Blue => [129, 162, 190],
            Color::Magenta => [178, 148, 187],
            Color::Cyan => [138, 190, 183],
            Color::White => [197, 200, 198],
            Color::DimBlack => [40, 42, 46],
            Color::DimRed => [165, 66, 66],
            Color::DimGreen => [140, 148, 64],
            Color::DimYellow => [222, 147, 95],
            Color::DimBlue => [95, 129, 157],
            Color::DimMagenta => [133, 103, 143],
            Color::DimCyan => [94, 141, 135],
            Color::DimWhite => [112, 120, 128],
        }
    }
}

fn main() -> Result<()> {
    let assets_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("assets");

    fs::create_dir_all(&assets_path)
        .map_err(|err| format!("failed to create {assets_path:?}: {err}"))?;

    for color in Color::iter() {
        const SWATCH_SIZE: u32 = 16;
        let swatch = image::RgbImage::from_pixel(SWATCH_SIZE, SWATCH_SIZE, Rgb(color.rgb()));
        let swatch_path =
            assets_path.join(format!("swatch_{color}_{SWATCH_SIZE}x{SWATCH_SIZE}.png"));

        swatch
            .save(&swatch_path)
            .map_err(|err| format!("failed to save swatch to {swatch_path:?}: {err}"))?;

        macro_rules! font_path {
            () => {
                "../../assets/JetBrainsMono-Regular.ttf"
            };
        }

        let font = ab_glyph::FontArc::try_from_slice(include_bytes!(font_path!()))
            .map_err(|err| format!("failed to load font at {}: {err}", font_path!()))?;

        const TEXT_CONTAINER_WIDTH: u32 = 164;
        const TEXT_CONTAINER_HEIGHT: u32 = 16;

        let mut bg_text = image::RgbImage::from_pixel(
            TEXT_CONTAINER_WIDTH,
            TEXT_CONTAINER_HEIGHT,
            Rgb(color.rgb()),
        );

        let mut fg_text = image::RgbImage::from_pixel(
            TEXT_CONTAINER_WIDTH,
            TEXT_CONTAINER_HEIGHT,
            Rgb([197, 200, 198]),
        );

        const TEXT_SCALE: f32 = 18.0;
        const TEXT: &str = "Hello, world!";

        let (text_width, _text_height) = text_size(TEXT_SCALE, &font, TEXT);

        draw_text_mut(
            &mut bg_text,
            Rgb([197, 200, 198]),
            (TEXT_CONTAINER_WIDTH / 2 - text_width / 2) as i32,
            0,
            TEXT_SCALE,
            &font,
            TEXT,
        );

        draw_text_mut(
            &mut fg_text,
            Rgb(color.rgb()),
            (TEXT_CONTAINER_WIDTH / 2 - text_width / 2) as i32,
            0,
            TEXT_SCALE,
            &font,
            TEXT,
        );

        let bg_text_path = assets_path.join(format!(
            "bg_text_{color}_{TEXT_CONTAINER_WIDTH}x{TEXT_CONTAINER_HEIGHT}.png"
        ));
        let fg_text_path = assets_path.join(format!(
            "fg_text_{color}_{TEXT_CONTAINER_WIDTH}x{TEXT_CONTAINER_HEIGHT}.png"
        ));

        bg_text
            .save(&bg_text_path)
            .map_err(|err| format!("failed to save text to {bg_text_path:?}: {err}"))?;

        fg_text
            .save(&fg_text_path)
            .map_err(|err| format!("failed to save text to {fg_text_path:?}: {err}"))?;
    }

    Ok(())
}
