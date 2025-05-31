use crate::local_coordinate_system::LocalCoordinateSystem;
use crate::ray::Ray;
use crate::vector::Vector;

impl LocalCoordinateSystem {
    /// Gets z-axis as [Vector].
    /// 
    /// Should be unitized.
    /// 
    /// It is defined by right hand thumb rule.
    pub fn get_z(&self) -> Vector {
        self.x.get_cross_product(&self.y).get_unitized()
    }
    
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
        Ray::new(self.origin, self.get_z())
    }
}