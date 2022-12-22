Transform images and videos to ascii with a cli tool.
Remember this is still a work in progress. If you find a bug or a possible improvement you can open an issue.

<img src="example.png" width="500">

# Installation
Can currently only be installed by cloning this repository and compiling the project
```shell
git clone https://github.com/enedon/paxcii.git
cd paxcii
cargo build --release
./target/release/paxcii
```

# How to use
```shell
Transform images and videos to ascii

Usage: paxcii [OPTIONS]

Options:
  -i, --image <example.jpg>        Path to input image file
  -v, --video <video>              Path to input video file
  -o, --output-file <example.txt>  Print output to specified file instead of stdout
  -n, --no-color                   Prints ascii without colors
  -c, --char-set <char-set>        Choose character set to use for ascii. Options: light(default when color is disabled)/medium(default)/filled
  -W, --width <width>              Ascii output width. Uses terminal width by default.
  -H, --height <height>            Ascii output height. Uses terminal height by default.
      --no-preserve-aspect-ratio   Doesn't preserve aspect ratio of input in output
  -h, --help                       Print help information
  -V, --version                    Print version information
```