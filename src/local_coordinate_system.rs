use crate::point::Point;
use crate::vector::Vector;

/// Represents a local coordinate system.
/// 
/// This coordinate system is a cartesian coordinate system with Z axis defined by the right hand
/// thumb rule.
///
/// # Example
///
/// ```
/// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
/// use meshmeshmesh::point::Point;
/// use meshmeshmesh::vector::Vector;
///
/// let origin = Point::new(0.0, 5.0, -1.2);
/// let x = Vector::new(0.0, 0.0, 1.0);
/// let y = Vector::new(0.0, -1.0, 0.0);
///
/// let result = LocalCoordinateSystem::new(origin, x, y);
///
/// assert_eq!(result.origin, Point::new(0.0, 5.0, -1.2));
/// assert_eq!(result.x, Vector::new(0.0, 0.0, 1.0));
/// assert_eq!(result.y, Vector::new(0.0, -1.0, 0.0));
///
/// ```
#[derive(Debug, Clone, Copy)]
pub struct LocalCoordinateSystem {
    /// The origin of the coordinate system.
    pub origin: Point,
    /// The x-axis. Should be unitized.
    pub x: Vector,
    /// The y-axis. Should be unitized.
    pub y: Vector,
}

impl PartialEq for LocalCoordinateSystem {
    fn eq(&self, other: &Self) -> bool {
        if self.origin != other.origin { 
            return false;
        }

        if self.x != other.x {
            return false;
        }

        if self.y != other.y {
            return false;
        }
        
        true
    }
}

impl LocalCoordinateSystem {
    /// Returns a new [LocalCoordinateSystem].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let origin = Point::new(0.0, 5.0, -1.2);
    /// let x = Vector::new(0.0, 0.0, 1.0);
    /// let y = Vector::new(0.0, -1.0, 0.0);
    ///
    /// let result = LocalCoordinateSystem::new(origin, x, y);
    /// 
    /// assert_eq!(result.origin, Point::new(0.0, 5.0, -1.2));
    /// assert_eq!(result.x, Vector::new(0.0, 0.0, 1.0));
    /// assert_eq!(result.y, Vector::new(0.0, -1.0, 0.0));
    /// 
    /// ```
    pub fn new(origin: Point, x: Vector, y: Vector) -> LocalCoordinateSystem {
        let x_unitized = x.get_unitized();
        let y_unitized = y.get_unitized();

        LocalCoordinateSystem {origin, x: x_unitized, y: y_unitized}
    }
    
    /// Creates a [LocalCoordinateSystem] same as GlobalCoordinateSystem.
    pub fn global() -> LocalCoordinateSystem {
        LocalCoordinateSystem::new(Point::new(0.0,0.0,0.0), Vector::x_unit(), Vector::y_unit())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let origin = Point::new(0.0, 5.0, -1.2);
        let x = Vector::new(0.0, 0.0, 1.0);
        let y = Vector::new(0.0, -1.0, 0.0);
        
        let result = LocalCoordinateSystem::new(origin, x, y);
        
        assert_eq!(result.origin, Point::new(0.0, 5.0, -1.2));
        assert_eq!(result.x, Vector::new(0.0, 0.0, 1.0));
        assert_eq!(result.y, Vector::new(0.0, -1.0, 0.0));
    }
}

