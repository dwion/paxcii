use clap::{Arg, ArgAction, Command as ClapCommand, ArgMatches};
use crate::{CHARS_LIGHT, CHARS_MEDIUM, CHARS_FILLED, ERROR_MSG, Settings};

pub fn create_cli() -> ArgMatches {
    let cmd = ClapCommand::new("paxcii")
        .about("Transform images and videos to ascii")
        .version("0.1.0")
        .arg_required_else_help(true)
        .arg(
            Arg::new("image")
                .short('i')
                .long("image")
                .help("Path to input image file")
                .value_name("example.jpg")
                .num_args(1)
                .action(ArgAction::Set)
                .required_unless_present("video")
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
                    Then you can run the shell script to see the output")
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

pub fn process_args(cmd: ArgMatches) -> Result<(Settings, String), ()> {
    let mut settings = Settings::default();
    let mut path = String::new();

    // Get path of input file
    if cmd.contains_id("image") {
        path = cmd.get_one::<String>("image").unwrap().clone();
    } else if cmd.contains_id("video") {
        settings.is_video = true;
        path = cmd.get_one::<String>("video").unwrap().clone();
    }

    // Get output file path
    if cmd.contains_id("output-file") {
        settings.output_file = cmd.get_one::<String>("output-file").unwrap().clone();
    }

    // Get audio file path
    if cmd.contains_id("audio-file") {
        settings.audio_file = cmd.get_one::<String>("audio-file").unwrap().clone();
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
                eprintln!("{}Invalid value for argument 'char-set'. Value can only be: light/medium/filled", ERROR_MSG);
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
                eprintln!("{}Argument 'width' must be a number", ERROR_MSG);
                return Err(());
            }
        }
    }
    if cmd.contains_id("height") {
        let h = cmd.get_one::<String>("height").unwrap();
        match h.parse::<u32>() {
            Ok(h) => settings.height = h,
            Err(_) => {
                eprintln!("{}Argument 'height' must be a number", ERROR_MSG);
                return Err(());
            }
        }
    }

    Ok((settings, path))
}

fn get_term_size() -> Option<(u32, u32)> {
    if let Some((w, h)) = term_size::dimensions() {
        Some((w as u32, h as u32))
    } else {
        eprintln!("\x1b[33;1mwarning\x1b[0m: Unable to get terminal size");
        None
    }
}