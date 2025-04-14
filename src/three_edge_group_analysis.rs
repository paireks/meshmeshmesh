use crate::three_edge_group::ThreeEdgeGroup;

impl ThreeEdgeGroup {
    /// It tries to find which [Edge] is a neighbour (0 = first, 1 = second, 2 = third).
    /// 
    /// If there is no neighbour - it returns `None`.
    /// 
    /// # Examples
    /// 
    /// In this example below the second edge is a neighbour, so 1 is returned.
    /// 
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::three_edge_group::ThreeEdgeGroup;
    ///
    /// let input = ThreeEdgeGroup::new(Edge::new(0, 1), Edge::new(1, 2), Edge::new(2, 0));
    /// let potential_neighbour  = ThreeEdgeGroup::new(Edge::new(5, 2), Edge::new(2, 1), Edge::new(1, 5));
    /// 
    /// let actual = input.which_edge_is_neighbour_to(&potential_neighbour);
    /// let expected = Some(1);
    /// assert_eq!(actual, expected);
    /// 
    /// ```
    /// 
    /// In this example below there is no neighbour, so `None` is returned.
    /// 
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::three_edge_group::ThreeEdgeGroup;
    ///
    /// let input = ThreeEdgeGroup::new(Edge::new(0, 1), Edge::new(1, 2), Edge::new(2, 0));
    /// let potential_neighbour  = ThreeEdgeGroup::new(Edge::new(5, 2), Edge::new(2, 9), Edge::new(9, 5));
    ///
    /// let actual = input.which_edge_is_neighbour_to(&potential_neighbour);
    /// assert!(actual.is_none());
    ///
    /// ```
    pub fn which_edge_is_neighbour_to(&self, other: &ThreeEdgeGroup) -> Option<usize> {
        if self.first.eq_regardless_of_direction(&other.first) || self.first.eq_regardless_of_direction(&other.second) || self.first.eq_regardless_of_direction(&other.third) { 
            Some(0)
        }
        else if self.second.eq_regardless_of_direction(&other.first) || self.second.eq_regardless_of_direction(&other.second) || self.second.eq_regardless_of_direction(&other.third) { 
            Some(1)
        }
        else if self.third.eq_regardless_of_direction(&other.first) || self.third.eq_regardless_of_direction(&other.second) || self.third.eq_regardless_of_direction(&other.third) { 
            Some(2)
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::edge::Edge;
    use super::*;

    #[test]
    fn test_which_edge_is_neighbour_to_0() {
        let input = ThreeEdgeGroup::new(Edge::new(2, 5), Edge::new(5, 7), Edge::new(7, 2));
        let potential_neighbour  = ThreeEdgeGroup::new(Edge::new(5, 2), Edge::new(2, 1), Edge::new(1, 5));

        let actual = input.which_edge_is_neighbour_to(&potential_neighbour);
        let expected = Some(0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_which_edge_is_neighbour_to_1() {
        let input = ThreeEdgeGroup::new(Edge::new(0, 1), Edge::new(1, 2), Edge::new(2, 0));
        let potential_neighbour  = ThreeEdgeGroup::new(Edge::new(5, 2), Edge::new(2, 1), Edge::new(1, 5));
        
        let actual = input.which_edge_is_neighbour_to(&potential_neighbour);
        let expected = Some(1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_which_edge_is_neighbour_to_2() {
        let input = ThreeEdgeGroup::new(Edge::new(5, 8), Edge::new(8, 1), Edge::new(1, 5));
        let potential_neighbour  = ThreeEdgeGroup::new(Edge::new(5, 2), Edge::new(2, 1), Edge::new(1, 5));

        let actual = input.which_edge_is_neighbour_to(&potential_neighbour);
        let expected = Some(2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_which_edge_is_neighbour_to_none() {
        let input = ThreeEdgeGroup::new(Edge::new(0, 1), Edge::new(1, 2), Edge::new(2, 0));
        let potential_neighbour  = ThreeEdgeGroup::new(Edge::new(5, 2), Edge::new(2, 9), Edge::new(9, 5));
        
        let actual = input.which_edge_is_neighbour_to(&potential_neighbour);
        assert!(actual.is_none());
    }
}