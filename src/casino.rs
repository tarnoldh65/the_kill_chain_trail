//! Vendor casino night: a 5x5 slot machine where the SOC gambles chips from its budget
//! on a tool or service it has not bought. The five rows are the betting lines.

use crate::game::{Item, roll};

/// Chips the player can take from the budget.
pub const STAKES: [i64; 3] = [2_000, 4_000, 10_000];
/// What each betting line costs per spin, and what a cash-back line pays.
pub const LINE_BET: i64 = 100;
pub const CASH_BACK: i64 = 125;
/// Odds of cash back on a bet line that missed the jackpot.
const CASH_ODDS: u32 = 10;
pub const REELS: usize = 5;
/// Symbols on the reels, the sprites in `art::SYMBOLS`.
pub const SYMBOLS: usize = 10;
/// Milliseconds the first reel spins, and how much longer each later reel spins.
const SPIN_MS: u32 = 700;
const STAGGER_MS: u32 = 250;
/// Odds in hundredths that a losing line shows three of a kind.
const THREE_ODDS: u32 = 60;
/// Odds in hundredths that a line not bet on shows four or five of a kind.
const TEASE_ODDS: u32 = 15;

/// Prizes the house puts up: anything not owned yet, except the coffee.
pub fn prizes(owned: &[Item], seed: u32) -> Vec<Item> {
    let mut left: Vec<Item> = Item::ALL
        .into_iter()
        .filter(|i| !matches!(i, Item::Coffee | Item::CoffeeSubscription) && !owned.contains(i))
        .collect();
    let mut picked = Vec::new();
    for salt in 0..3 {
        if left.is_empty() {
            break;
        }
        picked.push(left.remove(roll(seed, salt) as usize % left.len()));
    }
    picked
}

