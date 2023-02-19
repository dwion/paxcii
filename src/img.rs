use crate::settings::PaxciiSettings;
use image::{imageops::FilterType, DynamicImage};

// Characters used to display ASCII output
pub const CHARS_LIGHT: [char; 11] = [' ', ' ', '.', ':', '!', '+', '*', 'e', '$', '@', '8'];
pub const CHARS_MEDIUM: [char; 5] = ['.', '*', 'e', 's', '◍'];
pub const CHARS_FILLED: [char; 4] = ['░', '▒', '▓', '█'];

/// Transforms an image into ascii.
/// If `resize` is set to true then it also resizes the image.
pub fn img_to_ascii(mut img: DynamicImage, s: &PaxciiSettings, resize: bool) -> String {
    if resize {
        // Resize image to fit desired ascii output size
        if s.preserve_aspect_ratio {
            img = img.resize(s.width, s.height, FilterType::Triangle);
        } else {
            img = img.resize_exact(s.width, s.height, FilterType::Triangle);
        }
    }
    let img = img.to_rgb8();

    // The ascii image that this function will return
    // note that the defined capacity isn't precise
    let mut ascii_img = String::with_capacity(img.len() / 3 * 2);

    // For keeping track of position in image width
    let mut row_index = 1;

    for p in img.pixels() {
        // If at end of pixel row print newline
        if row_index == img.width() {
            ascii_img += "\n";
            row_index = 1;
        } else {
            // Determine what ASCII character to use for this pixel
            let brightness = if s.color {
                0.267 * p.0[0] as f32 + 0.642 * p.0[1] as f32 + 0.091 * p.0[2] as f32
            } else {
                0.2126 * p.0[0] as f32 + 0.7152 * p.0[1] as f32 + 0.0722 * p.0[2] as f32
            };
            let size = s.char_set.len() - 1;
            let char_set_index = (size as f32 * brightness / 255.).round() as usize;

            // Creates the ascii pixel from two ascii characters, colors it if needed
            let ascii_pixel = if s.color {
                truecolor(p.0, s.char_set[char_set_index])
            } else {
                format!(
                    "{}{}",
                    s.char_set[char_set_index], s.char_set[char_set_index]
                )
            };

            ascii_img += &ascii_pixel;
            row_index += 1;
        }
    }
    // Turns all ansi attributes off
    ascii_img.push_str("\x1b[0m");
    ascii_img
}

// Creates a string composed of an ansi escape sequence for color and two ascii characters,
// who compose the ascii pixel
// https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
fn truecolor(rgb: [u8; 3], ascii_char: char) -> String {
    format!(
        "\x1b[38;2;{};{};{}m{}{}",
        rgb[0], rgb[1], rgb[2], ascii_char, ascii_char
    )
}
