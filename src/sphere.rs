use crate::ray::Ray;
use crate::vec::Vec3;
use crate::hitable::Hitable;
use crate::hitable::HitRecord;
use crate::material::Material;
use crate::aabb::AABB;

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
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    material: Box<Material>
}

impl Sphere {
    pub fn new(orig: Vec3, rad: f32, material: Box<Material>) -> Self {
        Sphere { radius: rad, center0: orig , center1: orig, time0: 0.0, time1: 1.0, material }
    }

    pub fn new_moving_sphere(center0: Vec3, center1: Vec3, time0: f32, time1: f32, radius: f32, material: Box<Material>) -> Self {
        Sphere { center0, center1, time0, time1, radius, material }
    }

    pub fn center(&self, time: f32) -> Vec3 {
         return self.center0 + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0));
    }
}

impl<'a> Hitable for Sphere {
    fn hit(&self, r: &Ray, t_range: ::std::ops::Range<f32>) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time);
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let t = (-b - (b * b - a * c).sqrt()) / a;

            if t < t_range.end && t > t_range.start {
                let p = r.point_at_parameter(t);
                let normal = (p - self.center(r.time)) / self.radius;
                return Some(HitRecord::new(t, p, normal, &*self.material));
            }

            let t = (-b + (b * b - a * c).sqrt()) / a;
            if t < t_range.end && t > t_range.start {
                let p = r.point_at_parameter(t);
                let normal = (p - self.center(r.time)) / self.radius;
                return Some(HitRecord::new(t, p, normal, &*self.material));
            }
        }

        return None;
    }

    fn bounding_box(&self) -> Option<AABB> {
        //let box0 = AABB::new(self.center0 - Vec3::new(self.radius, self.radius, self.radius), self.center0 + Vec3::new(self.radius, self.radius, self.radius));
        //let box1 = AABB::new(self.center1 - Vec3::new(self.radius, self.radius, self.radius), self.center1 + Vec3::new(self.radius, self.radius, self.radius));
        //return Some(AABB::surrounding_box(box0, box1));

        return Some(AABB::new(
            self.center0 - Vec3::new(self.radius, self.radius, self.radius),
            self.center0 + Vec3::new(self.radius, self.radius, self.radius),
        ));
    }
}