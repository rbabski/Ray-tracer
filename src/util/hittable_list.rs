use crate::util::{material::Material, objects::{hittable::{HitRecord, Hittable}, sphere::Sphere}, ray::Ray, vec3d::Point3D};


#[derive(Default)]
pub struct HittableList {
    objects : Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn push(&mut self, hittable: Box<dyn Hittable + 'static>) {
        self.objects.push(hittable);
    }

    pub fn add_sphere(&mut self, center: Point3D, radius: f64, material: Material) -> &mut Self {
        self.push(Box::new(Sphere::new(center, radius, material)));
        self
    }
}


impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
    
        for obj in self.objects.iter() {
            if obj.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
    
        hit_anything
    }
}
