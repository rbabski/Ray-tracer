
use crate::util::vec3d::{Point3D, Vec3D};
use crate::util::ray::Ray;


pub struct HitRecord {
    pub point: Point3D,
    pub normal: Vec3D,
    pub t: f64
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool;
}
