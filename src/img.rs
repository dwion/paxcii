use image::{RgbImage, Rgb};
use std::fs;
use crate::{Settings, ERROR_MSG};

pub fn img_to_ascii(img: RgbImage, settings: &Settings) -> Result<(), ()> {
    let mut i = 0;
    let mut ascii_img = String::new();

    for pixel in img.pixels() {
        // If at end of pixel row print newline
        if i == img.width() - 1 {
            ascii_img += "\n";
            i = 0;

        } else {
            let rgb = [pixel[0] as f32, pixel[1] as f32, pixel[2] as f32];

            // Determine what ASCII character to use for this pixel
            let brightness = if settings.color {
                0.267 * rgb[0] + 0.642 * rgb[1] + 0.091 * rgb[2]
            } else {
                0.2126 * rgb[0] + 0.7152 * rgb[1] + 0.0722 * rgb[2]
            };
            let size = settings.char_set.len() - 1;
            let index = (size as f32 * brightness / 255.).round() as usize;

            // Creates the ascii pixel from two ascii characters, colors it if needed
            let ascii_pixel = if settings.color {
                truecolor(pixel, settings.char_set[index])
            } else {
                format!("{}{}", settings.char_set[index], settings.char_set[index])
            };

            ascii_img += &ascii_pixel;
            i += 1;
        }
    }
    // Turns all ansi attributes off
    ascii_img.push_str("\x1b[0m");

    if settings.output_file.is_empty() {
        print!("{}", ascii_img);
    } else {
        // Try to write ascii image to file
        match fs::write(&settings.output_file, ascii_img) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("{}Failed to write to output file", ERROR_MSG);
                eprintln!("{}", err);
                return Err(());
            }
        };
    }
    Ok(())
}

// Creates a string composed of an ansi escape sequence for color and two ascii characters,
// who compose the ascii pixel
// https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
fn truecolor(rgb: &Rgb<u8>, ascii_char: char) -> String{
    format!("\x1b[38;2;{};{};{}m{}{}", rgb[0], rgb[1], rgb[2], ascii_char, ascii_char)
}
