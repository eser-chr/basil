use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use num_traits::{Float};
// use math::sqrt;

#[derive(Debug, Clone, Copy)]
struct Point<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Point<T>
where
    T: Float + Copy,
{
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    fn mul(&self, factor: T) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    fn length(&self) -> T
    where
        T: Float,
    {
        self.dot(self).sqrt()
    }

    fn div(&self, denominator: T) -> Result<Self, &str> {
        if denominator == T::zero() {
            Err("Cannot divide with 0. Singularity issues")
        } else {
            Ok((*self).mul(T::one() / denominator))
        }
    }

fn normalize(&self) -> Result<Self, &'static str> {
    self.div(self.length()).map_err(|_| "Zero Norm Point, encountered!")
}

}

impl<T> Add for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> AddAssign for Point<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> SubAssign for Point<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Point<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

impl<T> PartialEq for Point<T>
where
    T: PartialEq + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// TESTS

#[cfg(test)]
mod tests {
    use num_traits::Pow;

    use super::*;

    #[test]
    fn display() {
        let p: Point<f64> = Point::new(1.0, 2.0, 3.0);
        assert_eq!(format!("{}", p), "(1.00, 2.00, 3.00)")
    }

    #[test]
    fn add() {
        let p1: Point<f64> = Point::new(1.0, 2.0, 3.0);
        let p2: Point<f64> = Point::new(1.0, 2.0, 3.0);
        let p3 = p1 + p2;
        assert_eq!(p3.x, 2.0);
        assert_eq!(p3.y, 4.0);
        assert_eq!(p3.z, 6.0);
    }

    #[test]
    fn sub() {
        let p1: Point<f64> = Point::new(1.0, 2.0, 3.0);
        let p2: Point<f64> = Point::new(1.0, 2.0, 3.0);
        let p3 = p1 - p2;
        assert_eq!(p3.x, 0.0);
        assert_eq!(p3.y, 0.0);
        assert_eq!(p3.z, 0.0);
    }

    #[test]
    fn add_ass() {
        let mut p1: Point<f64> = Point::new(1.0, 2.0, 3.0);
        let p2: Point<f64> = Point::new(1.0, 2.0, 3.0);
        p1 += p2;
        assert_eq!(p1.x, 2.0);
        assert_eq!(p1.y, 4.0);
        assert_eq!(p1.z, 6.0);
    }
    #[test]
    fn sub_ass() {
        let mut p1: Point<f64> = Point::new(1.0, 2.0, 3.0);
        let p2: Point<f64> = Point::new(1.0, 2.0, 3.0);
        p1 -= p2;
        assert_eq!(p1.x, 0.0);
        assert_eq!(p1.y, 0.0);
        assert_eq!(p1.z, 0.0);
    }

    #[test]
    fn test_eqaulity() {
        let p1: Point<f64> = Point::new(1.0, 2.0, 3.0);
        let p2: Point<f64> = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p1, p2);
        let p3 = p1 - p2;
        assert_ne!(p2, p3);
    }
    #[test]
    fn test_length() {
        let p1: Point<f64> = Point::new(0.0, 3.0, 4.0);
        assert_eq!(p1.length(), 5.0)
    }
    #[test]
    fn test_dot() {
        let p1: Point<f64> = Point::new(1.0, 2.0, 3.0);
        let p2: Point<f64> = Point::new(1.0, 2.0, 3.0);

        let a = p1.dot(&p2) / (p1.length().pow(2));

        assert_eq!(a, 1.0)
    }
    #[test]
    fn cross() {
        let p1: Point<f64> = Point::new(1.0, 2.0, 3.0);
        let p2: Point<f64> = Point::new(1.0, 2.0, 3.0);
        let zero = Point::new(0.0, 0.0, 0.0);
        assert_eq!(p1.cross(&p2), zero)
    }
    
    #[test]
    fn test_mult() {
        let p1: Point<f64> = Point::new(1.0, 2.0, 3.0);
        let p2: Point<f64> = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p1.mul(1.0), p2);
        assert_eq!(p1.mul(2.0), Point::new(2.0, 4.0, 6.0));
    }
    #[test]
    fn test_div() {
        let p1: Point<f64> = Point::new(1.0, 2.0, 3.0);
        // let p2: Point<f64> = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p1.div(1.0).unwrap(), p1);
        assert!(matches!(
            p1.div(0.0),
            Err("Cannot divide with 0. Singularity issues")
        ));
    }
    #[test]
    fn test_normalize() {
        let p1: Point<f64> = Point::new(3.0, 3.0, 3.0);
        let zero = Point::new(0.0, 0.0, 0.0);
        let p1_len = p1.length();

        assert_ne!(p1.length(), 0.0);
        assert_eq!(p1.normalize().unwrap(), Point::new(3.0/p1_len, 3.0/p1_len, 3.0/p1_len));

        assert_eq!(zero.length(), 0.0);
        assert!(matches!(
            zero.normalize(),
            Err("Zero Norm Point, encountered!")
        ));
    }
}
