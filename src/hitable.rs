use crate::ray::Ray;
use crate::vec::Vec3;
use crate::material::Material;
use crate::aabb::AABB;
use crate::texture::Texture;
use crate::material::Isotropic;

use std::sync::Arc;
use rand::prelude::*;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub u: f32,
    pub v: f32,
    pub normal: Vec3,
    pub material: &'a Material
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vec3, n: Vec3, material: &'a Material, u: f32, v: f32) -> Self {
        HitRecord { t, p, normal: n, material, u, v }
    }
}

pub trait Hitable: Send + Sync {
    fn hit(&self, ray: &Ray, t_range: ::std::ops::Range<f32>) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
}

pub struct HitableList {
  list: Vec<Box<Hitable>>
}

impl HitableList {
  pub fn new() -> Self {
    HitableList { list: Vec::new() }
  }

  pub fn from_list(list: Vec<Box<Hitable>>) -> Self {
    HitableList { list }
  }

  pub fn push(&mut self, hitable: Box<Hitable>) {
    self.list.push(hitable);
  }
}

impl Hitable for HitableList {
  fn hit(&self, ray: &Ray, t_range: ::std::ops::Range<f32>) -> Option<HitRecord> {
    let mut current = None;
    let mut closest_so_far = t_range.end;

    for hitable in &(self.list) {
      let hr = hitable.hit(ray, t_range.start..closest_so_far);
      if let Some(HitRecord {t, ..}) = hr {
        closest_so_far = t;
        current = hr;
      }
    }
    
    return current;
  }

  fn bounding_box(&self) -> Option<AABB> {
    if self.list.len() < 1 {
      return None;
    }

    let mut surrounding_box: AABB;
    let first = self.list[0].bounding_box();

    match first {
      Some(b) => surrounding_box = b,
      None => return None
    }

    for i in 1..self.list.len() {
      if let Some(bbox) = self.list[i].bounding_box() {
        surrounding_box = AABB::surrounding_box(&bbox, &surrounding_box);
      } else {
        return None;
      }
    }

    Some(surrounding_box)
  }
}

pub struct FlipNormal {
  pub hitable: Box<Hitable>
}

impl FlipNormal {
  pub fn new(hitable: Box<Hitable>) -> Self {
    FlipNormal { hitable }
  }
}

impl Hitable for FlipNormal {
   fn hit(&self, ray: &Ray, t_range: ::std::ops::Range<f32>) -> Option<HitRecord> {
     if let Some(mut hit) = self.hitable.hit(ray, t_range) {
       hit.normal = hit.normal * -1.0;
       return Some(hit);
     }
     
     None
   }

   fn bounding_box(&self) -> Option<AABB> {
     self.hitable.bounding_box()
   }
}

pub struct Translate {
  offset: Vec3,
  hitable: Arc<Hitable>
}

impl Translate {
  pub fn new(hitable: Arc<Hitable>, offset: Vec3) -> Self {
    Translate { offset, hitable }
  }
}

impl Hitable for Translate {
  fn hit(&self, ray: &Ray, t_range: ::std::ops::Range<f32>) -> Option<HitRecord> {
    let ray_moved = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
    if let Some(hit)  = self.hitable.hit(&ray_moved, t_range) {
      return Some(HitRecord::new(
          hit.t, 
          hit.p + self.offset,
          hit.normal, 
          hit.material,
          hit.u, 
          hit.v
        ));
    }

    None
  }

  fn bounding_box(&self) -> Option<AABB> {
    if let Some(bbox) = self.hitable.bounding_box() {
      return Some(AABB::new(bbox.min + self.offset, bbox.max + self.offset));
    }
    None
  }
}

pub struct RotateY {
  hitable: Arc<Hitable>,
  bbox: Option<AABB>,
  sin_theta: f32,
  cos_theta: f32
}

