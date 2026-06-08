use std::fmt;

/// Errors that can arise in tropical-synth operations.
#[derive(Debug, Clone, PartialEq)]
pub enum TropicalSynthError {
    /// Dimension mismatch (e.g., evaluating a 3-variable polynomial with 2 inputs).
    DimensionMismatch { expected: usize, got: usize },
    /// A polynomial with zero monomials was used where at least one is required.
    EmptyPolynomial,
    /// Invalid morph parameter (must be in [0, 1]).
    InvalidMorphParameter(f64),
    /// Invalid MIDI CC number (must be 0–127).
    InvalidCCNumber(u8),
    /// An empty timbre space (no patches).
    EmptyTimbreSpace,
}

impl fmt::Display for TropicalSynthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionMismatch { expected, got } => {
                write!(f, "dimension mismatch: expected {expected}, got {got}")
            }
            Self::EmptyPolynomial => write!(f, "polynomial has no monomials"),
            Self::InvalidMorphParameter(t) => {
                write!(f, "morph parameter {t} out of range [0, 1]")
            }
            Self::InvalidCCNumber(cc) => write!(f, "invalid MIDI CC number: {cc}"),
            Self::EmptyTimbreSpace => write!(f, "timbre space has no patches"),
        }
    }
}

impl std::error::Error for TropicalSynthError {}
