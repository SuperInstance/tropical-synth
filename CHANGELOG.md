# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-08

### Added
- Tropical semiring (`max-plus`) with `Tropical` type and operator overloads
- `TropicalPolynomial` and `TropicalMonomial` with evaluation and active-region classification
- `NewtonPolytope` vertex extraction from tropical polynomials
- `SynthPatch` parameter mapping from tropical vertex coordinates to synth controls
- `MorphPath` for piecewise-linear interpolation between patches
- `TimbreSpace` for navigating the full sound-design space via tropical geometry
- `MidiCCMapper` for converting patches to standard MIDI CC messages
- Comprehensive test suite with algebraic property checks
