use macroquad::audio::{
    PlaySoundParams, Sound, load_sound_from_bytes, play_sound, play_sound_once, stop_sound,
};

use crate::ui::{Cue, Music};

/// Names of the music tracks, in the order `tracks` builds them.
pub const TRACKS: [&str; 3] = ["Packet Storm", "Zero Day Drive", "Neon Firewall"];

const RATE: u32 = 44_100;
/// Samples per sixteenth note, about 128 BPM.
const STEP: usize = 5_168;
const MUSIC_VOLUME: f32 = 0.4;

#[derive(Debug, Clone, Copy)]
enum Wave {
    Square,
    Triangle,
    Noise,
}

fn freq(midi: i32) -> f32 {
    440.0 * 2f32.powf((midi - 69) as f32 / 12.0)
}

/// One note with a linear decay envelope.
fn note(wave: Wave, midi: i32, samples: usize, volume: f32) -> Vec<f32> {
    let hz = freq(midi);
    let mut seed = 0x1234_5678u32;
    (0..samples)
        .map(|i| {
            let phase = (i as f32 * hz / RATE as f32).fract();
            let value = match wave {
                Wave::Square => {
                    if phase < 0.5 {
                        1.0
                    } else {
                        -1.0
                    }
                }
                Wave::Triangle => 4.0 * (phase - 0.5).abs() - 1.0,
                Wave::Noise => {
                    seed ^= seed << 13;
                    seed ^= seed >> 17;
                    seed ^= seed << 5;
                    seed as f32 / u32::MAX as f32 * 2.0 - 1.0
                }
            };
            value * volume * (1.0 - i as f32 / samples as f32)
        })
        .collect()
}

/// Kick drum: a triangle wave sweeping down in pitch.
fn kick(volume: f32) -> Vec<f32> {
    let samples = STEP;
    let mut phase = 0.0f32;
    (0..samples)
        .map(|i| {
            let t = i as f32 / samples as f32;
            phase = (phase + (45.0 + 120.0 * (1.0 - t).powi(3)) / RATE as f32).fract();
            (4.0 * (phase - 0.5).abs() - 1.0) * volume * (1.0 - t)
        })
        .collect()
}

/// Plays notes back to back; lengths are in half steps.
fn melody(wave: Wave, notes: &[(i32, usize)], volume: f32) -> Vec<f32> {
    notes
        .iter()
        .flat_map(|&(midi, steps)| note(wave, midi, steps * STEP / 2, volume))
        .collect()
}

/// Adds `samples` into `buffer` at `start`, truncating at the end of the buffer.
fn mix(buffer: &mut [f32], samples: &[f32], start: usize) {
    for (out, sample) in buffer[start..].iter_mut().zip(samples) {
        *out += sample;
    }
}

fn tracks() -> [Vec<f32>; 3] {
    [packet_storm(), zero_day_drive(), neon_firewall()]
}

/// Four bars of minor-key techno: kick, off-beat hats, octave bass, and a chord arpeggio.
fn packet_storm() -> Vec<f32> {
    // Chord roots (A3, F3, G3, E3) with minor or major thirds.
    let chords = [(57, 3), (53, 4), (55, 4), (52, 3)];
    let mut buffer = vec![0.0; chords.len() * 16 * STEP];
    let kick = kick(0.45);
    let hat = note(Wave::Noise, 0, STEP / 3, 0.1);

    for (bar, &(root, third)) in chords.iter().enumerate() {
        let arp = [0, third, 7, 12];
        for step in 0..16 {
            let start = (bar * 16 + step) * STEP;
            if step % 4 == 0 {
                mix(&mut buffer, &kick, start);
            }
            if step % 4 == 2 {
                mix(&mut buffer, &hat, start);
            }
            if step % 2 == 0 {
                let octave = if step % 4 == 2 { 0 } else { -12 };
                mix(
                    &mut buffer,
                    &note(Wave::Square, root - 12 + octave, 2 * STEP, 0.18),
                    start,
                );
            }
            mix(
                &mut buffer,
                &note(Wave::Square, root + 12 + arp[step % 4], STEP, 0.1),
                start,
            );
        }
    }
    buffer
}

