// TODO (maybe)
// usage example
// AUR

//! Transform images and videos to ascii.
//!
//! <img src="https://i.ibb.co/PxCqqvw/example.png" alt="example"><br>
//! <https://www.youtube.com/watch?v=-JT_XlLnAas>
//!
//! ## Dependencies
//! You will need ffmpeg and linux.\
//! Also you should have a fast terminal for videos.
//!
//! ## How to use
//! The docs have basic documentation.\
//! The cli can also serve as an example.
//!
//! ## Features
//! There are two features: webcam and audio.
//!
//! The webcam uses the [nokhwa](https://crates.io/crates/nokhwa) crate.\
//! The audio uses the [rodio](https://crates.io/crates/rodio) crate.

#[cfg(feature = "audio")]
pub mod audio;
pub mod img;
pub mod settings;
pub mod video;
#[cfg(feature = "webcam")]
pub mod webcam;

pub use img::AsciiImage;
pub use settings::PaxciiSettings;
pub use video::AsciiVideo;
#[cfg(feature = "webcam")]
pub use webcam::webcam;
