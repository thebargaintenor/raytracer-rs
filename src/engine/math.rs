extern crate rand;

use std::f64::consts::PI;

use rand::Rng;

use super::vec3::Vec3;

/// Returns a point within a unit sphere centered at origin
pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    // generate random point in spherical coordinates
    let rho = rng.gen::<f64>();
    let theta = 2.0 * PI * rng.gen::<f64>();
    let phi = PI * rng.gen::<f64>();

    let x = rho * phi.sin() * theta.cos();
    let y = rho * phi.sin() * theta.sin();
    let z = rho * phi.cos();

    Vec3::new(x, y, z)
}

/// Returns random point in unit circle about origin
pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();

    let r = rng.gen::<f64>();
    let theta = 2.0 * PI * rng.gen::<f64>();

    let x = r * theta.cos();
    let y = r * theta.sin();

    Vec3::new(x, y, 0.0)
}

/// Reflect a vector relative to plane normal
pub fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - (normal * (incident.dot(normal) * 2.0))
}

/// Handles the bending of a ray at the interface of two materials
pub fn refract(incident: Vec3, normal: Vec3, refractive_idx_ratio: f64) -> Option<Vec3> {
    let uv = incident.unit();
    let dtheta = uv.dot(normal);
    let discriminant = 1.0 - refractive_idx_ratio.powi(2) * (1.0 - dtheta.powi(2));

    if discriminant > 0.0 {
        let refracted: Vec3 =
            (uv - normal) * dtheta * refractive_idx_ratio - (normal * discriminant.sqrt());
        return Some(refracted);
    }

    None
}

/// Approximates dielectric material reflectivity
pub fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
