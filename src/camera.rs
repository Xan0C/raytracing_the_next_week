use crate::vec::Vec3;
use crate::ray::Ray;

use std::f32::consts::PI;
use rand::random;

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
    pub lens_radius: f32
}

fn random_in_unit_disk() -> Vec3 {
    let mut p;
    while {
        p = Vec3::new(random::<f32>(), random::<f32>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);
        p.dot(p) >= 1.0
    } {}

    return p;
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {
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
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = rd.x * u + rd.y * v;
        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset)
    }
}