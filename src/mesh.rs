use serde::{Deserialize, Serialize};
use crate::point::Point3D;

/// Represents a mesh object in three-dimensional space.
#[derive(Deserialize, Serialize)]
pub struct Mesh {
    /// The list of coordinates for the mesh vertices.
    pub coordinates: Vec<f64>,
    /// The list of indices for the mesh triangles.
    pub indices: Vec<i32>,
}

impl PartialEq for Mesh {
    fn eq(&self, other: &Self) -> bool {

        if self.coordinates.len() != other.coordinates.len() {
            return false;
        }
        for i in 0..self.coordinates.len() {
            if self.coordinates[i] != other.coordinates[i] {
                return false;
            }
        }
        if self.indices.len() != other.indices.len() {
            return false;
        }
        for i in 0..self.indices.len() {
            if self.indices[i] != other.indices[i] {
                return false;
            }
        }

        true
    }
}

impl Mesh {
    /// Returns a new Mesh
    pub fn new(coordinates: Vec<f64>, indices: Vec<i32>) -> Mesh {Mesh {coordinates, indices}}

    /// Converts Mesh into list of Point3Ds
    pub fn to_points(&self) -> Vec<Point3D> {
        let mut points = Vec::<Point3D>::new();
        let coordinates_length: usize = self.coordinates.len();
        let mut i = 0;
        while i < coordinates_length {
            points.push(Point3D::new(self.coordinates[i], self.coordinates[i+1], self.coordinates[i+2]));
            i = i + 3;
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;
    use serde_json::to_string;
    use super::*;

    #[test]
    fn test_new() {
        let result = Mesh::new(vec![0.0, 0.0, 0.0,
                                    10.0, 0.0, 0.0,
                                    10.0, -15.0, 0.0],
                               vec![0, 1, 2]);
        assert_eq!(result.coordinates, vec![0.0, 0.0, 0.0,
                                            10.0, 0.0, 0.0,
                                            10.0, -15.0, 0.0]);
        assert_eq!(result.indices, vec![0, 1, 2]);
    }

    #[test]
    fn test_to_points() {
        let input = Mesh::new(vec![0.0, 0.0, 0.0,
                                   10.0, 0.0, 0.0,
                                   10.0, -15.0, 0.0],
                              vec![0, 1, 2]);
        let actual = input.to_points();
        let expected = vec![Point3D::new(0.0, 0.0, 0.0),
                            Point3D::new(10.0, 0.0, 0.0),
                            Point3D::new(10.0, -15.0, 0.0)];
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i].eq(&actual[i]), true);
        }
    }

    #[test]
    fn test_partialeq_true() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_coordinates_count_false() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0,
                               5.0, 1.0, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_different_coordinates_false() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 2.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_indices_count_false() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2, 2, 1, 0]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_different_indices_false() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 2, 1]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_to_json() {
        let input = Mesh::new(vec![0.0, 0.0, 0.0,
                                   10.0, 0.0, 0.0,
                                   10.0, -15.0, 0.0],
                              vec![0, 1, 2]);
        let input_serialized = to_string(&input);
        assert_eq!(input_serialized.is_ok(), true);
        let input_serialized_string = input_serialized.ok().unwrap();
        assert_eq!(input_serialized_string, "{\"coordinates\":[0.0,0.0,0.0,10.0,0.0,0.0,10.0,-15.0,0.0],\"indices\":[0,1,2]}");
    }

    #[test]
    fn test_from_json() {
        let json = "{\"coordinates\":[0.0,0.0,0.0,10.0,0.0,0.0,10.0,-15.0,0.0],\"indices\":[0,1,2]}";
        let actual_result = from_str::<Mesh>(json);
        assert_eq!(actual_result.is_ok(), true);
        let actual = actual_result.ok().unwrap();
        let expected = Mesh::new(vec![0.0, 0.0, 0.0,
                                      10.0, 0.0, 0.0,
                                      10.0, -15.0, 0.0],
                                 vec![0, 1, 2]);
        assert_eq!(expected.eq(&actual), true);
    }
}