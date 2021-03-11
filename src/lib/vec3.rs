use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[allow(dead_code)]
pub static ZERO: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

#[allow(dead_code)]
pub static ONE: Vec3 = Vec3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

#[allow(dead_code)]
pub static X: Vec3 = Vec3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

#[allow(dead_code)]
pub static Y: Vec3 = Vec3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

#[allow(dead_code)]
pub static Z: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn from_f64(e: f64) -> Vec3 {
        Vec3::new(e, e, e)
    }

    pub fn len_sq(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn norm(&self) -> Vec3 {
        *self / self.len()
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        let x = (self.y * rhs.z) - (self.z * rhs.y);
        let y = (self.x * rhs.z) - (self.z * rhs.x);
        let z = (self.x * rhs.y) - (self.y * rhs.x);
        Vec3::new(x, y, z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        return rhs * self;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a.len_sq(), 14.0);
        assert_eq!(a.len(), 14.0f64.sqrt());
    }

    #[test]
    fn norm() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a.norm().len_sq(), 1.0);
    }

    #[test]
    fn neg() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(-a, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn scale() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let s = 2.0;
        let r = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(v * s, r);
        assert_eq!(s * v, r);
    }

    #[test]
    fn div() {
        let a = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(a / 2.0, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn add() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(10.0, 20.0, 30.0);
        assert_eq!(a + b, Vec3::new(11.0, 22.0, 33.0));
    }

    #[test]
    fn sub() {
        let a = Vec3::new(10.0, 20.0, 30.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a - b, Vec3::new(9.0, 18.0, 27.0));
    }

    #[test]
    fn mul() {
        let a = Vec3::new(10.0, 20.0, 30.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a * b, Vec3::new(10.0, 40.0, 90.0));
    }

    #[test]
    fn dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(10.0, 20.0, 30.0);
        assert_eq!(a.dot(&b), 140.0);
        assert_eq!(a.dot(&ZERO), 0.0);
    }

    #[test]
    fn cross() {
        assert_eq!(X.cross(&Y), Z);
        assert_eq!(ZERO.cross(&Y), ZERO);
    }
}
