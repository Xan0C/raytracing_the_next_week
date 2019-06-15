use crate::ray::Ray;
use crate::vec::Vec3;
use crate::hitable::Hitable;
use crate::hitable::HitRecord;
use crate::material::Material;

use rand::random;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    while {
         p = Vec3::new(random::<f32>(), random::<f32>(), random::<f32>()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
         p.len_squared() >= 1.0
    } {}

    return p;
}

pub struct Sphere {
    radius: f32,
    center: Vec3,
    material: Box<Material>
}

impl Sphere {
    pub fn new(orig: Vec3, rad: f32, material: Box<Material>) -> Self {
        Sphere { radius: rad, center: orig , material }
    }
}

impl<'a> Hitable for Sphere {
    fn hit(&self, r: &Ray, t_range: ::std::ops::Range<f32>) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let t = (-b - (b * b - a * c).sqrt()) / a;

            if t < t_range.end && t > t_range.start {
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(t, p, normal, &*self.material));
            }

            let t = (-b + (b * b - a * c).sqrt()) / a;
            if t < t_range.end && t > t_range.start {
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(t, p, normal, &*self.material));
            }
        }

        return None;
    }
}