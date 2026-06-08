use tropical_synth::{Tropical, TropicalMonomial, TropicalPolynomial, TimbreSpace, MidiCCMapper};

fn main() {
    // Build a tropical polynomial in two variables:
    //   p(x, y) = max(0 + 2x, 1 + 2y, 3 + x + y)
    // This defines a piecewise-linear surface with three "sectors".
    let poly = TropicalPolynomial::new(vec![
        TropicalMonomial::new(0.0, vec![2, 0]), // 2x
        TropicalMonomial::new(1.0, vec![0, 2]), // 1 + 2y
        TropicalMonomial::new(3.0, vec![1, 1]), // 3 + x + y
    ]);

    // Evaluate at a point in parameter space
    let val = poly.evaluate(&[1.0, 2.0]).unwrap();
    println!("p(1, 2) = {} (max of 2, 5, 6 = 6)", val);

    // Find which monomial is active (dominates) at this point
    let active = poly.active_monomial(&[1.0, 2.0]).unwrap();
    println!("Active monomial index: {} (the 3+x+y term)", active);

    // Build a timbre space — each vertex of the Newton polytope becomes a synth patch
    let space = TimbreSpace::new(poly).unwrap();
    println!("Timbre space has {} patches", space.len());

    for (i, patch) in space.patches().iter().enumerate() {
        println!(
            "  Patch {}: osc={}, cutoff={:.0}Hz, attack={:.3}s",
            i,
            patch.oscillators.len(),
            patch.filter.cutoff_hz,
            patch.envelope.attack_s,
        );
    }

    // Find the active patch at a point
    let active_patch = space.active_patch(&[1.0, 2.0]).unwrap();
    println!("\nActive patch at (1,2): cutoff={:.0}Hz", active_patch.filter.cutoff_hz);

    // Morph between two patches
    let morph = space.morph(0, 1).unwrap();
    let mid = morph.sample(4);
    println!("Morph from patch 0 to 1 at t=0.5: cutoff={:.0}Hz", mid[2].filter.cutoff_hz);

    // Map a patch to MIDI CC messages
    let mapper = MidiCCMapper::new(0);
    let msgs = mapper.map_patch(space.patch(0).unwrap()).unwrap();
    println!("\nMIDI CC messages for patch 0:");
    for m in &msgs {
        println!("  CC{} = {}", m.cc, m.value);
    }
}
