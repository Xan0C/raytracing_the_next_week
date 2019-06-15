use crate::vec::Vec3;
use crate::ray::Ray;
use crate::color;
use crate::hitable::Hitable;
use crate::hitable::HitableList;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::material::Diffuse;
use crate::material::Metal;
use crate::material::Dielectric;
use crate::bvh_node::BvhTree;

use rand::random;
use std::f32::consts::PI;
use lodepng::RGB;

use rand::distributions::Uniform;
use rand::prelude::*;
use rayon::prelude::*;

fn create_scene() -> Vec<Box<Hitable>> {
    let mut world: Vec<Box<Hitable>> = Vec::new();

    world.push(Box::new(Sphere::new(
        Vec3 { x: 0.0, y: -1000.0, z: 0.0 },
        1000.0,
        Box::new(Diffuse::new(Vec3::new(0.5, 0.5, 0.5)))
    )));
    world.push(Box::new(Sphere::new(
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        1.0,
        Box::new(Dielectric::new(1.5))
    )));
    world.push(Box::new(Sphere::new(
        Vec3 { x: -4.0, y: 1.0, z: 0.0 },
        1.0,
        Box::new(Diffuse::new(Vec3::new(0.4, 0.2, 0.1)))
    )));
    world.push(Box::new(Sphere::new(
        Vec3 { x: 4.0, y: 1.0, z: 0.0 },
        1.0,
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f32>();
            let center = Vec3::new(a as f32 + 0.9 * random::<f32>(), 0.2, b as f32 + 0.9 * random::<f32>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 { //diffuse
                    world.push(Box::new(Sphere::new(
                        center,
                        //center + Vec3::new(0.0, 0.5 * random::<f32>(), 0.0),
                        //0.0,
                        //1.0,
                        0.2,
                        Box::new(Diffuse::new(Vec3::new(random::<f32>() * random::<f32>(), random::<f32>() * random::<f32>(), random::<f32>() * random::<f32>())))
                    )));
                } else if choose_mat < 0.95 {
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new( Vec3::new(0.5 * (1.0 * random::<f32>()), 0.5 * (1.0 * random::<f32>()), 0.5 * (1.0 * random::<f32>()) ), 0.5 * random::<f32>()))
                    )));
                } else {
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Dielectric::new(1.5))
                    )));
                }
            } 
        }
    }

    return world;
}

fn color_ray(r: &Ray, world: &Hitable, depth: u32) -> Vec3 {
    if let Some(record) = world.hit(&r, 0.001..std::f32::MAX) {

        if depth > 50 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let scattered = record.material.scatter(&r, &record);

        if let Some(hit) = scattered.ray {
            return color_ray(&hit, world, depth+1) * scattered.attenuation;
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }

    } else {
        let unit_direction = r.direction().normalize();
        let t = (unit_direction.y + 1.0) * 0.5;
        return Vec3 { x: 1.0, y: 1.0, z: 1.0 } * (1.0 - t) + Vec3 { x: 0.5, y: 0.7, z: 1.0 } * t;
    }
}

pub fn render(width: usize, height: usize, samples: usize) -> Vec<RGB<u8>> {
    let nx = width;
    let ny = height;
    let ns = samples;

    let mut pixels: Vec<RGB<u8>> = vec![RGB {r: 0, g: 0, b: 0}; nx * ny];
    
    let mut scene = create_scene();
    let hitableList = Box::new(HitableList::from_list(scene));
    //let mut wrapper: Vec<Box<Hitable>> = vec!(hitableList);
    //let bvhTree = BvhTree::new(&mut wrapper);

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f32 / ny as f32, aperture, dist_to_focus, 0.0, 1.0);

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
            col += color_ray(&r, &*hitableList, 0);
        }

        col /= ns as f32;
        col = Vec3::new( col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

        let icolor = color::Color (
            (255.99 * col.x) as u8,
            (255.99 * col.y) as u8,
            (255.99 * col.z) as u8
        );

        p.r = icolor.0;
        p.g = icolor.1;
        p.b = icolor.2;
    });

    return pixels;
}