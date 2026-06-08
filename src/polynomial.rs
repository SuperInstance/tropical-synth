use crate::TropicalSynthError;

/// A tropical monomial: `c ⊗ x₁^a₁ ⊗ x₂^a₂ ⊗ …`
///
/// In max-plus algebra, this evaluates to `c + a₁·x₁ + a₂·x₂ + …`.
#[derive(Debug, Clone, PartialEq)]
pub struct TropicalMonomial {
    pub coefficient: f64,
    pub exponents: Vec<u32>,
}

impl TropicalMonomial {
    /// Create a new tropical monomial with the given coefficient and exponents.
    pub fn new(coefficient: f64, exponents: Vec<u32>) -> Self {
        Self { coefficient, exponents }
    }

    /// The number of variables (arity).
    pub fn arity(&self) -> usize {
        self.exponents.len()
    }

    /// Evaluate the monomial at a point.
    pub fn evaluate(&self, point: &[f64]) -> Result<f64, TropicalSynthError> {
        if point.len() != self.arity() {
            return Err(TropicalSynthError::DimensionMismatch {
                expected: self.arity(),
                got: point.len(),
            });
        }
        let val = self.coefficient
            + self.exponents.iter().zip(point.iter()).map(|(e, x)| *e as f64 * x).sum::<f64>();
        Ok(val)
    }
}

/// A tropical polynomial: the max of tropical monomials.
#[derive(Debug, Clone, PartialEq)]
pub struct TropicalPolynomial {
    pub monomials: Vec<TropicalMonomial>,
}

impl TropicalPolynomial {
    /// Create a new tropical polynomial from monomials.
    pub fn new(monomials: Vec<TropicalMonomial>) -> Self {
        Self { monomials }
    }

    /// The arity (number of variables). Returns 0 if empty.
    pub fn arity(&self) -> usize {
        self.monomials.first().map(|m| m.arity()).unwrap_or(0)
    }

    /// Evaluate the polynomial at a point (takes the max of all monomials).
    pub fn evaluate(&self, point: &[f64]) -> Result<f64, TropicalSynthError> {
        if self.monomials.is_empty() {
            return Err(TropicalSynthError::EmptyPolynomial);
        }
        let vals: Result<Vec<f64>, _> = self
            .monomials
            .iter()
            .map(|m| m.evaluate(point))
            .collect();
        Ok(vals?.into_iter().fold(f64::NEG_INFINITY, f64::max))
    }

    /// Return the index of the active monomial (the one achieving the max) at a point.
    pub fn active_monomial(&self, point: &[f64]) -> Result<usize, TropicalSynthError> {
        if self.monomials.is_empty() {
            return Err(TropicalSynthError::EmptyPolynomial);
        }
        let mut best_idx = 0;
        let mut best_val = f64::NEG_INFINITY;
        for (i, m) in self.monomials.iter().enumerate() {
            let v = m.evaluate(point)?;
            if v > best_val {
                best_val = v;
                best_idx = i;
            }
        }
        Ok(best_idx)
    }

    /// The active regions (which monomial dominates) aren't computed analytically here,
    /// but we can sample a grid and classify each point.
    #[allow(clippy::type_complexity)]
    pub fn classify_grid(&self, ranges: &[(f64, f64)], resolution: usize) -> Result<Vec<Vec<(Vec<f64>, usize)>>, TropicalSynthError> {
        // Returns per-monomial lists of (point, monomial_index)
        let mut regions: Vec<Vec<(Vec<f64>, usize)>> = vec![Vec::new(); self.monomials.len()];
        let dim = ranges.len();
        if dim == 0 || resolution == 0 {
            return Ok(regions);
        }
        // Simple grid sampling for 1D or 2D
        let steps: Vec<Vec<f64>> = ranges
            .iter()
            .map(|(lo, hi)| {
                let step = (hi - lo) / (resolution as f64 - 1.0);
                (0..resolution).map(|i| lo + step * i as f64).collect()
            })
            .collect();

        match dim {
            1 => {
                for x in &steps[0] {
                    let idx = self.active_monomial(&[*x])?;
                    regions[idx].push((vec![*x], idx));
                }
            }
            2 => {
                for x in &steps[0] {
                    for y in &steps[1] {
                        let idx = self.active_monomial(&[*x, *y])?;
                        regions[idx].push((vec![*x, *y], idx));
                    }
                }
            }
            _ => {
                // For higher dimensions, skip grid — return empty regions
            }
        }
        Ok(regions)
    }
}

