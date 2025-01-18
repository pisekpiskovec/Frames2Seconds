use regex::Regex;
use text_io::read;

fn max_fps_settings() -> f32 {
    print!("Enter file's FPS: ");
    read!()
}

fn calculate_miliseconds(max_dot_seconds: u32, max_fps: f32, frame_of_second: u32) -> u32 {
    ((max_dot_seconds * frame_of_second) as f32 / max_fps) as u32
}

fn main() {
    const MAX_DOT_SECONDS: u32 = 999999999;

    let regexp = Regex::new(r"[\d]+$").unwrap();
    let mut max_fps: f32 = max_fps_settings();
    loop {
        print!("Enter frame number: ");
        let cin: String = read!();

        if regexp.is_match(&cin) {
            let result =
                calculate_miliseconds(MAX_DOT_SECONDS, max_fps, cin.parse::<u32>().unwrap());
            println!("{}", result);
        } else {
            max_fps = max_fps_settings();
        }
    }
}
