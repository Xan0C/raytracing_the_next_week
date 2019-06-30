use crate::vec::Vec3;
use crate::ray::Ray;

#[derive(Clone,Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB {min, max}
    }

    pub fn hit(&self, r: &Ray, mut tmin: f32, mut tmax: f32) -> bool {
        
        let mint = (self.min.x - r.origin.x) / r.direction.x;
        let maxt = (self.max.x - r.origin.x) / r.direction.x;

        let t0 = ffmin(mint, maxt);
        let t1 = ffmax(mint, maxt);

        tmin = ffmax(t0, tmin);
        tmax = ffmin(t1, tmax);

        if tmax <= tmin {
            return false;
        }

        let mint = (self.min.y - r.origin.y) / r.direction.y;
        let maxt = (self.max.y - r.origin.y) / r.direction.y;

        let t0 = ffmin(mint, maxt);
        let t1 = ffmax(mint, maxt);

        tmin = ffmax(t0, tmin);
        tmax = ffmin(t1, tmax);

        if tmax <= tmin {
            return false;
        }

        let mint = (self.min.z - r.origin.z) / r.direction.z;
        let maxt = (self.max.z - r.origin.z) / r.direction.z;

        let t0 = ffmin(mint, maxt);
        let t1 = ffmax(mint, maxt);

        tmin = ffmax(t0, tmin);
        tmax = ffmin(t1, tmax);

        if tmax <= tmin {
            return false;
        }

        true
    }

    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        let small = Vec3::new(
            ffmin(box0.min.x, box1.min.x),
            ffmin(box0.min.y, box1.min.y),
            ffmin(box0.min.z, box1.min.z));
        let big = Vec3::new(
            ffmax(box0.max.x, box1.max.x),
            ffmax(box0.max.y, box1.max.y),
            ffmax(box0.max.z, box1.max.z));

        return AABB::new(small, big);
    }
}

fn ffmax(a: f32, b: f32) -> f32 {
  if a > b { a } else { b }
}

fn ffmin(a: f32, b: f32) -> f32 {
  if a < b { a } else { b }
}