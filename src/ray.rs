#![allow(dead_code)]
use core::f32;
use std::sync::Arc;

use crate::{
    rtweekend::INFINITY,
    vec3::{Point3, Vec3},
};

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn default() -> Self {
        Self {
            min: INFINITY,
            max: f32::NEG_INFINITY,
        }
    }

    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

pub const EMPTY_INTERVAL: Interval = Interval {
    min: f32::INFINITY,
    max: f32::NEG_INFINITY,
};

pub const UNIVERSE_INTERVAL: Interval = Interval {
    min: f32::INFINITY,
    max: f32::NEG_INFINITY,
};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
}

#[derive(Clone, Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -(*outward_normal),
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin;
        let a = r.direction.squared_length();
        let h = r.direction.dot(&oc);
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.point = r.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}

// HittableList struct to hold a list of hittable objects.
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    // Constructor to create an empty list of hittables.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    // Constructor to create a list with one initial hittable object.
    pub fn from_object(object: Arc<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    // Method to clear all objects from the list.
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    // Method to add a hittable object to the list.
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

// Implement the Hittable trait for HittableList.
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        // Iterate over all objects and find the closest hit.
        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use super::Vec3;

    #[test]
    fn test_at() {
        let ray = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(ray.at(3.0), Vec3::new(4.0, 7.0, 10.0));
    }
}
