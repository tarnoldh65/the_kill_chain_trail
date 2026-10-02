//! The top-ten scoreboard of IPOs, kept in a file natively and in localStorage in the browser.

use crate::game::GameState;

const KEEP: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Score {
    pub points: i64,
    pub company: String,
    pub lead: String,
}

/// Adds a finished game if it reached the IPO, keeping the ten best, highest first.
pub fn record(scores: &mut Vec<Score>, game: &GameState) {
    let Some(points) = game.score() else {
        return;
    };
    scores.push(Score {
        points,
        company: game.company.clone(),
        lead: game.lead.clone(),
    });
    scores.sort_by_key(|s| std::cmp::Reverse(s.points));
    scores.truncate(KEEP);
}

/// One score per line as `points<TAB>company<TAB>lead`; names never contain tabs.
fn encode(scores: &[Score]) -> String {
    scores
        .iter()
        .map(|s| format!("{}\t{}\t{}", s.points, s.company, s.lead))
        .collect::<Vec<_>>()
        .join("\n")
}

fn decode(text: &str) -> Vec<Score> {
    text.lines()
        .filter_map(|line| {
            let mut fields = line.split('\t');
            Some(Score {
                points: fields.next()?.parse().ok()?,
                company: fields.next()?.to_string(),
                lead: fields.next()?.to_string(),
            })
        })
        .collect()
}

pub fn load() -> Vec<Score> {
    decode(&read())
}

pub fn save(scores: &[Score]) {
    write(&encode(scores));
}

#[cfg(not(target_arch = "wasm32"))]
const FILE: &str = "top_ten.txt";

#[cfg(not(target_arch = "wasm32"))]
fn read() -> String {
    std::fs::read_to_string(FILE).unwrap_or_default()
}

/// A scoreboard that cannot be saved is not worth crashing over.
#[cfg(not(target_arch = "wasm32"))]
fn write(text: &str) {
    std::fs::write(FILE, text).ok();
}

// Provided by web/top-ten.js.
#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn top_ten_read(buffer: *mut u8, capacity: u32) -> u32;
    fn top_ten_write(text: *const u8, length: u32);
}

/// Lets the JS bundle check that web/top-ten.js matches this build.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
extern "C" fn top_ten_crate_version() -> u32 {
    1
}

#[cfg(target_arch = "wasm32")]
fn read() -> String {
    let mut buffer = vec![0; 4096];
    let length = unsafe { top_ten_read(buffer.as_mut_ptr(), buffer.len() as u32) };
    buffer.truncate(length as usize);
    String::from_utf8(buffer).unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
fn write(text: &str) {
    unsafe { top_ten_write(text.as_ptr(), text.len() as u32) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{Outcome, Profile};

    fn finished(company: &str, valuation: i64, outcome: Outcome) -> GameState {
        let mut game = GameState::new(company, "Alex", Profile::Fintech, 1);
        game.valuation = valuation;
        game.outcome = Some(outcome);
        game
    }

    #[test]
    fn the_board_keeps_the_ten_best_ipos_highest_first() {
        let mut scores = Vec::new();
        for i in 1..=12 {
            record(
                &mut scores,
                &finished(&format!("Co{i}"), i * 100_000_000, Outcome::Ipo),
            );
        }

        assert_eq!(scores.len(), 10);
        assert_eq!(scores[0].points, 1200);
        assert_eq!(scores[0].company, "Co12");
        assert_eq!(scores[9].points, 300);
        assert!(scores.windows(2).all(|w| w[0].points >= w[1].points));
    }

    #[test]
    fn other_endings_never_make_the_board() {
        let mut scores = Vec::new();
        for outcome in [
            Outcome::Pulled,
            Outcome::ShutDown,
            Outcome::Fired,
            Outcome::TeamCollapsed,
            Outcome::Bankrupt,
        ] {
            record(&mut scores, &finished("Acme", 2_000_000_000, outcome));
        }

        assert!(scores.is_empty());
    }

    #[test]
    fn scores_survive_encoding() {
        let mut scores = Vec::new();
        record(
            &mut scores,
            &finished("Acme Corp", 1_500_000_000, Outcome::Ipo),
        );
        record(&mut scores, &finished("Initech", 900_000_000, Outcome::Ipo));

        assert_eq!(decode(&encode(&scores)), scores);
        assert!(decode("").is_empty());
        assert!(decode("garbage\nmore").is_empty());
    }
}
