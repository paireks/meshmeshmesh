use crate::edge::Edge;

impl Edge {
    /// Checks if both [Edge]s are the same, even if their direction is opposite.
    /// 
    /// # Examples
    /// 
    /// This example below shows the case where 2 edges are exactly the same, so it returns `true`.
    /// 
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// let a = Edge::new(0, 1);
    /// let b = Edge::new(0, 1);
    /// 
    /// let actual = a.eq_regardless_of_direction(&b);
    /// 
    /// assert_eq!(actual, true);
    /// 
    /// ```
    /// 
    /// This example below shows the case where 2 edges are the same, but their direction is reversed,
    /// so it still returns `true.
    /// 
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// let a = Edge::new(0, 1);
    /// let b = Edge::new(1, 0);
    ///
    /// let actual = a.eq_regardless_of_direction(&b);
    ///
    /// assert_eq!(actual, true);
    /// ```
    /// 
    /// This example below shows the case where 2 edges are not the same, so it returns `false`.
    /// 
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// let a = Edge::new(0, 1);
    /// let b = Edge::new(1, 2);
    ///
    /// let actual = a.eq_regardless_of_direction(&b);
    ///
    /// assert_eq!(actual, false);
    /// ```
    pub fn eq_regardless_of_direction(&self, other:&Edge) -> bool {
        if self.start.eq(&other.start) && self.end.eq(&other.end) { 
            true
        }
        else if self.start.eq(&other.end) && self.end.eq(&other.start) {
            true
        }
        else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_regardless_of_direction_true_same() {
        let a = Edge::new(0, 1);
        let b = Edge::new(0, 1);
        
        let actual = a.eq_regardless_of_direction(&b);
        
        assert_eq!(actual, true);
    }

    #[test]
    fn test_eq_regardless_of_direction_true_reversed() {
        let a = Edge::new(0, 1);
        let b = Edge::new(1, 0);

        let actual = a.eq_regardless_of_direction(&b);

        assert_eq!(actual, true);
    }

    #[test]
    fn test_eq_regardless_of_direction_false() {
        let a = Edge::new(0, 1);
        let b = Edge::new(1, 2);

        let actual = a.eq_regardless_of_direction(&b);

        assert_eq!(actual, false);
    }
}