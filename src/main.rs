
use std::fs::File;
use std::io::Write;


fn main() -> std::io::Result<()>{
    let mut file = File::create("output/file.ppm")?;

    let image_width = 256;
    let image_height = 256;

    write!(file, "P3\n{image_width} {image_height}\n255\n")?;

    let mut r: f32;
    let mut g: f32;
    let mut b: f32;

    for j in 0..image_height {
        for i in 0..image_width {
            r = i as f32 / (image_width - 1) as f32;
            g = j as f32 / (image_height - 1) as f32;
            b = 0.0;

            let ir= (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            write!(file, "{ir} {ig} {ib}\n")?;
        }
    }

    Ok(())
}
