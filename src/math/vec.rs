use std::ops::{Add, Sub, Mul, Index, IndexMut};

/// A three element vector
#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Vec3 {
    /// Store xyz as an array of 3 numbers
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f32 { self.e[0] }    
    pub fn y(&self) -> f32 { self.e[1] } 
    pub fn z(&self) -> f32 { self.e[2] }
    pub fn r(&self) -> f32 { self.e[0] }    
    pub fn g(&self) -> f32 { self.e[1] } 
    pub fn b(&self) -> f32 { self.e[2] }    

    pub fn length(&self) -> f32 { self.squared_length().sqrt() }
    pub fn squared_length(&self) -> f32 { self.e.iter().map(|v| v * v).sum() }
    pub fn normalize(&self) -> Vec3 {
        let v = self.clone();
        let k = 1.0 / v.length();
        v * k
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        let mut e: [f32; 3] = [0.0; 3];
        for i in 0..3 {
            e[i] = self.e[i] + other.e[i];
        }
        Vec3 { e: e }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        let mut e: [f32; 3] = [0.0; 3];
        for i in 0..3 {
            e[i] = self.e[i] - other.e[i];
        }
        Vec3 { e: e }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        let mut e: [f32; 3] = [0.0; 3];
        for i in 0..3 {
            e[i] = self.e[i] * other;
        }
        Vec3 { e: e }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x()
        )
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        &self.e[i]
    }
} 

impl IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f32 {
        &mut self.e[index]
    }
}

#[cfg(test)]
mod tests {
use super::Vec3;

    #[test]
    fn vec3_new() {
        let v = Vec3::new(1f32, 2f32, 3f32);
        assert_eq!(v, Vec3{ e: [1.0, 2.0, 3.0 ]});
    }

    #[test]
    fn vec3_index() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v[0], v.x());
        assert_eq!(v[0], v.r());
        assert_eq!(v[1], v.y());
        assert_eq!(v[1], v.g());
        assert_eq!(v[2], v.z());
        assert_eq!(v[2], v.b());
    }

    #[test]
    fn vec3_index_mut() {
        let mut v = Vec3::new(0.0, 0.0, 0.0);
        v[0] = 1.0;
        v[1] = 2.0;
        v[2] = 3.0;

        for i in 0..3 {
            assert_eq!(v[i], (i + 1) as f32);
        }
    }

    #[test]
    fn vec3_add() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let v = a + b;
        for i in 0..3 {
            assert_eq!(v[i], (2 * (i + 2) + 1) as f32);
        }
    }

    #[test]
    fn vec3_sub() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let v = b - a;
        for i in 0..3 {
            assert_eq!(v[i], 3.0);
        }
    }

    #[test]
    fn vec3_mul() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let mut v = a * 2.0;
        for i in 0..3 {
            assert_eq!(v[i], (i + 1) as f32 * 2.0);
        }

        let b = Vec3::new(2.0, 4.0, 6.0);
        v = b * 0.5;
        for i in 0..3 {
            assert_eq!(v[i], (i + 1) as f32);
        }
    }
    
    #[test]
    fn vec3_len() {
        let v = Vec3::new(3.0, 4.0, 12.0);
        assert_eq!(v.squared_length(), 13_f32.powf(2.0));
        assert_eq!(v.length(), 13_f32);        
    }

    #[test]
    fn vec3_normalize() {
        let v = Vec3::new(3.0, 4.0, 5.0).normalize();
        assert_eq!(v.length(), 1.0);        
        assert_eq!(v.squared_length(), 1.0);
    }

    #[test]
    fn vec3_cross() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        // Check perpendicular
        assert_eq!(a * b, Vec3::new(0.0, 0.0, 1.0));
        
        // check random
        let i = Vec3::new(1.0, 2.0, 3.0);
        let j = Vec3::new(3.0, 2.0, 1.0);
        assert_eq!(i * j, Vec3::new(-4.0, 8.0, -4.0));        
    }
}