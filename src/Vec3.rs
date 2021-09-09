use core::f64;
use std::{ops, usize};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec3{

   pub x: f64,
   pub y: f64,
   pub z: f64,
}

impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Vec3{
        Vec3 { x, y, z}
    }

    pub fn x(&self) -> f64{
        self.x
    }

    pub fn y(&self) -> f64{
        self.y
    }

    pub fn z(&self) -> f64{
        self.z
    }

    pub fn dot(vec1: &Vec3, vec2: &Vec3) -> f64{
        vec1.x() * vec2.x() + vec1.y() * vec2.y() + vec1.z() * vec2.z()
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3{
        Vec3{
            x: v1.y() * v2.z() - v1.z() * v2.y(),
            y: v1.z() * v2.x() - v1.x() * v2.z(),
            z: v1.x() * v2.y() - v1.y() * v2.x(),
        }
    }

    pub fn length(self) -> f64 {
       (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn squared_length(self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn unit_vector(v: &Vec3) -> Vec3{
        *v / v.length()
    } 
}

//type aliasing
pub type Point3 = Vec3;
pub type Color = Vec3;

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

impl ops::AddAssign for Vec3{
    fn add_assign(&mut self, rhs:Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::MulAssign<f64> for Vec3{
    fn mul_assign(&mut self, rhs: f64){
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vec3{
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0/rhs;
    }
}

//may need <'a> (lifetime) at fn index<'a>
impl ops::Index<usize> for Vec3{
    type Output = f64;
    fn index(&self, index: usize) -> &f64{
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _=> unreachable!(),
        }
    }
}

impl ops::IndexMut<usize> for Vec3{
    //type Output = &mut f64;
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _=> unreachable!(),
        }
    }
}


//Tests

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

    #[test]
    fn test_add_assign(){
        let mut vec1 = Vec3{x: 5.0, y: 5.0, z: 5.0};
        let vec2 = Vec3{x: 5.0, y: 5.0, z: 5.0};
        vec1 += vec2;
        assert_eq!(vec1, Vec3{x: 10.0, y: 10.0, z: 10.0});
    }

    //needs testing when extention is working
    #[test]
    fn test_mul_assign(){
        let mut vec1 = Vec3{x: 5.0, y: 5.0, z: 5.0};
        let num = 2.0;
        vec1 *= num;
        assert_eq!(vec1, Vec3{x: 10.0, y: 10.0, z: 10.0});
    }

    #[test]
    fn test_div_assign(){
        let mut vec1 = Vec3{x: 10.0, y: 10.0, z: 10.0};
        let num = 2.0;
        vec1 /= num;
        assert_eq!(vec1, Vec3{x: 5.0, y: 5.0, z: 5.0});
    }

    #[test]
    fn test_index(){
        let vec1 = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(vec1[0], 1.0);
        assert_eq!(vec1[1], 2.0);
        assert_eq!(vec1[2], 3.0);
    }

    //look at how to properly test this method
    #[test]
    fn test_index_mut(){
        let vec1 = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(vec1[0], 1.0);
        assert_eq!(vec1[1], 2.0);
        assert_eq!(vec1[2], 3.0);
    }

    #[test]
    fn test_length(){
        let vec1 = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(vec1.length(), 6.0)
    }

    #[test]
    fn test_unit_vector(){
        let vec1 = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(vec1.unit_vector(), Vec3{x: 1.0, y: 1.0, z: 1.0});
    }

    #[test]
    fn test_dot(){
        let vec1 = Vec3{x: 1.0,y: 2.0,z: 3.0};
        let vec2 = Vec3{x: 3.0, y: 2.0, z: 1.0};
        assert_eq!(vec1.dot(&vec2), 10.0)
    }

    #[test]
    fn test_cross(){
        let vec1 = Vec3{x: 1.0,y: 2.0,z: 3.0};
        let vec2 = Vec3{x: 3.0,y: 2.0,z: 1.0};
        assert_eq!(vec1.cross(&vec2), Vec3{x: -4.0,y: 8.0,z: -4.0})
    }
}