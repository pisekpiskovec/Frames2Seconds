use text_io::read;

fn max_fps_settings() -> f32 {
    print!("Enter file's FPS: ");
    let ret: f32 = read!();
    ret
}

fn main() {
    let max_fps: f32 = max_fps_settings();
    // let cin: String = read!();
    println!("{}", max_fps);
    read!();
}
