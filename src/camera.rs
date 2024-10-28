use std::{borrow::BorrowMut, io};

use log::info;

use crate::{
    color::write_color,
    ray::{HitRecord, Hittable, Interval, Ray},
    rtweekend::{degrees_to_radians, random_double, INFINITY},
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub img_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f32,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pixel_samples_scale: f32,
    img_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            img_width: 100,
            img_height: 1,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            samples_per_pixel: 10,
            pixel_samples_scale: 0.0,
            max_depth: 10,
            vfov: 90.0,
            look_from: Point3::zero(),
            look_at: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.img_width, self.img_height);
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        for j in 0..self.img_height {
            info!("Scanlines remaining: {}", self.img_width - j);
            for i in 0..self.img_width {
                let mut pixel_color = Vec3::zero();
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                pixel_color = pixel_color * self.pixel_samples_scale;

                write_color(&mut handle, pixel_color.rgba()).unwrap();
            }
        }
    }

    fn initialize(&mut self) {
        // Image size
        self.img_width = 400;
        self.img_height = {
            let img_height = (self.img_width as f32 / self.aspect_ratio) as i32;
            img_height.max(1)
        };

        // pixel sample
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;

        // Camera
        let focal_length = (self.look_from - self.look_at).length();
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.img_width as f32 / self.img_height as f32);
        self.center = self.look_from;

        // calc basic vectors for camera coordinate frame
        self.w = (self.look_from - self.look_at).unit();
        self.u = self.vup.cross(&self.w).unit();
        self.v = self.w.cross(&self.u);

        // viewport space
        let viewport_u = self.u * viewport_width;
        let viewport_v = (-self.v) * viewport_height;

        self.pixel_delta_u = viewport_u / self.img_width as f32;
        self.pixel_delta_v = viewport_v / self.img_height as f32;

        let viewport_upper_left =
            self.center - (self.w * focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f32 + offset.x))
            + (self.pixel_delta_v * (j as f32 + offset.y));
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }

        let mut rec = HitRecord::default();
        let color_vec = if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::new(r.origin, r.direction);
            let mut attenuation = Vec3::zero();
            if rec.mat.as_ref().unwrap().borrow_mut().scatter(
                r,
                &rec,
                &mut attenuation,
                &mut scattered,
            ) {
                Vec3::elemul(Self::ray_color(&scattered, depth - 1, world), attenuation)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        } else {
            let unit_dir = r.direction.unit();
            let a = 0.5 * (unit_dir.y + 1.0);

            Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
        };

        color_vec
    }
}
