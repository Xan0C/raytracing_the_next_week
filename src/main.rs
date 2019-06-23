//#![feature(const_fn)]

extern crate rand;
extern crate lodepng;
extern crate lazy_static;

mod ray;
mod vec;
mod color;
mod sphere;
mod hitable;
mod camera;
mod material;
mod math;
mod render;
mod aabb;
mod bvh_node;
mod perlin;
mod texture;
mod xy_rect;
mod box_model;

use std::time::Instant;

fn main() {
    let start = Instant::now();

    let width: usize = 1280;
    let height: usize = 720;
    let rays = 200;

    let pixels = render::render(width, height, rays);

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