
/// Gets [Mesh]'s face neighbours.
///
/// Each neighbour is an index to a specific face.
/// 
/// There is an assumption that each face's edge has only 1 face neighbour. It might not always be
/// true, because it is possible that some faces have multiple faces as a neighbours.
/// 
/// If index is `None` = there is no neighbour face for that edge.
#[derive(Debug, Clone, Copy)]
pub struct FaceNeighbours {
    /// Neighbour of the first edge of that face.
    pub first: Option<usize>,
    /// Neighbour of the second edge of that face.
    pub second: Option<usize>,
    /// Neighbour of the third edge of that face.
    pub third: Option<usize>,
}

impl PartialEq for FaceNeighbours {
    fn eq(&self, other: &Self) -> bool {
        self.first.eq(&other.first) && self.second.eq(&other.second) && self.third.eq(&other.third)
    }
}

impl FaceNeighbours {
    
    /// Creates a new [FaceNeighbours].
    /// 
    /// # Example
    /// 
    /// ```
    /// use meshmeshmesh::face_neighbours::FaceNeighbours;
    /// 
    /// let result = FaceNeighbours::new(Some(0), None, Some(1)); // This means that face has first neighbour on that edge index 0, on the second edge it doesn't have any neighbour, and on the third edge it has neighbour index 1.
    ///
    /// assert!(result.first.eq(&Some(0)));
    /// assert!(result.second.is_none());
    /// assert!(result.third.eq(&Some(1)));
    /// ```
    pub fn new(first: Option<usize>, second: Option<usize>, third: Option<usize>) -> FaceNeighbours { FaceNeighbours { first, second, third } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = FaceNeighbours::new(Some(0), None, Some(1)); // This means that face has first neighbour on that edge index 0, on the second edge it doesn't have any neighbour, and on the third edge it has neighbour index 1.
        
        assert!(result.first.eq(&Some(0)));
        assert!(result.second.is_none());
        assert!(result.third.eq(&Some(1)));
    }

    #[test]
    fn test_partialeq_true() {
        let a = FaceNeighbours::new(Some(0), None, Some(1));
        let b  = FaceNeighbours::new(Some(0), None, Some(1));
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = FaceNeighbours::new(Some(0), None, Some(1));
        let b  = FaceNeighbours::new(Some(2), None, Some(1));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_second_different_false() {
        let a = FaceNeighbours::new(Some(0), None, Some(1));
        let b  = FaceNeighbours::new(Some(0), Some(3), Some(1));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_third_different_false() {
        let a = FaceNeighbours::new(Some(0), None, Some(1));
        let b  = FaceNeighbours::new(Some(0), None, Some(5));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = FaceNeighbours::new(Some(0), None, Some(1));
        let b  = FaceNeighbours::new(Some(2), Some(3), Some(0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }
}