# tropical-synth

[![crates.io](https://img.shields.io/crates/v/tropical-synth.svg)](https://crates.io/crates/tropical-synth)
[![docs.rs](https://docs.rs/tropical-synth/badge.svg)](https://docs.rs/tropical-synth)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## The Idea

In the tropical semiring, addition is `max` and multiplication is `+`. This isn't just notation — it fundamentally changes what a polynomial looks like. A tropical polynomial like:

```
max(2x + 1, 3y, x + y + 4)
```

is a **piecewise-linear surface** whose "corners" form a convex polytope (the Newton polytope). These corners are where the active term changes — where two monomials are equal.

Here's the insight for sound design: **that piecewise-linear surface is a natural synthesizer parameter space.** Each vertex of the Newton polytope is a distinct patch. Each edge is a morphing path. Navigate the surface and you traverse every possible sound the polynomial can produce, with guaranteed smooth transitions.

## Why This Works

Standard synthesizer parameter spaces are arbitrary — 128 knobs with no geometric structure. You twist knobs and hope. Tropical geometry gives you a parameter space with **intrinsic geometry**: nearby points sound similar because they share active monomials, and the boundaries between regions are exactly where the character changes.

The max-plus semiring also has a deep connection to neural networks: a tropical rational function `max(f₁,...,fₙ) - max(g₁,...,gₘ)` is mathematically equivalent to a two-layer ReLU network. So tropical-synth isn't just a quirky math trick — it's exploring the same parameter space as deep learning, but with the algebra fully exposed.

## How To Use It

Build a tropical polynomial from monomials, then extract the timbre space:

```rust
use tropical_synth::{TropicalPolynomial, TropicalMonomial, TimbreSpace};

// Each monomial: coefficient + exponents
// This defines a 2D parameter space with 3 regions
let poly = TropicalPolynomial::new(vec![
    TropicalMonomial::new(0.0, vec![2, 0]),   // region 1: bright
    TropicalMonomial::new(1.0, vec![0, 2]),   // region 2: warm  
    TropicalMonomial::new(3.0, vec![1, 1]),   // region 3: balanced
]);

// The Newton polytope tells you where the vertices are
let newton = poly.newton_polytope();
println!("{} vertices = {} distinct patches", newton.vertices.len(), newton.vertices.len());

// Build the navigable timbre space
let space = TimbreSpace::from_polynomial(&poly);

// Query any point in parameter space → get a synthesizer patch
let patch = space.patch_at(&[0.5, 0.5]);
println!("Oscillator: {:?}", patch.oscillator.waveform);
println!("Filter cutoff: {:.1} Hz", patch.filter.cutoff);
```

## SynthPatch Structure

Each vertex of the Newton polytope maps to a full synthesizer voice:

```
SynthPatch
├── oscillator: waveform (Sine/Saw/Square/Triangle/Noise), detune, pulse_width
├── filter: type (LowPass/HighPass/BandPass/Notch), cutoff, resonance
├── envelope: attack, decay, sustain, release
└── effects: kind (Reverb/Delay/Chorus/Distortion), mix, parameter
```

The mapping from tropical coordinates to synth parameters uses the monomial exponents as dimensional weights and the coefficient as an offset. Higher-dimensional polynomials give more nuanced parameter spaces.

## Morphing Between Patches

Edges of the Newton polytope define natural morphing paths:

```rust
use tropical_synth::MorphPath;

// Morph between two patches over 128 steps (MIDI resolution)
let morph = MorphPath::new(&patch_a, &patch_b, 128);
for i in 0..128 {
    let interpolated = morph.at(i);
    // Send to synthesizer
}
```

The morphing is linear in tropical space, which means it respects the piecewise-linear geometry — you never pass through a parameter combination that isn't "on the surface."

## MIDI Integration

```rust
use tropical_synth::MidiCCMapper;

let mapper = MidiCCMapper::new();
let cc_messages = mapper.patch_to_cc(&patch);
for (cc, value) in cc_messages {
    // Send MIDI CC message on your DAW/controller
}
```

## Module Map

| Module | What it does |
|---|---|
| `semiring` | `Tropical<T>` — the max-plus arithmetic. `a ⊕ b = max(a,b)`, `a ⊗ b = a+b` |
| `polynomial` | `TropicalMonomial`, `TropicalPolynomial`, `NewtonPolytope` — build and evaluate tropical surfaces |
| `patch` | `SynthPatch` + oscillator/filter/envelope/effects parameters |
| `morph` | `MorphPath` — interpolate between patches along polytope edges |
| `timbre` | `TimbreSpace` — the full navigable sound-design surface |
| `midi` | `MidiCCMapper` — convert patches to MIDI CC messages |

## Prior Art

The connection between tropical geometry and sound design was explored by Sancristoforo (2018) in Max/MSP, but this is the first general-purpose library implementation. The tropical-ReLU equivalence is due to Zhang et al. (2018).

## Links

- [Documentation](https://docs.rs/tropical-synth)
- [Repository](https://github.com/SuperInstance/tropical-synth)
- [crates.io](https://crates.io/crates/tropical-synth)

## License

MIT
