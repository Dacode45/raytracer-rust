use super::vec::Vec3;

#[derive(Debug,PartialEq)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray {a: a, b: b}
    }

    pub fn origin(&self) -> &Vec3 { &self.a }
    pub fn direction(&self) -> &Vec3 { &self.b }
    pub fn point_at_param(&self, t: f32) -> Vec3 { self.a + (self.b * t) }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use math::vec::Vec3;

    #[test]
    fn ray_new() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(1.0, 0.0, 0.0);
        let r = Ray::new(a, b);
        assert_eq!(r, Ray{a: a, b: b})
    }

    #[test]
    fn ray_origin_direction() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(1.0, 0.0, 0.0);
        let r = Ray::new(a, b);
        assert_eq!(r.origin(), &a);
        assert_eq!(r.direction(), &b);
    }

    #[test]
    fn ray_point_at_param() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(1.0, 0.0, 0.0);
        let r = Ray::new(a, b);
        assert_eq!(r.point_at_param(0.5), a + (b * 0.5));
    }
}