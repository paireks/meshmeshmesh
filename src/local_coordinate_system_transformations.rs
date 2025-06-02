use crate::local_coordinate_system::LocalCoordinateSystem;

impl LocalCoordinateSystem {
    
    /// Creates the new [LocalCoordinateSystem] but flipped, meaning the z-axis and x-axis are
    /// reversed, but the y_axis stays the same.
    /// 
    /// It can be useful if you'd like to e.g. reverse the z-axis.
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
    /// let input = LocalCoordinateSystem::new(origin, x, y);
    /// let actual = input.get_flipped_around_y_axis();
    ///
    /// assert_eq!(actual.origin, Point::new(0.0, 5.0, -1.2));
    /// assert_eq!(actual.x, Vector::new(0.0, 0.0, -1.0));
    /// assert_eq!(actual.y, Vector::new(0.0, -1.0, 0.0));
    /// ```
    pub fn get_flipped_around_y_axis(&self) -> LocalCoordinateSystem {
        LocalCoordinateSystem::new(self.origin, self.x.get_reversed(), self.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::vector::Vector;
    use super::*;

    #[test]
    fn test_get_flipped_around_y_axis() {
        let origin = Point::new(0.0, 5.0, -1.2);
        let x = Vector::new(0.0, 0.0, 1.0);
        let y = Vector::new(0.0, -1.0, 0.0);
        
        let input = LocalCoordinateSystem::new(origin, x, y);
        let actual = input.get_flipped_around_y_axis();
        
        assert_eq!(actual.origin, Point::new(0.0, 5.0, -1.2));
        assert_eq!(actual.x, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(actual.y, Vector::new(0.0, -1.0, 0.0));
    }
}