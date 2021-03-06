use crate::vec3::Vec3;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction: direction.norm(),
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn unit_dir(&self) -> &Vec3 {
        &self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::X;

    #[test]
    fn at() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), X);
        assert_eq!(r.at(6.0), Vec3::new(7.0, 2.0, 3.0));
    }
}
