use crate::point::Point;
use crate::vector::Vector;

/// Represents a Ray object in three-dimensional space.
pub struct Ray {
    /// The origin [Point] from which we shoot the [Vector].
    pub origin: Point,

    /// The direction [Vector] of the [Ray].
    pub direction: Vector,
}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {

        if self.origin != other.origin {
            return false;
        }

        if self.direction != other.direction {
            return false;
        }

        true
    }
}


impl Ray {
    /// Creates a new [Ray].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let result = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
    ///
    /// assert_eq!(result.origin.eq(&Point::new(0.0, 1.0, -2.5)), true);
    /// assert_eq!(result.direction.eq(&Vector::new(1.0, 0.0, 0.0)), true);
    /// ```
    pub fn new(origin: Point, direction: Vector) -> Ray {Ray {origin, direction}}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let result = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));

        assert_eq!(result.origin.eq(&Point::new(0.0, 1.0, -2.5)), true);
        assert_eq!(result.direction.eq(&Vector::new(1.0, 0.0, 0.0)), true);
    }

    #[test]
    fn test_partialeq_true() {
        let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
        let b = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_different_origin_false() {
        let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
        let b = Ray::new(Point::new(0.0, 1.1, -2.5), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_different_direction_false() {
        let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
        let b = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 5.0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_different_all_false() {
        let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
        let b = Ray::new(Point::new(0.0, 1.1, -2.5), Vector::new(1.0, 0.0, 5.0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }
}