use crate::vec::Vec3;
use crate::ray::Ray;
use crate::color;
use crate::hitable::*;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::material::*;
use crate::texture::*;
use crate::xy_rect::*;
use crate::box_model::*;
use crate::bvh_node::*;

use lodepng::RGB;
use rand::distributions::Uniform;
use rand::prelude::*;
use rayon::prelude::*;
use std::sync::Arc;

fn create_scene() -> Vec<Box<Hitable>> {
    let mut world: Vec<Box<Hitable>> = Vec::new();
/*
    let checkerTexture = Box::new(
        CheckerTexture::new(
            Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
            Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9)))
        )
    );
*/
    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    let perlin_tex_sphere = Arc::new(NoiseTexture::new(8.0));

    world.push(Box::new(Sphere::new(
        Vec3 { x: 0.0, y: -1000.0, z: 0.0 },
        1000.0,
        Arc::new(Diffuse::new(perlin_texture))
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
            Arc::new(ImageTexture::from_image("earthmap.png"))
            //perlin_tex_sphere
        ))
    )));
    world.push(Box::new(Sphere::new(
        Vec3 { x: 4.0, y: 1.0, z: 0.0 },
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))
    )));
/*
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
                        Box::new(Diffuse::new(
                            Box::new(
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
*/
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

    //world.push(Box::new(
    //    BvhTree::new(boxlist)
    //))
    
    return world;
}

fn color_ray(r: &Ray, world: &Hitable, depth: u32) -> Vec3 {
    if let Some(record) = world.hit(&r, 0.001..std::f32::MAX) {

        if depth > 50 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let scattered = record.material.scatter(&r, &record);
        let emitted = record.material.emitted(record.u, record.v, record.p);

        if let Some(hit) = scattered.ray {
            return emitted + color_ray(&hit, world, depth+1) * scattered.attenuation;
        } else {
            emitted
        }

    } else {
        Vec3::zero()
    }
}

pub fn render(width: usize, height: usize, samples: usize) -> Vec<RGB<u8>> {
    let nx = width;
    let ny = height;
    let ns = samples;

    let mut pixels: Vec<RGB<u8>> = vec![RGB {r: 0, g: 0, b: 0}; nx * ny];
    
    let mut scene = the_next_week();
    let hitableList = Box::new(HitableList::from_list(scene));
    //let mut wrapper: Vec<Box<Hitable>> = vec!(hitableList);
    //let bvhTree = BvhTree::new(&mut scene);

    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 40.0, nx as f32 / ny as f32, aperture, dist_to_focus, 0.0, 1.0);

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