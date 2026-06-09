# tropical-synth

**Sound design via tropical geometry — draw a tropical curve, hear the sound.**

## The Problem

Synthesizer patch design is an art form. You twiddle knobs until something sounds right, but there's no mathematical framework that tells you *why* a patch works or *how* to smoothly morph between two patches. The space of possible sounds is vast and unstructured.

## The Key Insight

**Tropical geometry replaces addition with max and multiplication with addition.** This simple change turns smooth algebraic curves into piecewise-linear ones with sharp corners and edges. And those corners and edges map perfectly to synthesizer parameters:

- Each **vertex** of a tropical polynomial → a distinct synth patch
- Each **edge** between vertices → a smooth morphing path between patches
- The **Newton polytope** (the convex hull of exponent vectors) → the entire timbre space

The "aha moment" is that **tropical polynomials are the same as ReLU neural networks**. A tropical polynomial max(c₁ + a₁·x, c₂ + a₂·x, ...) is exactly a single-layer ReLU network with specific weight constraints. This means:
- Sound design = network architecture design
- Patch morphing = interpolation along tropical edges
- Timbre space = Newton polytope of the tropical polynomial

This crate implements:
- The **tropical semiring** (max-plus algebra) with proper `Add` and `Mul` operators
- **Tropical polynomials** with evaluation and active-region classification
- **Newton polytopes** (vertices as synth patches)
- **SynthPatch** — full synthesizer parameters derived from tropical vertices
- **MorphPath** — linear interpolation between patches along tropical edges
- **TimbreSpace** — the complete navigable sound-design space
- **MIDI CC mapping** — convert any patch to standard MIDI control messages

## Architecture

```
    ┌──────────────────────────────────────────────┐
    │            TimbreSpace                        │
    │ (polynomial → patches → morph paths)          │
    └───────────────┬──────────────────────────────┘
                    │
    ┌───────────────▼──────────────────────────────┐
    │          TropicalPolynomial                   │
    │ (max of tropical monomials)                   │
    │ evaluate(), active_monomial(), classify_grid()│
    └───────────────┬──────────────────────────────┘
                    │
         ┌──────────┴──────────┐
         │                     │
┌────────▼────────┐  ┌────────▼────────┐
│ Tropical        │  │ Newton          │
│ Monomial        │  │ Polytope        │
│ (c ⊗ x₁^a₁ ⊗ …)│  │ (vertices =     │
│                 │  │  patches)       │
└─────────────────┘  └────────┬────────┘
                              │
                    ┌─────────▼─────────┐
                    │ SynthPatch        │
                    │ (oscillators,     │
                    │  filter, envelope,│
                    │  effects)         │
                    └─────────┬─────────┘
                              │
              ┌───────────────┼───────────────┐
              │               │               │
     ┌────────▼──────┐ ┌─────▼──────┐ ┌──────▼──────┐
     │  MorphPath    │ │  MIDI CC   │ │  Patch      │
     │  (lerp at t)  │ │  Mapper    │ │  from_vertex│
     └───────────────┘ └────────────┘ └─────────────┘
```

### Module Overview

| Module | Purpose |
|--------|---------|
| `semiring` | The tropical semiring: `Tropical(f64)` with max-plus algebra |
| `polynomial` | Tropical monomials, polynomials, Newton polytopes |
| `patch` | SynthPatch with oscillators, filter, envelope, effects |
| `morph` | MorphPath — linear interpolation between patches |
| `timbre` | TimbreSpace — the full navigable sound space |
| `midi` | MIDI CC mapping from patches to control messages |
| `error` | Error types |

## The Math: Tropical Geometry

### The Tropical Semiring (ℝ ∪ {−∞}, max, +)

| Operation | Standard | Tropical |
|-----------|----------|----------|
| Addition (⊕) | a + b | max(a, b) |
| Multiplication (⊗) | a × b | a + b |
| Zero (additive identity) | 0 | −∞ |
| One (multiplicative identity) | 1 | 0 |
| Power: xⁿ | x × x × ... × x | x + x + ... + x = n·x |

```rust
use tropical_synth::Tropical;

let a = Tropical(3.0);
let b = Tropical(5.0);

// Tropical addition = max
assert_eq!(a + b, Tropical(5.0));

// Tropical multiplication = addition
assert_eq!(a * b, Tropical(8.0));

// Identities
assert_eq!(Tropical::ZERO + a, a);  // -∞ ⊕ a = a
assert_eq!(Tropical::ONE * a, a);   // 0 ⊗ a = a

// Power = scalar multiplication
assert_eq!(Tropical(3.0).pow(4), Tropical(12.0));
```

### Tropical Polynomials

A tropical polynomial is the max of tropical monomials:

```
p(x) = max(c₁ + a₁·x₁ + b₁·x₂, c₂ + a₂·x₁ + b₂·x₂, ...)
```

Each monomial is a hyperplane in the input space. The polynomial's graph is a **piecewise-linear convex function** whose "corners" are where the active monomial changes.

