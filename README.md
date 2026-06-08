# tropical-synth

[![crates.io](https://img.shields.io/crates/v/tropical-synth.svg)](https://crates.io/crates/tropical-synth)
[![docs.rs](https://docs.rs/tropical-synth/badge.svg)](https://docs.rs/tropical-synth)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Sound design via tropical geometry — draw a tropical curve, hear the sound.**

Tropical geometry replaces ordinary addition with `max` and multiplication with `+`
(the **max-plus semiring**). A tropical polynomial like `max(a, b, c) + max(d, e)`
defines a piecewise-linear surface whose vertices and edges form a natural parameter
space for synthesizer patches.

`tropical-synth` maps that surface to audio: **vertices become control points**
(synth patches), **edges become morphing paths** between patches, and the full
tropical polytope is a navigable **timbre space** for sound design.

## Features

- **Tropical semiring** — `Tropical<f64>` with `max`-plus arithmetic (`⊕`, `⊗`)
- **Tropical polynomials** — piecewise-linear functions with monomial terms and
  Newton polytope vertex extraction
- **Synth patches** — `SynthPatch` with oscillator, filter, envelope, and effects
  parameters derived from tropical vertex coordinates
- **Morph paths** — linear interpolation between patches along tropical edges
- **Timbre space** — full navigable sound-design space built from a tropical
  polynomial, with nearest-vertex lookup and interpolation
- **MIDI CC mapping** — `MidiCCMapper` converts patch parameters to MIDI CC
  messages for hardware integration

## Quick Start

```rust
use tropical_synth::{Tropical, TropicalPolynomial, TropicalMonomial};

// Build a tropical polynomial: max(0 + 2x, 1 + 2y, 3 + x + y)
let poly = TropicalPolynomial::new(vec![
    TropicalMonomial::new(0.0, vec![2, 0]),
    TropicalMonomial::new(1.0, vec![0, 2]),
    TropicalMonomial::new(3.0, vec![1, 1]),
]);

// Evaluate at a point
let val = poly.evaluate(&[1.0, 2.0]);

// Extract the Newton polytope vertices
let newton = poly.newton_polytope();
println!("{} vertices in polytope", newton.vertices.len());
```

## Building a Timbre Space

```rust
use tropical_synth::{TimbreSpace, TropicalPolynomial, TropicalMonomial};

let poly = TropicalPolynomial::new(vec![
    TropicalMonomial::new(0.0, vec![2, 0]),
    TropicalMonomial::new(-1.0, vec![0, 2]),
]);

let space = TimbreSpace::from_polynomial(&poly);
let patch = space.patch_at(&[0.5, 0.5]);
```

## Module Overview

| Module | Description |
|---|---|
| `semiring` | `Tropical<T>` — max-plus semiring arithmetic |
| `polynomial` | `TropicalMonomial`, `TropicalPolynomial`, `NewtonPolytope` |
| `patch` | `SynthPatch`, `OscillatorParams`, `FilterParams`, `EnvelopeParams` |
| `morph` | `MorphPath` — interpolation between patches |
| `timbre` | `TimbreSpace` — navigable sound-design space |
| `midi` | `MidiCCMapper` — parameter → MIDI CC conversion |
| `error` | Error types |

## Links

- [Documentation](https://docs.rs/tropical-synth)
- [Repository](https://github.com/nightshift-crates/tropical-synth)
- [Crates.io](https://crates.io/crates/tropical-synth)

## License

MIT
