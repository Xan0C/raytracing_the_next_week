use crate::bvh_node::BvhTree;
use crate::hitable::Hitable;
use crate::renderer;
use crate::camera::Camera;

use lodepng::RGB;

pub trait Renderable {
     fn render(&self, camera: &Camera, width: usize, height: usize, samples: usize) -> Vec<RGB<u8>>;
}

pub struct Scene<'a> {
    pub bvh: BvhTree<'a>,
    pub max_ray_depth: u32
}

impl<'a> Scene<'a> {
    pub fn new(models: &'a mut Vec<Box<Hitable>>, max_ray_depth: u32) -> Self {
        Scene {
            bvh: BvhTree::new(models),
            max_ray_depth
        }
    }
}

impl<'a> Renderable for Scene<'a> {
    fn render(&self, camera: &Camera, width: usize, height: usize, samples: usize) -> Vec<RGB<u8>> {
        renderer::render(self, camera, width, height, samples)
    }
}