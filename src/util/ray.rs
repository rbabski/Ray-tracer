use crate::util::vec3d::{Point3D, Vec3D};


#[derive(Debug, Clone, Copy)]
pub struct Ray{
    pub origin: Point3D,
    pub direction: Vec3D
}

impl Ray {
    pub fn new(origin: Vec3D, direction: Vec3D) -> Ray{
        Ray{origin, direction}
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.origin + t * self.direction
    }
}