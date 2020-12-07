pub mod lambertian;

use super::color::Color;
use super::hit::Hit;
use super::ray::Ray;
use super::vec3::Vec3;

use self::lambertian::Lambertian;

pub enum Material {
    Lambertian(Lambertian),
}

pub struct Scatter {
    color: *const Color,
    scattered: *const Ray
}

pub trait Scatterable {
    fn scatter(&self, incident: &Ray, hit: &Hit) -> Option<Scatter>;
}

impl Scatterable for Material {
    fn scatter(&self, incident: &Ray, hit: &Hit) -> Option<Scatter> {
        match *self {
            Material::Lambertian(ref mat) => mat.scatter(incident, hit)
        }
    }
}