/// Four bars of driving techno in D minor: rolling sixteenth bass, snare, and a triangle lead.
fn zero_day_drive() -> Vec<f32> {
    // Chord roots (D3, Bb2, C3, A2) with minor or major thirds.
    let chords = [(50, 3), (46, 4), (48, 4), (45, 3)];
    let mut buffer = vec![0.0; chords.len() * 16 * STEP];
    let kick = kick(0.4);
    let snare = note(Wave::Noise, 0, STEP, 0.2);

    for (bar, &(root, third)) in chords.iter().enumerate() {
        let lead = [0, 7, third + 12, 7, 12, third, 7, 12];
        for step in 0..16 {
            let start = (bar * 16 + step) * STEP;
            if step % 4 == 0 {
                mix(&mut buffer, &kick, start);
            }
            if step % 8 == 4 {
                mix(&mut buffer, &snare, start);
            }
            let bass = [0, 0, 12, 0][step % 4];
            mix(
                &mut buffer,
                &note(Wave::Square, root - 12 + bass, STEP, 0.14),
                start,
            );
            if step % 2 == 0 {
                mix(
                    &mut buffer,
                    &note(Wave::Triangle, root + 12 + lead[step / 2], 2 * STEP, 0.22),
                    start,
                );
            }
        }
    }
    buffer
}

/// Four bars of broken-beat techno in E minor: syncopated kicks, a triangle pad, and a pentatonic lead.
fn neon_firewall() -> Vec<f32> {
    // Chord roots (E3, C3, D3, B2) with minor or major thirds.
    let chords = [(52, 3), (48, 4), (50, 4), (47, 4)];
    // Lead notes above E4 on steps 0, 3, 6, 8, 11, and 14 of each bar.
    let leads = [
        [7, 10, 12, 10, 7, 3],
        [3, 5, 7, 5, 3, 0],
        [5, 7, 10, 12, 15, 12],
        [10, 7, 5, 3, 5, 7],
    ];
    let mut buffer = vec![0.0; chords.len() * 16 * STEP];
    let kick = kick(0.4);
    let snare = note(Wave::Noise, 0, STEP, 0.18);
    let hat = note(Wave::Noise, 0, STEP / 4, 0.05);

    for (bar, (&(root, third), lead)) in chords.iter().zip(&leads).enumerate() {
        let bar_start = bar * 16 * STEP;
        for interval in [0, third, 7] {
            mix(
                &mut buffer,
                &note(Wave::Triangle, root + interval, 16 * STEP, 0.08),
                bar_start,
            );
        }
        for step in 0..16 {
            let start = bar_start + step * STEP;
            if [0, 3, 10].contains(&step) {
                mix(&mut buffer, &kick, start);
            }
            if step % 8 == 4 {
                mix(&mut buffer, &snare, start);
            }
            mix(&mut buffer, &hat, start);
            if let Some(i) = [0, 3, 6, 8, 11, 14].iter().position(|&s| s == step) {
                mix(
                    &mut buffer,
                    &note(Wave::Square, 64 + lead[i], 2 * STEP, 0.1),
                    start,
                );
            }
        }
    }
    buffer
}

/// A triangle wave sliding up to a higher pitch and back down, like a siren.
fn siren(low: i32, high: i32, samples: usize, volume: f32) -> Vec<f32> {
    let mut phase = 0.0f32;
    (0..samples)
        .map(|i| {
            let rise = 1.0 - (2.0 * i as f32 / samples as f32 - 1.0).abs();
            let hz = freq(low) + (freq(high) - freq(low)) * rise;
            phase = (phase + hz / RATE as f32).fract();
            (4.0 * (phase - 0.5).abs() - 1.0) * volume
        })
        .collect()
}

