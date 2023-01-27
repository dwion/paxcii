use crate::{img::img_to_ascii, settings::PaxciiSettings};
use image::{RgbImage, DynamicImage};
use std::{thread, time, process::Command};
use std::io::{self, stdout, Write};
use std::fs;
use anyhow;

/// Struct holding the ascii video, output of [`video_to_ascii`]
pub struct AsciiVideo {
    fps: u64,
    frames: Vec<String>,
}

impl AsciiVideo {
    /// Print the ascii video to stdout
    pub fn play(self) {
        let mut lock = stdout().lock();

        for frame in &self.frames {
            let instant = time::Instant::now();

            // Write frame to stdout
            write!(lock, "\x1b[2J{}", frame).expect("Failed to write to stdout");

            // Sleep between frames
            thread::sleep(time::Duration::from_micros((1_000_000 / self.fps) - instant.elapsed().as_micros() as u64));
        }
    }

    /// Write a bash script with the specified file name that plays the video in the terminal
    pub fn write_bash_script(self, filename: &str) -> anyhow::Result<()> {
        // String that will hold the final script
        // 24 is size of bash script added to each frame to print it, note this can vary a bit
        let mut script = String::with_capacity(self.frames.len() * (self.frames[0].len() + 24));

        for frame in &self.frames {
            // wrap frame in echo command, follow it by sleep command
            script.push_str(&format!("echo -e \"\x1b[2J{}\"\nsleep {}\n", frame, 1. / self.fps as f32))
        }

        // Write bash script to file
        fs::write(filename, script)?;
        Ok(())
    }
}

/// Transforms a whole video into ascii.
/// Takes a videos data in rgb bytes and returns a vector containing the ascii frames.
/// 
/// Input video must be resized to the values in [`PaxciiSettings`].
/// If the video comes from [`ffmpeg_video`] you don't have to worry.
/// 
/// Use the `play` method to print the video to stdout.
pub fn video_to_ascii(video: Vec<u8>, s: &PaxciiSettings) -> AsciiVideo {
    // Byte size of one frame
    let frame_size = s.width * s.height * 3;

    // Variable that will hold the frames of the ascii video
    let mut frames = vec![String::with_capacity(video.len() / frame_size as usize)];

    // for frame in video
    for i in 0..(video.len() as u32 / frame_size) {
        // Get bytes of one frame
        let frame = video[(i * frame_size) as usize..((i + 1) * frame_size) as usize].to_owned();
        // Convert bytes to `DynamicImage`
        let frame = 
            DynamicImage::ImageRgb8(RgbImage::from_raw(s.width, s.height, frame)
            .expect("Error in `video_to_ascii` function when converting bytes to `DynamicImage`. \
                Problem might lie in width and height values in `PaxciiSettings`.")
        );

        // Add ascii frame to video
        frames.push(img_to_ascii(frame, &s, false));
    }
    AsciiVideo { fps: s.fps, frames }
}

/// Uses the ffmpeg and ffprobe command to split video into frames and
/// if necessary change fps, width and height values in [`PaxciiSettings`].
/// Returns the rgb bytes of the video that you can then give to [`video_to_ascii`].
pub fn ffmpeg_video(path: &str, mut s: &mut PaxciiSettings) -> anyhow::Result<Vec<u8>> {
    let vsize;
    (vsize, s.fps) = ffprobe_video_info(path)?;
    if s.preserve_aspect_ratio {
        (s.width, s.height) = keep_aspect_ratio(vsize, [s.width, s.height]);
    }

    let cmd = Command::new("ffmpeg")
        .args(["-i", &path])
        .args(["-vf", &format!("format=rgb24, scale={}:{}", s.width, s.height)])
        .args(["-f", "rawvideo"])
        .arg("-")
        .output()?;

    if cmd.status.success() {
        Ok(cmd.stdout)
    } else {
        eprintln!("ffmpeg stderr: {}", String::from_utf8_lossy(&cmd.stderr));
        Err(io::Error::new(io::ErrorKind::Other, "ffmpeg error").into())
    }
}

/// Returns width, height and fps of the specified video using the ffprobe command.
/// 
/// Used in [`ffmpeg_video`].
pub fn ffprobe_video_info(path: &str) -> anyhow::Result<([u32; 2], u64)> {
    let cmd = Command::new("ffprobe")
        .args(["-select_streams", "v:0"])
        .args(["-show_entries", "stream=width,height,r_frame_rate"])
        .args(["-of", "csv=s=x:p=0"])
        .arg(path)
        .output()?;

    if cmd.status.success() {
        let string = String::from_utf8(cmd.stdout)?;
        let vector = string.trim().split("x").collect::<Vec<&str>>();
        let w = vector[0].parse::<u32>()?;
        let h = vector[1].parse::<u32>()?;

        let fps = vector[2].split("/").collect::<Vec<&str>>()[0].parse::<u64>()?;

        Ok(([w, h], fps))
    } else {
        eprintln!("ffprobe stderr: {}", String::from_utf8_lossy(&cmd.stderr));
        Err(io::Error::new(io::ErrorKind::Other, "ffprobe error").into())
    }
}

/// Resizes to keep aspect ratio. Returns one `new_size` with one of the values modified.
/// Larger value is resized. For example if width is smaller than height then height gets resized and width stays the same.
/// `original_size` is the dimensions of the input video or image.
fn keep_aspect_ratio(original_size: [u32; 2], new_size: [u32; 2]) -> (u32, u32) {
    let wratio = new_size[0] as f32 / original_size[0] as f32;
    let hratio = new_size[1] as f32 / original_size[1] as f32;
    let ratio = wratio.min(hratio);

    let w = (original_size[0] as f32 * ratio).round() as u32;
    let h = (original_size[1] as f32 * ratio).round() as u32;   
    (w, h)
}