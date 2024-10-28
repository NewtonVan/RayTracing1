mod camera;
mod color;
mod material;
mod ray;
mod rtweekend;
mod vec3;

use std::sync::Arc;

use camera::Camera;
use flexi_logger::{Logger, WriteMode};
use material::{Dielectric, Lambertian, Material, Metal};
use ray::{HittableList, Sphere};
use vec3::{Point3, Vec3};

fn main() {
    // Initialize the logger with buffered output and directing to stderr
    Logger::try_with_str("info")
        .unwrap()
        .write_mode(WriteMode::BufferAndFlush)
        .log_to_stderr() // Ensure output goes to stderr
        .start()
        .unwrap();

    // predefine material
    let material_ground = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    // world
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        &(material_ground as Arc<dyn Material>),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        &(material_center as Arc<dyn Material>),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        &(material_left as Arc<dyn Material>),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        &(material_bubble as Arc<dyn Material>),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        &(material_right as Arc<dyn Material>),
    )));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.img_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.look_from = Point3::new(-2.0, 2.0, 1.0);
    camera.look_at = Point3::new(0.0, 0.0, -1.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    // defocus blur
    camera.defocus_angle = 10.0;
    camera.focus_dist = 3.4;

    camera.render(&world);
}