/// Four bars of frantic techno for the coffee run, about 170 BPM: broken kicks,
/// a chromatic bass line, clashing car horns, and a siren every other bar.
fn rush_hour() -> Vec<f32> {
    let step = STEP * 3 / 4;
    let bass = [
        40, 40, 52, 40, 41, 40, 52, 43, 40, 40, 52, 40, 46, 45, 44, 43,
    ];
    let mut buffer = vec![0.0; 4 * 16 * step];
    let kick = kick(0.35);
    let snare = note(Wave::Noise, 0, step, 0.2);

    for bar in 0..4 {
        let bar_start = bar * 16 * step;
        if bar % 2 == 1 {
            mix(&mut buffer, &siren(76, 83, 16 * step, 0.07), bar_start);
        }
        for (s, &midi) in bass.iter().enumerate() {
            let start = bar_start + s * step;
            if [0, 6, 10].contains(&s) {
                mix(&mut buffer, &kick, start);
            }
            // Snare on the backbeat, rolling through the end of the last bar.
            if s % 8 == 4 || bar == 3 && s >= 12 {
                mix(&mut buffer, &snare, start);
            }
            mix(&mut buffer, &note(Wave::Square, midi, step, 0.14), start);
            let horn = if bar % 2 == 0 {
                [3, 11].contains(&s)
            } else {
                s == 7
            };
            if horn {
                for midi in [65, 68] {
                    mix(
                        &mut buffer,
                        &note(Wave::Square, midi, 2 * step, 0.09),
                        start,
                    );
                }
            }
        }
    }
    buffer
}

/// The synthesized sound for each cue.
fn effect(cue: Cue) -> Vec<f32> {
    match cue {
        Cue::Select => note(Wave::Square, 81, STEP / 2, 0.25),
        Cue::Alarm => melody(Wave::Square, &[(76, 2), (69, 2), (76, 2), (69, 2)], 0.3),
        Cue::Win => melody(Wave::Square, &[(72, 2), (76, 2), (79, 2), (84, 6)], 0.3),
        Cue::Lose => melody(Wave::Triangle, &[(67, 4), (64, 4), (60, 4), (55, 10)], 0.5),
        // A soft blip for each day that passes.
        Cue::Tick => note(Wave::Square, 96, STEP / 8, 0.12),
        // Two bright coin notes.
        Cue::Payday => melody(Wave::Square, &[(84, 1), (91, 3)], 0.2),
        // A siren that rises and falls three times.
        Cue::Klaxon => melody(
            Wave::Square,
            &[(70, 2), (75, 2), (70, 2), (75, 2), (70, 2), (75, 2)],
            0.3,
        ),
        // A low, dramatic diminished fall with a noise hit.
        Cue::Sting => {
            let mut sting = melody(Wave::Triangle, &[(62, 2), (59, 2), (56, 8)], 0.6);
            mix(&mut sting, &note(Wave::Noise, 0, STEP * 2, 0.3), 0);
            sting
        }
        Cue::Fanfare => melody(
            Wave::Square,
            &[(67, 1), (72, 1), (76, 1), (79, 3), (76, 1), (79, 4)],
            0.3,
        ),
        // Two triangle tones with a long ring, like a struck bell.
        Cue::Bell => {
            let mut bell = note(Wave::Triangle, 84, STEP * 12, 0.5);
            mix(&mut bell, &note(Wave::Triangle, 91, STEP * 8, 0.3), 0);
            mix(&mut bell, &note(Wave::Square, 96, STEP * 2, 0.1), 0);
            bell
        }
    }
}

/// Encodes samples as a 16-bit mono PCM WAV file.
fn wav(samples: &[f32]) -> Vec<u8> {
    let data_len = samples.len() as u32 * 2;
    let mut bytes = Vec::with_capacity(44 + data_len as usize);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + data_len).to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes()); // PCM
    bytes.extend_from_slice(&1u16.to_le_bytes()); // mono
    bytes.extend_from_slice(&RATE.to_le_bytes());
    bytes.extend_from_slice(&(RATE * 2).to_le_bytes());
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.extend_from_slice(&16u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_len.to_le_bytes());
    for sample in samples {
        bytes
            .extend_from_slice(&((sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16).to_le_bytes());
    }
    bytes
}

pub struct Audio {
    /// One sound per cue, in `Cue::ALL` order.
    effects: Vec<Sound>,
    music: Vec<Sound>,
    rush_hour: Sound,
}

