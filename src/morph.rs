use crate::{SynthPatch, TropicalSynthError};

/// Morph between two patches along a tropical edge.
///
/// Tropical line segments are linear in the max-plus world, which means
/// the interpolation between patches is simply piecewise-linear — perfect
/// for smooth synth morphing.
#[derive(Debug, Clone)]
pub struct MorphPath {
    from: SynthPatch,
    to: SynthPatch,
    t: f64,
}

impl MorphPath {
    /// Create a new morph path between two patches.
    pub fn new(from: SynthPatch, to: SynthPatch) -> Self {
        Self { from, to, t: 0.0 }
    }

    /// Set the morph parameter `t ∈ [0, 1]`.
    pub fn set_t(&mut self, t: f64) -> Result<(), TropicalSynthError> {
        if !(0.0..=1.0).contains(&t) {
            return Err(TropicalSynthError::InvalidMorphParameter(t));
        }
        self.t = t;
        Ok(())
    }

    /// Get the current morph parameter.
    pub fn t(&self) -> f64 {
        self.t
    }

    /// Compute the interpolated patch at the current `t`.
    pub fn current_patch(&self) -> SynthPatch {
        self.from.lerp(&self.to, self.t)
    }

    /// Sample the morph path at multiple t values.
    pub fn sample(&self, steps: usize) -> Vec<SynthPatch> {
        (0..=steps)
            .map(|i| {
                let t = i as f64 / steps as f64;
                self.from.lerp(&self.to, t)
            })
            .collect()
    }

    /// The source patch.
    pub fn from_patch(&self) -> &SynthPatch {
        &self.from
    }

    /// The destination patch.
    pub fn to_patch(&self) -> &SynthPatch {
        &self.to
    }

    /// Reverse the morph direction.
    pub fn reverse(&self) -> MorphPath {
        MorphPath {
            from: self.to.clone(),
            to: self.from.clone(),
            t: 1.0 - self.t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{OscillatorParams, FilterParams, EnvelopeParams};

    fn make_patch(cutoff: f64, amp: f64) -> SynthPatch {
        SynthPatch {
            oscillators: vec![OscillatorParams { amplitude: amp, ..Default::default() }],
            filter: FilterParams { cutoff_hz: cutoff, ..Default::default() },
            envelope: EnvelopeParams::default(),
            effects: vec![],
        }
    }

    #[test]
    fn morph_at_zero_is_from() {
        let a = make_patch(200.0, 0.5);
        let b = make_patch(8000.0, 1.0);
        let morph = MorphPath::new(a.clone(), b);
        let p = morph.current_patch();
        assert!((p.filter.cutoff_hz - 200.0).abs() < 1e-9);
    }

    #[test]
    fn morph_at_one_is_to() {
        let a = make_patch(200.0, 0.5);
        let b = make_patch(8000.0, 1.0);
        let mut morph = MorphPath::new(a, b.clone());
        morph.set_t(1.0).unwrap();
        let p = morph.current_patch();
        assert!((p.filter.cutoff_hz - 8000.0).abs() < 1e-9);
    }

    #[test]
    fn invalid_t_rejected() {
        let a = SynthPatch::simple();
        let b = SynthPatch::simple();
        let mut morph = MorphPath::new(a, b);
        assert!(morph.set_t(-0.1).is_err());
        assert!(morph.set_t(1.1).is_err());
    }

    #[test]
    fn sample_count() {
        let a = SynthPatch::simple();
        let b = SynthPatch::simple();
        let morph = MorphPath::new(a, b);
        let samples = morph.sample(4);
        assert_eq!(samples.len(), 5); // 0..=4
    }

    #[test]
    fn reverse_flips() {
        let a = make_patch(100.0, 0.1);
        let b = make_patch(5000.0, 0.9);
        let mut morph = MorphPath::new(a, b);
        morph.set_t(0.3).unwrap();
        let rev = morph.reverse();
        assert!((rev.t() - 0.7).abs() < 1e-9);
    }
}
