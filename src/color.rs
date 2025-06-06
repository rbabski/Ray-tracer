
use crate::vec3d::Vec3D;

pub type Color = Vec3D;


pub fn write_color<T: std::io::Write>(dest: &mut T, color: &Color) -> std::io::Result<()>{
    let r = color.x;
    let g = color.y;
    let b = color.z;

    let rbyte = (255.999 * r) as u8;
    let gbyte = (255.999 * g) as u8;
    let bbyte = (255.999 * b) as u8;

    writeln!(dest, "{} {} {}", rbyte, gbyte, bbyte)
}
