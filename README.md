# Frames2Seconds

Convert frame of second to milliseconds.

## Install
Download the latest release and run it in a terminal.

## Usage
```bash
# To use the internal shell run
frames2seconds

# You can specify files FPS on startup (default is 29.97 FPS)
frames2seconds -m <number>

# Or run the calculation one-time only
frames2seconds -f <number>
```

## Internal shell
When run, you'll be welcomed with promt `Enter frame number: `.
Unless specified (command argument), the default value of file's FPS is `29.97`. To change the value, enter non-numeric value and new prompt will appear - `Enter file's FPS: `.

### Clipboard
The result is copied to the clipboard by default. To alternate this preference run with an argument
```bash
frames2seconds -d
```

## Build
```bash
# Clone the repository
git clone https://github.com/pisekpiskovec/Frames2Seconds.git

# Build (debug version)
cd Frames2Seconds
cargo build
```
