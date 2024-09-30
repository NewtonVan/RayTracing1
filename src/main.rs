mod color;
mod ray;
mod vec3;

use color::write_color;
use flexi_logger::{Logger, WriteMode};
use log::info;
use ray::Ray;
use std::io;
use vec3::{Point3, Vec3};

fn main() {
    // Initialize the logger with buffered output and directing to stderr
    Logger::try_with_str("info")
        .unwrap()
        .write_mode(WriteMode::BufferAndFlush)
        .log_to_stderr() // Ensure output goes to stderr
        .start()
        .unwrap();

    // Image size
    let aspect_ratio: f32 = 16.0 / 9.0;
    let img_width = 400;
    let img_height = {
        let img_height = (img_width as f32 / aspect_ratio) as i32;
        img_height.max(1)
    };

    // Camera
    let focal_length = 1f32;
    let viewport_height = 2f32;
    let viewport_width = viewport_height * (img_width as f32 / img_height as f32);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // viewport space
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / img_width as f32;
    let pixel_delta_v = viewport_v / img_height as f32;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    println!("P3\n{} {}\n255", img_width, img_height);
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for j in 0..img_height {
        info!("Scanlines remaining: {}", img_width - j);
        for i in 0..img_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f32) + (pixel_delta_v * j as f32);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r);

            write_color(&mut handle, pixel_color).unwrap();
        }
    }
}

#[allow(unused_variables)]
pub fn ray_color(r: &Ray) -> image::Rgba<u8> {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        Vec3::new(1.0, 0.0, 0.0).rgba()
    } else {
        let unit_dir = r.direction.unit();
        let a = 0.5 * (unit_dir.y + 1.0);

        (Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a).rgba()
    }
}

pub fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> bool {
    let oc = *center - r.origin;
    let a = r.direction.dot(&r.direction);
    let b = -2.0 * r.direction.dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminannt = b * b - 4.0 * a * c;

    discriminannt >= 0.0
}
