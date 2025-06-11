
use std::ops::Mul;
use crate::util::vec3d::Vec3D;

pub type Color = Vec3D;


pub fn linear_to_gamma(lin_comp: f64) -> f64 {
    if lin_comp > 0.0 {lin_comp.sqrt()} else {0.0}
}


pub fn clamp(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x > 0.999 {
        0.999
    } else {
        x
    }
}


pub fn write_color<T: std::io::Write>(dest: &mut T, color: &Color) -> std::io::Result<()>{
    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);

    let rbyte = (256.0 * clamp(r)) as u8;
    let gbyte = (256.0 * clamp(g)) as u8;
    let bbyte = (256.0 * clamp(b)) as u8;

    writeln!(dest, "{} {} {}", rbyte, gbyte, bbyte)
}




impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
