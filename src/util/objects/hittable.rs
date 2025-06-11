

use crate::util::material::{Material};
use crate::util::vec3d::{dot, Point3D, Vec3D};
use crate::util::ray::Ray;


pub trait Hittable {
    fn  hit(&self, ray: Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool;
}


#[derive(Clone, Default)]
pub struct HitRecord {
    pub point: Point3D,
    pub normal: Vec3D,
    pub t: f64,
    pub front_face: bool,
    pub material: Material
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: Ray, out_normal: Vec3D) {
        self.front_face = dot(ray.direction, out_normal) < 0.0;
        self.normal = if self.front_face {out_normal} else {-out_normal};
    }
}
