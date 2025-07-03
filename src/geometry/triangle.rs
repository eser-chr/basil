use super::surface::Surface;
use super::vec3d::Vec3d as Point;
use num_traits::{Float, FromPrimitive};

pub struct Triangle<'a, T> {
    p1: &'a Point<T>,
    p2: &'a Point<T>,
    p3: &'a Point<T>,
}

impl<'a, T> Triangle<'a, T>
where
    T: Float + FromPrimitive + Copy,
{
    /// Collinearity is acceptable.
    pub fn new(p1: &'a Point<T>, p2: &'a Point<T>, p3: &'a Point<T>) -> Self {
        Self { p1, p2, p3 }
    }

    pub fn area(&self) -> T {
        let ab = *self.p2 - *self.p1;
        let ac = *self.p3 - *self.p1;
        let cross = ab.cross(&ac);
        cross.length() * T::from_f64(0.5).unwrap()
    }

    pub fn barycentre(&self) -> Point<T> {
        let three = T::from_f64(3.0).unwrap();
        ((*self.p1) + (*self.p2) + (*self.p3)).div(three).unwrap()
    }

    pub fn surface(&self) -> Result<Surface<T>, &str> {
        if self.area() == T::zero() {
            Err("Collinearity or single point")
        } else {
            let ab = *self.p2 - *self.p1;
            let ac = *self.p3 - *self.p1;
            let n = ab.cross(&ac);
            Ok(Surface { n })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_area() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 0.0, 0.0);
        let p3 = Point::new(0.0, 1.0, 0.0);

        assert_eq!(Triangle::new(&p1, &p2, &p3).area(), 0.5);
        assert_eq!(Triangle::new(&p1, &p1, &p3).area(), 0.0); // Collinear
    }

    #[test]
    fn t_barycenter() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 0.0, 0.0);
        let p3 = Point::new(0.0, 1.0, 0.0);

        assert_eq!(
            Triangle::new(&p1, &p2, &p3).barycentre(),
            Point::new(1.0 / 3.0, 1.0 / 3.0, 0.0)
        );
    }
    #[test]
    fn surface() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 0.0, 0.0);
        let p3 = Point::new(0.0, 1.0, 0.0);

        assert_eq!(
            Triangle::new(&p1, &p2, &p3).surface().unwrap().n,
            Point::new(0.0,0.0,1.0)
        );
    }
}
