use crate::vec;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: vec::Vec3,
    pub direction: vec::Vec3,
    pub time: f32
}

impl Ray {
    pub fn new(origin: vec::Vec3, dir: vec::Vec3, time: f32) -> Self {
        Ray { origin: origin, direction: dir, time }
    }

    pub fn origin(&self) -> vec::Vec3 {
        return self.origin;
    }

    pub fn direction(&self) -> vec::Vec3 {
        return self.direction;
    }

    pub fn point_at_parameter(&self, t: f32) -> vec::Vec3 {
        return self.origin + self.direction * t;
    }
}