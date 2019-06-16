use crate::ray::Ray;
use crate::vec::Vec3;
use crate::hitable::HitRecord;
use crate::sphere;
use crate::math;
use crate::perlin;

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
}

pub struct Diffuse {
    albedo: Box<Texture>
}

impl Diffuse {
    pub fn new(a: Box<Texture>) -> Self {
        Diffuse { albedo: a }
    }
}

impl Material for Diffuse {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Scatter {
        let target = record.p + record.normal + sphere::random_in_unit_sphere();
        let scattered = Ray::new(record.p, target - record.p, ray.time);

        return Scatter::new(self.albedo.value(0.0, 0.0, &record.p), Some(scattered));
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

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

pub struct ConstantTexture {
    color: Vec3
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> Self {
        ConstantTexture { color }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<Texture>,
    even: Box<Texture>
}

impl CheckerTexture {
    pub fn new(odd: Box<Texture>, even: Box<Texture>) -> Self {
        CheckerTexture { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines = (p.x * 10.0).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();

        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}

pub struct NoiseTexture {
    scale: f32
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        NoiseTexture { scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        //return Vec3::new(1.0, 1.0, 1.0) * perlin::turb((*p * self.scale), 7);

        return Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * p.z + 10.0 * perlin::turb(*p, 7)).sin());
    }
}

pub struct ImageTexture {
    nx: usize,
    ny: usize
}

impl ImageTexture {
    pub fn new(nx: usize, ny: usize) -> Self {
        ImageTexture { nx, ny }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let mut i = u * self.nx as f32;
        let mut j = (1.0 - v) * self.ny as f32 - 0.001;

        if i < 0.0 {
            i = 0.0;
        }

        if j < 0.0 {
            j = 0.0;
        }

        if i > (self.nx - 1) as f32 {
            i = (self.nx - 1) as f32;
        }

        if j > (self.ny - 1) as f32 {
            j = (self.ny - 1) as f32;
        }

        Vec3::zero()
    }
}