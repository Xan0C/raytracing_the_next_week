use crate::aabb::AABB;
use crate::hitable::Hitable;
use crate::ray::Ray;
use crate::hitable::HitRecord;

use std::cmp::Ordering;

use rand::Rng;

#[derive(Copy, Clone)]
struct NodeId {
    index: usize
}

pub struct BvhTree<'a> {
    nodes: Vec<BvhNode<'a>>,
    root: NodeId
}

struct BvhNode<'a> {
    bbox: Option<AABB>,
    left: Option<NodeId>,
    right: Option<NodeId>,
    hitable: Option<&'a Box<Hitable>>
}

impl<'a> BvhTree<'a> {
    pub fn new(l: &'a mut [Box<Hitable>]) -> Self {
        let mut tree = BvhTree { nodes: Vec::new(), root: NodeId { index: 0 }};
        tree.root = tree.build(l);
        return tree;
    }

    pub fn print(&self) {
      println!("BVH Tree with {} Nodes", self.nodes.len());
    }

    fn build(&mut self, l: &'a mut [Box<Hitable>]) -> NodeId {
        let axis: i32 = rand::thread_rng().gen_range(0, 3);
        let left: NodeId;
        let right: NodeId;

        match axis {
            0 => l.sort_by(|a, b| box_x_compare(a, b)),
            1 => l.sort_by(|a, b| box_y_compare(a, b)),
            2 => l.sort_by(|a, b| box_z_compare(a, b)),
            _ => panic!("unexpected axis")
        }

        if l.len() == 1 {
            return self.add_leaf(&l[0]);
        } else if l.len() == 2 {
            left = self.add_leaf(&l[0]);
            right = self.add_leaf(&l[1]);
        } else {
            let half_len = l.len() / 2;
            let (left_hitables, right_hitables) = l.split_at_mut(half_len);

            left = self.build(left_hitables);
            right = self.build(right_hitables);
        }

        if let Some(left_box) = self.nodes[left.index].bbox {
            if let Some(right_box) = self.nodes[right.index].bbox {
                return self.add_node(AABB::surrounding_box(&left_box, &right_box), left, right);
            }
        }

        panic!("No bounding box in bvh_tree constructor");
    }

    fn add_leaf(&mut self, hitable: &'a Box<Hitable>) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(BvhNode {
            bbox: hitable.bounding_box(),
            left: None,
            right: None,
            hitable: Some(hitable)
        });

        NodeId { index: next_index }
    }

    fn add_node(&mut self, bbox: AABB, left: NodeId, right: NodeId) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(BvhNode {
            left: Some(left),
            right: Some(right),
            bbox: Some(bbox),
            hitable: None
        });

        NodeId { index: next_index }
    }

    fn hit_tree(&self, id: NodeId, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
      let node = &self.nodes[id.index];

      if node.bbox.is_none() || node.bbox.is_some() && node.bbox.unwrap().hit(r, tmin, tmax) {
            match node.hitable {
                Some(ref hitable) => return hitable.hit(r, tmin..tmax),
                None => { }
            }

            let mut hit_left: Option<HitRecord> = None;
            let mut hit_right: Option<HitRecord> = None;

            if let Some(ref left) = node.left {
                //println!("check if left node is hit");
                hit_left = self.hit_tree(*left, r, tmin, tmax);
            }

            if let Some(ref right) = node.right {
                //println!("check if right node is hit");
                hit_right = self.hit_tree(*right, r, tmin, tmax);
            }

            match hit_left {
              Some(left) => {
                match hit_right {
                  Some(right) => if left.t < right.t { return hit_left; } else { return hit_right; },
                  None => return hit_left
                }
              },
              None => {}
            }

            match hit_right {
              Some(_right) => return hit_right,
              None => {}
            }
        }
        return None;
    }
}

impl<'a> Hitable for BvhTree<'a> {
    fn bounding_box(&self) -> Option<AABB> {
        self.nodes[self.root.index].bbox
    }

    fn hit(&self, r: &Ray, t_range: ::std::ops::Range<f32>) -> Option<HitRecord> {
        self.hit_tree(self.root, r, t_range.start, t_range.end)
    }
}

fn box_x_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
  if let Some(box_left) = a.bounding_box() {
    if let Some(box_right) = b.bounding_box() {
      if let Some(cmp) = box_left.min.x.partial_cmp(&box_right.min.x) {
        return cmp;
      } else {
        panic!("Can't compare");
      }
    }
  }

  panic!("No bounding box in BvhNode::new");
}

fn box_y_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
  if let Some(box_left) = a.bounding_box() {
    if let Some(box_right) = b.bounding_box() {
      if let Some(cmp) = box_left.min.y.partial_cmp(&box_right.min.y) {
        return cmp;
      } else {
        panic!("Can't compare");
      }
    }
  }

  panic!("No bounding box in BvhNode::new");
}

fn box_z_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
  if let Some(box_left) = a.bounding_box() {
    if let Some(box_right) = b.bounding_box() {
      if let Some(cmp) = box_left.min.z.partial_cmp(&box_right.min.z) {
        return cmp;
      } else {
        panic!("Can't compare");
      }
    }
  }

  panic!("No bounding box in BvhNode::new");
}