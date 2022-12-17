Transform images to ascii art with a cli tool

<img src="example.png" width="500">

# Installation
Can currently only be installed by cloning this repository and compiling the project yourself
```shell
git clone https://github.com/enedon/paxcii.git
cd paxcii
cargo build --release
./target/release/paxcii
```

# How to use
```shell
Transform images to ascii art

Usage: paxcii [OPTIONS] --image <image>

Options:
  -i, --image <image>             Path to input image file
  -n, --no-color                  Prints ascii without colors
  -c, --char-set <char-set>       Choose character set to use for ascii. Options: light(default when color is disabled)/medium(default)/filled
  -W, --width <width>             Ascii output width. Uses terminal width by default.
  -H, --height <height>           Ascii output height. Uses terminal height by default.
      --no-preserve-aspect-ratio  Doesn't preserve aspect ratio of input in output
  -h, --help                      Print help information
  -V, --version                   Print version information
```