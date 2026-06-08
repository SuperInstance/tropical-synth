use crate::{SynthPatch, TropicalSynthError};

/// Standard MIDI CC numbers for synth parameters.
pub mod cc {
    pub const MODWHEEL: u8 = 1;
    pub const BREATH: u8 = 2;
    pub const FOOT: u8 = 4;
    pub const VOLUME: u8 = 7;
    pub const PAN: u8 = 10;
    pub const EXPRESSION: u8 = 11;
    pub const BRIGHTNESS: u8 = 74;
    pub const SOUND_TIMBER: u8 = 71;
    pub const FILTER_RESONANCE: u8 = 71;
    pub const ATTACK: u8 = 73;
    pub const RELEASE: u8 = 72;
    pub const DECAY: u8 = 75;
    pub const SUSTAIN_LEVEL: u8 = 70;
    pub const REVERB: u8 = 91;
    pub const CHORUS: u8 = 93;
    pub const DETUNE: u8 = 94;
}

/// A MIDI CC message: (channel, cc_number, value 0–127).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MidiCC {
    pub channel: u8,
    pub cc: u8,
    pub value: u8,
}

/// Maps a SynthPatch to MIDI CC messages.
#[derive(Debug, Clone)]
pub struct MidiCCMapper {
    pub channel: u8,
}

impl MidiCCMapper {
    pub fn new(channel: u8) -> Self {
        Self { channel: channel.min(15) }
    }

    /// Convert a f64 in [0.0, 1.0] to a MIDI value (0–127).
    fn to_midi(val: f64) -> u8 {
        (val.clamp(0.0, 1.0) * 127.0).round() as u8
    }

    /// Convert a frequency (20–20000 Hz) to a normalized 0–1 range (logarithmic).
    fn freq_to_normalized(freq: f64) -> f64 {
        const MIN_F: f64 = 20.0;
        const MAX_F: f64 = 20000.0;
        if freq <= MIN_F { return 0.0; }
        if freq >= MAX_F { return 1.0; }
        (freq.ln() - MIN_F.ln()) / (MAX_F.ln() - MIN_F.ln())
    }

    /// Map a SynthPatch to a series of MIDI CC messages.
    pub fn map_patch(&self, patch: &SynthPatch) -> Result<Vec<MidiCC>, TropicalSynthError> {
        let mut msgs = Vec::new();

        // Overall amplitude from first oscillator
        if let Some(osc) = patch.oscillators.first() {
            msgs.push(MidiCC { channel: self.channel, cc: cc::VOLUME, value: Self::to_midi(osc.amplitude) });
            msgs.push(MidiCC { channel: self.channel, cc: cc::DETUNE, value: Self::to_midi(osc.detune_cents / 100.0) });
        }

        // Filter
        msgs.push(MidiCC { channel: self.channel, cc: cc::BRIGHTNESS, value: Self::to_midi(Self::freq_to_normalized(patch.filter.cutoff_hz)) });
        msgs.push(MidiCC { channel: self.channel, cc: cc::FILTER_RESONANCE, value: Self::to_midi(patch.filter.resonance) });

        // Envelope
        msgs.push(MidiCC { channel: self.channel, cc: cc::ATTACK, value: Self::to_midi(patch.envelope.attack_s / 5.0) });
        msgs.push(MidiCC { channel: self.channel, cc: cc::DECAY, value: Self::to_midi(patch.envelope.decay_s / 5.0) });
        msgs.push(MidiCC { channel: self.channel, cc: cc::SUSTAIN_LEVEL, value: Self::to_midi(patch.envelope.sustain) });
        msgs.push(MidiCC { channel: self.channel, cc: cc::RELEASE, value: Self::to_midi(patch.envelope.release_s / 5.0) });

        // Effects
        for fx in &patch.effects {
            let cc_num = match fx.kind {
                crate::EffectKind::Reverb => cc::REVERB,
                crate::EffectKind::Chorus => cc::CHORUS,
                _ => cc::SOUND_TIMBER,
            };
            msgs.push(MidiCC { channel: self.channel, cc: cc_num, value: Self::to_midi(fx.mix) });
        }

        Ok(msgs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{OscillatorParams, FilterParams, EnvelopeParams, EffectParams, EffectKind};

    #[test]
    fn map_simple_patch() {
        let mapper = MidiCCMapper::new(0);
        let patch = SynthPatch::simple();
        let msgs = mapper.map_patch(&patch).unwrap();
        assert!(!msgs.is_empty());
        // All values should be 0–127
        for m in &msgs {
            assert!(m.value <= 127);
        }
    }

    #[test]
    fn map_with_effects() {
        let mapper = MidiCCMapper::new(1);
        let patch = SynthPatch {
            oscillators: vec![OscillatorParams::default()],
            filter: FilterParams::default(),
            envelope: EnvelopeParams::default(),
            effects: vec![
                EffectParams::new(EffectKind::Reverb, 0.5),
                EffectParams::new(EffectKind::Chorus, 0.3),
            ],
        };
        let msgs = mapper.map_patch(&patch).unwrap();
        // Should include reverb and chorus CCs
        assert!(msgs.iter().any(|m| m.cc == cc::REVERB));
        assert!(msgs.iter().any(|m| m.cc == cc::CHORUS));
    }

    #[test]
    fn channel_clamped() {
        let mapper = MidiCCMapper::new(200);
        assert_eq!(mapper.channel, 15);
    }

    #[test]
    fn cutoff_maps_to_brightness() {
        let mapper = MidiCCMapper::new(0);
        let mut p1 = SynthPatch::simple();
        p1.filter.cutoff_hz = 20.0; // min
        let mut p2 = SynthPatch::simple();
        p2.filter.cutoff_hz = 20000.0; // max

        let msgs1 = mapper.map_patch(&p1).unwrap();
        let msgs2 = mapper.map_patch(&p2).unwrap();

        let br1: u8 = msgs1.iter().find(|m| m.cc == cc::BRIGHTNESS).unwrap().value;
        let br2: u8 = msgs2.iter().find(|m| m.cc == cc::BRIGHTNESS).unwrap().value;
        assert!(br2 > br1);
    }
}
