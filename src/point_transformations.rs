use std::ops;
use crate::point::Point;
use crate::vector::Vector;

impl ops::Add<Vector> for Point {
    type Output = Point;

    /// Adds [Vector] to the [Point].
    ///
    /// The result is a new Point.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let a = Point::new(5.231, -0.341, 11.034);
    /// let b = Vector::new(-12.564, 5.642, 7.731);
    /// let result = a + b;
    /// let expected = Point::new(5.231+(-12.564), -0.341+5.642, 11.034+7.731);
    /// assert_eq!(result.eq(&expected), true);
    /// ```
    fn add(self, vector: Vector) -> Point {
        Point {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Point;

    /// Subtracts [Vector] from the [Point].
    ///
    /// The result is a new Point.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let a = Point::new(5.231, -0.341, 11.034);
    /// let b = Vector::new(-12.564, 5.642, 7.731);
    /// let result = a - b;
    /// let expected = Point::new(5.231-(-12.564), -0.341-5.642, 11.034-7.731);
    /// assert_eq!(result.eq(&expected), true);
    /// ```
    fn sub(self, vector: Vector) -> Point {
        Point {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add_vector() {
        let a = Point::new(5.231, -0.341, 11.034);
        let b = Vector::new(-12.564, 5.642, 7.731);
        let result = a + b;
        let expected = Point::new(5.231+(-12.564), -0.341+5.642, 11.034+7.731);
        assert_eq!(result.eq(&expected), true);
    }

    #[test]
    fn test_point_subtract_vector() {
        let a = Point::new(5.231, -0.341, 11.034);
        let b = Vector::new(-12.564, 5.642, 7.731);
        let result = a - b;
        let expected = Point::new(5.231-(-12.564), -0.341-5.642, 11.034-7.731);
        assert_eq!(result.eq(&expected), true);
    }
}