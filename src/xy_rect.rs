use crate::hitable::Hitable;
use crate::hitable::HitRecord;
use crate::aabb::AABB;
use crate::vec::Vec3;
use crate::material::Material;
use crate::ray::Ray;

use std::sync::Arc;

pub struct XYRect {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Arc<Material>
}

pub struct XZRect {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Arc<Material>
}

pub struct YZRect {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Arc<Material>
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Arc<Material>) -> Self {
        XYRect { x0, x1, y0, y1, k, material }
    }
}

impl XZRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Arc<Material>) -> Self {
        XZRect { x0, x1, z0, z1, k, material }
    }
}

impl YZRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Arc<Material>) -> Self {
        YZRect { y0, y1, z0, z1, k, material }
    }
}

impl Hitable for XYRect {
    fn hit(&self, ray: &Ray, t_range: ::std::ops::Range<f32>) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;

        if t < t_range.start || t > t_range.end {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            ray.point_at_parameter(t),
            Vec3::new(0.0, 0.0, 1.0),
            &*self.material,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0)
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(Vec3::new(self.x0, self.y0, self.k - 0.0001), Vec3::new(self.x1, self.y1, self.k + 0.0001)))
    }
}

impl Hitable for XZRect {
    fn hit(&self, ray: &Ray, t_range: ::std::ops::Range<f32>) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;

        if t < t_range.start || t > t_range.end {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            ray.point_at_parameter(t),
            Vec3::new(0.0, 1.0, 0.0),
            &*self.material,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0)
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(Vec3::new(self.x0, self.k - 0.0001, self.z0), Vec3::new(self.x1, self.k + 0.0001, self.z1)))
    }
}

impl Hitable for YZRect {
    fn hit(&self, ray: &Ray, t_range: ::std::ops::Range<f32>) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;

        if t < t_range.start || t > t_range.end {
            return None;
        }

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            ray.point_at_parameter(t),
            Vec3::new(1.0, 0.0, 0.0),
            &*self.material,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0)
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(Vec3::new(self.k - 0.0001, self.y0, self.z0), Vec3::new(self.k + 0.0001, self.y1, self.z1)))
    }
}