mod camera;
mod color;
mod ray;
mod rtweekend;
mod vec3;

use std::sync::Arc;

use camera::Camera;
use flexi_logger::{Logger, WriteMode};
use ray::{HittableList, Sphere};
use vec3::Point3;

fn main() {
    // Initialize the logger with buffered output and directing to stderr
    Logger::try_with_str("info")
        .unwrap()
        .write_mode(WriteMode::BufferAndFlush)
        .log_to_stderr() // Ensure output goes to stderr
        .start()
        .unwrap();

    // world
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.img_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world);
}
