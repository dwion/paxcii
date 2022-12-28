use rodio::Decoder;
use std::io::BufReader;
use std::fs::File;
use crate::ERROR_MSG;

pub fn play_audio(path: String) -> Result<(), ()> {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = match File::open(path) {
        Ok(x) => x,
        Err(err) => {
            eprintln!("{}Failed to open input file for audio. {}", ERROR_MSG, err);
            return Err(());
        }
    };

    let decoder = match Decoder::new(BufReader::new(file)) {
        Ok(x) => x,
        Err(err) => {
            eprintln!("{}Failed to decode audio file. {}", ERROR_MSG, err);
            return Err(());
        }
    };

    sink.append(decoder);
    sink.sleep_until_end();
    Ok(())
}