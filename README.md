Transform images and videos to ascii with a cli tool.<br>
Remember this is still a work in progress. Contributions are welcome

# Examples
<img src="example.png" width="500"><br>
https://www.youtube.com/watch?v=-JT_XlLnAas

# Dependencies
You will need ffmpeg and linux. Might also work on MacOS but I haven't tested.

# Installation
Can currently only be installed by cloning this repository and compiling the project yourself
```
git clone https://github.com/enedon/paxcii.git
cd paxcii
cargo build --release
./target/release/paxcii
```

# How to use
```
Transform images and videos to ascii

Usage: paxcii [OPTIONS]

Options:
  -i, --image <example.jpg>       Path to input image file
  -v, --video <example.mp4>       Path to input video file
  -o, --output-file <example.sh>  Print ascii output to shell script with specified name instead of stdout. Then you can run the shell script to see the output
  -a, --audio-file <example.mp3>  Path to audio file to be played while the video is running. Supported fromats: mp3/wav/ogg/flac/m4a/aac
  -n, --no-color                  Prints ascii without colors
  -c, --char-set <char-set>       Choose character set to use for ascii. Options: light(default when color is disabled)/medium(default)/filled
  -W, --width <width>             Ascii output width. Uses terminal width by default.
  -H, --height <height>           Ascii output height. Uses terminal height by default.
      --no-preserve-aspect-ratio  Doesn't preserve aspect ratio of input in output
  -h, --help                      Print help information
  -V, --version                   Print version information
```