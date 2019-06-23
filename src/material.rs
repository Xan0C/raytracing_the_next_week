use crate::ray::Ray;
use crate::vec::Vec3;
use crate::hitable::HitRecord;
use crate::sphere;
use crate::math;
use crate::texture::Texture;

use std::sync::Arc;
use rand::random;

pub struct Scatter {
    pub attenuation: Vec3,
    pub ray: Option<Ray>,
}

impl Scatter {
    pub fn new(attenuation: Vec3, ray: Option<Ray>) -> Self {
        Scatter {attenuation, ray}
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Scatter;
    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        Vec3::zero()
    }
}

pub struct Diffuse {
    albedo: Arc<Texture>
}

impl Diffuse {
    pub fn new(a: Arc<Texture>) -> Self {
        Diffuse { albedo: a }
    }
}

impl Material for Diffuse {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Scatter {
        let target = record.p + record.normal + sphere::random_in_unit_sphere();
        let scattered = Ray::new(record.p, target - record.p, ray.time);

        return Scatter::new(self.albedo.value(record.u, record.v, &record.p), Some(scattered));
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32
}

impl Metal {
    pub fn new(a: Vec3, fuzz: f32) -> Self {
        let mut f = 1.0;
        if fuzz < 1.0 {
            f = fuzz;
        }
        Metal { albedo: a, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Scatter {
        let reflected = math::reflect(ray.direction().normalize(), record.normal);
        let scattered = Ray::new(record.p, reflected + sphere::random_in_unit_sphere() * self.fuzz, ray.time);

        if scattered.direction().dot(record.normal) > 0.0 {
            return Scatter::new(self.albedo, Some(scattered));
        }

        return Scatter::new(self.albedo, None);
    }
}

pub struct Dielectric {
    pub ref_idx: f32
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Scatter {
        let reflected = math::reflect(ray.direction(), record.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let outward_normal: Vec3;
        let reflect_prob: f32;
        let ni_over_nt: f32;
        let cosine: f32;


        if ray.direction().dot(record.normal) > 0.0 {
            outward_normal = -record.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * ray.direction().dot(record.normal) / ray.direction().len();
        } else {
            outward_normal = record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -ray.direction().dot(record.normal) / ray.direction().len();
        }

        let refracted = math::refract(ray.direction(), outward_normal, ni_over_nt);

        if refracted.is_some() {
            reflect_prob = math::schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }

        if random::<f32>() < reflect_prob {
            return Scatter::new(attenuation, Some(Ray::new(record.p, reflected, ray.time)));
        } else {
            return Scatter::new(attenuation, Some(Ray::new(record.p, refracted.unwrap(), ray.time)))
        }
    }
}

pub struct DiffuseLight {
    emit: Arc<Texture>
}

impl DiffuseLight {
    pub fn new(emit: Arc<Texture>) -> Self {
        DiffuseLight { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Scatter {
        Scatter::new(Vec3::zero(), None)
    }

    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.emit.value(u, v, &p)
    }
}

pub struct Isotropic {
    albedo: Arc<Texture>
}

impl Isotropic {
    pub fn new(albedo: Arc<Texture>) -> Self {
        Isotropic { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Scatter {
        let scattered = Ray::new(record.p, sphere::random_in_unit_sphere(), ray.time);
        let attenuation = self.albedo.value(record.u, record.v, &record.p);

        Scatter::new(attenuation, Some(scattered))
    }
}