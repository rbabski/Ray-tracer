use crate::util::{objects::hittable::{HitRecord, Hittable}, ray::Ray};


#[derive(Default)]
pub struct HittableList {
    objects : Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn push(&mut self, hitable: Box<dyn Hittable + 'static>) {
        self.objects.push(hitable);
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
