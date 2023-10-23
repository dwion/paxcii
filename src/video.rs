use crate::img::AsciiImage;
use crate::settings::PaxciiSettings;
use anyhow::{self, bail};
use image::{DynamicImage, RgbImage};
use std::fs;
use std::io::{stdout, Write};
use std::{process::Command, thread, time};

/// Struct holding the ascii video, output of `video_to_ascii`
pub struct AsciiVideo {
    pub fps: f32,
    pub settings: PaxciiSettings,
    pub ascii_frames: Option<Vec<String>>,
    pub raw_video: Option<Vec<u8>>,
    // mp3 encoded
    pub audio: Option<Vec<u8>>,
}

impl AsciiVideo {
    /// Simplest way to transform video to ascii.\
    /// Will open video and transform it to ascii.
    /// The frames can then either be played using `play` or `write_bash_script`
    /// or accessed from `ascii_frames` under the returned [`AsciiVideo`] struct.
    pub fn open(path: &str, settings: PaxciiSettings) -> anyhow::Result<Self> {
        let mut ascii_video = AsciiVideo::new(settings);
        ascii_video.ffmpeg(path)?;
        ascii_video.video_to_ascii();
        Ok(ascii_video)
    }
    pub fn new(settings: PaxciiSettings) -> Self {
        AsciiVideo {
            fps: 30.,
            settings,
            ascii_frames: None,
            raw_video: None,
            audio: None,
        }
    }
    /// Transforms a whole video into ascii.\
    /// Takes a videos data in rgb bytes and returns the ascii frames.
    ///
    /// Input video size must be same as values in [`PaxciiSettings`].
    pub fn video_to_ascii(&mut self) {
        if self.raw_video.is_none() {
            eprintln!("`raw_video` is None. Can't make ascii frames");
            return;
        }
        let raw_video = self.raw_video.as_ref().unwrap();
        let mut ascii_image = AsciiImage {
            settings: self.settings.clone(),
            image: None,
            ascii: None,
        };

        // Byte size of one frame
        let frame_size = self.settings.width * self.settings.height * 3;

        // Variable that will hold the frames of the ascii video
        self.ascii_frames = Some(vec![String::with_capacity(
            raw_video.len() / frame_size as usize,
        )]);

        // for frame in video
        for i in 0..(raw_video.len() as u32 / frame_size) {
            // Get bytes of one frame
            let frame =
                raw_video[(i * frame_size) as usize..((i + 1) * frame_size) as usize].to_owned();
            // Convert bytes to `DynamicImage`
            ascii_image.image = Some(DynamicImage::ImageRgb8(
                RgbImage::from_raw(self.settings.width, self.settings.height, frame).expect(
                    "Error in `video_to_ascii` function when converting bytes to `DynamicImage`. \
                Problem might lie in width and height values in `PaxciiSettings`.",
                ),
            ));

            // Convert frame to ascii
            ascii_image.image_to_ascii(false);
            // Add ascii frame to video
            self.ascii_frames
                .as_mut()
                .unwrap()
                .push(ascii_image.ascii.take().unwrap());
        }
    }
    /// Uses the ffmpeg and ffprobe command to split video into resized frames and
    /// change fps in [`PaxciiSettings`].\
    /// If `keep_aspect_ratio` is true then this function will also adjust width and height.\
    /// After `video_to_ascii` can be used to convert raw video to ascii frames.
    pub fn ffmpeg(&mut self, path: &str) -> anyhow::Result<()> {
        let vsize = self.ffprobe(path)?;
        (self.settings.width, self.settings.height) = if self.settings.keep_aspect_ratio {
            keep_aspect_ratio(vsize, (self.settings.width, self.settings.height))
        } else {
            (self.settings.width, self.settings.height)
        };

        let cmd = Command::new("ffmpeg")
            .args(["-i", &path])
            .args([
                "-vf",
                &format!(
                    "format=rgb24, scale={}:{}",
                    self.settings.width, self.settings.height
                ),
            ])
            .args(["-f", "rawvideo"])
            .arg("-")
            .output()?;

        if cmd.status.success() {
            self.raw_video = Some(cmd.stdout);
            Ok(())
        } else {
            bail!("ffmpeg stderr: {}", String::from_utf8_lossy(&cmd.stderr))
        }
    }

    /// Sets fps using ffprobe command.\
    /// Returns video width and height.\
    /// Used in `ffmpeg_video`.
    fn ffprobe(&mut self, path: &str) -> anyhow::Result<(u32, u32)> {
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

            let fps_division = vector[2].split("/").collect::<Vec<&str>>();
            self.fps = (fps_division[0].parse::<f32>()? / fps_division[1].parse::<f32>()?).round();

            Ok((w, h))
        } else {
            bail!("ffprobe stderr: {}", String::from_utf8_lossy(&cmd.stderr))
        }
    }
    /// Print the ascii video to stdout
    pub fn play(mut self) -> anyhow::Result<()> {
        if self.ascii_frames.is_none() {
            bail!("`ascii_frames` is empty. Can't play video in terminal.")
        }
        let mut lock = stdout().lock();
        let fps = (1_000_000. / self.fps).round() as u64;

        // Format frames
        let mut frames = Vec::with_capacity(self.ascii_frames.as_ref().unwrap().capacity());
        for frame in self.ascii_frames.take().unwrap() {
            frames.push(format!("\x1b[2J{frame}"));
        }

        for frame in frames {
            let instant = time::Instant::now();

            // Write frame to stdout
            lock.write_all(frame.as_bytes())
                .expect("Failed to write to stdout");

            // Sleep between frames if time between two frames hasn't been longer than fps
            if let Some(time) = fps.checked_sub(instant.elapsed().as_micros() as u64) {
                thread::sleep(time::Duration::from_micros(time));
            } else {
                eprintln!("{} {}", self.fps, instant.elapsed().as_micros());
                bail!("Terminal prints too slowly for video fps");
            }
        }
        Ok(())
    }
    /// Write a bash script with the specified file name that plays the video in the terminal
    pub fn write_bash_script(&self, filename: &str) -> anyhow::Result<()> {
        if self.ascii_frames.is_none() {
            bail!("`ascii_frames` is empty. Can't write video to bash script.");
        }

        let ascii_frames = self.ascii_frames.as_ref().unwrap();
        // String that will hold the final script
        // 24 is size of bash script added to each frame to print it, note this can vary a bit
        let mut script = String::with_capacity(ascii_frames.len() * (ascii_frames[0].len() + 24));

        for frame in ascii_frames {
            // wrap frame in echo command, follow it by sleep command
            script.push_str(&format!(
                "echo -e \"\x1b[2J{}\"\nsleep {}\n",
                frame,
                1. / self.fps as f32
            ))
        }

        // Write bash script to file
        fs::write(filename, script)?;
        Ok(())
    }
}

/// Resizes to keep aspect ratio. Returns one `new_size` with one of the values modified.
/// Larger value is resized. For example if width is smaller than height then height gets resized and width stays the same.
/// `original_size` is the dimensions of the input video or image.
fn keep_aspect_ratio(original_size: (u32, u32), new_size: (u32, u32)) -> (u32, u32) {
    let wratio = new_size.0 as f32 / original_size.0 as f32;
    let hratio = new_size.1 as f32 / original_size.1 as f32;
    let ratio = wratio.min(hratio);

    let w = (original_size.0 as f32 * ratio).round() as u32;
    let h = (original_size.1 as f32 * ratio).round() as u32;
    (w, h)
}