```rust
use tropical_synth::{TropicalPolynomial, TropicalMonomial};

let poly = TropicalPolynomial::new(vec![
    TropicalMonomial::new(0.0, vec![2, 0]),  // 2x₁
    TropicalMonomial::new(0.0, vec![0, 2]),  // 2x₂
    TropicalMonomial::new(3.0, vec![1, 1]),  // 3 + x₁ + x₂
]);

// Evaluate at a point
let val = poly.evaluate(&[1.0, 2.0]).unwrap();
// max(2·1, 2·2, 3+1+2) = max(2, 4, 6) = 6

// Which monomial is active?
let idx = poly.active_monomial(&[10.0, 0.0]).unwrap();
// At (10, 0): 2·10=20 wins → monomial 0
```

### Newton Polytopes

The **Newton polytope** of a tropical polynomial is the convex hull of the exponent vectors. Each vertex of this polytope corresponds to a monomial, and in this crate, to a **synth patch**.

The edges of the Newton polytope are the **morph paths** — smooth transitions between patches where one monomial hands off to another.

### The Connection to ReLU Networks

A tropical polynomial `max(c₁ + a₁·x, ..., cₙ + aₙ·x)` is exactly a single-layer ReLU network:

```
y = max_i (cᵢ + aᵢ · x) = max(W·x + b)
```

This means:
- Training a ReLU network = fitting a tropical polynomial
- Network width = number of monomials
- Network depth = degree of the polynomial
- **Sound design = network architecture design**

## Quick Start

```rust
use tropical_synth::{
    TropicalPolynomial, TropicalMonomial, TimbreSpace, MorphPath
};

// Define a tropical polynomial (3 patches in 2D parameter space)
let poly = TropicalPolynomial::new(vec![
    TropicalMonomial::new(0.0, vec![2, 0]),
    TropicalMonomial::new(0.0, vec![0, 2]),
    TropicalMonomial::new(3.0, vec![1, 1]),
]);

// Build the timbre space
let space = TimbreSpace::new(poly).unwrap();
println!("{} patches", space.len());

// Get the active patch at a parameter point
let patch = space.active_patch(&[5.0, 0.0]).unwrap();
println!("Filter cutoff: {:.0} Hz", patch.filter.cutoff_hz);

// Morph between two patches
let mut morph = space.morph(0, 1).unwrap();
morph.set_t(0.5).unwrap();
let mid = morph.current_patch();
println!("Midpoint cutoff: {:.0} Hz", mid.filter.cutoff_hz);
```

## SynthPatch: Vertex → Sound

Each vertex of the Newton polytope maps to a full synth patch:

```rust
use tropical_synth::SynthPatch;

// From a tropical vertex (exponents, coefficient)
let patch = SynthPatch::from_vertex(&[2, 1], 1.5);
// - Exponent sum → number of oscillators (1-3)
// - First exponent → waveform (saw, square, triangle, sine, noise)
// - Coefficient → filter cutoff (logarithmic)
// - Second exponent → envelope attack

println!("Oscillators: {}", patch.oscillators.len());
println!("Waveform: {:?}", patch.oscillators[0].waveform);
println!("Cutoff: {:.0} Hz", patch.filter.cutoff_hz);
println!("Attack: {:.3}s", patch.envelope.attack_s);
```

## MIDI CC Mapping

Any patch can be converted to standard MIDI CC messages:

```rust
use tropical_synth::{SynthPatch, MidiCCMapper};

let mapper = MidiCCMapper::new(0); // channel 0
let patch = SynthPatch::simple();
let messages = mapper.map_patch(&patch).unwrap();

for msg in &messages {
    println!("CC{} = {} (ch{})", msg.cc, msg.value, msg.channel);
}
// Output: CC7 (volume), CC74 (brightness), CC73 (attack), CC72 (release), etc.
```

Standard CC mappings:
| Parameter | CC Number |
|-----------|-----------|
| Volume | 7 |
| Brightness (filter cutoff) | 74 |
| Filter resonance | 71 |
| Attack | 73 |
| Release | 72 |
| Decay | 75 |
| Sustain | 70 |
| Reverb | 91 |
| Chorus | 93 |
| Detune | 94 |

## Performance

- **Tropical evaluation**: O(n) where n = number of monomials
- **Active monomial**: O(n) linear scan
- **Grid classification**: O(resolution^d) where d = dimensionality
- **Patch morphing**: O(1) — linear interpolation of numeric fields
- **MIDI mapping**: O(1) per patch

All operations are real-time safe — no allocation in the hot path.

## Comparison

| Feature | tropical-synth | Traditional synths | Neural audio |
|---------|---------------|-------------------|-------------|
| Patch structure | Tropical vertices | Knob positions | Network weights |
| Morphing | Tropical edge interpolation | Crossfade | Latent interpolation |
| Space structure | Newton polytope | Unstructured | Latent space |
| Mathematical foundation | Tropical geometry / ReLU networks | None | Deep learning |
| MIDI output | ✅ Built-in | ✅ Native | ❌ |
| Patch from math | ✅ from_vertex() | ❌ | ❌ |

## SuperInstance Ecosystem

`tropical-synth` integrates with:
- `lotka-beats` — species timbre profiles use tropical patches
- `groovemesh-plr` — PLR transitions mapped to tropical morph paths
- `spreadsheet-engine` — tropical polynomials as formula cell inputs
- `noether-guard` — conservation checking for energy-like tropical quantities

## License

MIT
