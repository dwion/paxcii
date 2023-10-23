use crate::settings::PaxciiSettings;
use image::io::Reader as ImageReader;
use image::{imageops::FilterType, DynamicImage};
use std::fs;

pub struct AsciiImage {
    pub settings: PaxciiSettings,
    pub image: Option<DynamicImage>,
    pub ascii: Option<String>,
}

impl AsciiImage {
    /// Simplest way to turn image into ascii.\
    /// Ascii image is stored in `ascii` in [`AsciiImage`] struct.
    pub fn open(path: &str, settings: PaxciiSettings) -> anyhow::Result<AsciiImage> {
        let mut ascii_image = AsciiImage::new(settings);
        ascii_image.image = Some(ImageReader::open(path)?.decode()?);
        ascii_image.image_to_ascii(true);

        Ok(ascii_image)
    }
    pub fn new(settings: PaxciiSettings) -> AsciiImage {
        AsciiImage {
            settings,
            image: None,
            ascii: None,
        }
    }
    /// Prints the ascii to terminal.
    pub fn print(&self) {
        if self.ascii.is_none() {
            eprintln!("`ascii` is None. No image to print");
        } else {
            println!("{}", self.ascii.as_ref().unwrap())
        }
    }
    /// Writes ascii image to file.
    pub fn write(&self, path: &str) -> anyhow::Result<()> {
        if let Some(ascii) = &self.ascii {
            fs::write(path, ascii)?;
            Ok(())
        } else {
            anyhow::bail!("Couldn't write ascii image to file. Ascii missing")
        }
    }
    /// Transforms an image into ascii.\
    /// Also optionally resizes image.
    pub fn image_to_ascii(&mut self, resize: bool) {
        if self.image.is_none() {
            eprintln!("`image` parameter is not set. Can't convert image to ascii");
            return;
        }
        let img = self.image.as_ref().unwrap();

        // Resize image to fit desired ascii output size
        let img = if resize {
            if self.settings.keep_aspect_ratio {
                img.resize(
                    self.settings.width,
                    self.settings.height,
                    FilterType::Triangle,
                )
                .to_rgb8()
            } else {
                img.resize_exact(
                    self.settings.width,
                    self.settings.height,
                    FilterType::Triangle,
                )
                .to_rgb8()
            }
        } else {
            img.to_rgb8()
        };

        // The ascii image that this function will return
        // the defined capacity is an approximation
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
                let brightness = if self.settings.color {
                    0.267 * p.0[0] as f32 + 0.642 * p.0[1] as f32 + 0.091 * p.0[2] as f32
                } else {
                    0.2126 * p.0[0] as f32 + 0.7152 * p.0[1] as f32 + 0.0722 * p.0[2] as f32
                };
                let size = self.settings.char_set.len() - 1;
                let char_set_index = (size as f32 * brightness / 255.).round() as usize;

                // Creates the ascii pixel from two ascii characters, colors it if needed
                let ascii_pixel = if self.settings.color {
                    truecolor(p.0, self.settings.char_set[char_set_index])
                } else {
                    format!(
                        "{}{}",
                        self.settings.char_set[char_set_index],
                        self.settings.char_set[char_set_index]
                    )
                };

                ascii_img += &ascii_pixel;
                row_index += 1;
            }
        }
        // Turns all ansi attributes off
        ascii_img.push_str("\x1b[0m");
        self.ascii = Some(ascii_img);
    }
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
