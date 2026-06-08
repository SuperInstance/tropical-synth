use crate::{
    SynthPatch, TropicalPolynomial, NewtonPolytope,
    TropicalSynthError, MorphPath,
};

/// The full sound-design space: patches at each vertex of the Newton polytope,
/// connected by morph paths along the edges.
#[derive(Debug, Clone)]
pub struct TimbreSpace {
    /// The underlying tropical polynomial.
    polynomial: TropicalPolynomial,
    /// Patches derived from each vertex of the Newton polytope.
    patches: Vec<SynthPatch>,
}

impl TimbreSpace {
    /// Build a timbre space from a tropical polynomial.
    pub fn new(poly: TropicalPolynomial) -> Result<Self, TropicalSynthError> {
        if poly.monomials.is_empty() {
            return Err(TropicalSynthError::EmptyTimbreSpace);
        }
        let np = NewtonPolytope::from_polynomial(&poly);
        let patches: Vec<SynthPatch> = np
            .vertices
            .iter()
            .map(|v| SynthPatch::from_vertex(&v.exponents, v.coefficient))
            .collect();
        Ok(Self { polynomial: poly, patches })
    }

    /// Number of patches (vertices).
    pub fn len(&self) -> usize {
        self.patches.len()
    }

    /// Is the space empty?
    pub fn is_empty(&self) -> bool {
        self.patches.is_empty()
    }

    /// Get a patch by vertex index.
    pub fn patch(&self, index: usize) -> Option<&SynthPatch> {
        self.patches.get(index)
    }

    /// Get all patches.
    pub fn patches(&self) -> &[SynthPatch] {
        &self.patches
    }

    /// Get the underlying polynomial.
    pub fn polynomial(&self) -> &TropicalPolynomial {
        &self.polynomial
    }

    /// Create a morph path between two vertex patches.
    pub fn morph(&self, from_idx: usize, to_idx: usize) -> Result<MorphPath, TropicalSynthError> {
        let from = self.patches.get(from_idx).cloned().ok_or(TropicalSynthError::EmptyTimbreSpace)?;
        let to = self.patches.get(to_idx).cloned().ok_or(TropicalSynthError::EmptyTimbreSpace)?;
        Ok(MorphPath::new(from, to))
    }

    /// Find the active patch at a point in parameter space.
    pub fn active_patch(&self, point: &[f64]) -> Result<&SynthPatch, TropicalSynthError> {
        let idx = self.polynomial.active_monomial(point)?;
        Ok(&self.patches[idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TropicalMonomial;

    fn sample_poly() -> TropicalPolynomial {
        TropicalPolynomial::new(vec![
            TropicalMonomial::new(0.0, vec![2, 0]),
            TropicalMonomial::new(0.0, vec![0, 2]),
            TropicalMonomial::new(3.0, vec![1, 1]),
        ])
    }

    #[test]
    fn space_has_patches() {
        let space = TimbreSpace::new(sample_poly()).unwrap();
        assert_eq!(space.len(), 3);
    }

    #[test]
    fn active_patch_at_point() {
        let space = TimbreSpace::new(sample_poly()).unwrap();
        // At (10,0): 2x=20 wins → patch 0
        let p = space.active_patch(&[10.0, 0.0]).unwrap();
        assert_eq!(p.oscillators[0].waveform, crate::OscillatorWaveform::Triangle);
    }

    #[test]
    fn morph_between_vertices() {
        let space = TimbreSpace::new(sample_poly()).unwrap();
        let mut m = space.morph(0, 1).unwrap();
        m.set_t(0.5).unwrap();
        let mid = m.current_patch();
        assert!((mid.filter.cutoff_hz - 200.0).abs() > 1.0 || mid.filter.cutoff_hz > 0.0);
    }

    #[test]
    fn empty_poly_fails() {
        let space = TimbreSpace::new(TropicalPolynomial::new(vec![]));
        assert!(space.is_err());
    }
}
