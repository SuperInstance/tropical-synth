use serde::{Serialize, Deserialize};

/// Oscillator waveform types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OscillatorWaveform {
    Sine,
    Saw,
    Square,
    Triangle,
    Noise,
}

/// Parameters for a single oscillator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OscillatorParams {
    pub waveform: OscillatorWaveform,
    pub frequency_ratio: f64, // relative to base pitch (1.0 = unison)
    pub amplitude: f64,       // 0.0–1.0
    pub detune_cents: f64,    // cents offset
}

impl Default for OscillatorParams {
    fn default() -> Self {
        Self {
            waveform: OscillatorWaveform::Saw,
            frequency_ratio: 1.0,
            amplitude: 0.8,
            detune_cents: 0.0,
        }
    }
}

/// Filter types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    Notch,
}

/// Filter parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterParams {
    pub filter_type: FilterType,
    pub cutoff_hz: f64,   // 20–20000 Hz
    pub resonance: f64,   // 0.0–1.0
    pub env_amount: f64,  // envelope modulation depth
}

impl Default for FilterParams {
    fn default() -> Self {
        Self {
            filter_type: FilterType::LowPass,
            cutoff_hz: 1000.0,
            resonance: 0.5,
            env_amount: 0.0,
        }
    }
}

/// ADSR envelope parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvelopeParams {
    pub attack_s: f64,
    pub decay_s: f64,
    pub sustain: f64,    // 0.0–1.0
    pub release_s: f64,
}

impl Default for EnvelopeParams {
    fn default() -> Self {
        Self {
            attack_s: 0.01,
            decay_s: 0.1,
            sustain: 0.7,
            release_s: 0.3,
        }
    }
}

/// Effect kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectKind {
    Reverb,
    Delay,
    Chorus,
    Distortion,
    Phaser,
}

/// Parameters for a single effect.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectParams {
    pub kind: EffectKind,
    pub mix: f64,     // 0.0–1.0 (dry/wet)
    pub param1: f64,  // effect-specific param (e.g., delay time, reverb size)
}

impl EffectParams {
    pub fn new(kind: EffectKind, mix: f64) -> Self {
        Self { kind, mix, param1: 0.5 }
    }
}

/// A synthesizer patch mapped from a tropical vertex.
///
/// The coordinates of the tropical vertex are translated into synth parameters:
/// - Exponent sums → harmonic content / waveform
/// - Coefficient → filter cutoff
/// - Dimension position → envelope shape
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SynthPatch {
    pub oscillators: Vec<OscillatorParams>,
    pub filter: FilterParams,
    pub envelope: EnvelopeParams,
    pub effects: Vec<EffectParams>,
}

impl SynthPatch {
    /// Create a patch from a tropical vertex described by (exponents, coefficient).
    ///
    /// The mapping:
    /// - First exponent → base waveform (higher = brighter)
    /// - Coefficient → filter cutoff
    /// - Exponent sum → number of oscillators
    /// - Second exponent (if present) → envelope attack
    pub fn from_vertex(exponents: &[u32], coefficient: f64) -> Self {
        let waveform = match exponents.first().copied().unwrap_or(0) % 5 {
            0 => OscillatorWaveform::Saw,
            1 => OscillatorWaveform::Square,
            2 => OscillatorWaveform::Triangle,
            3 => OscillatorWaveform::Sine,
            _ => OscillatorWaveform::Noise,
        };

        let exp_sum: u32 = exponents.iter().sum();
        let num_oscs = ((exp_sum as usize) % 3 + 1).min(3);

        let oscillators: Vec<OscillatorParams> = (0..num_oscs)
            .map(|i| OscillatorParams {
                waveform,
                frequency_ratio: 1.0 + i as f64 * 0.5,
                amplitude: 0.8 / (i + 1) as f64,
                detune_cents: i as f64 * 7.0,
            })
            .collect();

        let cutoff = 200.0 * (2.0_f64).powf(coefficient.clamp(-2.0, 5.0));
        let attack = exponents.get(1).copied().unwrap_or(0) as f64 * 0.05;

        Self {
            oscillators,
            filter: FilterParams {
                filter_type: FilterType::LowPass,
                cutoff_hz: cutoff.clamp(20.0, 20000.0),
                resonance: 0.3 + (coefficient.abs() % 1.0) * 0.5,
                env_amount: 0.2,
            },
            envelope: EnvelopeParams {
                attack_s: attack.clamp(0.001, 5.0),
                decay_s: 0.1,
                sustain: 0.7,
                release_s: 0.3,
            },
            effects: vec![],
        }
    }

