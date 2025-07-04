use crate::util::{material::{Material}, objects::hittable::{HitRecord, Hittable}, ray::Ray, vec3d::{dot, Point3D}};


pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
    pub material: Material
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material
        }
    }
}


impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool{
        let o_c = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = dot(ray.direction, o_c);
        let c = o_c.length_squared() - self.radius*self.radius; 

        let delta = h*h - a * c;
        
        if delta < 0.0 {
            return false;
        }

        let sqrt_d = delta.sqrt();

        let try_roots = [(h - sqrt_d) / a, (h + sqrt_d) / a];

        let root;
        if let Some(valid_root) = try_roots.into_iter().find(|&r| r > t_min && r < t_max) {
            root = valid_root;
        } else {
            return false;
        }

        hit_rec.t = root;
        hit_rec.point = ray.at(hit_rec.t);

        let out_normal = (hit_rec.point - self.center) / self.radius;
        hit_rec.set_face_normal(ray, out_normal);
        hit_rec.material = self.material.clone();

        return true;
    }




}