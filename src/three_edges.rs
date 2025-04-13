use crate::edge::Edge;

/// Represents a group of 3 edges of the single face of the [Mesh].
///
/// Sometimes for some operations it is easier to work on grouped edges rather than on the
/// flattened list of all ids.
#[derive(Debug, Clone, Copy)]
pub struct ThreeEdges {
    /// First edge of the face.
    pub first: Edge,
    /// Second edge of the face.
    pub second: Edge,
    /// Third edge of the face.
    pub third: Edge,
}

impl PartialEq for ThreeEdges {
    fn eq(&self, other: &Self) -> bool {
        self.first.eq(&other.first) && self.second.eq(&other.second) && self.third.eq(&other.third)
    }
}

impl ThreeEdges {
    /// Creates a new [ThreeEdges].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::three_edges::ThreeEdges;
    ///
    /// let first = Edge::new(0, 1);
    /// let second = Edge::new(1, 2);
    /// let third = Edge::new(2, 0);
    ///
    /// let actual = ThreeEdges::new(first, second, third);
    ///
    /// assert!(actual.first.eq(&first));
    /// assert!(actual.second.eq(&second));
    /// assert!(actual.third.eq(&third));
    /// ```
    pub fn new(first: Edge, second: Edge, third: Edge) -> ThreeEdges { ThreeEdges { first, second, third } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let first = Edge::new(0, 1);
        let second = Edge::new(1, 2);
        let third = Edge::new(2, 0);
        
        let actual = ThreeEdges::new(first, second, third);
        
        assert!(actual.first.eq(&first));
        assert!(actual.second.eq(&second));
        assert!(actual.third.eq(&third));
    }

    #[test]
    fn test_partialeq_true() {
        let a = ThreeEdges::new(Edge::new(0, 1), Edge::new(1, 2), Edge::new(2, 0));
        let b  = ThreeEdges::new(Edge::new(0, 1), Edge::new(1, 2), Edge::new(2, 0));
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = ThreeEdges::new(Edge::new(0, 1), Edge::new(1, 2), Edge::new(2, 0));
        let b  = ThreeEdges::new(Edge::new(0, 8), Edge::new(1, 2), Edge::new(2, 0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_second_different_false() {
        let a = ThreeEdges::new(Edge::new(0, 1), Edge::new(1, 2), Edge::new(2, 0));
        let b  = ThreeEdges::new(Edge::new(0, 1), Edge::new(8, 2), Edge::new(2, 0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_third_different_false() {
        let a = ThreeEdges::new(Edge::new(0, 1), Edge::new(1, 2), Edge::new(2, 0));
        let b  = ThreeEdges::new(Edge::new(0, 1), Edge::new(1, 2), Edge::new(2, 8));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = ThreeEdges::new(Edge::new(0, 1), Edge::new(1, 2), Edge::new(2, 0));
        let b  = ThreeEdges::new(Edge::new(2, 1), Edge::new(1, 0), Edge::new(0, 2));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }
}
