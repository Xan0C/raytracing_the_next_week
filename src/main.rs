//#![feature(const_fn)]
extern crate rand;
extern crate lodepng;
extern crate lazy_static;
extern crate clap;

mod ray;
mod vec;
mod sphere;
mod hitable;
mod camera;
mod material;
mod math;
mod aabb;
mod bvh_node;
mod perlin;
mod texture;
mod xy_rect;
mod box_model;
mod scene;
mod renderer;

use crate::hitable::*;
use crate::sphere::Sphere;
use crate::material::*;
use crate::texture::*;
use crate::xy_rect::*;
use crate::box_model::*;
use crate::scene::*;
use crate::vec::Vec3;
use crate::camera::Camera;

use rand::prelude::*;
use clap::{App, Arg};
use std::sync::Arc;
use std::time::Instant;

fn create_scene() -> Vec<Box<Hitable>> {
    let mut world: Vec<Box<Hitable>> = Vec::new();

    let checkerTexture = Arc::new(
        CheckerTexture::new(
            Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
            Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9)))
        )
    );

    let perlin_tex_sphere = Arc::new(NoiseTexture::new(8.0));

    world.push(Box::new(Sphere::new(
        Vec3 { x: 0.0, y: -1000.0, z: 0.0 },
        1000.0,
        Arc::new(Diffuse::new(checkerTexture))
    )));

    world.push(Box::new(Sphere::new(
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        1.0,
        Arc::new(Dielectric::new(1.5))
    )));
    world.push(Box::new(Sphere::new(
        Vec3 { x: -4.0, y: 1.0, z: 0.0 },
        1.0,
        Arc::new(Diffuse::new(
            perlin_tex_sphere
        ))
    )));
    world.push(Box::new(Sphere::new(
        Vec3 { x: 4.0, y: 1.0, z: 0.0 },
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f32>();
            let center = Vec3::new(a as f32 + 0.9 * random::<f32>(), 0.2, b as f32 + 0.9 * random::<f32>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 { //diffuse
                    world.push(Box::new(Sphere::new_moving_sphere(
                        center,
                        center + Vec3::new(0.0, 0.5 * random::<f32>(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Arc::new(Diffuse::new(
                            Arc::new(
                                ConstantTexture::new(
                                    Vec3::new(random::<f32>() * random::<f32>(), random::<f32>() * random::<f32>(), random::<f32>() * random::<f32>())
                                )
                            )
                        ))
                    )));
                } else if choose_mat < 0.95 {
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new( Vec3::new(0.5 * (1.0 * random::<f32>()), 0.5 * (1.0 * random::<f32>()), 0.5 * (1.0 * random::<f32>()) ), 0.5 * random::<f32>()))
                    )));
                } else {
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5))
                    )));
                }
            } 
        }
    }

    return world;
}

fn simple_light() -> Vec<Box<Hitable>> {
    let mut world: Vec<Box<Hitable>> = Vec::new();

    let perlin_texture_ground = Arc::new(NoiseTexture::new(4.0));
    let perlin_texture_sphere = Arc::new(NoiseTexture::new(4.0));

    world.push(Box::new(Sphere::new(
        Vec3 { x: 0.0, y: -1000.0, z: 0.0 },
        1000.0,
        Arc::new(Diffuse::new(perlin_texture_ground))
    )));

    world.push(Box::new(Sphere::new(
        Vec3 { x: 0.0, y: 2.0, z: 0.0 },
        2.0,
        Arc::new(Diffuse::new(perlin_texture_sphere))
    )));

    world.push(Box::new(Sphere::new(
        Vec3 { x: 0.0, y: 7.0, z: 0.0 },
        2.0,
        Arc::new(DiffuseLight::new(
            Arc::new(ConstantTexture::new(Vec3::new(4.0, 4.0, 4.0)))
        ))
    )));

    world.push(Box::new(XYRect::new(
        3.0, 5.0, 1.0, 3.0, -2.0,
        Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(Vec3::new(4.0, 4.0, 4.0)))))
    )));

    return world;
}

fn light_and_sphere() -> Vec<Box<Hitable>> {
    let mut world: Vec<Box<Hitable>> = Vec::new();

    let white: Arc<Material> = Arc::new(Diffuse::new(
        Arc::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)))
    ));

    let light: Arc<Material> = Arc::new(DiffuseLight::new(
        Arc::new(ConstantTexture::new(Vec3::new(7.0, 7.0, 7.0)))
    ));

    world.push(Box::new(XZRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        Arc::clone(&light)
    )));

     world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.0))
    )));

    return world;
}

