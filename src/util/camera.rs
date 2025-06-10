use std::f64::INFINITY;

use crate::util::{color::{write_color, Color}, hittable_list::HittableList, objects::hittable::{HitRecord, Hittable}, ray::Ray, vec3d::{Point3D, Vec3D}};


#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: i32,
    camera_center: Point3D,
    pixel00_loc: Point3D,
    pixel_delta_u: Vec3D,
    pixel_delta_v: Vec3D
}

impl Camera {
    pub fn render<T: std::io::Write>(&mut self, file: &mut T, world: &HittableList) -> std::io::Result<()> {
        self.initialize();

        write!(file, "P3\n{} {}\n255\n", self.image_width, self.image_height)?;

        for j in 0..self.image_height {
            let line = self.image_height - j;
            println!("\rScanlines remaining: {line}");
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.camera_center;

                let ray = Ray::new(self.camera_center, ray_direction);
                
                let pixel_color = Camera::ray_color(ray, &world);

                write_color(file, &pixel_color)?;
        }
    }
    Ok(())
    
    }


    fn initialize(&mut self) {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
    
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        self.image_height = if image_height < 1 { 1 } else { image_height };

        // camera

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center = Point3D::new(0.0, 0.0, 0.0);

        // delta vectors pixel to pixel

        let viewport_u = Vec3D::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3D::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / image_width as f64;
        self.pixel_delta_v = viewport_v / image_height as f64;


        // upper left pixel location

        let viewport_upper_left = camera_center - Vec3D::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    
    }



    fn ray_color(ray: Ray, world: &HittableList) -> Color {
        let mut rec = HitRecord::default();
    
        if world.hit(ray, 0.0, INFINITY, &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0,1.0,1.0));
        }

        let unit_dir = ray.direction.to_unit();
        let a = 0.5*(unit_dir.y + 1.0);
        (1.0 - a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
    }
}