/// Jackpot odds per bet line, 1 in this many: longer for pricier prizes.
pub fn odds(prize: Item) -> u32 {
    match prize.price() {
        ..30_000 => 200,
        30_000..=60_000 => 300,
        _ => 400,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bet {
    Center,
    Middle,
    All,
}

impl Bet {
    /// The rows it bets on.
    pub fn lines(self) -> &'static [usize] {
        match self {
            Self::Center => &[2],
            Self::Middle => &[1, 2, 3],
            Self::All => &[0, 1, 2, 3, 4],
        }
    }

    pub fn cost(self) -> i64 {
        self.lines().len() as i64 * LINE_BET
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum End {
    Jackpot,
    /// The chips no longer cover a bet.
    Busted,
    /// Walked out early, paying the house its surcharge.
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Casino {
    pub prize: Item,
    pub stake: i64,
    pub chips: i64,
    /// Symbols by row, then reel.
    pub grid: [[usize; REELS]; 5],
    /// The last spin's bet and its winning lines.
    pub bet: Option<Bet>,
    pub wins: Vec<usize>,
    /// Milliseconds since the reels started, while they spin.
    pub spin_ms: Option<u32>,
    /// What the spin pays once the reels stop.
    payout: i64,
    jackpot: bool,
    pub end: Option<End>,
    seed: u32,
    rolls: u32,
}

impl Casino {
    pub fn new(prize: Item, stake: i64, seed: u32) -> Self {
        let mut casino = Self {
            prize,
            stake,
            chips: stake,
            grid: [[0; REELS]; 5],
            bet: None,
            wins: Vec::new(),
            spin_ms: None,
            payout: 0,
            jackpot: false,
            end: None,
            seed,
            rolls: 0,
        };
        for row in 0..5 {
            let matches = casino.miss();
            casino.grid[row] = casino.line(matches);
        }
        casino
    }

    fn below(&mut self, n: u32) -> u32 {
        self.rolls += 1;
        roll(self.seed, self.rolls) % n
    }

    /// Whether a spin of this bet can start now.
    pub fn can_spin(&self, bet: Bet) -> bool {
        self.end.is_none() && self.spin_ms.is_none() && self.chips >= bet.cost()
    }

    /// Pays for the bet and spins: each bet line rolls for the jackpot, then cash back,
    /// and the grid is drawn to show it. Lines not bet on sometimes tease a win.
    pub fn spin(&mut self, bet: Bet) {
        if !self.can_spin(bet) {
            return;
        }
        self.chips -= bet.cost();
        self.bet = Some(bet);
        self.wins.clear();
        self.payout = 0;
        self.jackpot = false;
        for row in 0..5 {
            let matches = if bet.lines().contains(&row) {
                if self.below(odds(self.prize)) == 0 {
                    self.jackpot = true;
                    5
                } else if self.below(CASH_ODDS) == 0 {
                    self.payout += CASH_BACK;
                    4
                } else {
                    self.miss()
                }
            } else if self.below(100) < TEASE_ODDS {
                4 + self.below(2) as usize
            } else {
                self.miss()
            };
            if bet.lines().contains(&row) && matches >= 4 {
                self.wins.push(row);
            }
            self.grid[row] = self.line(matches);
        }
        self.spin_ms = Some(0);
    }

    /// How many of a kind a losing line shows: often three, otherwise two.
    fn miss(&mut self) -> usize {
        if self.below(100) < THREE_ODDS { 3 } else { 2 }
    }

    /// A line whose most common symbol shows exactly `matches` times.
    fn line(&mut self, matches: usize) -> [usize; REELS] {
        let symbol = self.below(SYMBOLS as u32) as usize;
        let mut others: Vec<usize> = (0..SYMBOLS).filter(|&s| s != symbol).collect();
        let mut line = [symbol; REELS];
        let mut reels: Vec<usize> = (0..REELS).collect();
        for _ in matches..REELS {
            let reel = reels.remove(self.below(reels.len() as u32) as usize);
            line[reel] = others.remove(self.below(others.len() as u32) as usize);
        }
        line
    }

    /// Whether a reel has stopped spinning.
    pub fn stopped(&self, reel: usize) -> bool {
        self.spin_ms
            .is_none_or(|ms| ms >= SPIN_MS + STAGGER_MS * reel as u32)
    }

    /// Spins the reels on; when the last one stops, the spin pays and the night may end.
    pub fn tick(&mut self, dt: f32) {
        let Some(ms) = self.spin_ms else {
            return;
        };
        let ms = ms + (dt * 1000.0) as u32;
        self.spin_ms = Some(ms);
        if !self.stopped(REELS - 1) {
            return;
        }
        self.spin_ms = None;
        self.chips += self.payout;
        if self.jackpot {
            self.end = Some(End::Jackpot);
        } else if self.chips < LINE_BET {
            self.end = Some(End::Busted);
        }
    }

    /// Walks out between spins.
    pub fn leave(&mut self) {
        if self.end.is_none() && self.spin_ms.is_none() {
            self.end = Some(End::Left);
        }
    }

    /// The chips that go back into the budget when the night ends.
    pub fn returned(&self) -> i64 {
        match self.end {
            Some(End::Jackpot) => self.chips,
            Some(End::Left) => self.chips / 2,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The most times one symbol shows on a line.
    fn matches(line: &[usize; REELS]) -> usize {
        (0..SYMBOLS)
            .map(|s| line.iter().filter(|&&x| x == s).count())
            .max()
            .unwrap()
    }

    fn finish(casino: &mut Casino) {
        casino.tick(10.0);
    }

    #[test]
    fn prizes_are_three_distinct_unowned_items_and_never_coffee() {
        for seed in 0..200 {
            let owned = [Item::Siem, Item::Edr];
            let prizes = prizes(&owned, seed);
            assert_eq!(prizes.len(), 3);
            assert!(prizes[0] != prizes[1] && prizes[1] != prizes[2] && prizes[0] != prizes[2]);
            for prize in prizes {
                assert!(!owned.contains(&prize));
                assert!(!matches!(prize, Item::Coffee | Item::CoffeeSubscription));
            }
        }
        let all_but_one: Vec<Item> = Item::ALL.into_iter().filter(|&i| i != Item::Waf).collect();
        assert_eq!(prizes(&all_but_one, 1), [Item::Waf]);
    }

    #[test]
    fn pricier_prizes_have_longer_odds() {
        assert_eq!(odds(Item::Posters), 200);
        assert_eq!(odds(Item::IntelFeeds), 300);
        assert_eq!(odds(Item::Edr), 300);
        assert_eq!(odds(Item::Siem), 400);
    }

    #[test]
    fn a_spin_costs_a_hundred_per_line_and_is_refused_when_short() {
        let mut casino = Casino::new(Item::Siem, 2_000, 1);
        casino.spin(Bet::All);
        assert_eq!(casino.chips, 1_500, "winnings wait for the reels");
        assert!(!casino.can_spin(Bet::Center), "still spinning");
        finish(&mut casino);

        casino.chips = 250;
        assert!(!casino.can_spin(Bet::Middle));
        assert!(casino.can_spin(Bet::Center));
        casino.spin(Bet::Middle);
        assert_eq!(casino.chips, 250);
    }

    #[test]
    fn wins_come_at_their_odds_and_pay_back_on_four_of_a_kind() {
        let (mut jackpots, mut cash, mut spins) = (0, 0, 0);
        for seed in 0..40 {
            let mut casino = Casino::new(Item::Posters, 10_000_000, seed);
            for _ in 0..2_500 {
                let chips = casino.chips;
                casino.spin(Bet::All);
                finish(&mut casino);
                spins += 5;
                for &row in &casino.wins {
                    match matches(&casino.grid[row]) {
                        5 => jackpots += 1,
                        4 => cash += 1,
                        n => panic!("a win with {n} of a kind"),
                    }
                }
                let won = casino
                    .wins
                    .iter()
                    .filter(|&&r| matches(&casino.grid[r]) == 4);
                assert_eq!(casino.chips, chips - 500 + CASH_BACK * won.count() as i64);
                casino.end = None;
            }
        }
        // 500,000 lines: about 2,500 jackpots at 1 in 200, and cash back 1 in 10 of the rest.
        assert!((2_200..2_800).contains(&jackpots), "{jackpots}");
        assert!((48_500..51_000).contains(&cash), "{cash}");
        assert!(spins == 500_000);
    }

    #[test]
    fn losing_lines_show_three_and_unbet_lines_tease() {
        let (mut threes, mut teases, mut lines) = (0, 0, 0);
        let mut casino = Casino::new(Item::Siem, 10_000_000, 3);
        for _ in 0..2_000 {
            casino.spin(Bet::Center);
            finish(&mut casino);
            for row in 0..5 {
                let n = matches(&casino.grid[row]);
                if row == 2 {
                    assert_eq!(n >= 4, casino.wins == [2], "only wins show four or more");
                } else {
                    lines += 1;
                    teases += (n >= 4) as usize;
                }
                threes += (n == 3) as usize;
            }
            casino.end = None;
        }
        assert!(teases * 100 / lines >= 10, "{teases} of {lines}");
        assert!(threes * 100 / 10_000 >= 40, "{threes}");
    }

    #[test]
    fn the_night_ends_on_a_jackpot_a_bust_or_walking_out() {
        let mut casino = Casino::new(Item::Posters, 2_000, 1);
        casino.jackpot = true;
        casino.spin_ms = Some(0);
        finish(&mut casino);
        assert_eq!(casino.end, Some(End::Jackpot));
        assert_eq!(casino.returned(), 2_000);

        let mut casino = Casino::new(Item::Siem, 2_000, 1);
        casino.leave();
        assert_eq!(casino.end, Some(End::Left));
        assert_eq!(casino.returned(), 1_000, "a 50% surcharge");

        let casino = (0..)
            .map(|seed| {
                let mut casino = Casino::new(Item::Siem, 2_000, seed);
                while casino.end.is_none() {
                    casino.spin(Bet::Center);
                    finish(&mut casino);
                }
                casino
            })
            .find(|c| c.end != Some(End::Jackpot))
            .unwrap();
        assert_eq!(casino.end, Some(End::Busted));
        assert!(casino.chips < LINE_BET);
        assert_eq!(casino.returned(), 0, "the house keeps the change");
    }

    #[test]
    fn reels_stop_left_to_right() {
        let mut casino = Casino::new(Item::Siem, 2_000, 1);
        casino.spin(Bet::Center);
        assert!(!casino.stopped(0));
        casino.tick(SPIN_MS as f32 / 1000.0);
        assert!(casino.stopped(0) && !casino.stopped(1));
        casino.tick(1.0);
        assert!((0..REELS).all(|r| casino.stopped(r)));
        assert_eq!(casino.spin_ms, None);
    }

    #[test]
    fn the_same_seed_plays_the_same_night() {
        let night = |seed| {
            let mut casino = Casino::new(Item::Siem, 4_000, seed);
            for _ in 0..10 {
                casino.spin(Bet::Middle);
                finish(&mut casino);
            }
            casino
        };
        assert_eq!(night(9), night(9));
        assert_ne!(night(9).grid, night(10).grid);
    }
}
