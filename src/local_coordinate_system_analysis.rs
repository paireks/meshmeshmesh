use crate::local_coordinate_system::LocalCoordinateSystem;
use crate::ray::Ray;

impl LocalCoordinateSystem {
    /// Gets x-axis as [Ray]
    pub fn get_x_ray(&self) -> Ray {
        Ray::new(self.origin, self.x)
    }

    /// Gets y-axis as [Ray]
    pub fn get_y_ray(&self) -> Ray {
        Ray::new(self.origin, self.y)
    }

    /// Gets z-axis as [Ray]
    pub fn get_z_ray(&self) -> Ray {
        Ray::new(self.origin, self.z)
    }
}