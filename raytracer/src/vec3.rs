use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: Vec3, v: Vec3) -> Self {
        Self {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn unit_vector(v: Self) -> Self {
        v / v.length()
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<f64>();
        let y = rng.gen::<f64>();
        let z = rng.gen::<f64>();
        Self { x, y, z, }
    }
    pub fn random_(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(min..max);
        let y = rng.gen_range(min..max);
        let z = rng.gen_range(min..max);
        Self { x, y, z, }
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_(-1.0, 1.0);
            if p.squared_length() <= 1.0 {
                return p;
            }
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
    }
    #[test]
    fn test_add() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(3.0, 4.0, 5.0)
        )
    }
    #[test]
    fn test_add_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(3.0, 4.0, 5.0))
    }
    #[test]
    fn test_add_f64() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + 233.0,
            Vec3::new(234.0, 233.0, 232.0)
        )
    }
    // #[test]
    // fn test_add_assign_f64() {
    //     let mut x = Vec3::new(1.0, 0.0, -1.0);
    //     x += 233.0;
    //     assert_eq!(x, Vec3::new(234.0, 233.0, 232.0))
    // }
    // #[test]
    // fn test_sub() {
    //     assert_eq!(
    //         Vec3::new(1.0, 0.0, -1.0) - Vec3::new(2.0, 4.0, 6.0),
    //         Vec3::new(-1.0, -4.0, -7.0)
    //     )
    // }
    // #[test]
    // fn test_sub_assign() {
    //     let mut x = Vec3::new(1.0, 0.0, -1.0);
    //     x -= Vec3::new(2.0, 4.0, 6.0);
    //     assert_eq!(x, Vec3::new(-1.0, -4.0, -7.0))
    // }
    // #[test]
    // fn test_sub_f64() {
    //     assert_eq!(Vec3::new(1.0, 0.0, -1.0) - 1.0, Vec3::new(0.0, -1.0, -2.0))
    // }
    // #[test]
    // fn test_sub_assign_f64() {
    //     let mut x = Vec3::new(1.0, 0.0, -1.0);
    //     x -= 1.0;
    //     assert_eq!(x, Vec3::new(0.0, -1.0, -2.0))
    // }
    // #[test]
    // fn test_mul() {
    //     assert_eq!(Vec3::new(1.0, 0.0, -1.0) * Vec3::ones(), 0.0);
    // }
    // #[test]
    // fn test_mul_assign() {
    //     let mut x = Vec3::new(1.0, 0.0, -1.0);
    //     x *= 2.0;
    //     assert_eq!(x, Vec3::new(2.0, 0.0, -2.0));
    // }
    // #[test]
    // fn test_mul_f64() {
    //     assert_eq!(Vec3::new(1.0, 0.0, -1.0) * 1.0, Vec3::new(1.0, 0.0, -1.0));
    // }
    // #[test]
    // fn test_div() {
    //     assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
    // }
    // #[test]
    // fn test_elemul() {
    //     assert_eq!(
    //         Vec3::elemul(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0)),
    //         Vec3::new(1.0, 4.0, 9.0)
    //     );
    // }
    // #[test]
    // fn test_cross() {
    //     assert_eq!(
    //         Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0)),
    //         Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
    //     );
    // }
    // #[test]
    // fn test_neg() {
    //     assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
    // }
    #[test]
    fn test_squared_length() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).squared_length(), 14.0);
    }
    // #[test]
    // fn test_length() {
    //     assert_eq!(
    //         Vec3::new(3.0, 4.0, 5.0).length(),
    //         ((3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0) as f64).sqrt()
    //     );
    // }
    // #[test]
    // fn test_unit() {
    //     assert_eq!(Vec3::new(233.0, 0.0, 0.0).unit(), Vec3::new(1.0, 0.0, 0.0));
    //     assert_eq!(
    //         Vec3::new(-233.0, 0.0, 0.0).unit(),
    //         Vec3::new(-1.0, 0.0, 0.0)
    //     );
    // }
    // #[test]
    // #[should_panic]
    // fn test_unit_panic() {
    //     Vec3::new(0.0, 0.0, 0.0).unit();
    // }
}
