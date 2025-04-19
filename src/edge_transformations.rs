use crate::edge::Edge;

impl Edge {

    /// Gets reversed version of the same [Edge].
    ///
    /// In other words: the output [Edge] will have start and end reversed.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::edge::Edge;
    ///
    /// let input = Edge::new(0, 1);
    /// let actual = input.get_reversed();
    ///
    /// assert!(actual.start.eq(&1));
    /// assert!(actual.end.eq(&0));
    /// ```
    pub fn get_reversed(&self) -> Edge {
        Edge::new(self.end, self.start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_reversed() {
        let input = Edge::new(0, 1);
        let actual = input.get_reversed();
        
        assert!(actual.start.eq(&1));
        assert!(actual.end.eq(&0));
    }
}