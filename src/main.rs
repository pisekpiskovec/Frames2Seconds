use text_io::read;

static mut MAX_FPS: f32 = 29.97;

fn max_fps_settings() {
    print!("Enter file's FPS: ");
    unsafe {
     MAX_FPS = read!();   
    }
}

fn main() {
    let cin: String = read!();
}