impl RotateY {
  pub fn new(hitable: Arc<Hitable>, angle: f32) -> Self {
    let radians = (std::f32::consts::PI / 180.0) * angle;
    let sin_theta = radians.sin();
    let cos_theta = radians.cos();

    let bbox = hitable.bounding_box().unwrap();
    let mut min = Vec3::max();
    let mut max = Vec3::min();

    for i in 0..2 {
      for j in 0..2 {
        for k in 0..2 {
          let x = i as f32 * bbox.max.x + (1 - i) as f32 * bbox.min.x;
          let y = j as f32 * bbox.max.y + (1 - j) as f32 * bbox.min.y;
          let z = k as f32 * bbox.max.z + (1 - k) as f32 * bbox.min.z;

          let newx = cos_theta * x + sin_theta * z;
          let newz = -sin_theta * x + cos_theta * z;

          let tester = Vec3::new(newx, y, newz);

          set_min_max(&mut min, &mut max, &tester);
        }
      }
    }

    RotateY { hitable, sin_theta, cos_theta, bbox: Some(AABB::new(min, max)) }
  }
}

impl Hitable for RotateY {
  fn hit(&self, ray: &Ray, t_range: ::std::ops::Range<f32>) -> Option<HitRecord> {
    let mut origin: Vec3 = ray.origin;
    let mut direction: Vec3 = ray.direction;

    origin.x = self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z;
    origin.z = self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z;

    direction.x = self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z;
    direction.z = self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z;

    let ray_rotated = Ray::new(origin, direction, ray.time);

    if let Some(hitRecord) = self.hitable.hit(&ray_rotated, t_range) {
      let mut p: Vec3 = hitRecord.p;
      let mut normal: Vec3 = hitRecord.normal;
      p.x = self.cos_theta * hitRecord.p.x + self.sin_theta * hitRecord.p.z;
      p.y = -self.sin_theta * hitRecord.p.x + self.cos_theta * hitRecord.p.z;

      normal.x = self.cos_theta * hitRecord.normal.x + self.sin_theta * hitRecord.p.z;
      normal.z = -self.sin_theta * hitRecord.normal.x + self.cos_theta * hitRecord.p.z;

      return Some(HitRecord::new(
        hitRecord.t,
        p,
        normal,
        hitRecord.material,
        hitRecord.u,
        hitRecord.v
      ));
    }
    None
  }

  fn bounding_box(&self) -> Option<AABB> {
    self.bbox
  }
}

pub struct ConstantMedium {
  boundary: Arc<Hitable>,
  density: f32,
  phase_function: Isotropic
}

impl ConstantMedium {
  pub fn new(boundary: Arc<Hitable>, density: f32, texture: Arc<Texture>) -> Self {
    ConstantMedium { boundary, density, phase_function: Isotropic::new(texture) }
  }
}

impl Hitable for ConstantMedium {
  fn hit(&self, ray: &Ray, t_range: ::std::ops::Range<f32>) -> Option<HitRecord> {
    if let Some(record1) = self.boundary.hit(ray, std::f32::MIN..std::f32::MAX) {
      if let Some(record2) = self.boundary.hit(ray, record1.t+0.0001..std::f32::MAX) {
        let mut tmin = record1.t;
        let mut tmax = record2.t;

        if tmin < t_range.start {
          tmin = t_range.start;
        }

        if tmax > t_range.end {
          tmax = t_range.end;
        }

        if tmin >= tmax {
          return None;
        }

        if tmin < 0.0 {
          tmin = 0.0;
        }

        let distance_inside_boundary = (tmax - tmin) * ray.direction.len();
        let hit_distance = -(1.0 / self.density) * random::<f32>().log2();

        if hit_distance < distance_inside_boundary {
          let t = record1.t + hit_distance / ray.direction.len();
          let p = ray.point_at_parameter(t);
          let normal = Vec3::new(1.0, 0.0, 0.0);
          
          return Some(HitRecord::new(
            t,
            p,
            normal,
            &self.phase_function,
            0.0,
            0.0
          ));
        }

        return None;
      }
    }
    None
  }

  fn bounding_box(&self) -> Option<AABB> {
    self.boundary.bounding_box()
  }
}

fn set_min_max(min: &mut Vec3, max: &mut Vec3, minMax: &Vec3) {
  if minMax.x > max.x {
    max.x = minMax.x;
  }

  if minMax.y > max.y {
    max.y = minMax.y;
  }

  if minMax.z > max.z {
    max.z = minMax.z;
  }

  if minMax.x < min.x {
    min.x = minMax.x;
  }

  if minMax.y < min.y {
    min.y = minMax.y;
  }

  if minMax.z < min.z {
    min.z = minMax.z;
  }
}