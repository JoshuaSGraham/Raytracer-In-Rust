use core::f64;
use std::{ops, usize};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3{

    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {

    fn x(&self) -> f64{
        self.x
    }

    fn y(&self) -> f64{
        self.y
    }

    fn z(&self) -> f64{
        self.z
    }

    fn dot(&self, other_vec: &Vec3) -> f64{
        (self.x * other_vec.x) + (self.y * other_vec.y) + (self.z * other_vec.z) 
    }

    fn cross(&self, other_vec: &Vec3) -> Vec3{
        Vec3{
            x: self.y * other_vec.z - self.z * other_vec.y,
            y: self.z * other_vec.x - other_vec.x * other_vec.z,
            z: self.x * other_vec.y - other_vec.y * other_vec.x,
        }
    }
}


impl ops::Add for Vec3{
    type Output = Self;
    fn add(self, rhs:Self) -> Self {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3{
    type Output = Self;
    fn sub(self, rhs:Self) -> Self{
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Neg for Vec3{
    type Output = Self;
    fn neg(self) -> Self{
        Self{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Div<f64> for Vec3{
    type Output = Vec3;
    fn div(self, rhs:f64) -> Vec3{
        Vec3{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Mul for Vec3{
    type Output = Self;
    fn mul(self, rhs:Self) -> Self{
        Self{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs:f64) -> Vec3{
        Vec3{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_add(){
        let vec1 = Vec3{x: 1.0, y: 1.0, z: 1.0};
        let vec2 = Vec3{x: 1.0, y: 1.0, z: 1.0};
        assert_eq!(vec1 + vec2, Vec3{x: 2.0, y: 2.0, z: 2.0});

        let vec4 = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let vec5 = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(vec4 + vec5, Vec3{x: 2.0, y: 4.0, z: 6.0});
    }

    #[test]
    fn test_sub(){
        let vec1 = Vec3{x: 1.0, y: 1.0, z: 1.0};
        let vec2 = Vec3{x: 1.0, y: 1.0, z: 1.0};
        assert_eq!(vec1 - vec2, Vec3{x: 0.0, y: 0.0, z: 0.0});

        let vec4 = Vec3{x: 2.0, y: 4.0, z: 6.0};
        let vec5 = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(vec4 - vec5, Vec3{x: 1.0, y: 2.0, z: 3.0});
    }

    #[test]
    fn test_neg(){
        let vec1 = Vec3{x: 1.0, y: 1.0, z: 1.0};
        assert_eq!(-vec1, Vec3{x: -1.0, y: -1.0, z: -1.0});
    }

    #[test]
    fn test_mul_vec(){
        let vec1 = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let vec2 = Vec3{x: 1.0, y: 5.0, z: 7.0};
        assert_eq!(vec1 * vec2, Vec3{x: 1.0,y: 10.0,z: 21.0});
    }

     #[test]
    fn test_mul_num(){
        let vec1 = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let num = 2.0;
        assert_eq!(vec1 * num, Vec3{x: 2.0, y: 4.0, z: 6.0})
    }

    #[test]
    fn test_div(){
        let vec1 = Vec3{x: 2.0, y: 4.0, z: 6.0};
        let num = 2.0;
        assert_eq!(vec1 / num, Vec3{x: 1.0, y: 2.0, z: 3.0});
    }
}