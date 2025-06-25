use std::ops;
use crate::rotation::Rotation;

impl ops::Mul<Rotation> for Rotation {
    type Output = Rotation;

    /// Multiplies a [Rotation] with another one.
    ///
    /// The result is a new Rotation.
    ///
    /// This can be useful when you want to combine multiple Rotations into single one.
    ///
    /// # Example
    ///
    /// Here is an example of multiplying a Vector with a 2.5 value.
    ///
    /// ```
    /// use meshmeshmesh::rotation::Rotation;
    ///
    /// let a = Rotation::new(-0.16408259600363556,  -0.62745925514862166, 0.7364020684484639, 0.1925714890680042);
    /// let b = Rotation::new(0.12038213521910773, -0.637288205465847, 0.747937726665425, -0.14128355079330665);
    ///
    /// let actual = a * b;
    ///
    /// let expected = Rotation::new(0.04636,0.1773,0.22009,-0.95811);
    /// 
    /// assert!(expected.eq_with_tolerance(&actual, 0.0001));
    /// 
    /// ```
    fn mul(self, other: Rotation) -> Rotation {
        Rotation::new(
            self.qw * other.qx + self.qx * other.qw + self.qy * other.qz - self.qz * other.qy,
            self.qw * other.qy - self.qx * other.qz + self.qy * other.qw + self.qz * other.qx,
            self.qw * other.qz + self.qx * other.qy - self.qy * other.qx + self.qz * other.qw,
            self.qw * other.qw - self.qx * other.qx - self.qy * other.qy - self.qz * other.qz,
        )
    }
}

impl Rotation {
    /// Gets inverted version of quaternion which defines [Rotation].
    /// 
    /// # Example
    /// 
    /// ```
    /// use meshmeshmesh::rotation::Rotation;
    /// 
    /// let input = Rotation::new(-0.16408259600363556,  -0.62745925514862166, 0.7364020684484639, 0.1925714890680042);
    /// let actual = input.get_inverted();
    /// let expected = Rotation::new(0.16408259600363556,  0.62745925514862166, -0.7364020684484639, 0.1925714890680042);
    /// 
    /// assert_eq!(expected, actual);
    /// 
    /// ```
    pub fn get_inverted(&self) -> Rotation {
        Rotation::new(-self.qx, -self.qy, -self.qz, self.qw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let a = Rotation::new(-0.16408259600363556,  -0.62745925514862166, 0.7364020684484639, 0.1925714890680042);
        let b = Rotation::new(0.12038213521910773, -0.637288205465847, 0.747937726665425, -0.14128355079330665);
        
        let actual = a * b;
        
        let expected = Rotation::new(0.04636,0.1773,0.22009,-0.95811);
        
        assert!(expected.eq_with_tolerance(&actual, 0.0001));
    }
    
    #[test]
    fn test_get_inverted() {
        let input = Rotation::new(-0.16408259600363556,  -0.62745925514862166, 0.7364020684484639, 0.1925714890680042);
        let actual = input.get_inverted();
        let expected = Rotation::new(0.16408259600363556,  0.62745925514862166, -0.7364020684484639, 0.1925714890680042);
        
        assert_eq!(expected, actual);
    }
}