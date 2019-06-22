use crate::ray::Ray;
use crate::vec::Vec3;
use crate::material::Material;
use crate::aabb::AABB;

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

    for hitable in &(self.list) {
      if let Some(bbox) = hitable.bounding_box() {
        surrounding_box = AABB::surrounding_box(bbox, surrounding_box);
      } else {
        return None;
      }
    }

    Some(surrounding_box)
  }
}