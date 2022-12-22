// TODO
// Option for display in 8 bit color (?)
// // https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
// Video output to file
// Audio
// Buffers
// Crate (?)

use image::imageops::FilterType;
use std::process::ExitCode;

mod cli;
mod img;
use img::img_to_ascii;
mod video;
use video::*;

fn main() -> ExitCode {
    let cmd = cli::create_cli();

    let (mut settings, path) = match cli::process_args(cmd) {
        Ok(x) => x,
        Err(_) => return ExitCode::FAILURE
    };

    if !settings.is_video {
        let mut img = image::open(&path).expect("Failed to load image");

        // Resize image to fit desired ascii output size
        if settings.preserve_aspect_ratio {
            img = img.resize(settings.width, settings.height, FilterType::Triangle);
        } else {
            img = img.resize_exact(settings.width, settings.height, FilterType::Triangle);
        }

        // Convert 'DynamicImage' type to 'RgbImage'
        let img = img.to_rgb8();

        match img_to_ascii(img, &settings) {
            Ok(_) => (),
            Err(_) => return ExitCode::FAILURE
        }
    } else {
        let (vsize, fps) = match get_video_info(&path) {
            Ok(x) => x,
            Err(_) => return ExitCode::FAILURE
        };

        if settings.preserve_aspect_ratio {
            (settings.width, settings.height) = resize_video_dimensions(vsize, &settings);
        }

        let video = ffmpeg_cmd([settings.width, settings.height], &path);

        match video_to_ascii(video, settings, fps) {
            Ok(_) => (),
            Err(_) => return ExitCode::FAILURE
        }
    }

    ExitCode::SUCCESS
}

// Bold and red 'error: '
const ERROR_MSG: &str = "\x1b[31;1merror\x1b[0m: ";

// Characters used to display ASCII output
const CHARS_LIGHT: [char; 11] = [' ', ' ', '.', ':', '!', '+', '*', 'e', '$', '@', '8'];
const CHARS_MEDIUM: [char; 5] = ['.', '*', 'e', 's', '◍'];
const CHARS_FILLED: [char; 4] = ['░', '▒', '▓', '█'];

// Settings to be used when converting image to ascii
pub struct Settings {
    is_video: bool,
    output_file: String,
    color: bool,
    char_set: Vec<char>,
    width: u32,
    height: u32,
    preserve_aspect_ratio: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            is_video: false,
            output_file: String::new(),
            color: true,
            char_set: Vec::from(CHARS_MEDIUM),
            width: 42,
            height: 32,
            preserve_aspect_ratio: true,
        }
    }
}
