use macroquad::audio::{
    PlaySoundParams, Sound, load_sound_from_bytes, play_sound, set_sound_volume, stop_sound,
};

use std::f32::consts::TAU;

use crate::options::Options;
use crate::ui::{Cue, Music};

/// Names of the music tracks, in the order `tracks` builds them.
pub const TRACKS: [&str; 3] = ["Packet Storm", "Zero Day Drive", "Neon Firewall"];

const RATE: u32 = 44_100;
/// Samples per sixteenth note, about 128 BPM.
const STEP: usize = 5_168;
/// Music gain per volume step, so the default of 8 plays at 0.4.
const MUSIC_STEP: f32 = 0.05;

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

/// One-pole low-pass filter; `cutoff` gives the cutoff in Hz at each sample.
fn low_pass(samples: &mut [f32], cutoff: impl Fn(usize) -> f32) {
    let mut out = 0.0;
    for (i, sample) in samples.iter_mut().enumerate() {
        out += (1.0 - (-TAU * cutoff(i) / RATE as f32).exp()) * (*sample - out);
        *sample = out;
    }
}

const BAR: usize = 16 * STEP;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Section {
    Intro,
    Melody,
    Groove,
    Breakdown,
}

/// The sections of an arranged song and their lengths in bars, 90 seconds in all;
/// it ends on the groove again.
const ARRANGEMENT: [(Section, usize); 5] = [
    (Section::Intro, 8),
    (Section::Melody, 8),
    (Section::Groove, 8),
    (Section::Breakdown, 16),
    (Section::Groove, 8),
];

/// A silent note in a melody.
const REST: i32 = 0;

