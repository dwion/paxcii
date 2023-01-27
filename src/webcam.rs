use anyhow;
use nokhwa::{Camera, utils::*, pixel_format::RgbFormat};
use image::{DynamicImage};
use std::io::{stdout, Write};
use crate::{PaxciiSettings, img::img_to_ascii};

/// Prints webcam input to stdout. Uses the `nokhwa` crate for capturing webcam input.
pub fn webcam(camera_index: u32, s: &PaxciiSettings) -> anyhow::Result<()> {
    // First camera in system
    let index = CameraIndex::Index(camera_index); 

    // Request the absolute highest frame rate CameraFormat that can be decoded to RGB.
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    // Start camera
    let mut camera = Camera::new(index, requested)?;
    camera.open_stream()?;

    let mut lock = stdout().lock();

    loop {
        // Convert frame from camera to ascii and print it to stdout
        match camera.frame() {
            Ok(frame) => {
                // Decode frame
                let frame = DynamicImage::from(frame.decode_image::<RgbFormat>()?);

                let ascii = img_to_ascii(frame, s, true);
                
                // Write frame to stdout
                write!(lock, "27[2J{}", ascii).expect("Failed to write to stdout");
            },
            Err(err) => {
                eprintln!("webcam error: {}", err);
                break
            }
        }
    }
    Ok(())
}