# Contributing to tropical-synth

Thank you for your interest in tropical geometry for sound design!

## Getting Started

```bash
git clone https://github.com/SuperInstance/tropical-synth.git
cd tropical-synth
cargo test
```

## Architecture Decisions

### Why max-plus (tropical) semiring?

The max-plus semiring has deep connections to:
1. **Piecewise-linear functions** — tropical polynomials are convex PL functions
2. **ReLU neural networks** — a tropical polynomial = a single ReLU layer
3. **Optimal control** — tropical algebra models shortest paths
4. **Algebraic geometry** — tropical varieties approximate classical varieties

For synthesis, the key property is that tropical polynomials produce **piecewise-linear functions with sharp corners** — exactly the kind of parameter curves that produce interesting timbral changes.

### Why map vertices to synth patches?

Each monomial of a tropical polynomial corresponds to a vertex of the Newton polytope. The vertex's exponent vector encodes which coordinate directions dominate, and the coefficient shifts the evaluation. This naturally maps to:
- Exponent → waveform selection (which harmonic content dominates)
- Coefficient → filter cutoff (how much energy passes through)
- Dimension → envelope shape (temporal evolution)

### Why linear interpolation for morphing?

Tropical line segments in max-plus algebra are **piecewise-linear** in standard arithmetic. This means morphing between patches is simply `lerp(a, b, t) = a + t*(b - a)` for each parameter. This produces smooth, predictable transitions — no sudden jumps.

## How to Add a New Waveform

1. Add the variant to `OscillatorWaveform` in `patch.rs`
2. Update `SynthPatch::from_vertex()` to map to the new waveform
3. Update `OscillatorParams::default()` if needed
4. Write tests

## How to Add a New Effect

1. Add the variant to `EffectKind` in `patch.rs`
2. Update the MIDI mapper in `midi.rs` to map it to a CC number
3. Write tests

## How to Add Higher-Order Tropical Operations

The semiring currently supports `+` (max), `*` (add), and `pow` (scalar multiply). For more advanced operations:
- **Tropical matrix multiplication** — for multi-layer tropical networks
- **Tropical eigenvalues** — for fixed-point analysis of timbre spaces
- **Tropical discriminants** — for detecting vertex mergers

## Testing

```bash
cargo test                    # All tests
cargo test test_semiring      # Semiring axiom tests
cargo test test_polynomial    # Polynomial evaluation tests
cargo test test_patch         # Patch generation tests
cargo test test_morph         # Morphing tests
cargo test test_midi          # MIDI mapping tests
```

Key invariants to test:
- Semiring axioms: associativity, commutativity, distributivity, identities
- Evaluation correctness: verify against hand-computed values
- Active monomial consistency: the active monomial's value should equal the polynomial's value
- Morph endpoints: morph at t=0 should equal `from`, t=1 should equal `to`
- MIDI range: all CC values should be 0–127

## Code Style

- `cargo fmt` — no debate
- `cargo clippy` — warnings are errors in CI
- Doc comments on all `pub` items
- Builder pattern where appropriate

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` new features
- `fix:` bug fixes
- `docs:` documentation changes

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
