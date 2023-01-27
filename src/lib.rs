// TODO
// (?) - will maybe do
// crates.io
// Clean code
// Optimize
// Make webcam and audio an optional feature (?)
// Option for display in 8 bit color (?)
// https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
// Mic audio over webcam (?)

//! Transform images and videos to ascii.
//! 
//! <img src="https://i.ibb.co/PxCqqvw/example.png" alt="example"><br>
//! <https://www.youtube.com/watch?v=-JT_XlLnAas>
//! 
//! ## Dependencies
//! You will need ffmpeg and linux
//! 
//! ## How to use
//! The docs have basic documentation.
//! For an example go take a look at the cli source code.
//! 
//! ## Features
//! There are two features: webcam and audio. Both are enabled by default.
//! 
//! The webcam uses the [nokhwa](https://crates.io/crates/nokhwa) crate.\
//! The audio uses the [rodio](https://crates.io/crates/rodio) crate.
//! 
//! You can disable them by adding `default-features = false` to the dependency like so:
//! ```toml
//! paxcii = { version = "*", default-features = false }
//! ```

pub mod img;
pub mod video;
pub mod settings;
#[cfg(feature = "webcam")]
pub mod webcam;
#[cfg(feature = "audio")]
pub mod audio;

pub use settings::PaxciiSettings;
#[cfg(feature = "webcam")]
pub use webcam::webcam;