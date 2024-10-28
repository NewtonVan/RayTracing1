#![allow(dead_code)]
use crate::{
    ray::{HitRecord, Ray},
    rtweekend::random_double,
    vec3::Vec3,
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    #[allow(unused_variables)]
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vec();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&r_in.direction, &rec.normal);
        let reflected = reflected.unit() + Vec3::random_unit_vec() * self.fuzz;
        *scattered = Ray::new(rec.point, reflected);
        *attenuation = self.albedo;

        Vec3::dot(&scattered.direction, &rec.normal) > 0.0
    }
}

pub struct Dielectric {
    refraction_index: f32,
}
impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction.unit();
        let cos_theta = Vec3::dot(&(-unit_direction), &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let no_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if no_refract || Self::reflectance(cos_theta, refraction_ratio) > random_double() {
                Vec3::reflect(&unit_direction, &rec.normal)
            } else {
                Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        *scattered = Ray::new(rec.point, direction);

        true
    }
}

impl Dielectric {
    fn reflectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
        let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0)
    }
}
