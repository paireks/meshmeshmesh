use crate::bounding_area::BoundingArea;

impl BoundingArea {
    /// Gets area of the [BoundingArea].
    /// 
    /// Can be useful to get the size of this object.
    /// 
    /// # Example
    /// 
    /// ```
    /// use meshmeshmesh::bounding_area::BoundingArea;
    /// 
    /// let input = BoundingArea::new(-10.590682, 17.025842, 32.495452, 59.892292);
    /// 
    /// let actual = input.get_area();
    /// 
    /// assert_eq!(actual, 756.6054893841599);
    /// 
    /// ```
    pub fn get_area(&self) -> f64 {
        (self.max_x - self.min_x) * (self.max_y - self.min_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_area() {
        let input = BoundingArea::new(-10.590682, 17.025842, 32.495452, 59.892292);
        
        let actual = input.get_area();
        
        assert_eq!(actual, 756.6054893841599);
    }
}