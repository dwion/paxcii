use crate::{Settings, ERROR_MSG, img::img_to_ascii};
use image::{ImageBuffer, Rgb};
use std::{thread, time, process::Command, cmp::max, fs};
use crate::audio::play_audio;

pub fn video_to_ascii(video: Vec<u8>, settings: Settings, fps: u64) -> Result<(), ()> {
    // Byte size of one frame
    let frame_size = settings.width * settings.height * 3;

    // String for output file
    let mut file = String::new();

    // for frame in video
    for i in 0..(video.len() as u32 / frame_size) {
        let instant = time::Instant::now();

        // Get bytes of one frame
        let frame = &video[(i * frame_size) as usize..((i + 1) * frame_size) as usize];

        // Construct RgbImage from bytes
        let frame: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(settings.width, settings.height, frame.to_vec()).unwrap();

        let mut ascii = img_to_ascii(frame, &settings);

        if settings.output_file.is_empty() {
            // Clear screen and print frame
            print!("27[2J{}", ascii);
        } else {
            ascii.push_str("\n");
            file.push_str(&format!("echo -e \"27[2J{}\"\nsleep {} \n", ascii, 1. / fps as f32));
            continue
        }

        // Play audio if first frame and requested by user
        if i == 0 {
            if !settings.audio_file.is_empty() {
                let audio_path = settings.audio_file.clone();
                thread::spawn(|| {
                    match play_audio(audio_path) {
                        Ok(_) => (),
                        Err(_) => panic!()
                    }
                });
            }
        }

        // Sleep between frames
        thread::sleep(time::Duration::from_micros((1_000_000 / fps) - instant.elapsed().as_micros() as u64));
    }

    // Write video to file if requested by user
    if !settings.output_file.is_empty() {
        match fs::write(settings.output_file, file) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("{}Failed to write to output file. {}", ERROR_MSG, err);
                return Err(())
            }
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
