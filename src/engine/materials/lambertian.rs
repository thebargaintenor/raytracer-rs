use super::super::math;
use super::{
    Color,
    Hit,
    Ray,
    Scatter,
    Scatterable,
    Vec3,
};

pub struct Lambertian {
    albedo: Color,
}

impl Scatterable for Lambertian {
    /// Ray interaction with diffuse material
    #[allow(unused_variables)] // incident ray not a factor in calculation
    fn scatter(&self, incident: &Ray, hit: &Hit) -> Option<Scatter> {
        let target: Vec3 = hit.point() + hit.normal() + math::random_in_unit_sphere();
        let scattered = Ray::new(hit.point(), target - hit.point());

        Some(Scatter {
            color: &self.albedo,
            scattered: &scattered,
        })
    }
}
