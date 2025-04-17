use crate::face_neighbours::FaceNeighbours;

impl FaceNeighbours {
    /// Gets which edge (0 = first, 1 = second, 2 = third) is a neighbour.
    /// 
    /// `None` is returned if there is no neighbour edge.
    /// 
    /// # Examples
    /// 
    /// This example below shows finding out that second edge is the one that is a neighbour.
    /// 
    /// ```
    /// use meshmeshmesh::face_neighbours::FaceNeighbours;
    /// 
    /// let face_neighbours = FaceNeighbours::new(Some(5), Some(7), None);
    /// let actual = face_neighbours.get_which_edge_by_face_neighbour_id(7);
    /// 
    /// assert_eq!(actual, Some(1))
    /// 
    /// ```
    ///
    /// This example below shows finding out that there is no edge that is a neighbour.
    ///
    /// ```
    /// use meshmeshmesh::face_neighbours::FaceNeighbours;
    ///
    /// let face_neighbours = FaceNeighbours::new(Some(5), Some(7), None);
    /// let actual = face_neighbours.get_which_edge_by_face_neighbour_id(8);
    ///
    /// assert!(actual.is_none())
    ///
    /// ```
    pub fn get_which_edge_by_face_neighbour_id(&self, face_neighbour_id: usize) -> Option<usize> {
        if self.first == Some(face_neighbour_id) { 
            Some(0)
        }
        else if self.second == Some(face_neighbour_id) {
            Some(1)
        }
        else if self.third == Some(face_neighbour_id) {
            Some(2)
        }
        else {
            None
        }
    }

    /// Checks if all edges have its own neighbour.
    ///
    /// If yes = it returns `true`.
    ///
    /// If no = it returns `false`.
    ///
    /// # Examples
    ///
    /// In this example below there is 1 `None` for second edge, so `false` is returned.
    ///
    /// ```
    /// use meshmeshmesh::face_neighbours::FaceNeighbours;
    /// 
    /// let face_neighbours = FaceNeighbours::new(Some(5), None, Some(7));
    /// let actual = face_neighbours.has_all_neighbours();
    ///
    /// assert!(!actual)
    ///
    /// ```
    ///
    /// In this example below all edges have `Some` neighbour, so `true` is returned.
    ///
    /// ```
    /// use meshmeshmesh::face_neighbours::FaceNeighbours;
    ///
    /// let face_neighbours = FaceNeighbours::new(Some(5), Some(3), Some(7));
    /// let actual = face_neighbours.has_all_neighbours();
    ///
    /// assert!(actual)
    ///
    /// ```
    pub fn has_all_neighbours(&self) -> bool {
        self.first.is_some() && self.second.is_some() && self.third.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_which_edge_by_face_neighbour_id_0() {
        let face_neighbours = FaceNeighbours::new(Some(5), Some(7), None);
        let actual = face_neighbours.get_which_edge_by_face_neighbour_id(5);
        
        assert_eq!(actual, Some(0))
    }

    #[test]
    fn test_get_which_edge_by_face_neighbour_id_1() {
        let face_neighbours = FaceNeighbours::new(Some(5), Some(7), None);
        let actual = face_neighbours.get_which_edge_by_face_neighbour_id(7);

        assert_eq!(actual, Some(1))
    }

    #[test]
    fn test_get_which_edge_by_face_neighbour_id_2() {
        let face_neighbours = FaceNeighbours::new(None, Some(7), Some(5));
        let actual = face_neighbours.get_which_edge_by_face_neighbour_id(5);

        assert_eq!(actual, Some(2))
    }

    #[test]
    fn test_get_which_edge_by_face_neighbour_id_none() {
        let face_neighbours = FaceNeighbours::new(Some(5), Some(7), None);
        let actual = face_neighbours.get_which_edge_by_face_neighbour_id(8);
        
        assert!(actual.is_none())
    }
    
    #[test]
    fn test_has_all_neighbours_1_none() {
        let face_neighbours = FaceNeighbours::new(None, Some(5), Some(7));
        let actual = face_neighbours.has_all_neighbours();

        assert!(!actual)
    }

    #[test]
    fn test_has_all_neighbours_2_none() {
        let face_neighbours = FaceNeighbours::new(Some(5), None, Some(7));
        let actual = face_neighbours.has_all_neighbours();
        
        assert!(!actual)
    }

    #[test]
    fn test_has_all_neighbours_3_none() {
        let face_neighbours = FaceNeighbours::new(Some(5), Some(7), None);
        let actual = face_neighbours.has_all_neighbours();

        assert!(!actual)
    }

    #[test]
    fn test_has_all_neighbours_all_none() {
        let face_neighbours = FaceNeighbours::new(None, None, None);
        let actual = face_neighbours.has_all_neighbours();

        assert!(!actual)
    }

    #[test]
    fn test_has_all_neighbours_no_none() {
        let face_neighbours = FaceNeighbours::new(Some(5), Some(3), Some(7));
        let actual = face_neighbours.has_all_neighbours();
        
        assert!(actual)
    }
}