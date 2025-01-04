use crate::bounding_box::BoundingBox;
use crate::mesh::Mesh;

impl Mesh {

    /// Gets number of all faces that defines a Mesh
    pub fn get_number_of_faces(&self) -> usize {
        self.indices.len() / 3
    }

    /// Gets number of all vertices that defines a Mesh
    pub fn get_number_of_vertices(&self) -> usize {
        self.coordinates.len() / 3
    }

    /// Calculates the Bounding Box for given Mesh
    pub fn get_bounding_box(&self) -> BoundingBox {
        let x_coordinates = self.get_x_coordinates();
        let y_coordinates = self.get_y_coordinates();
        let z_coordinates = self.get_z_coordinates();

        let min_x = x_coordinates.iter().copied().reduce(f64::min).unwrap();
        let max_x = x_coordinates.iter().copied().reduce(f64::max).unwrap();

        let min_y = y_coordinates.iter().copied().reduce(f64::min).unwrap();
        let max_y = y_coordinates.iter().copied().reduce(f64::max).unwrap();

        let min_z = z_coordinates.iter().copied().reduce(f64::min).unwrap();
        let max_z = z_coordinates.iter().copied().reduce(f64::max).unwrap();

        BoundingBox::new(min_x, max_x, min_y, max_y, min_z, max_z)
    }

    /// Gets only x coordinates of Mesh
    pub fn get_x_coordinates(&self) -> Vec<f64> {
        self.coordinates.iter().skip(0).step_by(3).copied().collect()
    }

    /// Gets only y coordinates of Mesh
    pub fn get_y_coordinates(&self) -> Vec<f64> {
        self.coordinates.iter().skip(1).step_by(3).copied().collect()
    }

    /// Gets only z coordinates of Mesh
    pub fn get_z_coordinates(&self) -> Vec<f64> {
        self.coordinates.iter().skip(2).step_by(3).copied().collect()
    }

    /// Gets all faces' start indices
    pub fn get_start_indices(&self) -> Vec<usize> {
        self.indices.iter().skip(0).step_by(3).copied().collect()
    }

    /// Gets all faces' middle indices
    pub fn get_middle_indices(&self) -> Vec<usize> {
        self.indices.iter().skip(1).step_by(3).copied().collect()
    }

    /// Gets all faces' end indices
    pub fn get_end_indices(&self) -> Vec<usize> {
        self.indices.iter().skip(2).step_by(3).copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number_of_faces() {
        let input = Mesh::new(
            vec![
                // Base
                -2.0,1.0,0.0,
                8.0,1.0,0.0,
                8.0,11.0,0.0,
                -2.0,11.0,0.0,

                // Top
                3.0,6.0,4.0
            ],
            vec![
                // Base faces
                0,1,2,
                0,2,3,

                // Side faces
                0,1,4,
                1,2,4,
                2,3,4,
                3,0,4
            ]);

        let expected = 6;
        let actual = input.get_number_of_faces();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_number_of_vertices() {
        let input = Mesh::new(
            vec![
                // Base
                -2.0,1.0,0.0,
                8.0,1.0,0.0,
                8.0,11.0,0.0,
                -2.0,11.0,0.0,

                // Top
                3.0,6.0,4.0
            ],
            vec![
                // Base faces
                0,1,2,
                0,2,3,

                // Side faces
                0,1,4,
                1,2,4,
                2,3,4,
                3,0,4
            ]);

        let expected = 5;
        let actual = input.get_number_of_vertices();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_bounding_box() {
        let input = Mesh::new(
            vec![
                // Base
                -2.0,1.0,0.0,
                8.0,1.0,0.0,
                8.0,11.0,0.0,
                -2.0,11.0,0.0,

                // Top
                3.0,6.0,4.0
            ],
            vec![
                // Base faces
                0,1,2,
                0,2,3,

                // Side faces
                0,1,4,
                1,2,4,
                2,3,4,
                3,0,4
            ]);
        let expected = BoundingBox::new(-2.0, 8.0, 1.0, 11.0, 0.0, 4.0);
        let actual = input.get_bounding_box();

        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    fn test_get_different_coordinates() {
        let input = Mesh::new(
            vec![
                // Base
                -2.0,1.0,0.0,
                8.0,1.0,0.0,
                8.0,11.0,0.0,
                -2.0,11.0,0.0,

                // Top
                3.0,6.0,4.0
            ],
            vec![
                // Base faces
                0,1,2,
                0,2,3,

                // Side faces
                0,1,4,
                1,2,4,
                2,3,4,
                3,0,4
            ]);
        let expected_x = vec![-2.0, 8.0, 8.0, -2.0, 3.0];
        let actual_x = input.get_x_coordinates();
        assert_eq!(expected_x, actual_x);

        let expected_y = vec![1.0, 1.0, 11.0, 11.0, 6.0];
        let actual_y = input.get_y_coordinates();
        assert_eq!(expected_y, actual_y);

        let expected_z = vec![0.0, 0.0, 0.0, 0.0, 4.0];
        let actual_z = input.get_z_coordinates();
        assert_eq!(expected_z, actual_z);
    }

    #[test]
    fn test_get_different_indices() {
        let input = Mesh::new(
            vec![
                // Base
                -2.0,1.0,0.0,
                8.0,1.0,0.0,
                8.0,11.0,0.0,
                -2.0,11.0,0.0,

                // Top
                3.0,6.0,4.0
            ],
            vec![
                // Base faces
                0,1,2,
                0,2,3,

                // Side faces
                0,1,4,
                1,2,4,
                2,3,4,
                3,0,4
            ]);
        let expected_x = vec![0, 0, 0, 1, 2, 3];
        let actual_x = input.get_start_indices();
        assert_eq!(expected_x, actual_x);

        let expected_y = vec![1, 2, 1, 2, 3, 0];
        let actual_y = input.get_middle_indices();
        assert_eq!(expected_y, actual_y);

        let expected_z = vec![2, 3, 4, 4, 4, 4];
        let actual_z = input.get_end_indices();
        assert_eq!(expected_z, actual_z);
    }
}