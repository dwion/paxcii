use crate::{AsciiImage, PaxciiSettings};
use anyhow::{self, bail};
use image::DynamicImage;
use nokhwa::{pixel_format::RgbFormat, utils::*, Camera};
use std::io::{stdout, Write};

/// Prints webcam input to stdout. Uses the `nokhwa` crate for capturing webcam input.
pub fn webcam(camera_index: u32, settings: &PaxciiSettings) -> anyhow::Result<()> {
    let mut ascii_image = AsciiImage {
        settings: settings.clone(),
        image: None,
        ascii: None,
    };

    // First camera in system
    let index = CameraIndex::Index(camera_index);
    // Request the absolute highest frame rate CameraFormat that can be decoded to RGB.
    let requested =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
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

                ascii_image.image = Some(frame);
                ascii_image.image_to_ascii(true);

                // Write frame to stdout
                write!(lock, "\x1b[2J{}", ascii_image.ascii.take().unwrap())?;
            }
            Err(err) => {
                bail!("webcam error: {err}");
            }
        }
    }
}
