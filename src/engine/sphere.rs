use super::hit::{Hit, Hittable};
use super::materials::Material;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    fn create_hit_record(&self, r: &Ray, t: f64) -> Hit {
        let location: Vec3 = r.point_at(t);
        let normal: Vec3 = (location - self.center) / self.radius;

        Hit::new(t, location, normal, &self.material)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<Hit> {
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius.powi(2);

        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t = (-b - discriminant.sqrt()) / a;
        if t < tmax && t > tmin {
            return Some(self.create_hit_record(r, t));
        }

        let t = (-b + discriminant.sqrt()) / a;
        if t < tmax && t > tmin {
            return Some(self.create_hit_record(r, t));
        }

        None
    }
}
