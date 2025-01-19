use regex::Regex;
use text_io::read;

fn max_fps_settings() -> f32 {
    print!("Enter file's FPS: ");
    read!()
}

fn calculate_miliseconds(max_dot_seconds: u32, max_fps: f32, frame_of_second: u32) -> u32 {
    if max_fps <= 0.0 { eprintln!("Error: Can't divide with 0!"); return 0;}
    let tmp_a: u128 = max_dot_seconds as u128 * frame_of_second as u128;
    let tmp_b: f64 = (tmp_a as f64) / max_fps as f64;
    tmp_b as u32
}

fn main() {
    const MAX_DOT_SECONDS: u32 = 999999999;

    let regexp = Regex::new(r"[\d]+$").unwrap();
    let mut max_fps: f32 = max_fps_settings();
    loop {
        print!("Enter frame number: ");
        let cin: String = read!();
        print!("Enter frame number (or letters to reconfigure max FPS, press C-c to quit): ");

        if regexp.is_match(&cin) {
            let result =
                calculate_miliseconds(MAX_DOT_SECONDS, max_fps, cin.parse::<u32>().unwrap());
            println!("{}", result);
        } else {
            max_fps = max_fps_settings();
        }
    }
}
