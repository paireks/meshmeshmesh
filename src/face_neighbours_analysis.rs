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
}