/// An 8-bar melody, as (MIDI note, sixteenths) per bar.
type Melody = [&'static [(i32, usize)]; 8];

/// Low-pass cutoff for the melody at sample `i` of a section: it opens over the
/// melody section, stays open for the groove, and closes and reopens every 8 bars
/// of the breakdown.
fn melody_cutoff(section: Section, bars: usize, i: usize) -> f32 {
    let phrase = (i as f32 / (8 * BAR) as f32).fract();
    match section {
        Section::Melody => 150.0 * 2f32.powf(3.0 * i as f32 / (bars * BAR) as f32),
        Section::Breakdown => 150.0 * 2f32.powf(5.0 * phrase),
        Section::Intro | Section::Groove => 20_000.0,
    }
}

/// Renders a song in the `ARRANGEMENT` sections. `bar` draws one bar's drums, bass,
/// and backing at the start of the buffer it is given, for a section, bar within the
/// section, and key offset; the breakdown climbs a half step every 8 bars. The
/// `melody` plays through the low-pass filter in every section after the intro, and
/// snare rolls and a noise riser mark the section ends.
fn arrange(melody: Melody, bar: impl Fn(&mut [f32], Section, usize, i32)) -> Vec<f32> {
    let bars: usize = ARRANGEMENT.iter().map(|&(_, bars)| bars).sum();
    let mut buffer = vec![0.0; bars * BAR];
    let snare = note(Wave::Noise, 0, STEP, 0.15);
    let mut riser = note(Wave::Noise, 0, 4 * BAR, 0.15);
    riser.reverse();

    let mut section_start = 0;
    for (section, section_bars) in ARRANGEMENT {
        let breakdown = section == Section::Breakdown;
        let mut lead = vec![0.0; section_bars * BAR];
        for b in 0..section_bars {
            let bar_start = section_start + b * BAR;
            let key = if breakdown { (b / 8) as i32 } else { 0 };
            bar(&mut buffer[bar_start..], section, b, key);
            if breakdown && b == section_bars - 4 {
                mix(&mut buffer, &riser, bar_start);
            }
            // Snare rolls: a fill at the end of each section, accelerating into the drop.
            let roll = match (breakdown, section_bars - b) {
                (true, 1..=4) => [1, 1, 2, 4][section_bars - b - 1],
                (false, 1) => 1,
                _ => 0,
            };
            for step in 0..16 {
                if roll > 0 && step % roll == 0 && (breakdown || step >= 12) {
                    mix(&mut buffer, &snare, bar_start + step * STEP);
                }
            }
            if section != Section::Intro {
                let mut start = b * BAR;
                for &(midi, steps) in melody[b % 8] {
                    if midi != REST {
                        mix(
                            &mut lead,
                            &note(Wave::Square, midi + key, steps * STEP, 0.14),
                            start,
                        );
                    }
                    start += steps * STEP;
                }
            }
        }
        low_pass(&mut lead, |i| melody_cutoff(section, section_bars, i));
        mix(&mut buffer, &lead, section_start);
        section_start += section_bars * BAR;
    }
    buffer
}

/// Arranged minor-key techno in A minor. The intro is kick, off-beat hats, octave
/// bass, and a chord arpeggio; the main melody flows in quarters and eighths; the
/// groove adds sixteenth hats, a backbeat snare, and a triangle sub-bass.
fn packet_storm() -> Vec<f32> {
    // Chord roots (A3, F3, G3, E3) with minor or major thirds.
    let chords = [(57, 3), (53, 4), (55, 4), (52, 3)];
    let melody: Melody = [
        &[(69, 4), (72, 2), (76, 2), (74, 4), (72, 4)],
        &[(69, 4), (65, 2), (69, 2), (72, 6), (69, 2)],
        &[(71, 4), (74, 2), (79, 2), (77, 4), (74, 4)],
        &[(76, 6), (74, 2), (71, 4), (68, 4)],
        &[(69, 4), (72, 2), (76, 2), (81, 4), (79, 4)],
        &[(77, 4), (76, 2), (72, 2), (69, 4), (72, 4)],
        &[(74, 4), (71, 2), (74, 2), (79, 4), (77, 4)],
        &[(76, 4), (74, 2), (71, 2), (68, 8)],
    ];
    let kick = kick(0.4);
    let hat = note(Wave::Noise, 0, STEP / 3, 0.08);
    let tick = note(Wave::Noise, 0, STEP / 4, 0.04);
    let snare = note(Wave::Noise, 0, STEP, 0.15);

    arrange(melody, |buffer, section, bar, key| {
        let groove = section == Section::Groove;
        let breakdown = section == Section::Breakdown;
        let (root, third) = chords[bar % 4];
        let arp = [0, third, 7, 12];
        for step in 0..16 {
            let start = step * STEP;
            if step % 4 == 0 && !breakdown {
                mix(buffer, &kick, start);
            }
            if step % 4 == 2 {
                mix(buffer, &hat, start);
            } else if groove {
                mix(buffer, &tick, start);
            }
            if groove && step % 8 == 4 {
                mix(buffer, &snare, start);
            }
            if groove {
                let octave = if step % 2 == 1 { 0 } else { -12 };
                mix(
                    buffer,
                    &note(Wave::Square, root - 12 + octave, STEP, 0.1),
                    start,
                );
                mix(buffer, &note(Wave::Triangle, root - 24, STEP, 0.16), start);
            } else if step % 2 == 0 && !breakdown {
                let octave = if step % 4 == 2 { 0 } else { -12 };
                mix(
                    buffer,
                    &note(Wave::Square, root - 12 + octave, 2 * STEP, 0.16),
                    start,
                );
            }
            mix(
                buffer,
                &note(Wave::Square, root + 12 + key + arp[step % 4], STEP, 0.08),
                start,
            );
        }
    })
}

/// Arranged driving techno in D minor. The intro is a rolling sixteenth bass, snare,
/// and a triangle lead; the main melody is a staccato call-and-response riff that
/// ends in a chromatic slide; the groove adds sixteenth hats and a triangle sub-bass.
fn zero_day_drive() -> Vec<f32> {
    // Chord roots (D3, Bb2, C3, A2) with minor or major thirds.
    let chords = [(50, 3), (46, 4), (48, 4), (45, 3)];
    #[rustfmt::skip]
    let melody: Melody = [
        &[(74, 1), (REST, 1), (74, 1), (REST, 1), (77, 1), (REST, 1), (74, 2), (REST, 2), (72, 1), (REST, 1), (69, 2), (REST, 2)],
        &[(70, 1), (REST, 1), (70, 1), (REST, 1), (74, 1), (REST, 1), (70, 2), (REST, 2), (69, 1), (REST, 1), (65, 4)],
        &[(72, 1), (REST, 1), (72, 1), (REST, 1), (76, 1), (REST, 1), (72, 2), (REST, 2), (74, 1), (REST, 1), (76, 2), (REST, 2)],
        &[(73, 2), (76, 2), (81, 2), (REST, 2), (79, 1), (77, 1), (76, 1), (74, 1), (73, 4)],
        &[(86, 1), (REST, 1), (86, 1), (REST, 1), (84, 1), (REST, 1), (81, 2), (REST, 2), (77, 1), (REST, 1), (81, 2), (REST, 2)],
        &[(82, 1), (REST, 1), (82, 1), (REST, 1), (81, 1), (REST, 1), (77, 2), (REST, 2), (74, 1), (REST, 1), (77, 4)],
        &[(79, 1), (REST, 1), (79, 1), (REST, 1), (76, 1), (REST, 1), (72, 2), (REST, 2), (76, 1), (REST, 1), (79, 2), (REST, 2)],
        &[(81, 1), (80, 1), (79, 1), (78, 1), (77, 1), (76, 1), (75, 1), (74, 1), (73, 8)],
    ];
    let kick = kick(0.35);
    let snare = note(Wave::Noise, 0, STEP, 0.15);
    let tick = note(Wave::Noise, 0, STEP / 4, 0.05);

    arrange(melody, |buffer, section, bar, key| {
        let groove = section == Section::Groove;
        let breakdown = section == Section::Breakdown;
        let (root, third) = chords[bar % 4];
        let lead = [0, 7, third + 12, 7, 12, third, 7, 12];
        for step in 0..16 {
            let start = step * STEP;
            if !breakdown {
                if step % 4 == 0 {
                    mix(buffer, &kick, start);
                }
                if step % 8 == 4 {
                    mix(buffer, &snare, start);
                }
                let bass = [0, 0, 12, 0][step % 4];
                mix(
                    buffer,
                    &note(Wave::Square, root - 12 + bass, STEP, 0.12),
                    start,
                );
            }
            if groove {
                mix(buffer, &tick, start);
                mix(buffer, &note(Wave::Triangle, root - 12, STEP, 0.08), start);
            }
            if step % 2 == 0 {
                mix(
                    buffer,
                    &note(
                        Wave::Triangle,
                        root + 12 + key + lead[step / 2],
                        2 * STEP,
                        0.16,
                    ),
                    start,
                );
            }
        }
    })
}

/// Arranged broken-beat techno in E minor. The intro is syncopated kicks, a triangle
/// pad, and a pentatonic lead; the main melody soars in long, ringing notes; the
/// groove adds an extra kick and a triangle sub-bass.
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
    let melody: Melody = [
        &[(79, 6), (78, 2), (76, 8)],
        &[(72, 4), (76, 4), (79, 8)],
        &[(78, 6), (81, 2), (78, 4), (74, 4)],
        &[(75, 8), (78, 4), (83, 4)],
        &[(83, 6), (81, 2), (79, 8)],
        &[(84, 4), (83, 4), (79, 8)],
        &[(81, 6), (78, 2), (74, 4), (78, 4)],
        &[(75, 12), (71, 4)],
    ];
    let kick = kick(0.4);
    let snare = note(Wave::Noise, 0, STEP, 0.18);
    let hat = note(Wave::Noise, 0, STEP / 4, 0.05);

    arrange(melody, |buffer, section, bar, key| {
        let groove = section == Section::Groove;
        let breakdown = section == Section::Breakdown;
        let (root, third) = chords[bar % 4];
        let lead = leads[bar % 4];
        for interval in [0, third, 7] {
            mix(
                buffer,
                &note(Wave::Triangle, root + key + interval, 16 * STEP, 0.08),
                0,
            );
        }
        let kicks: &[usize] = if groove { &[0, 3, 8, 10] } else { &[0, 3, 10] };
        for step in 0..16 {
            let start = step * STEP;
            if !breakdown {
                if kicks.contains(&step) {
                    mix(buffer, &kick, start);
                }
                if step % 8 == 4 {
                    mix(buffer, &snare, start);
                }
            }
            if groove {
                mix(buffer, &note(Wave::Triangle, root - 12, STEP, 0.1), start);
            }
            mix(buffer, &hat, start);
            if let Some(i) = [0, 3, 6, 8, 11, 14].iter().position(|&s| s == step) {
                mix(
                    buffer,
                    &note(Wave::Square, 64 + key + lead[i], 2 * STEP, 0.08),
                    start,
                );
            }
        }
    })
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
    music_volume: f32,
    sound_volume: f32,
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
            music_volume: 0.0,
            sound_volume: 0.0,
        }
    }

    /// Sets both volumes, including the music already playing.
    pub fn set_volumes(&mut self, options: Options) {
        self.music_volume = options.music_volume as f32 * MUSIC_STEP;
        self.sound_volume = options.sound_volume as f32 / 10.0;
        for track in self.music.iter().chain([&self.rush_hour]) {
            set_sound_volume(track, self.music_volume);
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
                volume: self.music_volume,
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
        play_sound(
            &self.effects[cue as usize],
            PlaySoundParams {
                looped: false,
                volume: self.sound_volume,
            },
        );
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
    fn every_track_is_90_seconds_and_never_clips() {
        for (name, music) in TRACKS.iter().zip(tracks()) {
            assert_eq!(music.len(), 48 * BAR, "{name}");
            assert_eq!(music.len() / RATE as usize, 90, "{name}");
            assert!(peak(&music) > 0.3, "{name} is too quiet");
            assert!(peak(&music) <= 1.0, "{name} clips");
        }
    }

    #[test]
    fn every_section_is_audible_and_the_breakdown_drops_the_low_end() {
        let low_end = |samples: &[f32]| {
            let mut low = samples.to_vec();
            low_pass(&mut low, |_| 80.0);
            low.iter().map(|s| s * s).sum::<f32>()
        };
        for (name, music) in TRACKS.iter().zip(tracks()) {
            let mut rest = music.as_slice();
            let sections: Vec<&[f32]> = ARRANGEMENT
                .iter()
                .map(|&(_, bars)| {
                    let (section, tail) = rest.split_at(bars * BAR);
                    rest = tail;
                    section
                })
                .collect();

            for section in &sections {
                assert!(peak(section) > 0.3, "{name}");
            }
            // Per bar, the breakdown has far less low end than the groove.
            let per_bar = |i: usize| low_end(sections[i]) / ARRANGEMENT[i].1 as f32;
            assert!(per_bar(3) * 5.0 < per_bar(2), "{name}");
        }
    }

    #[test]
    fn low_pass_passes_steady_signals_and_softens_high_notes() {
        let mut steady = vec![0.5; 5000];
        low_pass(&mut steady, |_| 200.0);
        assert!((steady[4999] - 0.5).abs() < 0.001);

        let square = note(Wave::Square, 81, 5000, 0.5);
        let filtered = |cutoff: f32| {
            let mut samples = square.clone();
            low_pass(&mut samples, |_| cutoff);
            peak(&samples[1000..])
        };
        assert!(filtered(200.0) < filtered(2000.0));
        assert!(filtered(20_000.0) > 0.4);
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
