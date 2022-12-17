// TODO
// Separate code into multiple files as project grows
// Option for display in 8 bit color (?)
// // https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
// Video
// Audio
// Output to file
// Crate (?)

use image::{
    imageops::FilterType,
    DynamicImage,
};
use clap::{Arg, ArgAction, Command, ArgMatches};
use std::process::ExitCode;
use std::fs;

fn main() -> ExitCode {
    let cmd = cli();

    let (settings, path) = match process_args(cmd) {
        Ok(x) => x,
        Err(_) => return ExitCode::FAILURE
    };

    let img = image::open(path).expect("Failed to load image");

    // print!("{}[2J", 27 as char);
    match img_to_ascii(img, settings) {
        Ok(_) => (),
        Err(_) => return ExitCode::FAILURE
    }

    ExitCode::SUCCESS
}

// Characters used to display ASCII output
const CHARS_LIGHT: [char; 11] = [' ', ' ', '.', ':', '!', '+', '*', 'e', '$', '@', '8'];
const CHARS_MEDIUM: [char; 5] = ['.', '*', 'e', 's', '◍'];
const CHARS_FILLED: [char; 4] = ['░', '▒', '▓', '█'];

// Settings to be used when converting image to ascii
struct Settings {
    // is_video: bool,
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
            // is_video: false,
            output_file: String::new(),
            color: true,
            char_set: Vec::from(CHARS_MEDIUM),
            width: 42,
            height: 32,
            preserve_aspect_ratio: true,
        }
    }
}

fn process_args(cmd: ArgMatches) -> Result<(Settings, String), ()> {
    let mut settings = Settings::default();
    let mut path = String::new();

    // Get path of input file
    if cmd.contains_id("image") {
        path = cmd.get_one::<String>("image").unwrap().clone();
    // } else if cmd.contains_id("video") {
        // settings.is_video = true;
        // path = *cmd.get_one::<String>("video").unwrap();
    }

    // Get output file name
    if cmd.contains_id("output-file") {
        settings.output_file = cmd.get_one::<String>("output-file").unwrap().clone();
    }

    if cmd.get_flag("no-color") {
        settings.color = false;
        settings.char_set = Vec::from(CHARS_LIGHT);
    }

    if cmd.get_flag("no-preserve-aspect-ratio") {
        settings.preserve_aspect_ratio = false;
    }

    if cmd.contains_id("char-set") {
        let c = cmd.get_one::<String>("char-set").unwrap();

        settings.char_set = match c.as_str() {
            "light" => Vec::from(CHARS_LIGHT),
            "medium" => Vec::from(CHARS_MEDIUM),
            "filled" => Vec::from(CHARS_FILLED),
            _ => {
                eprintln!("\x1b[31;1merror\x1b[0m: Invalid value for argument 'char-set'. Value can only be: light/medium/filled");
                return Err(());
            }
        }
    }

    // Try to get terminal size from get_term_size and use it if we can
    if let Some(s) = get_term_size() {
        (settings.width, settings.height) = s;
    }
    if cmd.contains_id("width") {
        let w = cmd.get_one::<String>("width").unwrap();
        match w.parse::<u32>() {
            Ok(w) => settings.width = w,
            Err(_) => {
                eprintln!("\x1b[31;1merror\x1b[0m: Argument 'width' must be a number");
                return Err(());
            }
        }
    }
    if cmd.contains_id("height") {
        let h = cmd.get_one::<String>("height").unwrap();
        match h.parse::<u32>() {
            Ok(h) => settings.height = h,
            Err(_) => {
                eprintln!("\x1b[31;1merror\x1b[0m: Argument 'height' must be a number");
                return Err(());
            }
        }
    }

    Ok((settings, path))
}

fn cli() -> ArgMatches {
    let cmd = Command::new("paxcii")
        .about("Transform images to ascii art")
        .version("0.1.0")
        .arg_required_else_help(true)
        // .arg(
        //     Arg::new("video")
        //         .short('v')
        //         .long("video")
        //         .help("Path to input video file")
        //         .num_args(1)
        //         .action(ArgAction::Set)
        // )
        .arg(
            Arg::new("image")
                .short('i')
                .long("image")
                .help("Path to input image file")
                .value_name("example.jpg")
                .num_args(1)
                .action(ArgAction::Set)
                // .required_unless_present("video")
                .required(true)
        )
        .arg(
            Arg::new("output-file")
                .short('o')
                .long("output-file")
                .help("Print output to specified file instead of stdout")
                .value_name("example.txt")
                .num_args(1)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("no-color")
                .short('n')
                .long("no-color")
                .help("Prints ascii without colors")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("char-set")
                .short('c')
                .long("char-set")
                .help("Choose character set to use for ascii. Options: light(default when color is disabled)/medium(default)/filled")
                .num_args(1)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("width")
                .short('W')
                .long("width")
                .help("Ascii output width. Uses terminal width by default.")
                .num_args(1)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("height")
                .short('H')
                .long("height")
                .help("Ascii output height. Uses terminal height by default.")
                .num_args(1)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("no-preserve-aspect-ratio")
                .long("no-preserve-aspect-ratio")
                .help("Doesn't preserve aspect ratio of input in output")
                .action(ArgAction::SetTrue)
        )
        .get_matches();
    cmd
}

fn get_term_size() -> Option<(u32, u32)> {
    if let Some((w, h)) = term_size::dimensions() {
        Some((w as u32, h as u32))
    } else {
        eprintln!("\x1b[33;1mwarning\x1b[0m: Unable to get terminal size");
        None
    }
}

fn img_to_ascii(mut img: DynamicImage, settings: Settings) -> Result<(), ()> {
    // Resize image to fit desired ascii output size
    if settings.preserve_aspect_ratio {
        img = img.resize(settings.width, settings.height, FilterType::Triangle);
    } else {
        img = img.resize_exact(settings.width, settings.height, FilterType::Triangle);
    }

    // Convert 'DynamicImage' type to 'RGB32FImage'
    let img = img.to_rgb32f();

    let mut i = 0;
    let mut ascii_img = String::new();

    for pixel in img.pixels() {
        // If at end of pixel row print newline
        if i == img.width() - 1 {
            ascii_img += "\n";
            i = 0;

        } else {
            let rgb = [pixel[0], pixel[1], pixel[2]];

            // Determine what ASCII character to use for this pixel
            let brightness = if settings.color {
                0.267 * rgb[0] + 0.642 * rgb[1] + 0.091 * rgb[2]
            } else {
                0.2126 * rgb[0] + 0.7152 * rgb[1] + 0.0722 * rgb[2]
            };
            let size = settings.char_set.len() - 1;
            let index = (size as f32 * brightness).round() as usize;

            // Creates the ascii pixel from two ascii characters, colors it if needed
            let ascii_pixel = if settings.color {
                truecolor(rgb, settings.char_set[index])
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
        match fs::write(settings.output_file, ascii_img) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("\x1b[31;1merror\x1b[0m: Failed to write to output file");
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
fn truecolor(rgb: [f32; 3], ascii_char: char) -> String{
    format!("\x1b[38;2;{};{};{}m{}{}",
        (rgb[0] * 255.).round() as i32,
        (rgb[1] * 255.).round() as i32,
        (rgb[2] * 255.).round() as i32,
        ascii_char, ascii_char
    )
}
