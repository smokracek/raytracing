use crate::vec3::Vec3;

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(self, t: f64) -> Point3 {
        self.orig + (t * self.dir)
    }
}
