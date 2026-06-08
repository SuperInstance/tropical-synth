//! # tropical-synth
//!
//! Sound design via tropical geometry — draw a tropical curve, hear the sound.
//!
//! Tropical geometry replaces `+` with `max` and `×` with `+`. A tropical polynomial
//! like `max(a, b, c) + max(d, e)` defines a piecewise-linear surface. This crate
//! maps that surface to synthesizer parameters: vertices become control points,
//! edges become morphing paths.
//!
//! ## Core Concepts
//!
//! - **Tropical semiring** (`max-plus`): addition is `max`, multiplication is `+`
//! - **Tropical polynomial**: piecewise-linear function whose corners are vertices
//! - **SynthPatch**: synthesizer parameters derived from a tropical vertex
//! - **MorphPath**: linear interpolation between patches along tropical edges
//! - **TimbreSpace**: the full sound-design space navigable via tropical geometry
//!
//! ## Example
//!
//! ```rust
//! use tropical_synth::{Tropical, TropicalPolynomial, TropicalMonomial};
//!
//! let poly = TropicalPolynomial::new(vec![
//!     TropicalMonomial::new(0.0, vec![2, 0]),
//!     TropicalMonomial::new(1.0, vec![0, 2]),
//!     TropicalMonomial::new(3.0, vec![1, 1]),
//! ]);
//! let val = poly.evaluate(&[1.0, 2.0]);
//! ```

pub mod error;
pub mod semiring;
pub mod polynomial;
pub mod patch;
pub mod morph;
pub mod timbre;
pub mod midi;

pub use error::TropicalSynthError;
pub use semiring::Tropical;
pub use polynomial::{TropicalMonomial, TropicalPolynomial, NewtonPolytope};
pub use patch::{SynthPatch, OscillatorParams, OscillatorWaveform, FilterParams, FilterType, EnvelopeParams, EffectParams, EffectKind};
pub use morph::MorphPath;
pub use timbre::TimbreSpace;
pub use midi::MidiCCMapper;
