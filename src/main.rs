use std::fs::{create_dir_all, File};
use std::path::Path;

mod util; 

use crate::util::camera::Camera;
use crate::util::color::Color;
// use util::vec3d::{dot};
use crate::util::hittable_list::HittableList;

use crate::util::material::{Dielectric, Lambertian, Material, Metal};
use crate::util::objects::sphere::Sphere;
use crate::util::vec3d::{Point3D, Vec3D};


fn main() -> std::io::Result<()>{

    //colors

    //let ground = Color::new(0.8, 0.8, 0.0);
    let center = Color::new(0.1, 0.2, 0.5);
    //let left = Color::new(0.0, 0.0, 1.0);
    let right = Color::new(0.8, 0.6, 0.2);

    // materials

    //let material_ground = Material::Lambertian(Lambertian { albedo: ground });
    let material_center = Material::Lambertian(Lambertian { albedo: center });
    let material_left = Material::Dielectric(Dielectric { refraction_index: 1.5});
    let material_bubble = Material::Dielectric(Dielectric {refraction_index: 1.0/1.5});
    let material_right = Material::Metal(Metal { albedo: right, fuzz: 1.0});

    // world

    let mut world = HittableList::default();

    //let s1 = Box::new(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0, material_ground));
    //world.push(s1);

    let s2 = Box::new(Sphere::new(Point3D::new(0.0, 0.0, -1.2), 0.5, material_center));
    world.push(s2);

    let s3 = Box::new(Sphere::new(Point3D::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.push(s3);

    let s4 = Box::new(Sphere::new(Point3D::new(-1.0, 0.0, -1.0), 0.4, material_bubble));
    world.push(s4);

    let s5 = Box::new(Sphere::new(Point3D::new(1.0, 0.0, -1.0), 0.5, material_right));
    world.push(s5);

    // rendering

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;

    camera.samples_per_pixel = 20;
    camera.max_depth = 5;

    camera.vfov = 30.0;
    camera.lookfrom = Point3D::new(-2.0, 2.0, 1.0); 
    camera.lookat = Point3D::new(0.0, 0.0, -1.0);
    camera.vup = Vec3D::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 10.0;
    camera.focus_dist = 3.4;


    let output_dir = Path::new("output");
    if !output_dir.exists() {
        create_dir_all(output_dir)?; 
    }

    let mut file = File::create("output/file.ppm")?;

    camera.render(&mut file, &world)?;

    Ok(())
}