fn cornell_box() -> Vec<Box<Hitable>> {
    let mut world: Vec<Box<Hitable>> = Vec::new();

    let red: Arc<Material> = Arc::new(Diffuse::new(
        Arc::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)))
    ));

    let white: Arc<Material> = Arc::new(Diffuse::new(
        Arc::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)))
    ));

    let green: Arc<Material> = Arc::new(Diffuse::new(
        Arc::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)))
    ));

    let light: Arc<Material> = Arc::new(DiffuseLight::new(
        Arc::new(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0)))
    ));
    
    world.push(Box::new(FlipNormal::new(Box::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&green)
    )))));

    world.push(Box::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&red)
    )));

    world.push(Box::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::clone(&light)
    )));

    world.push(Box::new(FlipNormal::new(Box::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white)
    )))));

    world.push(Box::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&white)
    )));

    world.push(Box::new(FlipNormal::new(Box::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white)
    )))));

    world.push(
        Box::new(Translate::new(
            Arc::new(RotateY::new(
                Arc::new(BoxModel::new(Vec3::zero(), Vec3::new(165.0, 165.0, 165.0), Arc::clone(&white))),
                -18.0
            )),
            Vec3::new(130.0, 0.0, 65.0)
        ))
    );

    world.push(
        Box::new(Translate::new(
            Arc::new(RotateY::new(
                Arc::new(BoxModel::new(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), Arc::clone(&white))),
                15.0
            )),
            Vec3::new(265.0, 0.0, 295.0)
        ))
    );

    return world;
}

fn cornell_smoke() -> Vec<Box<Hitable>> {
    let mut world: Vec<Box<Hitable>> = Vec::new();

    let red: Arc<Material> = Arc::new(Diffuse::new(
        Arc::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)))
    ));

    let white: Arc<Material> = Arc::new(Diffuse::new(
        Arc::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)))
    ));

    let green: Arc<Material> = Arc::new(Diffuse::new(
        Arc::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)))
    ));

    let light: Arc<Material> = Arc::new(DiffuseLight::new(
        Arc::new(ConstantTexture::new(Vec3::new(7.0, 7.0, 7.0)))
    ));
    
    world.push(Box::new(FlipNormal::new(Box::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&green)
    )))));

    world.push(Box::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&red)
    )));

    world.push(Box::new(XZRect::new(
        113.0,
        443.0,
        127.0,
        432.0,
        554.0,
        Arc::clone(&light)
    )));

    world.push(Box::new(FlipNormal::new(Box::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white)
    )))));

    world.push(Box::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&white)
    )));

    world.push(Box::new(FlipNormal::new(Box::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white)
    )))));

 
    let box1 = Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BoxModel::new(Vec3::zero(), Vec3::new(165.0, 165.0, 165.0), Arc::clone(&white))),
            -18.0
        )),
        Vec3::new(130.0, 0.0, 65.0)
    ));


    let box2 = Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BoxModel::new(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), Arc::clone(&white))),
            15.0
        )),
        Vec3::new(265.0, 0.0, 295.0)
    ));

    world.push(Box::new(
        ConstantMedium::new(box1, 0.01, Arc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))))
    ));

    world.push(Box::new(
        ConstantMedium::new(box2, 0.01, Arc::new(ConstantTexture::new(Vec3::zero())))
    ));

    return world;
}

