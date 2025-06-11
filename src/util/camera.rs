

use crate::util::{color::{write_color, Color}, hittable_list::HittableList, objects::hittable::{HitRecord, Hittable}, ray::Ray, vec3d::{cross, Point3D, Vec3D}};
use rand::{rngs::ThreadRng, Rng};



#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub lookfrom: Point3D,
    pub lookat: Point3D,
    pub vup: Vec3D,

    image_height: i32,
    camera_center: Point3D,
    pixel00_loc: Point3D,
    pixel_delta_u: Vec3D,
    pixel_delta_v: Vec3D,
    pixel_samples_scale: f64,
    u: Vec3D,
    v: Vec3D, 
    w: Vec3D
}

impl Camera {
    pub fn render<T: std::io::Write>(&mut self, file: &mut T, world: &HittableList) -> std::io::Result<()> {
        self.initialize();
        let mut rng = rand::rng();

        write!(file, "P3\n{} {}\n255\n", self.image_width, self.image_height)?;

        for j in 0..self.image_height {
            let line = self.image_height - j;
            println!("\rScanlines remaining: {line}");
            for i in 0..self.image_width {
                /*let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.camera_center;

                let ray = Ray::new(self.camera_center, ray_direction);
                
                let pixel_color = Camera::ray_color(ray, &world);

                write_color(file, &pixel_color)?;*/

                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j, &mut rng);
                    pixel_color += Self::ray_color(ray, self.max_depth, &world);
                }
                pixel_color = self.pixel_samples_scale*pixel_color;
                
                write_color(file, &pixel_color)?;
        }
    }
    Ok(())
    
    }


    fn initialize(&mut self) {
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.camera_center = self.lookfrom;
    
        let image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if image_height < 1 { 1 } else { image_height };

        // camera

        let focal_length = (self.lookfrom - self.lookat).length();
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);

        self.w = (self.lookfrom - self.lookat).to_unit();
        self.u = (cross(self.vup, self.w)).to_unit();
        self.v = cross(self.w, self.u);

        // delta vectors pixel to pixel

        let viewport_u = viewport_width * self.u;
        let viewport_v = - viewport_height * self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / image_height as f64;


        // upper left pixel location

        let viewport_upper_left = self.camera_center - focal_length*self.w - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    
    }



    fn ray_color(ray: Ray, depth: i32, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color::new(0.0,0.0,0.0);
        }

        let mut rec: HitRecord = HitRecord::default();
    
        if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
            let mut attenuation = Color::new(0.0, 0.0, 0.0);
            let mut scattered = Ray::default(); 
    
            let material = &rec.material.clone();
            if material.scatter(ray, rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_color(scattered, depth - 1, world);
            }
    
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_dir = ray.direction.to_unit();
        let a = 0.5*(unit_dir.y + 1.0);
        (1.0 - a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
    }


    fn get_ray(&self, i: i32, j: i32, rng: &mut ThreadRng) -> Ray {
        let offset = Self::sample_square(rng);
        let pixel_sample = self.pixel00_loc + ((i as f64 + offset.x) * self.pixel_delta_u) + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.camera_center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }


    fn sample_square(rng: &mut ThreadRng) -> Vec3D {
        let x = rng.random_range(-0.5..0.5);
        let y = rng.random_range(-0.5..0.5);

        Vec3D::new(x, y, 0.0)
    }

}