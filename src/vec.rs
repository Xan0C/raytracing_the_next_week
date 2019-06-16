use std::ops::{Add, Div, Mul, Neg, Sub, AddAssign, DivAssign, MulAssign};
use std::cmp;
use std::fmt;

#[derive(Clone,Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 {x,y,z}
    }

    pub fn zero() -> Self {
        Vec3 {x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn values(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }

    pub fn len_squared(&self) -> f32 {
        return self.x*self.x + self.y * self.y + self.z * self.z;
    }

    pub fn len(&self) -> f32 {
        return self.len_squared().sqrt();
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(self.y * other.z - self.z * other.y,
             -(self.x * other.z - self.z * other.x),
             self.x * other.y - self.y * other.x)
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.len();

        return Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len
        };
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f32) -> Vec3 {
        return Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other
        };
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        };
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f32) -> Vec3 {
        return Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other
        };
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        };
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        return Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        };
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.x = self.x / other.x;
        self.y = self.y / other.y;
        self.z = self.z / other.z;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.x = self.x / other;
        self.y = self.y / other;
        self.z = self.z / other;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        };
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        return Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        };
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
        self.z = self.z * other.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.x = self.x * other;
        self.y = self.y * other;
        self.z = self.z * other;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    
    fn neg(self) -> Vec3 {
        return Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        };
    }
}

impl cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        return self.x == other.x 
            && self.y == other.y
            && self.z == other.z;
    }
}

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({} {} {})", self.x, self.y, self.z);
    }
}