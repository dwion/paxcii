//! Cli for paxcii.
//!
//! Also a good example on how to use paxcii library side.

use clap::{value_parser, Arg, ArgAction, ArgMatches, Command as ClapCommand};
use paxcii::{
    self,
    img::*,
    video::{ffmpeg_video, video_to_ascii},
    PaxciiSettings,
};
use std::{fs, process::ExitCode};

// Bold and red 'error: '
const ERR_MSG: &str = "\x1b[31;1merror\x1b[0m: ";

fn main() -> ExitCode {
    let cmd = create_cli();

    let processed_args = process_args(cmd);
    let mut settings = processed_args.settings;
    let path = processed_args.path;

    // Video
    if processed_args.video {
        // Get video bytes
        let video = match ffmpeg_video(&path, &mut settings) {
            Ok(x) => x,
            Err(err) => {
                eprintln!("{}Failed to get video bytes: {}", ERR_MSG, err);
                return ExitCode::from(1);
            }
        };

        let ascii_video = video_to_ascii(video, &settings);

        // Write bash script that plays video if requested by user
        if let Some(filename) = processed_args.output_file {
            match ascii_video.write_bash_script(&filename) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("{}failed to write bash script: {}", ERR_MSG, err);
                    return ExitCode::from(1);
                }
            };

        // Play with audio if requested by user
        } else if let Some(file_path) = processed_args.audio_file {
            if let Err(err) = ascii_video.play_with_audio(&file_path) {
                eprintln!("{}failed to play audio: {}", ERR_MSG, err);
                return ExitCode::from(1);
            };
        } else {
            ascii_video.play();
        }

    // Webcam
    } else if let Some(webcam_index) = processed_args.webcam {
        if let Err(err) = paxcii::webcam(webcam_index, &settings) {
            eprintln!("{}webcam failed: {}", ERR_MSG, err);
            return ExitCode::from(1);
        };

    // Image
    } else {
        // Open image
        let img = match image::open(&path) {
            Ok(x) => x,
            Err(err) => {
                eprintln!("{}failed to open image: {}", ERR_MSG, err);
                return ExitCode::from(1);
            }
        };

        let ascii = img_to_ascii(img, &settings, true);

        // Try to write ascii image to output file as shell script
        if let Some(filename) = processed_args.output_file {
            match fs::write(&filename, format!(r#"echo -e "{}""#, ascii)) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("{}failed to write to output file: {}", ERR_MSG, err);
                    return ExitCode::from(1);
                }
            }
        } else {
            print!("{}", ascii);
        }
    }
    ExitCode::from(0)
}

#[derive(Default)]
// Result of `process_args`
struct ProcessedArgs {
    settings: PaxciiSettings,
    path: String,
    video: bool,
    output_file: Option<String>,
    audio_file: Option<String>,
    webcam: Option<u32>,
}

fn process_args(cmd: ArgMatches) -> ProcessedArgs {
    let mut args = ProcessedArgs::default();

    // Get path of input file
    if let Some(x) = cmd.get_one::<String>("image") {
        args.path = x.clone();
    } else if let Some(x) = cmd.get_one::<String>("video") {
        args.video = true;
        args.path = x.clone();
    }

    // Get output file path
    if let Some(x) = cmd.get_one::<String>("output-file") {
        args.output_file = Some(x.clone());
    }

    // Get audio file path
    if let Some(x) = cmd.get_one::<String>("audio-file") {
        args.audio_file = Some(x.clone());
    }

    // Get webcam index
    if let Some(x) = cmd.get_one::<u32>("webcam") {
        args.webcam = Some(*x);
    }

    if cmd.get_flag("no-color") {
        args.settings.color = false;
        args.settings.char_set = Vec::from(CHARS_LIGHT);
    }

    if cmd.get_flag("no-preserve-aspect-ratio") {
        args.settings.preserve_aspect_ratio = false;
    }

    // Get character set
    if let Some(x) = cmd.get_one::<String>("char-set") {
        let c = x.clone();

        args.settings.char_set = match c.as_str() {
            "light" => Vec::from(CHARS_LIGHT),
            "medium" => Vec::from(CHARS_MEDIUM),
            "filled" => Vec::from(CHARS_FILLED),
            _ => {
                eprintln!("{}Invalid value for argument 'char-set'. Value can only be: light/medium/filled. `medium` will be used", ERR_MSG);
                Vec::from(CHARS_MEDIUM)
            }
        }
    }

    // Try to get terminal size from `get_term_size` and use it if we can
    if let Some(s) = get_term_size() {
        (args.settings.width, args.settings.height) = s;
    }

    if let Some(w) = cmd.get_one::<u32>("width") {
        args.settings.width = *w;
    }
    if let Some(h) = cmd.get_one::<u32>("height") {
        args.settings.height = *h;
    }

    args
}

fn get_term_size() -> Option<(u32, u32)> {
    if let Some((w, h)) = term_size::dimensions() {
        Some((w as u32, h as u32))
    } else {
        eprintln!("\x1b[33;1mwarning\x1b[0m: Unable to get terminal size");
        None
    }
}

// Creates the cli interface
fn create_cli() -> ArgMatches {
    let cmd = ClapCommand::new("paxcii")
        .about("Transform images and videos to ascii")
        .version("0.5.0")
        .arg_required_else_help(true)
        .arg(
            Arg::new("image")
                .short('i')
                .long("image")
                .help("Path to input image file")
                .value_name("example.jpg")
                .num_args(1)
                .action(ArgAction::Set)
                .required_unless_present_any(["video", "webcam"])
        )
        .arg(
            Arg::new("video")
                .short('v')
                .long("video")
                .help("Path to input video file")
                .value_name("example.mp4")
                .num_args(1)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("output-file")
                .short('o')
                .long("output-file")
                .help("\
                    Print ascii output to shell script with specified name instead of stdout. \
                    Then you can run the shell script to see the output.")
                .value_name("example.sh")
                .num_args(1)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("audio-file")
                .short('a')
                .long("audio-file")
                .help("Path to audio file to be played while the video is running. Supported fromats: mp3/wav/ogg/flac/m4a/aac")
                .value_name("example.mp3")
                .num_args(1)
                .conflicts_with_all(["image", "webcam", "output-file"])
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("webcam")
            .short('e')
            .long("webcam")
            .help("Take input from webcam with specified index")
            .value_name("0")
            .num_args(1)
            .value_parser(value_parser!(u32))
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
                .value_name("30")
                .num_args(1)
                .value_parser(value_parser!(u32))
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("height")
                .short('H')
                .long("height")
                .help("Ascii output height. Uses terminal height by default.")
                .value_name("30")
                .num_args(1)
                .value_parser(value_parser!(u32))
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("no-preserve-aspect-ratio")
                .short('p')
                .long("no-preserve-aspect-ratio")
                .help("Doesn't preserve aspect ratio of input in output")
                .action(ArgAction::SetTrue)
        )
        .get_matches();
    cmd
}

