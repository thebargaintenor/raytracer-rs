use super::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at(&self, time: f64) -> Vec3 {
        self.origin + time * self.direction
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}

#[test]
fn test_point_at() {
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let direction: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    let r: Ray = Ray::new(origin, direction);
    assert_eq!(r.point_at(0.5), Vec3::new(0.5, 0.5, 0.5));
}