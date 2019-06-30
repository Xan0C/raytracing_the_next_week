use crate::scene::Scene;
use crate::ray::Ray;
use crate::camera::Camera;
use crate::vec::Vec3;
use crate::hitable::Hitable;

use lodepng::RGB;
use rand::distributions::Uniform;
use rand::prelude::*;
use rayon::prelude::*;

fn color_ray(r: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    let hit = scene.bvh.hit(&r, 0.001..std::f32::MAX);

    match hit {
        Some(rec) => {
            let scattered = rec.material.scatter(&r, &rec);
            let emitted = rec.material.emitted(rec.u, rec.v, rec.p);

            if let Some(scattered_ray) = scattered.ray {
                return emitted + color_ray(&scattered_ray, scene, depth + 1) * scattered.attenuation;
            } else {
                return emitted;
            }
        },
        None => {
                return Vec3::zero();
            }
    }
}

pub fn render(scene: &Scene, camera: &Camera, width: usize, height: usize, samples: usize) -> Vec<RGB<u8>> {
    let nx = width;
    let ny = height;
    let ns = samples;

    let mut pixels: Vec<RGB<u8>> = vec![RGB {r: 0, g: 0, b: 0}; nx * ny];

    pixels.par_iter_mut().enumerate().for_each(|(i, p)| {
        let x = i % nx;
        let y = ny - (i - x) / nx;

        let mut col = Vec3::new(0.0, 0.0, 0.0);

        for _s in 0..ns {
            let mut rng = thread_rng();
            let uniform = Uniform::new(0.0, 1.0);
            let (r1, r2) = (uniform.sample(&mut rng), uniform.sample(&mut rng));

            let u = (x as f32 + r1) / nx as f32;
            let v = (y as f32 + r2) / ny as f32;
            let r = camera.get_ray(u, v);
            col += color_ray(&r, *&scene, 0);
        }

        col /= ns as f32;
        col = Vec3::new( col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

        p.r = (255.99 * col.x) as u8;
        p.g = (255.99 * col.y) as u8;
        p.b = (255.99 * col.z) as u8;
    });

    return pixels;
}