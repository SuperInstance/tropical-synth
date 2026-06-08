# Contributing to tropical-synth

Thank you for your interest in contributing!

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```

## Running Examples

```bash
cargo run --example basic
```

## Code Quality

Before submitting a PR:

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

## Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes with clear commit messages
4. Ensure CI passes (fmt, clippy, test)
5. Open a pull request against `main`

## Architecture

The crate is structured around tropical geometry → synth parameter mapping:

- **`semiring`** — The max-plus tropical number type
- **`polynomial`** — Tropical polynomials and Newton polytopes
- **`patch`** — Synth parameter structs and vertex-to-patch mapping
- **`morph`** — Interpolation between patches
- **`timbre`** — Full navigable sound-design space
- **`midi`** — MIDI CC output mapping

When adding new features, maintain the clean separation between algebraic structures and musical parameter mapping.
