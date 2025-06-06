mod vec3d;
mod color;

use std::fs::File;
use std::io::Write;

use vec3d::Vec3D;
use color::Color;

use crate::color::write_color;


fn main() -> std::io::Result<()>{
    let mut file = File::create("output/file.ppm")?;

    let image_width = 256;
    let image_height = 256;

    write!(file, "P3\n{image_width} {image_height}\n255\n")?;

    for j in 0..image_height {
        let line = image_height - j;
        println!("\rScanlines remaining: {line}");
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;
            
            let pixel_color = Color::new(r, g, b);
            write_color(&mut file, &pixel_color)?;
        }
    }

    Ok(())
}
