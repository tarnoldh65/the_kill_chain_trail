//! Music and volume settings from the options screen, saved between games.

use crate::audio::TRACKS;
use crate::street::Hop;
use crate::ui::{Input, Music};

/// Rows on the options screen: music, music volume, game play sounds.
pub const ROWS: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Options {
    pub music: Music,
    /// 1 (quietest) to 10 (loudest).
    pub music_volume: u8,
    /// 0 (off) to 10 (loudest).
    pub sound_volume: u8,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            music: Music::Track(0),
            music_volume: 8,
            sound_volume: 10,
        }
    }
}

/// Music by its 0-based place on the options screen; off comes after the last track.
fn music_at(i: usize) -> Music {
    if i < TRACKS.len() {
        Music::Track(i)
    } else {
        Music::Off
    }
}

fn music_index(music: Music) -> usize {
    match music {
        Music::Track(i) => i,
        Music::Off => TRACKS.len(),
    }
}

impl Options {
    /// Applies a key on the options screen: number keys pick the music, LEFT/RIGHT step the
    /// selected row.
    pub fn adjust(self, row: usize, input: Input) -> Self {
        let step = |value: u8, min: u8, up: bool| {
            if up {
                (value + 1).min(10)
            } else {
                value.saturating_sub(1).max(min)
            }
        };
        match (row, input) {
            (_, Input::Char(c @ '1'..='9')) if c as usize - '1' as usize <= TRACKS.len() => Self {
                music: music_at(c as usize - '1' as usize),
                ..self
            },
            (row, Input::Arrow(hop @ (Hop::Left | Hop::Right))) => {
                let up = hop == Hop::Right;
                match row {
                    0 => {
                        let i = music_index(self.music);
                        let i = if up {
                            (i + 1).min(TRACKS.len())
                        } else {
                            i.saturating_sub(1)
                        };
                        Self {
                            music: music_at(i),
                            ..self
                        }
                    }
                    1 => Self {
                        music_volume: step(self.music_volume, 1, up),
                        ..self
                    },
                    _ => Self {
                        sound_volume: step(self.sound_volume, 0, up),
                        ..self
                    },
                }
            }
            _ => self,
        }
    }
}

/// One line as `music music_volume sound_volume`, with music numbered as on screen.
fn encode(options: Options) -> String {
    format!(
        "{} {} {}",
        music_index(options.music) + 1,
        options.music_volume,
        options.sound_volume
    )
}

fn decode(text: &str) -> Options {
    let numbers: Vec<usize> = text
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();
    match numbers[..] {
        [music @ 1..=9, music_volume @ 1..=10, sound_volume @ 0..=10]
            if music <= TRACKS.len() + 1 =>
        {
            Options {
                music: music_at(music - 1),
                music_volume: music_volume as u8,
                sound_volume: sound_volume as u8,
            }
        }
        _ => Options::default(),
    }
}

pub fn load() -> Options {
    decode(&crate::storage::read("options"))
}

pub fn save(options: Options) {
    crate::storage::write("options", &encode(options));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_keys_pick_music_from_any_row() {
        let options = Options::default();
        for row in 0..ROWS {
            assert_eq!(options.adjust(row, Input::Char('3')).music, Music::Track(2));
            assert_eq!(options.adjust(row, Input::Char('4')).music, Music::Off);
            assert_eq!(options.adjust(row, Input::Char('5')), options);
        }
    }

    #[test]
    fn left_and_right_step_the_selected_row_within_its_range() {
        let mut options = Options::default();
        for _ in 0..20 {
            options = options.adjust(0, Input::Arrow(Hop::Right));
            options = options.adjust(1, Input::Arrow(Hop::Left));
            options = options.adjust(2, Input::Arrow(Hop::Left));
        }
        assert_eq!(options.music, Music::Off);
        assert_eq!(options.music_volume, 1);
        assert_eq!(options.sound_volume, 0);

        for _ in 0..20 {
            options = options.adjust(0, Input::Arrow(Hop::Left));
            options = options.adjust(1, Input::Arrow(Hop::Right));
            options = options.adjust(2, Input::Arrow(Hop::Right));
        }
        assert_eq!(options.music, Music::Track(0));
        assert_eq!(options.music_volume, 10);
        assert_eq!(options.sound_volume, 10);
    }

    #[test]
    fn options_survive_encoding() {
        let options = Options {
            music: Music::Off,
            music_volume: 3,
            sound_volume: 0,
        };
        assert_eq!(decode(&encode(options)), options);
        assert_eq!(decode(&encode(Options::default())), Options::default());
        assert_eq!(decode(""), Options::default());
        assert_eq!(decode("9 5 5"), Options::default());
        assert_eq!(decode("1 0 5"), Options::default());
        assert_eq!(decode("garbage"), Options::default());
    }
}
