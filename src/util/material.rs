

use rand::{Rng};

use crate::util::{color::Color, objects::hittable::HitRecord, ray::Ray, vec3d::{dot, Vec3D}};


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
    pub fn scatter(&self, _ray_in: Ray, rec: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
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
    pub albedo: Color,
    pub fuzz: f64
}

impl Metal {
    pub fn scatter(&self, ray_in: Ray, rec: HitRecord,  attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected = ray_in.direction.reflect(rec.normal);
        reflected = reflected.to_unit() + self.fuzz * Vec3D::random_unit_vector();
        
        *scattered = Ray::new(rec.point, reflected);
        *attenuation = self.albedo;

        dot(scattered.direction, rec.normal) > 0.0
    }
}



//////

#[derive(Clone, Default)]
pub struct Dielectric {
    pub refraction_index: f64
}

impl Dielectric {
    pub fn scatter(&self, ray_in: Ray, rec: HitRecord,  attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let ri = if rec.front_face {1.0 / self.refraction_index} else {self.refraction_index};

        let unit_direction = ray_in.direction.to_unit();

        let cos_theta = f64::min(dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let direction: Vec3D;

        let mut rng = rand::rng();
        let random = rng.random_range(0.0..1.0);
        if ri * sin_theta > 1.0 || Self::reflectance(cos_theta, ri) > random {
            direction = unit_direction.reflect(rec.normal);
        } else {
            direction = unit_direction.refract(rec.normal, ri);
        }

        *scattered = Ray::new(rec.point, direction);

        true
    }

    fn reflectance(cos: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0*r0;
        
        r0 + (1.0 - r0)*(1.0-cos).powi(5)
    }
}


impl Material {
    pub fn scatter(&self, ray_in: Ray, rec: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(mat) => mat.scatter(ray_in, rec, attenuation, scattered),
            Material::Metal(mat) => mat.scatter(ray_in, rec, attenuation, scattered),
            Material::Dielectric(mat) => mat.scatter(ray_in, rec, attenuation, scattered)
        }
    }
    
}