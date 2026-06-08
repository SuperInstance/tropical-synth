use std::ops::{Add, Mul};
use serde::{Serialize, Deserialize};

/// A tropical number: `f64` in the **max-plus** semiring.
///
/// - **Addition** (`⊕`) = `max(a, b)`
/// - **Multiplication** (`⊗`) = `a + b`
/// - **Zero** (additive identity) = `-∞`
/// - **One** (multiplicative identity) = `0.0`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Tropical(pub f64);

impl Tropical {
    /// The additive identity: negative infinity.
    pub const ZERO: Tropical = Tropical(f64::NEG_INFINITY);

    /// The multiplicative identity: zero.
    pub const ONE: Tropical = Tropical(0.0);

    /// Create a new tropical number.
    pub fn new(val: f64) -> Self {
        Tropical(val)
    }

    /// Exponentiation in the tropical semiring: `x ⊗ x ⊗ … ⊗ x` (n times) = `n * x`.
    pub fn pow(self, n: u32) -> Tropical {
        Tropical(self.0 * n as f64)
    }

    /// Inner f64 value.
    pub fn value(self) -> f64 {
        self.0
    }
}

/// Tropical addition: `max(a, b)`.
impl Add for Tropical {
    type Output = Tropical;
    fn add(self, rhs: Tropical) -> Tropical {
        Tropical(self.0.max(rhs.0))
    }
}

/// Tropical multiplication: `a + b`.
#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul for Tropical {
    type Output = Tropical;
    fn mul(self, rhs: Tropical) -> Tropical {
        Tropical(self.0 + rhs.0)
    }
}

impl std::fmt::Display for Tropical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == f64::NEG_INFINITY {
            write!(f, "-∞")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_is_max() {
        assert_eq!(Tropical(3.0) + Tropical(5.0), Tropical(5.0));
        assert_eq!(Tropical(10.0) + Tropical(2.0), Tropical(10.0));
    }

    #[test]
    fn multiplication_is_addition() {
        assert_eq!(Tropical(3.0) * Tropical(5.0), Tropical(8.0));
    }

    #[test]
    fn zero_is_additive_identity() {
        assert_eq!(Tropical::ZERO + Tropical(42.0), Tropical(42.0));
    }

    #[test]
    fn one_is_multiplicative_identity() {
        assert_eq!(Tropical::ONE * Tropical(42.0), Tropical(42.0));
    }

    #[test]
    fn pow_is_scalar_multiply() {
        assert_eq!(Tropical(3.0).pow(4), Tropical(12.0));
    }

    #[test]
    fn associativity_of_addition() {
        let a = Tropical(1.0);
        let b = Tropical(2.0);
        let c = Tropical(3.0);
        assert_eq!((a + b) + c, a + (b + c));
    }

    #[test]
    fn distributivity() {
        let a = Tropical(1.0);
        let b = Tropical(2.0);
        let c = Tropical(3.0);
        // a ⊗ (b ⊕ c) = a ⊗ b ⊕ a ⊗ c
        assert_eq!(a * (b + c), a * b + a * c);
    }
}
