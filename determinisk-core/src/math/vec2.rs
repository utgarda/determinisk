//! 2D vector type for physics calculations

use core::fmt;
use core::ops::{Add, Sub, Mul, Div, Neg};
use serde::{Serialize, Deserialize};

use super::Scalar;

/// 2D vector with fixed-point components
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vec2 {
    /// Zero vector
    pub const ZERO: Self = Vec2 { 
        x: Scalar::ZERO, 
        y: Scalar::ZERO 
    };
    
    /// Unit vector in X direction
    pub const UNIT_X: Self = Vec2 { 
        x: Scalar::ONE, 
        y: Scalar::ZERO 
    };
    
    /// Unit vector in Y direction
    pub const UNIT_Y: Self = Vec2 { 
        x: Scalar::ZERO, 
        y: Scalar::ONE 
    };
    
    /// Create a new vector
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 {
            x: Scalar::from_float(x),
            y: Scalar::from_float(y),
        }
    }
    
    /// Create from scalar components
    pub fn from_scalars(x: Scalar, y: Scalar) -> Self {
        Vec2 { x, y }
    }
    
    /// Dot product
    pub fn dot(&self, other: &Vec2) -> Scalar {
        self.x * other.x + self.y * other.y
    }
    
    /// Squared magnitude (avoids sqrt)
    pub fn magnitude_squared(&self) -> Scalar {
        self.x * self.x + self.y * self.y
    }
    
    /// Magnitude
    pub fn magnitude(&self) -> Scalar {
        self.magnitude_squared().sqrt()
    }
    
    /// Normalize the vector
    pub fn normalized(&self) -> Self {
        let mag = self.magnitude();
        if mag > Scalar::ZERO {
            *self / mag
        } else {
            *self
        }
    }
    
    /// Perpendicular vector (rotated 90 degrees counter-clockwise)
    pub fn perp(&self) -> Self {
        Vec2 {
            x: -self.y,
            y: self.x,
        }
    }
    
    /// Linear interpolation
    pub fn lerp(&self, other: &Vec2, t: Scalar) -> Self {
        *self + (*other - *self) * t
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Vec2 {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Scalar> for Vec2 {
    type Output = Self;
    
    fn mul(self, rhs: Scalar) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<Scalar> for Vec2 {
    type Output = Self;
    
    fn div(self, rhs: Scalar) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec2_basic_ops() {
        let a = Vec2::new(3.0, 4.0);
        let b = Vec2::new(1.0, 2.0);
        
        let sum = a + b;
        assert_eq!(sum.x.to_float(), 4.0);
        assert_eq!(sum.y.to_float(), 6.0);
        
        let diff = a - b;
        assert_eq!(diff.x.to_float(), 2.0);
        assert_eq!(diff.y.to_float(), 2.0);
    }
    
    #[test]
    fn test_vec2_magnitude() {
        let v = Vec2::new(3.0, 4.0);
        assert!((v.magnitude().to_float() - 5.0).abs() < 0.01);
        
        let v2 = Vec2::new(5.0, 12.0);
        assert!((v2.magnitude().to_float() - 13.0).abs() < 0.01);
    }
    
    #[test]
    fn test_vec2_normalize() {
        let v = Vec2::new(3.0, 4.0);
        let n = v.normalized();
        
        assert!((n.magnitude().to_float() - 1.0).abs() < 0.01);
        assert!((n.x.to_float() - 0.6).abs() < 0.01);
        assert!((n.y.to_float() - 0.8).abs() < 0.01);
    }
    
    #[test]
    fn test_vec2_dot_product() {
        let a = Vec2::new(2.0, 3.0);
        let b = Vec2::new(4.0, 5.0);
        
        let dot = a.dot(&b);
        assert_eq!(dot.to_float(), 23.0); // 2*4 + 3*5 = 8 + 15 = 23
    }
}