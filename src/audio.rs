use crate::video::AsciiVideo;
use anyhow::bail;
use rodio::Decoder;
use std::{
    io::{BufReader, Cursor},
    process::Command,
};

impl AsciiVideo {
    /// Separates audio from video and encodes it in mp3.
    pub fn ffmpeg_audio(&mut self, path: &str) -> anyhow::Result<()> {
        let cmd = Command::new("ffmpeg")
            .args(["-i", &path])
            .arg("-vn")
            .args(["-f", "mp3"])
            .arg("-")
            .output()?;
        if cmd.status.success() {
            self.audio = Some(cmd.stdout);
            Ok(())
        } else {
            bail!("ffmpeg stderr: {}", String::from_utf8_lossy(&cmd.stderr))
        }
    }
    /// Plays video in terminal with audio
    pub fn play_with_audio(mut self) -> anyhow::Result<()> {
        if self.audio.is_none() {
            bail!("Can't play video in terminal. `audio` is None")
        }
        if self.ascii_frames.is_none() {
            bail!("`ascii_frames` is empty. Can't play video in terminal.")
        }
        let (_stream, handle) = rodio::OutputStream::try_default()?;
        let sink = rodio::Sink::try_new(&handle)?;

        // Decode audio file
        let decoder = Decoder::new(BufReader::new(Cursor::new(self.audio.take().unwrap())))?;
        sink.append(decoder);

        sink.play();
        self.play()?;
        Ok(())
    }
}
