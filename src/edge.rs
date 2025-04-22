use std::cmp::Ordering;

/// Represents an edge.
#[derive(Debug, Clone, Copy, Eq, Hash)]
pub struct Edge {
    /// Index of vertex that represents start of the edge.
    pub start: usize,
    /// Index of vertex that represents end of the edge.
    pub end: usize,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.start.eq(&other.start) && self.end.eq(&other.end)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start).then(self.end.cmp(&other.end))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Edge {
    /// Creates a new [Edge].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::edge::Edge;
    ///
    /// let result = Edge::new(0, 1);
    ///
    /// assert!(result.start.eq(&0));
    /// assert!(result.end.eq(&(1)));
    /// ```
    pub fn new(start: usize, end: usize) -> Edge { Edge { start, end } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = Edge::new(0, 1);

        assert!(result.start.eq(&0));
        assert!(result.end.eq(&1));
    }

    #[test]
    fn test_partialeq_true() {
        let a = Edge::new(0, 1);
        let b  = Edge::new(0, 1);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = Edge::new(0, 1);
        let b  = Edge::new(2, 1);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_second_different_false() {
        let a = Edge::new(0, 1);
        let b  = Edge::new(0, 2);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = Edge::new(0, 1);
        let b  = Edge::new(2, 3);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }
}