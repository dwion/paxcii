//! Cli for paxcii.
//!
//! Also a good example on how to use paxcii library side.

use clap::{value_parser, Arg, ArgAction, ArgMatches, Command as ClapCommand};
use paxcii::*;

// Bold and red 'error: '
const ERR_MSG: &str = "\x1b[31;1merror\x1b[0m: ";

fn main() -> anyhow::Result<()> {
    let cmd = create_cli();

    let args = process_args(cmd);

    if args.video {
        eprint!("Opening video...");
        let mut ascii_video = AsciiVideo::open(&args.path, args.settings)?;
        eprintln!("Done");
        if let Some(output_file) = args.output_file {
            ascii_video.write_bash_script(&output_file)?;
        } else {
            if args.audio {
                eprint!("Extracting audio...");
                ascii_video.ffmpeg_audio(&args.path)?;
                eprintln!("Done");
                ascii_video.play_with_audio()?;
            } else {
                ascii_video.play()?;
            }
        }
    } else if let Some(index) = args.webcam {
        webcam(index, &args.settings)?;
    } else {
        let ascii_img = AsciiImage::open(&args.path, args.settings)?;
        if let Some(output_file) = args.output_file {
            ascii_img.write(&output_file)?;
        } else {
            ascii_img.print();
        }
    }

    Ok(())
}

// Result of `process_args`
#[derive(Default)]
struct ProcessedArgs {
    settings: PaxciiSettings,
    path: String,
    video: bool,
    audio: bool,
    output_file: Option<String>,
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

    if cmd.get_flag("audio") {
        args.audio = true;
    }

    // Get webcam index
    if let Some(x) = cmd.get_one::<u32>("webcam") {
        args.webcam = Some(*x);
    }

    if cmd.get_flag("no-color") {
        args.settings.color = false;
        args.settings.chars_light();
    }

    if cmd.get_flag("no-preserve-aspect-ratio") {
        args.settings.keep_aspect_ratio = false;
    }

    // Get character set
    if let Some(x) = cmd.get_one::<String>("char-set") {
        let c = x.clone();

        match c.as_str() {
            "light" => args.settings.chars_light(),
            "medium" => args.settings.chars_medium(),
            "filled" => args.settings.chars_filled(),
            _ => {
                eprintln!("{}Invalid value for argument 'char-set'. Value can only be: light/medium/filled. medium will be used", ERR_MSG);
                args.settings.chars_medium()
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
        .version("0.6.0")
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
                    Print ascii output to file with specified path instead of stdout. \
                    For image the ascii is written as is into the file. \
                    For video a bash script is created that plays the video when executed")
                .value_name("example.sh")
                .num_args(1)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("audio")
                .short('a')
                .long("audio")
                .help("Separates audio from video using ffmpeg and then plays the audio at the same time as the video.")
                .conflicts_with_all(["image", "webcam", "output-file"])
                .action(ArgAction::SetTrue)
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
                .help("Prints image without colors")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("char-set")
                .short('c')
                .long("char-set")
                .help("Choose character set to use for result. Options: light(default with no color)/medium(default)/filled")
                .num_args(1)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("width")
                .short('W')
                .long("width")
                .help("Image output width. Uses terminal width by default.")
                .value_name("30")
                .num_args(1)
                .value_parser(value_parser!(u32))
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("height")
                .short('H')
                .long("height")
                .help("Image output height. Uses terminal height by default.")
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
