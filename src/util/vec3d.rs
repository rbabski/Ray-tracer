
use std::{fmt, ops::{Add, AddAssign, Div, Mul, Neg, Sub}};

use rand::{Rng};
use rand_distr::{Distribution, StandardNormal};

pub type Point3D = Vec3D;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3D {
    pub x: f64, 
    pub y: f64,
    pub z: f64
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3D {
        Vec3D{x, y, z}
    }

    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn to_unit(&self) -> Vec3D {
        *self / self.length()
    }    

    pub fn random(min: f64, max: f64) -> Vec3D {
        let mut rng = rand::rng();

        let x = rng.random_range(min..max);
        let y = rng.random_range(min..max);
        let z = rng.random_range(min..max);

        Vec3D{x, y, z}
    }

    fn random_unit_vector() -> Vec3D {
        let mut rng = rand::rng();
    
        loop {
            let x: f64 = StandardNormal.sample(&mut rng);
            let y: f64 = StandardNormal.sample(&mut rng);
            let z: f64 = StandardNormal.sample(&mut rng);
        
            let v = Vec3D::new(x, y, z);
            let len_sq = v.length_squared();
    
            if len_sq > 1e-160 {
                return v / len_sq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3D) -> Vec3D {
        let v = Self::random_unit_vector();

        if dot(v, normal) > 0.0 {
            v
        } else {
            -v
        }
    }
}

impl fmt::Display for Vec3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Add for Vec3D {
    type Output = Self;

    fn add(self, other: Vec3D) -> Self {
        Vec3D{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl AddAssign for Vec3D {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}


impl Sub for Vec3D {
    type Output = Self;

    fn sub(self, other: Vec3D) -> Self {
        Vec3D{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Neg for Vec3D {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3D{
            x: -self.x,
            y: -self.y,
            z: -self.z 
        }
    }
}

impl Mul<f64> for Vec3D {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vec3D{
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl Mul<Vec3D> for f64 {
    type Output = Vec3D;

    fn mul(self, vec: Vec3D) -> Vec3D {
        Vec3D {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        }
    }
}

impl Div<f64> for Vec3D {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Vec3D {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

pub fn dot(u: Vec3D, v: Vec3D) -> f64{
    u.x * v.x + u.y * v.y + u.z * v.z
}

/*pub fn cross(u: Vec3D, v: Vec3D) -> Vec3D{
    Vec3D {
        x: u.y*v.z - u.z*v.y,
        y: u.z*v.x - u.x*v.z,
        z: u.x*v.y - u.y*v.x
    }
}*/