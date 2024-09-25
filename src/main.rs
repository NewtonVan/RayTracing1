use flexi_logger::{Logger, WriteMode};
use log::info;

fn main() {
    // Initialize the logger with buffered output and directing to stderr
    Logger::try_with_str("info")
        .unwrap()
        .write_mode(WriteMode::BufferAndFlush)
        .log_to_stderr() // Ensure output goes to stderr
        .start()
        .unwrap();

    let img_width = 256;
    let img_height = 256;

    println!("P3\n{} {}\n255", img_width, img_height);
    for j in 0..img_height {
        for i in 0..img_width {
            info!("Scanlines remaining: {}", img_width - j);
            let r = i as f32 / (img_width as f32 - 1.0);
            let g = j as f32 / (img_height as f32 - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
