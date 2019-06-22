use crate::vec::Vec3;
use crate::perlin;

use lodepng::RGB;

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
    data: Vec<RGB<u8>>,
    nx: usize,
    ny: usize
}

impl ImageTexture {
    pub fn new(data: Vec<RGB<u8>>, nx: usize, ny: usize) -> Self {
        ImageTexture { data, nx, ny }
    }

    pub fn from_image(filepath: &str) -> Self {
        
        let img = lodepng::decode24_file(filepath);
        
        if img.is_err() {
            panic!("can not decode image: {}", img.unwrap_err());
        }

        let data = img.unwrap();
        ImageTexture { data: data.buffer, nx: data.width, ny: data.height }
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

        let pixel = self.data[(i as usize) + self.nx * (j as usize)];

        //println!("pixel: {}, i: {}, j: {}", pixel, i, j);

        Vec3::new(pixel.r as f32 / 255.0, pixel.g as f32 / 255.0, pixel.b as f32 / 255.0)
    }
}