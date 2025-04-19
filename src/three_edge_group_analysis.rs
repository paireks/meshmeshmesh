use std::collections::HashMap;
use crate::edge::Edge;
use crate::three_edge_group::ThreeEdgeGroup;

impl ThreeEdgeGroup {
    /// It tries to find which Edge is a neighbour (0 = first, 1 = second, 2 = third).
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

    /// It creates the [Edge] hashmap as keys, while values are `vec`s of indices of faces.
    ///
    /// This way you can see which edges are connected to which faces.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::three_edge_group::ThreeEdgeGroup;
    ///
    /// let input = vec![
    ///     ThreeEdgeGroup::new(Edge::new(0, 2), Edge::new(2, 1), Edge::new(1, 0)), // first face
    ///     ThreeEdgeGroup::new(Edge::new(1, 2), Edge::new(2, 3), Edge::new(3, 1)), // second face
    ///     ThreeEdgeGroup::new(Edge::new(2, 4), Edge::new(4, 3), Edge::new(3, 2)), // third face
    ///     ThreeEdgeGroup::new(Edge::new(1, 3), Edge::new(3, 5), Edge::new(5, 1)), // fourth face
    ///     ThreeEdgeGroup::new(Edge::new(1, 3), Edge::new(3, 6), Edge::new(6, 1)), // fifth face
    ///     ThreeEdgeGroup::new(Edge::new(5, 3), Edge::new(3, 6), Edge::new(6, 5)), // sixth face
    /// ];
    /// 
    /// let actual = ThreeEdgeGroup::get_edge_with_face_ids_hashmap(&input);
    ///
    /// let expected = HashMap::from([
    ///     (Edge::new(0, 2), vec![0]),
    ///     (Edge::new(2, 1), vec![0]),
    ///     (Edge::new(1, 0), vec![0]),
    ///     (Edge::new(1, 2), vec![1]),
    ///     (Edge::new(2, 3), vec![1]),
    ///     (Edge::new(3, 1), vec![1]),
    ///     (Edge::new(2, 4), vec![2]),
    ///     (Edge::new(4, 3), vec![2]),
    ///     (Edge::new(3, 2), vec![2]),
    ///     (Edge::new(1, 3), vec![3, 4]),
    ///     (Edge::new(3, 5), vec![3]),
    ///     (Edge::new(5, 1), vec![3]),
    ///     (Edge::new(3, 6), vec![4, 5]),
    ///     (Edge::new(6, 1), vec![4]),
    ///     (Edge::new(5, 3), vec![5]),
    ///     (Edge::new(6, 5), vec![5]),
    /// ]);
    ///
    /// assert_eq!(actual, expected);
    /// 
    /// ```
    pub fn get_edge_with_face_ids_hashmap(three_edge_groups: &Vec<ThreeEdgeGroup>) -> HashMap<Edge, Vec<usize>> {
        let number_of_faces = three_edge_groups.len();
        let mut edge_hashmap: HashMap<Edge, Vec<usize>> = HashMap::new();

        for i in 0..number_of_faces {
            let current_group = three_edge_groups[i];
            edge_hashmap.entry(current_group.first).or_insert_with(Vec::new).push(i);
            edge_hashmap.entry(current_group.second).or_insert_with(Vec::new).push(i);
            edge_hashmap.entry(current_group.third).or_insert_with(Vec::new).push(i);
        }

        edge_hashmap
    }

