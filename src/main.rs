use std::fs::File;

mod util; 

use crate::util::camera::Camera;
// use util::vec3d::{dot};
use crate::util::hittable_list::HittableList;

use crate::util::objects::sphere::Sphere;
use crate::util::vec3d::Point3D;

fn main() -> std::io::Result<()>{
    // world

    let mut world = HittableList::default();

    let s1 = Box::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5));
    world.push(s1);

    let s2 = Box::new(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0));
    world.push(s2);

    // rendering

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;

    let mut file = File::create("output/file.ppm")?;

    camera.render(&mut file, &world)?;

    Ok(())
}
