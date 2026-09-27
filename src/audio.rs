use macroquad::audio::{
    PlaySoundParams, Sound, load_sound_from_bytes, play_sound, play_sound_once,
};

use crate::ui::Cue;

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

/// Four bars of minor-key techno: kick, off-beat hats, octave bass, and a chord arpeggio.
fn music() -> Vec<f32> {
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

fn select() -> Vec<f32> {
    note(Wave::Square, 81, STEP / 2, 0.25)
}

fn alarm() -> Vec<f32> {
    melody(Wave::Square, &[(76, 2), (69, 2), (76, 2), (69, 2)], 0.3)
}

fn win() -> Vec<f32> {
    melody(Wave::Square, &[(72, 2), (76, 2), (79, 2), (84, 6)], 0.3)
}

fn lose() -> Vec<f32> {
    melody(Wave::Triangle, &[(67, 4), (64, 4), (60, 4), (55, 10)], 0.5)
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
    select: Sound,
    alarm: Sound,
    win: Sound,
    lose: Sound,
    music: Sound,
}

async fn load(samples: Vec<f32>) -> Sound {
    load_sound_from_bytes(&wav(&samples)).await.unwrap()
}

impl Audio {
    pub async fn load() -> Self {
        Self {
            select: load(select()).await,
            alarm: load(alarm()).await,
            win: load(win()).await,
            lose: load(lose()).await,
            music: load(music()).await,
        }
    }

    pub fn start_music(&self) {
        play_sound(
            &self.music,
            PlaySoundParams {
                looped: true,
                volume: MUSIC_VOLUME,
            },
        );
    }

    pub fn play(&self, cue: Cue) {
        play_sound_once(match cue {
            Cue::Select => &self.select,
            Cue::Alarm => &self.alarm,
            Cue::Win => &self.win,
            Cue::Lose => &self.lose,
        });
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
    fn music_is_four_whole_bars_and_never_clips() {
        let music = music();

        assert_eq!(music.len(), 4 * 16 * STEP);
        assert!(peak(&music) > 0.3);
        assert!(peak(&music) <= 1.0);
    }

    #[test]
    fn sound_effects_are_audible_and_never_clip() {
        for effect in [select(), alarm(), win(), lose()] {
            assert!(!effect.is_empty());
            assert!(peak(&effect) > 0.1 && peak(&effect) <= 1.0);
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
