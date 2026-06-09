//! Basic tropical-synth usage: create a polynomial, build a timbre space.
//!
//! Run with: cargo run --example basic

use tropical_synth::{Tropical, TropicalPolynomial, TropicalMonomial, SynthPatch};

fn main() {
    // The tropical semiring
    let a = Tropical(3.0);
    let b = Tropical(5.0);
    println!("Tropical arithmetic:");
    println!("  {} ⊕ {} = {} (max)", a.0, b.0, (a + b).0);
    println!("  {} ⊗ {} = {} (add)", a.0, b.0, (a * b).0);
    println!("  {}^4 = {} (scalar mul)", a.0, a.pow(4).0);

    // A simple tropical polynomial: max(2x, 2y, 3+x+y)
    let poly = TropicalPolynomial::new(vec![
        TropicalMonomial::new(0.0, vec![2, 0]),
        TropicalMonomial::new(0.0, vec![0, 2]),
        TropicalMonomial::new(3.0, vec![1, 1]),
    ]);

    let val = poly.evaluate(&[1.0, 2.0]).unwrap();
    println!("\nPolynomial at (1, 2): {:.1}", val);

    let active = poly.active_monomial(&[10.0, 0.0]).unwrap();
    println!("Active monomial at (10, 0): #{}", active);

    // A synth patch from a tropical vertex
    let patch = SynthPatch::from_vertex(&[2, 1], 1.5);
    println!("\nPatch from vertex [2,1], c=1.5:");
    println!("  Oscillators: {}", patch.oscillators.len());
    println!("  Waveform: {:?}", patch.oscillators[0].waveform);
    println!("  Cutoff: {:.0} Hz", patch.filter.cutoff_hz);
}
