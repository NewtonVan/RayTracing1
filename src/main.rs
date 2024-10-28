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
use rtweekend::{random_double, random_double_in_range};
use vec3::{Point3, Vec3};

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

    // predefine material
    let ground_material = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &(ground_material as Arc<dyn Material>),
    )));

    let mat_point = Point3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f32 + 0.9 * random_double(),
                0.2,
                b as f32 + 0.9 * random_double(),
            );

            if (center - mat_point).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::elemul(Vec3::random(), Vec3::random());
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = random_double_in_range(0.0, 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Arc::new(Dielectric::new(1.5))
                };
                world.add(Arc::new(Sphere::new(center, 0.2, &sphere_material)));
            }
        }
    }
    let material1: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        &material1,
    )));
    let material2: Arc<dyn Material> = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        &material2,
    )));
    let material3: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        &material3,
    )));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.img_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    // defocus blur
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    camera.render(&world);
}
