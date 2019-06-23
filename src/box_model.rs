use crate::hitable::*;
use crate::vec::Vec3;
use crate::material::Material;
use crate::xy_rect::*;
use crate::ray::Ray;
use crate::aabb::AABB;

use std::sync::Arc;

pub struct BoxModel {
    hitable: Box<Hitable>,
    pmin: Vec3,
    pmax: Vec3
}

impl BoxModel {
    pub fn new(p0: Vec3, p1: Vec3, material: Arc<Material>) -> Self {
        let mut world: Vec<Box<Hitable>> = Vec::new();

        world.push(Box::new(XYRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, Arc::clone(&material))));
        world.push(Box::new(FlipNormal::new(Box::new(XYRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, Arc::clone(&material))))));

        world.push(Box::new(XZRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, Arc::clone(&material))));
        world.push(Box::new(FlipNormal::new(Box::new(XZRect::new(p0.x, p1.x, p0.z, p1.z, p0.y, Arc::clone(&material))))));

        world.push(Box::new(YZRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, Arc::clone(&material))));
        world.push(Box::new(FlipNormal::new(Box::new(YZRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, Arc::clone(&material))))));

        BoxModel { pmin: p0, pmax: p1, hitable: Box::new(HitableList::from_list(world)) }
    }
}

impl Hitable for BoxModel {
    fn hit(&self, ray: &Ray, t_range: ::std::ops::Range<f32>) -> Option<HitRecord> {
        self.hitable.hit(ray, t_range)
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(self.pmin, self.pmax))
    }
}