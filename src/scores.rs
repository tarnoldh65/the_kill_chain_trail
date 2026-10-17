//! The top-ten scoreboard of IPOs, saved through `storage`.

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
    decode(&crate::storage::read("top_ten"))
}

pub fn save(scores: &[Score]) {
    crate::storage::write("top_ten", &encode(scores));
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
