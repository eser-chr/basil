use super::vec3d::Vec3d;
use num_traits::{Float, FromPrimitive};


pub struct Line<T>{
    pub v:Vec3d<T>
}

impl <T> Line<T>
where T:Float + FromPrimitive +Copy{
// pub fn from_ref()
pub fn from_ref(v:&Vec3d<T>)->Self{
    Self { v: v.clone() }
}


/// Consumes the original vector
pub fn from(v: Vec3d<T>)->Self{
    Self{v}
}

pub fn length(&self)->T{
    self.v.length()
}

pub fn middle_point(&self)->Vec3d<T>{
    let half = T::from_f64(0.5).unwrap();
    self.v.mul(half)
}


}

pub struct LineRef<'a, T> {
    p1: &'a Vec3d<T>,
    p2: &'a Vec3d<T>,
}

impl<'a, T> LineRef<'a, T>
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

    pub fn to_line(&self)->Line<T>{
        Line { v: (*self.p1) - (*self.p2) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let p1 = Vec3d::new(0.0, 0.0, 1.0);
        let p2 = Vec3d::new(0.0, 0.0, 2.0);
        let l = LineRef::new(&p1, &p2);

        assert_eq!(l.length(), 1.0);

        let own_line = l.to_line();

        assert_eq!(own_line.length(), 1.0)
    }

    #[test]
    fn test_middle_point() {
        let p1 = Vec3d::new(0.0, 0.0, 1.0);
        let p2 = Vec3d::new(0.0, 0.0, 2.0);
        let l = LineRef::new(&p1, &p2);

        let middle_point = Vec3d::new(0.0,0.0,1.5);

        assert_eq!(l.middle_point(), middle_point);
        let own_line = l.to_line();
        assert_eq!(own_line.middle_point()+p2, middle_point);
    }

}
