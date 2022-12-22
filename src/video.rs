use crate::{Settings, ERROR_MSG, img::img_to_ascii};
use image::{ImageBuffer, Rgb};
use std::{thread, time::Duration, process::Command, cmp::max};

pub fn video_to_ascii(video: Vec<u8>, settings: Settings, fps: u64) -> Result<(), ()> {
    // Byte size of one frame
    let frame_size = settings.width * settings.height * 3;

    // for frame in video
    for i in 0..(video.len() as u32 / frame_size) {
        // Get bytes of one frame
        let frame = &video[(i * frame_size) as usize..((i + 1) * frame_size) as usize];

        // Construct RgbImage from bytes
        let frame: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(settings.width, settings.height, frame.to_vec()).unwrap();

        thread::sleep(Duration::from_millis(1000 / fps));

        // Clear screen for next frame
        print!("{}[2J", 27 as char);

        match img_to_ascii(frame, &settings) {
            Ok(_) => (),
            Err(_) => return Err(())
        }
    }
    Ok(())
}

// Gets width, height and fps of input video
pub fn get_video_info(path: &str) -> Result<([u32; 2], u64), ()> {
    let cmd = Command::new("ffprobe")
        .args(["-select_streams", "v:0"])
        .args(["-show_entries", "stream=width,height,r_frame_rate"])
        .args(["-of", "csv=s=x:p=0"])
        .arg(path)
        .output().expect("failed to execute ffprobe");

    if cmd.status.success() {
        let string = String::from_utf8(cmd.stdout).unwrap();
        let vector = string.trim().split("x").collect::<Vec<&str>>();
        let w = vector[0].parse::<u32>().unwrap();
        let h = vector[1].parse::<u32>().unwrap();

        let fps = vector[2].split("/").collect::<Vec<&str>>()[0].parse::<u64>().unwrap();

        Ok(([w, h], fps))
    } else {
        eprintln!("{}ffprobe failed to run. Exit code: {}", ERROR_MSG, cmd.status);
        Err(())
    }
}

// Resizes video dimensions keeping aspect ratio
pub fn resize_video_dimensions(vsize: [u32; 2], settings: &Settings) -> (u32, u32) {
    let wratio = settings.width as f32 / vsize[0] as f32;
    let hratio = settings.height as f32 / vsize[1] as f32;

    let ratio = wratio.min(hratio);
    let nw = max((vsize[0] as f32 * ratio).round() as u32, 1);
    let nh = max((vsize[1] as f32 * ratio).round() as u32, 1);

    (nw as u32, nh as u32)
}

// Splits video into frames and sends the raw data to stdout
pub fn ffmpeg_cmd(vsize: [u32; 2], path: &str) -> Vec<u8> {
    let cmd = Command::new("ffmpeg")
        .args(["-i", &path])
        .args(["-vf", &format!("format=rgb24, scale={}:{}", vsize[0], vsize[1])])
        .args(["-f", "rawvideo"])
        .arg("-")
        .output().expect("failed to execute ffmpeg");

    cmd.stdout
}