    /// A simple default patch.
    pub fn simple() -> Self {
        Self {
            oscillators: vec![OscillatorParams::default()],
            filter: FilterParams::default(),
            envelope: EnvelopeParams::default(),
            effects: vec![],
        }
    }

    /// Linearly interpolate all numeric fields toward another patch.
    pub fn lerp(&self, other: &SynthPatch, t: f64) -> SynthPatch {
        let lerp_f = |a: f64, b: f64| a + (b - a) * t;

        let oscillators = self
            .oscillators
            .iter()
            .zip(other.oscillators.iter())
            .map(|(a, b)| OscillatorParams {
                waveform: if t < 0.5 { a.waveform } else { b.waveform },
                frequency_ratio: lerp_f(a.frequency_ratio, b.frequency_ratio),
                amplitude: lerp_f(a.amplitude, b.amplitude),
                detune_cents: lerp_f(a.detune_cents, b.detune_cents),
            })
            .collect();

        let filter = FilterParams {
            filter_type: if t < 0.5 { self.filter.filter_type } else { other.filter.filter_type },
            cutoff_hz: lerp_f(self.filter.cutoff_hz, other.filter.cutoff_hz),
            resonance: lerp_f(self.filter.resonance, other.filter.resonance),
            env_amount: lerp_f(self.filter.env_amount, other.filter.env_amount),
        };

        let envelope = EnvelopeParams {
            attack_s: lerp_f(self.envelope.attack_s, other.envelope.attack_s),
            decay_s: lerp_f(self.envelope.decay_s, other.envelope.decay_s),
            sustain: lerp_f(self.envelope.sustain, other.envelope.sustain),
            release_s: lerp_f(self.envelope.release_s, other.envelope.release_s),
        };

        let effects = self
            .effects
            .iter()
            .zip(other.effects.iter())
            .map(|(a, b)| EffectParams {
                kind: if t < 0.5 { a.kind } else { b.kind },
                mix: lerp_f(a.mix, b.mix),
                param1: lerp_f(a.param1, b.param1),
            })
            .collect();

        SynthPatch {
            oscillators,
            filter,
            envelope,
            effects,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_vertex_saw() {
        let patch = SynthPatch::from_vertex(&[0, 1], 0.0);
        assert_eq!(patch.oscillators[0].waveform, OscillatorWaveform::Saw);
    }

    #[test]
    fn from_vertex_square() {
        let patch = SynthPatch::from_vertex(&[1], 0.0);
        assert_eq!(patch.oscillators[0].waveform, OscillatorWaveform::Square);
    }

    #[test]
    fn from_vertex_cutoff_scales() {
        let p1 = SynthPatch::from_vertex(&[0], 0.0);
        let p2 = SynthPatch::from_vertex(&[0], 2.0);
        assert!(p2.filter.cutoff_hz > p1.filter.cutoff_hz);
    }

    #[test]
    fn lerp_midpoint() {
        let a = SynthPatch::simple();
        let mut b = SynthPatch::simple();
        b.filter.cutoff_hz = 2000.0;
        b.oscillators[0].amplitude = 1.0;

        let mid = a.lerp(&b, 0.5);
        assert!((mid.filter.cutoff_hz - 1500.0).abs() < 1.0);
        assert!((mid.oscillators[0].amplitude - 0.9).abs() < 1e-9);
    }

    #[test]
    fn simple_patch_default() {
        let p = SynthPatch::simple();
        assert_eq!(p.oscillators.len(), 1);
        assert_eq!(p.filter.filter_type, FilterType::LowPass);
    }
}
