//! Fixed-point scalar type for deterministic arithmetic
//! 
//! For more complex linear algebra operations, consider using the nalgebra crate
//! with custom scalar types for deterministic computation.

use core::fmt;
use core::ops::{Add, Sub, Mul, Div, Neg};
use fixed::types::I16F16;
use serde::{Serialize, Deserialize};

/// Q16.16 fixed-point scalar for deterministic physics calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Scalar(pub I16F16);

impl Scalar {
    /// Zero value
    pub const ZERO: Self = Scalar(I16F16::ZERO);
    
    /// One value
    pub const ONE: Self = Scalar(I16F16::ONE);
    
    /// Two value
    pub const TWO: Self = Scalar(I16F16::from_bits(0x00020000));
    
    /// Half value
    pub const HALF: Self = Scalar(I16F16::from_bits(0x00008000));
    
    /// Create from floating-point value
    pub fn from_float(f: f32) -> Self {
        Scalar(I16F16::from_num(f))
    }
    
    /// Convert to floating-point value (for debugging/display only)
    pub fn to_float(&self) -> f32 {
        self.0.to_num()
    }
    
    /// Get the raw bits representation
    pub fn to_bits(&self) -> i32 {
        self.0.to_bits()
    }
    
    /// Create from raw bits
    pub fn from_bits(bits: i32) -> Self {
        Scalar(I16F16::from_bits(bits))
    }
    
    /// Absolute value
    pub fn abs(&self) -> Self {
        Scalar(self.0.abs())
    }
    
    /// Square root using Newton-Raphson method
    pub fn sqrt(&self) -> Self {
        if self.0 <= I16F16::ZERO {
            return Scalar::ZERO;
        }
        
        // Initial guess: right shift by 1 (divide by 2)
        let mut guess = Scalar(self.0 >> 1);
        
        // Newton-Raphson iterations
        for _ in 0..8 {
            let next = (guess + *self / guess) / Scalar::TWO;
            if (next.0 - guess.0).abs() < I16F16::from_bits(1) {
                break;
            }
            guess = next;
        }
        
        guess
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}", self.to_float())
    }
}

impl Add for Scalar {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        Scalar(self.0 + rhs.0)
    }
}

impl Sub for Scalar {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Scalar(self.0 - rhs.0)
    }
}

impl Mul for Scalar {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self::Output {
        Scalar(self.0 * rhs.0)
    }
}

impl Div for Scalar {
    type Output = Self;
    
    fn div(self, rhs: Self) -> Self::Output {
        Scalar(self.0 / rhs.0)
    }
}

impl Neg for Scalar {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        Scalar(-self.0)
    }
}

impl Default for Scalar {
    fn default() -> Self {
        Scalar::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scalar_basic_ops() {
        let a = Scalar::from_float(2.5);
        let b = Scalar::from_float(1.5);
        
        // Test exact fixed-point results
        assert_eq!(a + b, Scalar::from_float(4.0));
        assert_eq!(a - b, Scalar::from_float(1.0));
        assert_eq!(a * b, Scalar::from_float(3.75));
        
        // For division, we can test the exact bit representation
        let div_result = a / b;
        let expected = Scalar::from_float(5.0) / Scalar::from_float(3.0);
        assert_eq!(div_result.to_bits(), expected.to_bits());
    }
    
    #[test]
    fn test_scalar_sqrt() {
        let values = [0.0, 1.0, 4.0, 9.0, 16.0, 25.0];
        
        for &v in &values {
            let s = Scalar::from_float(v);
            let sqrt = s.sqrt();
            let expected = v.sqrt();
            
            // Check within fixed-point precision
            assert!((sqrt.to_float() - expected).abs() < 0.01);
        }
    }
    
    #[test]
    fn test_determinism() {
        // Same operations should produce bit-identical results
        let a = Scalar::from_float(1.234);
        let b = Scalar::from_float(5.678);
        
        let result1 = (a * b + a) / b;
        let result2 = (a * b + a) / b;
        
        assert_eq!(result1.to_bits(), result2.to_bits());
    }
}