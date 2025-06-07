use crate::bounding_box::BoundingBox;

impl BoundingBox {
    /// Compares given [BoundingBox] to other one, but with a `f64` tolerance.
    ///
    /// If any field absolute difference is > tolerance, then it should return `false`.
    ///
    /// # Examples
    ///
    /// In this example we can see the differences, so we expect `true`.
    ///
    /// ```
    ///
    /// use meshmeshmesh::bounding_box::BoundingBox;
    /// 
    /// let tolerance: f64 = 0.001;
    /// let a = BoundingBox::new(1.5, 1.65, -2.3, 0.7 + 0.0009, 3.9, 4.1);
    /// let b = BoundingBox::new(1.5, 1.65, -2.3, 0.7, 3.9, 4.1);
    ///
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    /// ```
    ///
    /// In this example we can see the `max_y` absolute difference is > tolerance, so we expect 'false'.
    ///
    /// ```
    ///
    /// use meshmeshmesh::bounding_box::BoundingBox;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = BoundingBox::new(1.5, 1.65, -2.3, 0.7 + 0.0011, 3.9, 4.1);
    /// let b = BoundingBox::new(1.5, 1.65, -2.3, 0.7, 3.9, 4.1);
    ///
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    /// ```
    pub fn eq_with_tolerance(&self, other:&BoundingBox, tolerance: f64) -> bool {
        if (self.min_x - other.min_x).abs() > tolerance {
            false
        }
        else if (self.max_x - other.max_x).abs() > tolerance {
            false
        }
        else if (self.min_y - other.min_y).abs() > tolerance {
            false
        }
        else if (self.max_y - other.max_y).abs() > tolerance {
            false
        }
        else if (self.min_z - other.min_z).abs() > tolerance {
            false
        }
        else if (self.max_z - other.max_z).abs() > tolerance {
            false
        }
        else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_with_tolerance_true() {
        let tolerance: f64 = 0.001;
        let a = BoundingBox::new(1.5, 1.65, -2.3, 0.7 + 0.0009, 3.9, 4.1);
        let b = BoundingBox::new(1.5, 1.65, -2.3, 0.7, 3.9, 4.1);
        
        assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    }

    #[test]
    fn test_eq_with_tolerance_false() {
        let tolerance: f64 = 0.001;
        let a = BoundingBox::new(1.5, 1.65, -2.3, 0.7 + 0.0011, 3.9, 4.1);
        let b = BoundingBox::new(1.5, 1.65, -2.3, 0.7, 3.9, 4.1);

        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }
}