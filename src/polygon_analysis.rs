use std::f64::consts::PI;
use crate::local_coordinate_system::LocalCoordinateSystem;
use crate::polygon::Polygon;
use crate::ray::Ray;
use crate::vector::Vector;

impl Polygon {
    /// Gets x-axis as [Ray].
    ///
    /// Its x-axis comes from first to the second [Point].
    pub fn get_x_ray(&self) -> Ray {
        Ray::new(self.vertices[0], Vector::from_2_points(&self.vertices[0], &self.vertices[1]))
    }

    /// Gets z-axis as [Ray]
    ///
    /// The z-axis is perpendicular to the plane of this [Polygon].
    pub fn get_z_ray(&self) -> Ray {
        let from_first_to_second = Vector::from_2_points(&self.vertices[0], &self.vertices[1]);
        let from_first_to_last = Vector::from_2_points(&self.vertices[0], &self.vertices[self.vertices.len() - 1]);

        Ray::new(self.vertices[0], from_first_to_second.get_cross_product(&from_first_to_last))
    }

    /// Gets [LocalCoordinateSystem] for given [Polygon].
    ///
    /// This system is located in the first [Point] of the [Polygon].
    ///
    /// Its x-axis points to the second [Point].
    ///
    /// The z-axis is perpendicular to the plane of this [Polygon].
    ///
    /// The y-axis is the x-axis rotated by 90 degrees angle using z-axis.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::polygon::Polygon;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = Polygon::new(vec![
    ///     Point::new(-15.519542, 33.6924, 54.752506),
    ///     Point::new(-6.776692, 72.957549, 102.8696),
    ///     Point::new(38.186615, 79.290175, 45.436313),
    ///     Point::new(20.315263, 45.368737, 19.312824),
    ///     Point::new(4.753062, 55.839337, 58.928299),
    /// ]);
    ///
    /// let actual = input.get_local_coordinate_system();
    ///
    /// let expected_origin = Point::new(-15.519542, 33.6924,54.752506);
    /// let expected_x = Vector::new(0.13940120784477725,0.6260669228918656,0.767207606396162);
    /// let expected_y = Vector::new(0.7651247740152495,0.4237333029589709,-0.4848032262182198);
    ///
    /// let expected = LocalCoordinateSystem::new(expected_origin, expected_x, expected_y);
    ///
    /// assert_eq!(expected, actual);
    ///
    /// ```
    pub fn get_local_coordinate_system(&self) -> LocalCoordinateSystem {
        let x_axis = self.get_x_ray();
        let z_axis = self.get_z_ray();
        let y_vector = x_axis.direction.get_rotated(&z_axis.direction, PI / 2.0);

        LocalCoordinateSystem::new(x_axis.origin, x_axis.direction, y_vector)
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use super::*;

    #[test]
    fn test_get_local_coordinate_system() {
        let input = Polygon::new(vec![
            Point::new(-15.519542, 33.6924, 54.752506),
            Point::new(-6.776692, 72.957549, 102.8696),
            Point::new(38.186615, 79.290175, 45.436313),
            Point::new(20.315263, 45.368737, 19.312824),
            Point::new(4.753062, 55.839337, 58.928299),
        ]);

        let actual = input.get_local_coordinate_system();

        let expected_origin = Point::new(-15.519542, 33.6924,54.752506);
        let expected_x = Vector::new(0.13940120784477725,0.6260669228918656,0.767207606396162);
        let expected_y = Vector::new(0.7651247740152495,0.4237333029589709,-0.4848032262182198);

        let expected = LocalCoordinateSystem::new(expected_origin, expected_x, expected_y);

        assert_eq!(expected, actual);
    }
}