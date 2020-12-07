use super::vec3::Vec3;
use super::ray::Ray;
use super::materials::Material;

pub struct Hit {
    t: f64,
    point: Vec3,
    normal: Vec3,
    material: *const Material
}

impl Hit {
    pub fn new (t: f64, point: Vec3, normal: Vec3, material: &Material) -> Hit {
        Hit{t, point, normal, material}
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn material(&self) -> *const Material {
        self.material
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<Hit>;
}