use super::vec3d::Vec3d;
use num_traits::{Float, FromPrimitive};

pub struct Line<'a, T> {
    p1: &'a Vec3d<T>,
    p2: &'a Vec3d<T>,
}

impl<'a, T> Line<'a, T>
where
    T: Float + FromPrimitive + Copy,
{
    pub fn new(p1: &'a Vec3d<T>, p2: &'a Vec3d<T>) -> Self {
        Self { p1, p2 }
    }

    pub fn length(&self) -> T {
        let tmp = (*self.p1) - (*self.p2);
        tmp.length()
    }

    pub fn middle_point(&self) -> Vec3d<T> {
        // Probably hot point. Consider different approach.
        let half = T::from_f64(0.5).unwrap();
        ((*self.p1) + (*self.p2)).mul(half)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let p1 = Vec3d::new(0.0, 0.0, 1.0);
        let p2 = Vec3d::new(0.0, 0.0, 2.0);
        let l = Line::new(&p1, &p2);
        assert_eq!(l.length(), 1.0)
    }

    #[test]
    fn test_middle_point() {
        let p1 = Vec3d::new(0.0, 0.0, 1.0);
        let p2 = Vec3d::new(0.0, 0.0, 2.0);
        let l = Line::new(&p1, &p2);

        let middle_point = Vec3d::new(0.0,0.0,1.5);

        assert_eq!(l.middle_point(), middle_point)
    }
}
