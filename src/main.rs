use std::fs::File;

mod util; 

use crate::util::camera::Camera;
use crate::util::color::Color;
// use util::vec3d::{dot};
use crate::util::hittable_list::HittableList;

use crate::util::material::{Lambertian, Material, Metal};
use crate::util::objects::sphere::Sphere;
use crate::util::vec3d::Point3D;


fn main() -> std::io::Result<()>{

    //colors

    let ground = Color::new(0.8, 0.8, 0.0);
    let center = Color::new(0.1, 0.2, 0.5);
    let left = Color::new(0.8, 0.8, 0.8);
    let right = Color::new(0.4, 0.4, 1.0);

    // materials

    let material_ground = Material::Lambertian(Lambertian { albedo: ground });
    let material_center = Material::Lambertian(Lambertian { albedo: center });
    let material_left = Material::Metal(Metal { albedo: left });
    let material_right = Material::Metal(Metal { albedo: right });

    // world

    let mut world = HittableList::default();

    let s1 = Box::new(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.push(s1);

    let s2 = Box::new(Sphere::new(Point3D::new(0.0, 0.0, -1.2), 0.5, material_center));
    world.push(s2);

    let s3: Box<Sphere> = Box::new(Sphere::new(Point3D::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.push(s3);

    let s4 = Box::new(Sphere::new(Point3D::new(1.0, 0.0, -1.0), 0.5, material_right));
    world.push(s4);

    // rendering

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 10;
    camera.max_depth = 5;

    let mut file = File::create("output/file.ppm")?;

    camera.render(&mut file, &world)?;

    Ok(())
}
