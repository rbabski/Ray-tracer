use std::fs::{create_dir_all, File};
use std::path::Path;

mod util;
mod config; 


fn main() -> std::io::Result<()>{
    let world = config::world_init();
    let mut camera = config::camera_init();


    let output_dir = Path::new("output");
    if !output_dir.exists() {
        create_dir_all(output_dir)?; 
    }

    let mut file = File::create("output/file.ppm")?;


    camera.render(&mut file, &world)?;

    
    Ok(())
}