    /// It creates the [Edge] hashmap as keys, while values are `vec`s of indices of faces.
    ///
    /// This way you can see which edges are connected to which faces.
    /// 
    /// In this specific method we treat 2 edges same even if they are reversed. In other words
    /// edge 0 - 1 is same as 1 - 0 here.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::three_edge_group::ThreeEdgeGroup;
    ///
    /// let input = vec![
    ///     ThreeEdgeGroup::new(Edge::new(0, 2), Edge::new(2, 1), Edge::new(1, 0)), // first face
    ///     ThreeEdgeGroup::new(Edge::new(1, 2), Edge::new(2, 3), Edge::new(3, 1)), // second face
    ///     ThreeEdgeGroup::new(Edge::new(2, 4), Edge::new(4, 3), Edge::new(3, 2)), // third face
    ///     ThreeEdgeGroup::new(Edge::new(1, 3), Edge::new(3, 5), Edge::new(5, 1)), // fourth face
    ///     ThreeEdgeGroup::new(Edge::new(1, 3), Edge::new(3, 6), Edge::new(6, 1)), // fifth face
    ///     ThreeEdgeGroup::new(Edge::new(5, 3), Edge::new(3, 6), Edge::new(6, 5)), // sixth face
    /// ];
    ///
    /// let actual = ThreeEdgeGroup::get_edge_with_face_ids_hashmap_with_reversed_edges_merged(&input);
    ///
    /// let expected = HashMap::from([
    ///     (Edge::new(0, 2), vec![0]),
    ///     (Edge::new(2, 1), vec![0, 1]),
    ///     (Edge::new(1, 0), vec![0]),
    ///     (Edge::new(2, 3), vec![1, 2]),
    ///     (Edge::new(3, 1), vec![1, 3, 4]),
    ///     (Edge::new(2, 4), vec![2]),
    ///     (Edge::new(4, 3), vec![2]),
    ///     (Edge::new(3, 5), vec![3, 5]),
    ///     (Edge::new(5, 1), vec![3]),
    ///     (Edge::new(3, 6), vec![4, 5]),
    ///     (Edge::new(6, 1), vec![4]),
    ///     (Edge::new(6, 5), vec![5]),
    /// ]);
    ///
    /// assert_eq!(actual, expected);
    ///
    /// ```
    pub fn get_edge_with_face_ids_hashmap_with_reversed_edges_merged(three_edge_groups: &Vec<ThreeEdgeGroup>) -> HashMap<Edge, Vec<usize>> {
        let number_of_faces = three_edge_groups.len();
        let mut edge_hashmap: HashMap<Edge, Vec<usize>> = HashMap::new();

        for i in 0..number_of_faces {
            let current_group = three_edge_groups[i];
            if edge_hashmap.contains_key(&current_group.first.get_reversed()) {
                edge_hashmap.entry(current_group.first.get_reversed()).or_insert_with(Vec::new).push(i);
            }
            else {
                edge_hashmap.entry(current_group.first).or_insert_with(Vec::new).push(i);
            }
            
            if edge_hashmap.contains_key(&current_group.second.get_reversed()) {
                edge_hashmap.entry(current_group.second.get_reversed()).or_insert_with(Vec::new).push(i);
            }
            else {
                edge_hashmap.entry(current_group.second).or_insert_with(Vec::new).push(i);
            }
            
            if edge_hashmap.contains_key(&current_group.third.get_reversed()) {
                edge_hashmap.entry(current_group.third.get_reversed()).or_insert_with(Vec::new).push(i);
            }
            else {
                edge_hashmap.entry(current_group.third).or_insert_with(Vec::new).push(i);
            }
        }

        edge_hashmap
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
    
    #[test]
    fn test_get_edge_with_face_ids_hashmap() {
        let input = vec![
            ThreeEdgeGroup::new(Edge::new(0, 2), Edge::new(2, 1), Edge::new(1, 0)), // first face
            ThreeEdgeGroup::new(Edge::new(1, 2), Edge::new(2, 3), Edge::new(3, 1)), // second face
            ThreeEdgeGroup::new(Edge::new(2, 4), Edge::new(4, 3), Edge::new(3, 2)), // third face
            ThreeEdgeGroup::new(Edge::new(1, 3), Edge::new(3, 5), Edge::new(5, 1)), // fourth face
            ThreeEdgeGroup::new(Edge::new(1, 3), Edge::new(3, 6), Edge::new(6, 1)), // fifth face
            ThreeEdgeGroup::new(Edge::new(5, 3), Edge::new(3, 6), Edge::new(6, 5)), // sixth face
        ];
        
        let actual = ThreeEdgeGroup::get_edge_with_face_ids_hashmap(&input);
        
        let expected = HashMap::from([
            (Edge::new(0, 2), vec![0]),
            (Edge::new(2, 1), vec![0]),
            (Edge::new(1, 0), vec![0]),
            (Edge::new(1, 2), vec![1]),
            (Edge::new(2, 3), vec![1]),
            (Edge::new(3, 1), vec![1]),
            (Edge::new(2, 4), vec![2]),
            (Edge::new(4, 3), vec![2]),
            (Edge::new(3, 2), vec![2]),
            (Edge::new(1, 3), vec![3, 4]),
            (Edge::new(3, 5), vec![3]),
            (Edge::new(5, 1), vec![3]),
            (Edge::new(3, 6), vec![4, 5]),
            (Edge::new(6, 1), vec![4]),
            (Edge::new(5, 3), vec![5]),
            (Edge::new(6, 5), vec![5]),
        ]);
        
        assert_eq!(actual, expected);
        
    }
    
    #[test]
    fn test_get_edge_with_face_ids_hashmap_with_reversed_edges_merged() {
        let input = vec![
            ThreeEdgeGroup::new(Edge::new(0, 2), Edge::new(2, 1), Edge::new(1, 0)), // first face
            ThreeEdgeGroup::new(Edge::new(1, 2), Edge::new(2, 3), Edge::new(3, 1)), // second face
            ThreeEdgeGroup::new(Edge::new(2, 4), Edge::new(4, 3), Edge::new(3, 2)), // third face
            ThreeEdgeGroup::new(Edge::new(1, 3), Edge::new(3, 5), Edge::new(5, 1)), // fourth face
            ThreeEdgeGroup::new(Edge::new(1, 3), Edge::new(3, 6), Edge::new(6, 1)), // fifth face
            ThreeEdgeGroup::new(Edge::new(5, 3), Edge::new(3, 6), Edge::new(6, 5)), // sixth face
        ];
        
        let actual = ThreeEdgeGroup::get_edge_with_face_ids_hashmap_with_reversed_edges_merged(&input);
        
        let expected = HashMap::from([
            (Edge::new(0, 2), vec![0]),
            (Edge::new(2, 1), vec![0, 1]),
            (Edge::new(1, 0), vec![0]),
            (Edge::new(2, 3), vec![1, 2]),
            (Edge::new(3, 1), vec![1, 3, 4]),
            (Edge::new(2, 4), vec![2]),
            (Edge::new(4, 3), vec![2]),
            (Edge::new(3, 5), vec![3, 5]),
            (Edge::new(5, 1), vec![3]),
            (Edge::new(3, 6), vec![4, 5]),
            (Edge::new(6, 1), vec![4]),
            (Edge::new(6, 5), vec![5]),
        ]);
        
        assert_eq!(actual, expected);
        
    }
}