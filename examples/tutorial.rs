//! Tutorial: build and navigate a complete timbre space.
//!
//! Run with: cargo run --example tutorial

use tropical_synth::{
    TropicalPolynomial, TropicalMonomial, TimbreSpace, MorphPath,
    SynthPatch, MidiCCMapper,
};

fn main() {
    println!("=== Tropical Synth Tutorial ===\n");

    // Step 1: Define a tropical polynomial
    println!("Step 1: Define polynomial");
    let poly = TropicalPolynomial::new(vec![
        TropicalMonomial::new(0.0, vec![2, 0]),  // warm bass
        TropicalMonomial::new(0.0, vec![0, 2]),  // bright lead
        TropicalMonomial::new(3.0, vec![1, 1]),  // mid pad
    ]);
    println!("  {} monomials, arity = {}", poly.monomials.len(), poly.arity());

    // Step 2: Build timbre space
    println!("\nStep 2: Build timbre space");
    let space = TimbreSpace::new(poly).unwrap();
    println!("  {} patches (vertices)", space.len());

    // Step 3: Inspect patches
    println!("\nStep 3: Patches at each vertex");
    for (i, patch) in space.patches().iter().enumerate() {
        println!("  Vertex {}: {} osc(s), cutoff={:.0}Hz, waveform={:?}",
            i, patch.oscillators.len(), patch.filter.cutoff_hz,
            patch.oscillators[0].waveform);
    }

    // Step 4: Find active patch at different points
    println!("\nStep 4: Active patches");
    let points = [(1.0, 0.0), (0.0, 5.0), (3.0, 3.0)];
    for (x, y) in points {
        let patch = space.active_patch(&[x, y]).unwrap();
        println!("  ({}, {}): cutoff={:.0}Hz", x, y, patch.filter.cutoff_hz);
    }

    // Step 5: Morph between patches
    println!("\nStep 5: Morph between vertex 0 and 1");
    let mut morph = space.morph(0, 1).unwrap();
    for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
        morph.set_t(t).unwrap();
        let p = morph.current_patch();
        println!("  t={:.2}: cutoff={:.0}Hz, amp={:.2}",
            t, p.filter.cutoff_hz, p.oscillators[0].amplitude);
    }

    // Step 6: Sample the morph path
    println!("\nStep 6: Sample morph path (5 steps)");
    let morph = space.morph(0, 2).unwrap();
    let samples = morph.sample(4);
    for (i, p) in samples.iter().enumerate() {
        println!("  Sample {}: cutoff={:.0}Hz", i, p.filter.cutoff_hz);
    }

    // Step 7: MIDI CC mapping
    println!("\nStep 7: MIDI CC mapping");
    let mapper = MidiCCMapper::new(0);
    let patch = SynthPatch::simple();
    let msgs = mapper.map_patch(&patch).unwrap();
    println!("  {} CC messages:", msgs.len());
    for msg in &msgs {
        println!("    CC{} = {} (ch{})", msg.cc, msg.value, msg.channel);
    }
}
