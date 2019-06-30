use crate::vec::Vec3;
use crate::ray::Ray;

use std::f32::consts::PI;
use rand::random;

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
    pub lens_radius: f32,
    pub time0: f32,
    pub time1: f32
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);
        if p.dot(p) < 1.0 {
            return p;
        }
    }
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32, time0: f32, time1: f32) -> Self {
        let theta = vfov * PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        
        Camera { 
            lower_left_corner: lookfrom - u * (half_width * focus_dist) - v * (half_height * focus_dist) - w * focus_dist,
            horizontal: u * (2.0 * half_width * focus_dist),
            vertical: v * (2.0 * half_height * focus_dist),
            origin: lookfrom,
            lens_radius: aperture / 2.0,
            time0,
            time1
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = rd.x * u + rd.y * v;
        let time = self.time0 + random::<f32>() * (self.time1 - self.time0);
        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset, time)
    }
}