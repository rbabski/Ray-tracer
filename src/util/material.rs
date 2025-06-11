
use crate::util::{color::Color, objects::hittable::HitRecord, ray::Ray, vec3d::Vec3D};


#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric)
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian(Lambertian::default())
    }
}


//////

#[derive(Clone, Default)]
pub struct Lambertian {
    pub albedo: Color
}

impl Lambertian {
    pub fn scatter(&self, ray_in: Ray, rec: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal + Vec3D::random_unit_vector();

        if scatter_direction.near_zero() {
            return false;
        }

        *scattered = Ray::new(rec.point, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}


///////

#[derive(Clone, Default)]
pub struct Metal {
    pub albedo: Color
}

impl Metal {
    pub fn scatter(&self, ray_in: Ray, rec: HitRecord,  attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = ray_in.direction.reflect(rec.normal);
        
        *scattered = Ray::new(rec.point, reflected);
        *attenuation = self.albedo;

        true
    }
}



//////

#[derive(Clone, Default)]
pub struct Dielectric {

}


impl Material {
    pub fn scatter(&self, ray_in: Ray, rec: HitRecord, attentuation: &mut Color, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(mat) => mat.scatter(ray_in, rec, attentuation, scattered),
            Material::Metal(mat) => mat.scatter(ray_in, rec, attentuation, scattered),
            Material::Dielectric(mat) => false
        }
    }
    
}