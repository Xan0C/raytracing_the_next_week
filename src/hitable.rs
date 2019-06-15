use crate::ray::Ray;
use crate::vec::Vec3;
use crate::material::Material;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vec3, n: Vec3, material: &'a Material) -> Self {
        HitRecord { t, p, normal: n, material }
    }
}

pub trait Hitable: Send + Sync {
    fn hit(&self, ray: &Ray, t_range: ::std::ops::Range<f32>) -> Option<HitRecord>;
}

pub struct HitableList {
  list: Vec<Box<Hitable>>
}

impl HitableList {
  pub fn new() -> Self {
    HitableList { list: Vec::new() }
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
}