fn the_next_week() -> Vec<Box<Hitable>> {
    let mut world: Vec<Box<Hitable>> = Vec::new();

    let white: Arc<Material> = Arc::new(Diffuse::new(
        Arc::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)))
    ));

    let ground: Arc<Material> = Arc::new(Diffuse::new(
        Arc::new(ConstantTexture::new(Vec3::new(0.48, 0.83, 0.53)))
    ));

    let w = 100;
    let y0 = 0;
    //let mut boxlist: Vec<Box<Hitable>> = Vec::new();

    for i in 0..20 {
        for j in 0..20 {
            let x0 = -1000 + i * w;
            let z0 = -1000 + j * w;

            let x1 = x0 + w;
            let y1 = 100.0 * (random::<f32>() + 0.01);

            let z1 = z0 + w;

            world.push(
                Box::new(
                    BoxModel::new(
                        Vec3::new(x0 as f32, y0 as f32, z0 as f32),
                        Vec3::new(x1 as f32, y1 as f32, z1 as f32),
                        Arc::clone(&ground)
                    )
                )
            );
        }
    }

    let light: Arc<Material> = Arc::new(DiffuseLight::new(
        Arc::new(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0)))
    ));

    world.push(Box::new(XZRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        Arc::clone(&light)
    )));

    let center = Vec3::new(400.0, 400.0, 200.0);

    world.push(Box::new(Sphere::new_moving_sphere(
        center,
        center + Vec3::new(30.0, 0.0, 0.0),
        0.0,
        1.0,
        50.0,
        Arc::new(Diffuse::new(
            Arc::new(ConstantTexture::new(Vec3::new(0.7, 0.3, 0.1)))
        ))
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5))
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.0))
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5))
    )));

    world.push(Box::new(ConstantMedium::new(
        Arc::new(Sphere::new(
            Vec3::new(360.0, 150.0, 145.0),
            70.0,
            Arc::new(Dielectric::new(1.5))
        )),
        0.2,
        Arc::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.9)))
    )));

    world.push(Box::new(ConstantMedium::new(
        Arc::new(Sphere::new(
            Vec3::new(0.0, 0.0, 0.0),
            50000.0,
            Arc::new(Dielectric::new(1.5))
        )),
        0.0001,
        Arc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0)))
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        Arc::new(Diffuse::new(Arc::new(
            ImageTexture::from_image("earthmap.png")
        )))
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Diffuse::new(Arc::new(
            NoiseTexture::new(0.1)
        )))
    )));

    let mut list = HitableList::new();

    for _ in 0..1000 {
        list.push(Box::new(Sphere::new(
            Vec3::new(165.0 *  random::<f32>(), 165.0 *  random::<f32>(), 165.0 *  random::<f32>()),
            10.0,
            Arc::clone(&white)
        )));
    }

    world.push(Box::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(list),
            15.0
        )),
        Vec3::new(-100.0, 270.0, 395.0)
    )));
    
    return world;
}

fn main() {
    let matches = App::new("rttnw")
        .author("Soeren Vullriede")
        .about("Ray Tracer built from the book raytracing the next week by Peter Shirley")
        .arg(Arg::with_name("output")
            .short("o")
            .value_name("FILE")
            .help("image destination file")
            .takes_value(true))
         .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .value_name("WIDTH")
            .help("image width in pixels")
            .takes_value(true))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .value_name("HEIGHT")
            .help("image height in pixels")
            .takes_value(true))
        .arg(Arg::with_name("samples")
            .short("s")
            .long("samples")
            .value_name("SAMPLES")
            .help("number of samples per pixel")
            .takes_value(true))
        .arg(Arg::with_name("max_ray_depth")
            .short("d")
            .long("max-ray-depth")
            .value_name("DEPTH")
            .help("maximum ray depth")
            .takes_value(true))
        .get_matches();

    let start = Instant::now();

    let width: usize = matches.value_of("width").unwrap_or("640").parse::<usize>().unwrap();;
    let height: usize = matches.value_of("height").unwrap_or("320").parse::<usize>().unwrap();;
    let rays = matches.value_of("samples").unwrap_or("10").parse::<usize>().unwrap();
    let max_ray_depth = matches.value_of("max_ray_depth").unwrap_or("10").parse::<u32>().unwrap();

    //scene
    let mut world = cornell_box();
    let scene = Scene::new(&mut world, max_ray_depth);

    //camera
    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 40.0, width as f32 / height as f32, aperture, dist_to_focus, 0.0, 1.0);

    //render
    let pixels = scene.render(&camera, width, height, rays);

    let time = Instant::now() - start;
    let time_secs = time.as_secs();
    let time_millis = time.subsec_millis();

    println!(
        "Done in {} seconds.",
        time_secs as f32 + time_millis as f32 / 1000.0
    );

    let filename = "output.png";
    match lodepng::encode24_file(filename, &pixels, width, height) {
        Ok(()) => {}
        Err(err) => println!("Error writing file \"{}\": {}", filename, err)
    }

}