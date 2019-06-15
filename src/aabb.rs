use crate::vec::Vec3;
use crate::ray::Ray;

#[derive(Clone,Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3
}

fn point_inside_min_max(min: f32, max:f32, ray_origin: f32, ray_dir: f32, mut tmin: f32, mut tmax: f32) -> bool {
    let inv_d = 1.0 / ray_dir;

    let mint = (min - ray_origin) * inv_d;
    let maxt = (max - ray_origin) * inv_d;

    let t0 = mint.min(maxt);
    let t1 = mint.max(maxt);

    tmin = tmin.max(t0);
    tmax = tmax.min(t1);

    if tmax <= tmin {
        return false;
    }

    return true;
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB {min, max}
    }

    pub fn hit(&self, ray: &Ray, mut tmin: f32, mut tmax: f32) -> bool {
        if !point_inside_min_max(self.min.x, self.max.x, ray.origin.x, ray.direction.x, tmin, tmax) {
            return false;
        }

        if !point_inside_min_max(self.min.y, self.max.y, ray.origin.y, ray.direction.y, tmin, tmax) {
            return false;
        }

        if !point_inside_min_max(self.min.x, self.max.x, ray.origin.x, ray.direction.x, tmin, tmax) {
            return false;
        }

        return true;
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
         let small = Vec3::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z)
        );

        let big = Vec3::new(
            box0.max.x.min(box1.max.x),
            box0.max.y.min(box1.max.y),
            box0.max.z.min(box1.max.z)
        );

        return AABB::new(small, big);
    }
}