/// A vertex of the Newton polytope (integer lattice point with exponent vector).
#[derive(Debug, Clone, PartialEq)]
pub struct PolytopeVertex {
    pub exponents: Vec<u32>,
    pub coefficient: f64,
}

/// The Newton polytope of a tropical polynomial.
#[derive(Debug, Clone)]
pub struct NewtonPolytope {
    pub vertices: Vec<PolytopeVertex>,
}

impl NewtonPolytope {
    /// Compute the Newton polytope vertices from a tropical polynomial.
    /// Each monomial contributes a vertex at its exponent vector.
    pub fn from_polynomial(poly: &TropicalPolynomial) -> Self {
        let vertices = poly
            .monomials
            .iter()
            .map(|m| PolytopeVertex {
                exponents: m.exponents.clone(),
                coefficient: m.coefficient,
            })
            .collect();
        Self { vertices }
    }

    /// Number of vertices.
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// Is the polytope empty?
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monomial_evaluate() {
        let m = TropicalMonomial::new(1.0, vec![2, 0]);
        assert_eq!(m.evaluate(&[3.0, 99.0]).unwrap(), 1.0 + 2.0 * 3.0);
    }

    #[test]
    fn monomial_dimension_mismatch() {
        let m = TropicalMonomial::new(0.0, vec![1, 1]);
        assert!(m.evaluate(&[1.0]).is_err());
    }

    #[test]
    fn polynomial_evaluate_max() {
        // max(0 + 2x, 0 + 2y, 3 + x + y) at (1,2) → max(2, 4, 6) = 6
        let poly = TropicalPolynomial::new(vec![
            TropicalMonomial::new(0.0, vec![2, 0]),
            TropicalMonomial::new(0.0, vec![0, 2]),
            TropicalMonomial::new(3.0, vec![1, 1]),
        ]);
        assert_eq!(poly.evaluate(&[1.0, 2.0]).unwrap(), 6.0);
    }

    #[test]
    fn polynomial_empty() {
        let poly = TropicalPolynomial::new(vec![]);
        assert!(poly.evaluate(&[]).is_err());
    }

    #[test]
    fn active_monomial_index() {
        let poly = TropicalPolynomial::new(vec![
            TropicalMonomial::new(0.0, vec![2, 0]), // 0: 2x
            TropicalMonomial::new(0.0, vec![0, 2]), // 1: 2y
            TropicalMonomial::new(3.0, vec![1, 1]), // 2: 3+x+y
        ]);
        // At (1,2): vals = [2, 4, 6] → active = 2
        assert_eq!(poly.active_monomial(&[1.0, 2.0]).unwrap(), 2);
        // At (10,0): vals = [20, 0, 13] → active = 0
        assert_eq!(poly.active_monomial(&[10.0, 0.0]).unwrap(), 0);
    }

    #[test]
    fn newton_polytope_vertices() {
        let poly = TropicalPolynomial::new(vec![
            TropicalMonomial::new(0.0, vec![2, 0]),
            TropicalMonomial::new(1.0, vec![0, 2]),
        ]);
        let np = NewtonPolytope::from_polynomial(&poly);
        assert_eq!(np.len(), 2);
        assert_eq!(np.vertices[0].exponents, vec![2, 0]);
        assert_eq!(np.vertices[1].coefficient, 1.0);
    }

    #[test]
    fn grid_classify_1d() {
        // max(0, x) — active region changes at x=0
        let poly = TropicalPolynomial::new(vec![
            TropicalMonomial::new(0.0, vec![0]),
            TropicalMonomial::new(0.0, vec![1]),
        ]);
        let regions = poly.classify_grid(&[(-2.0, 2.0)], 5).unwrap();
        assert!(!regions[0].is_empty()); // constant monomial active for x<0
        assert!(!regions[1].is_empty()); // linear monomial active for x>0
    }
}
