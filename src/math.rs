use crate::vec::Vec3;

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - n * (2.0 * v.dot(n));
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        let refracted = (uv - n * dt) * ni_over_nt - n * discriminant.sqrt();
        return Some(refracted);
    }

    return None;
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}