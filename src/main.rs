use std::fs::File;
use std::io::Write;

mod util; 

use util::vec3d::{dot, cross};
use util::color::{Color, write_color};

use crate::util::vec3d::{Point3D, Vec3D};
use util::ray::{Ray};


fn ray_color(ray: &Ray) -> Color{
    if is_hitting_sphere(Vec3D::new(0.0, 0.0, -1.0), 0.5, ray){
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_dir = ray.direction.to_unit();
    let a = 0.5*(unit_dir.y + 1.0);
    (1.0 - a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
}


fn is_hitting_sphere(center: Point3D, radius: f64, ray: &Ray) -> bool{
    let o_c = center - ray.origin;
    let a = dot(ray.direction, ray.direction);
    let b = 2.0 * dot(ray.direction, o_c);
    let c = dot(o_c, o_c) - radius*radius; 

    let delta = b*b - 4.0*a*c;
    delta >= 0.0
}


fn main() -> std::io::Result<()>{
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3D::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3D::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3D::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center - Vec3D::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


    let mut file = File::create("output/file.ppm")?;

    write!(file, "P3\n{image_width} {image_height}\n255\n")?;
    for j in 0..image_height {
        let line = image_height - j;
        println!("\rScanlines remaining: {line}");
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&ray);

            write_color(&mut file, &pixel_color)?;
        }
    }

    Ok(())
}
