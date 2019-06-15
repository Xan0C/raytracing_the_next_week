use crate::vec;

#[derive(Clone, Copy)]
pub struct Ray {
    pub a: vec::Vec3,
    pub b: vec::Vec3
}

impl Ray {
    pub fn new(origin: vec::Vec3, dir: vec::Vec3) -> Self {
        Ray { a: origin, b: dir }
    }

    pub fn origin(&self) -> vec::Vec3 {
        return self.a;
    }

    pub fn direction(&self) -> vec::Vec3 {
        return self.b;
    }

    pub fn point_at_parameter(&self, t: f32) -> vec::Vec3 {
        return self.a + self.b * t;
    }
}