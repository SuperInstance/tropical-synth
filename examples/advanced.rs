//! Advanced: multi-patch morphing, grid classification, and custom timbre spaces.
//!
//! Run with: cargo run --example advanced

use tropical_synth::{
    Tropical, TropicalPolynomial, TropicalMonomial, NewtonPolytope,
    SynthPatch, MorphPath, TimbreSpace, MidiCCMapper,
    OscillatorParams, OscillatorWaveform, FilterParams, FilterType,
    EnvelopeParams, EffectParams, EffectKind,
};

fn main() {
    println!("=== Advanced Tropical Synth ===\n");

    // ── 1. Semiring properties ──
    println!("1. Semiring properties");
    let a = Tropical(1.0);
    let b = Tropical(2.0);
    let c = Tropical(3.0);
    println!("   Associativity of ⊕: {} ⊕ ({}) ⊕ {}) = {}",
        a.0, b.0, c.0, ((a + b) + c).0);
    println!("   Distributivity: {} ⊗ ({} ⊕ {}) = {} ⊗ {} ⊕ {} ⊗ {}",
        a.0, b.0, c.0, a.0, b.0, a.0, c.0);
    let left = a * (b + c);
    let right = a * b + a * c;
    println!("   {} = {} ✓", left.0, right.0);

    // ── 2. Higher-degree polynomial ──
    println!("\n2. Higher-degree polynomial (4 monomials)");
    let poly = TropicalPolynomial::new(vec![
        TropicalMonomial::new(0.0, vec![3, 0]),   // 3x
        TropicalMonomial::new(0.0, vec![0, 3]),   // 3y
        TropicalMonomial::new(1.0, vec![2, 1]),   // 1+2x+y
        TropicalMonomial::new(2.0, vec![1, 2]),   // 2+x+2y
    ]);

    let np = NewtonPolytope::from_polynomial(&poly);
    println!("   {} vertices in Newton polytope", np.len());
    for v in &np.vertices {
        println!("     exponents={:?}, coefficient={:.1}", v.exponents, v.coefficient);
    }

    // ── 3. Active region classification ──
    println!("\n3. Active region classification");
    let regions = poly.classify_grid(&[(-5.0, 5.0), (-5.0, 5.0)], 11).unwrap();
    for (i, region) in regions.iter().enumerate() {
        println!("   Monomial {}: {} active points", i, region.len());
    }

    // ── 4. Custom patches with effects ──
    println!("\n4. Custom patch with effects");
    let pad = SynthPatch {
        oscillators: vec![
            OscillatorParams {
                waveform: OscillatorWaveform::Saw,
                frequency_ratio: 1.0,
                amplitude: 0.6,
                detune_cents: 0.0,
            },
            OscillatorParams {
                waveform: OscillatorWaveform::Saw,
                frequency_ratio: 1.001,
                amplitude: 0.4,
                detune_cents: 7.0,
            },
        ],
        filter: FilterParams {
            filter_type: FilterType::LowPass,
            cutoff_hz: 2000.0,
            resonance: 0.3,
            env_amount: 0.5,
        },
        envelope: EnvelopeParams {
            attack_s: 0.5,
            decay_s: 0.3,
            sustain: 0.6,
            release_s: 1.0,
        },
        effects: vec![
            EffectParams::new(EffectKind::Reverb, 0.4),
            EffectParams::new(EffectKind::Chorus, 0.3),
        ],
    };
    println!("   Oscillators: {}", pad.oscillators.len());
    println!("   Effects: {} ({:?}, {:?})",
        pad.effects.len(), pad.effects[0].kind, pad.effects[1].kind);

    // ── 5. Morphing with effects ──
    println!("\n5. Morph path with effects");
    let lead = SynthPatch {
        oscillators: vec![OscillatorParams {
            waveform: OscillatorWaveform::Square,
            frequency_ratio: 1.0,
            amplitude: 0.9,
            detune_cents: 0.0,
        }],
        filter: FilterParams {
            filter_type: FilterType::LowPass,
            cutoff_hz: 8000.0,
            resonance: 0.7,
            env_amount: 0.0,
        },
        envelope: EnvelopeParams {
            attack_s: 0.01,
            decay_s: 0.1,
            sustain: 0.8,
            release_s: 0.2,
        },
        effects: vec![EffectParams::new(EffectKind::Distortion, 0.2)],
    };

    let mut morph = MorphPath::new(pad.clone(), lead.clone());
    for t in [0.0, 0.5, 1.0] {
        morph.set_t(t).unwrap();
        let p = morph.current_patch();
        println!("   t={:.1}: cutoff={:.0}Hz, attack={:.3}s, oscs={}",
            t, p.filter.cutoff_hz, p.envelope.attack_s, p.oscillators.len());
    }

    // Reverse morph
    let rev = morph.reverse();
    println!("   Reversed: t={:.1}", rev.t());

    // ── 6. Full MIDI mapping ──
    println!("\n6. Full MIDI mapping");
    let mapper = MidiCCMapper::new(1); // channel 1
    let msgs = mapper.map_patch(&pad).unwrap();
    for msg in &msgs {
        println!("   CC{:03} = {:03} (ch{})", msg.cc, msg.value, msg.channel);
    }

    // ── 7. Timbre space with 5 monomials ──
    println!("\n7. Large timbre space");
    let big_poly = TropicalPolynomial::new(vec![
        TropicalMonomial::new(0.0, vec![4, 0]),
        TropicalMonomial::new(0.0, vec![0, 4]),
        TropicalMonomial::new(1.0, vec![3, 1]),
        TropicalMonomial::new(1.0, vec![1, 3]),
        TropicalMonomial::new(3.0, vec![2, 2]),
    ]);
    let space = TimbreSpace::new(big_poly).unwrap();
    println!("   {} patches in 5-vertex space", space.len());
    for (i, p) in space.patches().iter().enumerate() {
        println!("     Patch {}: {} osc, cutoff={:.0}Hz, {:?}",
            i, p.oscillators.len(), p.filter.cutoff_hz, p.oscillators[0].waveform);
    }
}
