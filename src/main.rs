// TODO
// Option for display in 8 bit color (?)
// // https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences

use image::imageops::FilterType;
use std::{process::ExitCode, fs};

mod cli;
mod img;
use img::img_to_ascii;
mod video;
use video::*;
mod audio;

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

        let ascii = img_to_ascii(img, &settings);

        if settings.output_file.is_empty() {
            print!("{}", ascii);
        } else {
            // Try to write ascii image to output file
            match fs::write(&settings.output_file, format!(r#"echo -e "{}""#, ascii)) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("{}Failed to write to output file. {}", ERROR_MSG, err);
                    return ExitCode::FAILURE
                }
            }
        }
    } else {
        // Get video resolution and fps
        let (vsize, fps) = match get_video_info(&path) {
            Ok(x) => x,
            Err(_) => return ExitCode::FAILURE
        };
        if settings.preserve_aspect_ratio {
            // Resize video dimensions preserving aspect ratio
            (settings.width, settings.height) = resize_video_dimensions(vsize, &settings);
        }

        // Get video bytes using ffmpeg command
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
    audio_file: String,
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
            audio_file: String::new(),
            color: true,
            char_set: Vec::from(CHARS_MEDIUM),
            width: 42,
            height: 32,
            preserve_aspect_ratio: true,
        }
    }
}
