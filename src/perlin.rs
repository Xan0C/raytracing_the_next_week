use crate::vec::Vec3;

use rand::random;
use lazy_static::lazy_static;

lazy_static! {
    static ref RANVEC: Vec<Vec3> = generate();
    static ref PERM_X: Vec<usize> = generate_perm();
    static ref PERM_Y: Vec<usize> = generate_perm();
    static ref PERM_Z: Vec<usize> = generate_perm();
}

pub fn noise(p: &Vec3) -> f32 {
    let u = p.x - p.x.floor();
    let v = p.y - p.y.floor();
    let w = p.z - p.z.floor();

    let i = p.x.floor() as usize;
    let j = p.y.floor() as usize;
    let k = p.z.floor() as usize;

    let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zero(); 2]; 2]; 2];

    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                c[di][dj][dk] = RANVEC[PERM_X[(i + di) & 255] ^ PERM_Y[(j + dj) & 255] ^ PERM_Z[(k + dk) & 255]];
            }
        }
    }

    return perlin_interp(&c, u, v, w);
}

pub fn turb(p: Vec3, depth: usize) -> f32 {
    let mut accum: f32 = 0.0;
    let mut temp_p: Vec3 = p;

    let mut weight = 1.0;

    for i in 0..depth {
        accum += weight * noise(&temp_p);
        weight *= 0.5;
        temp_p *= 2.0;
    }

    accum.abs()
}

fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut accum : f32 = 0.0;

    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let fi = i as f32;
                let fj = j as f32;
                let fk = k as f32;
                let weight = Vec3::new(u - fi, v - fj, w - fk);
                accum += (fi * uu + (1.0 - fi) * (1.0 - uu)) *
                         (fj * vv + (1.0 - fj) * (1.0 - vv)) *
                         (fk * ww + (1.0 - fk) * (1.0 - ww)) * c[i][j][k].dot(weight);   
            }
        }
    }

    accum
}

fn generate() -> Vec<Vec3> {
    let mut p = vec![Vec3::zero(); 256];
    for i in 0..256 {
        p[i] = Vec3::new(
            -1.0 + 2.0 * random::<f32>(),
            -1.0 + 2.0 * random::<f32>(),
            -1.0 + 2.0 * random::<f32>()
        ).normalize();
    }
    return p;
}

fn permute(p: &mut Vec<usize>, n: usize) {
    for i in (0..n).rev() {
        let target = (random::<f32>() * ( (i as f32) + 1.0)) as usize;
        let tmp = p[target];
        p[i] = p[target];
        p[target] = tmp;
    }
}

fn generate_perm() -> Vec<usize> {
    let mut p = vec![0; 256];
    for i in 0..256 {
        p[i] = i;
    }

    permute(&mut p, 256);
    return p;
}