async fn load(samples: Vec<f32>) -> Sound {
    load_sound_from_bytes(&wav(&samples)).await.unwrap()
}

impl Audio {
    pub async fn load() -> Self {
        Self {
            effects: {
                let mut effects = Vec::new();
                for cue in Cue::ALL {
                    effects.push(load(effect(cue)).await);
                }
                effects
            },
            music: {
                let mut music = Vec::new();
                for track in tracks() {
                    music.push(load(track).await);
                }
                music
            },
            rush_hour: load(rush_hour()).await,
        }
    }

    fn stop_music(&self) {
        for track in self.music.iter().chain([&self.rush_hour]) {
            stop_sound(track);
        }
    }

    fn loop_music(&self, sound: &Sound) {
        self.stop_music();
        play_sound(
            sound,
            PlaySoundParams {
                looped: true,
                volume: MUSIC_VOLUME,
            },
        );
    }

    pub fn play_music(&self, music: Music) {
        match music {
            Music::Track(i) => self.loop_music(&self.music[i]),
            Music::Off => self.stop_music(),
        }
    }

    /// Swaps in the traffic music for the coffee run, unless music is off.
    pub fn play_rush_hour(&self, music: Music) {
        if music != Music::Off {
            self.loop_music(&self.rush_hour);
        }
    }

    pub fn play(&self, cue: Cue) {
        play_sound_once(&self.effects[cue as usize]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn peak(samples: &[f32]) -> f32 {
        samples.iter().fold(0.0, |max, s| max.max(s.abs()))
    }

    #[test]
    fn midi_69_is_concert_a() {
        assert_eq!(freq(69), 440.0);
        assert_eq!(freq(81), 880.0);
    }

    #[test]
    fn notes_have_the_requested_length_and_decay() {
        for wave in [Wave::Square, Wave::Triangle, Wave::Noise] {
            let samples = note(wave, 69, 1000, 0.5);

            assert_eq!(samples.len(), 1000);
            assert!(peak(&samples) <= 0.5);
            assert!(samples[999].abs() < 0.01);
        }
    }

    #[test]
    fn every_track_is_four_whole_bars_and_never_clips() {
        for (name, music) in TRACKS.iter().zip(tracks()) {
            assert_eq!(music.len(), 4 * 16 * STEP, "{name}");
            assert!(peak(&music) > 0.3, "{name} is too quiet");
            assert!(peak(&music) <= 1.0, "{name} clips");
        }
    }

    #[test]
    fn rush_hour_is_four_fast_bars_and_never_clips() {
        let music = rush_hour();

        assert_eq!(music.len(), 4 * 16 * (STEP * 3 / 4));
        assert!(peak(&music) > 0.3 && peak(&music) <= 1.0);
    }

    #[test]
    fn sound_effects_are_audible_and_never_clip() {
        for cue in Cue::ALL {
            let sound = effect(cue);
            assert!(!sound.is_empty(), "{cue:?}");
            assert!(peak(&sound) > 0.1 && peak(&sound) <= 1.0, "{cue:?}");
        }
    }

    #[test]
    fn mix_truncates_at_the_end_of_the_buffer() {
        let mut buffer = vec![0.0; 4];

        mix(&mut buffer, &[1.0, 1.0, 1.0], 2);

        assert_eq!(buffer, [0.0, 0.0, 1.0, 1.0]);
    }

    #[test]
    fn wav_encodes_a_valid_pcm_header_and_samples() {
        let bytes = wav(&[0.0, 1.0, -1.0]);

        assert_eq!(&bytes[0..4], b"RIFF");
        assert_eq!(&bytes[8..16], b"WAVEfmt ");
        assert_eq!(u32::from_le_bytes(bytes[24..28].try_into().unwrap()), RATE);
        assert_eq!(&bytes[36..40], b"data");
        assert_eq!(bytes.len(), 44 + 6);
        assert_eq!(i16::from_le_bytes([bytes[46], bytes[47]]), i16::MAX);
        assert_eq!(i16::from_le_bytes([bytes[48], bytes[49]]), -i16::MAX);
    }
}
