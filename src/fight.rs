//! Fight for budget: a one-round street fight against the boss. The player starts on the
//! left facing right; positions are fighter centers in pixels across the 640px screen.

use crate::game::roll;
use crate::street::Hop;

pub const WIDTH: f32 = 640.0;
/// The amounts the player can fight for, from the easiest boss to the hardest.
pub const STAKES: [i64; 4] = [10_000, 40_000, 75_000, 100_000];
pub const ROUND_SECONDS: f32 = 60.0;
pub const PLAYER_HEALTH: i32 = 100;
/// Half a fighter's body width; centers stay this far from the walls.
pub const HALF_BODY: f32 = 32.0;
/// Closest two fighter centers can get.
const CLOSEST: f32 = 2.0 * HALF_BODY;
const PLAYER_SPEED: f32 = 160.0;
const STUN_SECONDS: f32 = 0.3;
const KNOCKBACK: f32 = 24.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strike {
    Punch,
    Kick,
}

impl Strike {
    /// Seconds of wind-up, then of the active window when it can land, then of recovery.
    fn timing(self) -> (f32, f32, f32) {
        match self {
            Self::Punch => (0.08, 0.1, 0.15),
            Self::Kick => (0.16, 0.12, 0.25),
        }
    }

    /// How far from the attacker's center it reaches.
    pub fn reach(self) -> f32 {
        match self {
            Self::Punch => 100.0,
            Self::Kick => 124.0,
        }
    }

    /// Damage at full strength; crouching attacks do a little less.
    fn damage(self, low: bool) -> i32 {
        match (self, low) {
            (Self::Punch, false) => 6,
            (Self::Punch, true) => 4,
            (Self::Kick, false) => 10,
            (Self::Kick, true) => 7,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Attack {
    pub strike: Strike,
    /// Thrown from a crouch, so it hits low and lands on either stance.
    pub low: bool,
    pub elapsed: f32,
    landed: bool,
}

impl Attack {
    fn new(strike: Strike, low: bool) -> Self {
        Self {
            strike,
            low,
            elapsed: 0.0,
            landed: false,
        }
    }

    /// Whether the limb is out: past the wind-up and not yet finished.
    pub fn extended(&self) -> bool {
        self.elapsed >= self.strike.timing().0
    }

    fn active(&self) -> bool {
        let (wind_up, active, _) = self.strike.timing();
        (wind_up..wind_up + active).contains(&self.elapsed)
    }

    fn done(&self) -> bool {
        let (wind_up, active, recovery) = self.strike.timing();
        self.elapsed >= wind_up + active + recovery
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fighter {
    pub x: f32,
    pub crouching: bool,
    pub attack: Option<Attack>,
    pub health: i32,
    /// Seconds left reeling from a hit, unable to act.
    pub stun: f32,
}

impl Fighter {
    fn new(x: f32, health: i32) -> Self {
        Self {
            x,
            crouching: false,
            attack: None,
            health,
            stun: 0.0,
        }
    }

    fn free(&self) -> bool {
        self.attack.is_none() && self.stun <= 0.0
    }

    fn throw(&mut self, strike: Strike) {
        if self.free() {
            self.attack = Some(Attack::new(strike, self.crouching));
        }
    }

    fn crouch(&mut self, crouching: bool) {
        if self.free() {
            self.crouching = crouching;
        }
    }
}

/// How tough the boss is at one stake.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tier {
    pub title: &'static str,
    pub health: i32,
    /// Percent of the base damage the boss deals.
    damage: i32,
    speed: f32,
    /// Seconds between the boss's decisions.
    think: f32,
    /// Percent chance of ducking a standing attack.
    duck: u32,
}

pub const TIERS: [Tier; 4] = [
    Tier {
        title: "THE CISO",
        health: 50,
        damage: 50,
        speed: 70.0,
        think: 0.9,
        duck: 10,
    },
    Tier {
        title: "THE CIO",
        health: 80,
        damage: 80,
        speed: 95.0,
        think: 0.7,
        duck: 25,
    },
    Tier {
        title: "THE CEO",
        health: 110,
        damage: 100,
        speed: 115.0,
        think: 0.5,
        duck: 40,
    },
    Tier {
        title: "THE BOARD CHAIR",
        health: 140,
        damage: 120,
        speed: 135.0,
        think: 0.35,
        duck: 55,
    },
];

#[derive(Debug, Clone, PartialEq)]
pub struct Fight {
    /// Index into `STAKES` and `TIERS`.
    pub stake: usize,
    pub player: Fighter,
    pub boss: Fighter,
    pub time_left: f32,
    /// Set once the round is over.
    pub won: Option<bool>,
    /// Seconds until the boss decides again.
    think: f32,
    /// The boss's walking direction until its next decision: forward (left) is negative.
    walk: f32,
    seed: u32,
    rolls: u32,
}

impl Eq for Fight {}

impl Fight {
    pub fn new(stake: usize, seed: u32) -> Self {
        Self {
            stake,
            player: Fighter::new(160.0, PLAYER_HEALTH),
            boss: Fighter::new(WIDTH - 160.0, TIERS[stake].health),
            time_left: ROUND_SECONDS,
            won: None,
            think: TIERS[stake].think,
            walk: 0.0,
            seed,
            rolls: 0,
        }
    }

    /// The next number in this fight's random sequence, from 0 to 99.
    fn percent(&mut self) -> u32 {
        self.rolls += 1;
        roll(self.seed, self.rolls) % 100
    }

    pub fn tier(&self) -> Tier {
        TIERS[self.stake]
    }

    pub fn amount(&self) -> i64 {
        STAKES[self.stake]
    }

    /// UP stands, DOWN crouches, Q punches, E kicks.
    pub fn press(&mut self, input: Input) {
        if self.won.is_some() {
            return;
        }
        match input {
            Input::Stand => self.player.crouch(false),
            Input::Crouch => self.player.crouch(true),
            Input::Punch | Input::Kick => {
                let strike = if input == Input::Punch {
                    Strike::Punch
                } else {
                    Strike::Kick
                };
                let was_free = self.player.free();
                self.player.throw(strike);
                // A sharp boss sees a standing attack coming and ducks under it.
                if was_free
                    && !self.player.crouching
                    && self.boss.free()
                    && self.percent() < self.tier().duck
                {
                    self.boss.crouching = true;
                    self.walk = 0.0;
                }
            }
        }
    }

    /// Runs the round on: walking, attacks, the boss, and the clock.
    pub fn tick(&mut self, dt: f32, held: Option<Hop>) {
        if self.won.is_some() {
            return;
        }
        if self.player.free() && !self.player.crouching {
            match held {
                Some(Hop::Left) => self.walk_player(-PLAYER_SPEED * dt),
                Some(Hop::Right) => self.walk_player(PLAYER_SPEED * dt),
                _ => {}
            }
        }
        self.think_boss(dt);

        let damage = self.tier().damage;
        swing(&mut self.player, &mut self.boss, dt, 100, 1.0);
        swing(&mut self.boss, &mut self.player, dt, damage, -1.0);
        self.player.x = self.player.x.clamp(HALF_BODY, self.boss.x - CLOSEST);
        self.boss.x = self
            .boss
            .x
            .clamp(self.player.x + CLOSEST, WIDTH - HALF_BODY);

        self.time_left = (self.time_left - dt).max(0.0);
        if self.boss.health <= 0 {
            self.won = Some(true);
        } else if self.player.health <= 0 {
            self.won = Some(false);
        } else if self.time_left == 0.0 {
            // The boss keeps the budget on a tie.
            self.won =
                Some(self.player.health * self.tier().health > self.boss.health * PLAYER_HEALTH);
        }
    }

    fn walk_player(&mut self, dx: f32) {
        self.player.x = (self.player.x + dx).clamp(HALF_BODY, self.boss.x - CLOSEST);
    }

    fn think_boss(&mut self, dt: f32) {
        let tier = self.tier();
        if self.boss.free() && !self.boss.crouching {
            self.boss.x = (self.boss.x + self.walk * tier.speed * dt)
                .clamp(self.player.x + CLOSEST, WIDTH - HALF_BODY);
        }
        self.think -= dt;
        if self.think > 0.0 || !self.boss.free() {
            return;
        }
        self.think = tier.think * (0.6 + self.percent() as f32 * 0.008);
        let distance = self.boss.x - self.player.x;
        if self.boss.health < tier.health && self.percent() < 15 {
            // Back off to catch a breath.
            self.boss.crouching = false;
            self.walk = 1.0;
        } else if distance > Strike::Kick.reach() {
            self.boss.crouching = false;
            self.walk = -1.0;
        } else {
            self.walk = 0.0;
            self.boss.crouching = self.percent() < 30;
            let strike = if distance > Strike::Punch.reach() || self.percent() < 50 {
                Strike::Kick
            } else {
                Strike::Punch
            };
            self.boss.throw(strike);
        }
    }

    /// Health shown on a 0-100 scale, so bars of different bosses compare.
    pub fn boss_percent(&self) -> i32 {
        self.boss.health.max(0) * 100 / self.tier().health
    }
}

/// Advances one fighter's attack and lands it on the other if it connects. `facing` is
/// the direction the attacker faces, and `damage` a percent of the base damage.
fn swing(attacker: &mut Fighter, target: &mut Fighter, dt: f32, damage: i32, facing: f32) {
    attacker.stun = (attacker.stun - dt).max(0.0);
    let Some(attack) = &mut attacker.attack else {
        return;
    };
    attack.elapsed += dt;
    let in_reach = (target.x - attacker.x).abs() <= attack.strike.reach();
    // Standing attacks go over a crouching target.
    let connects = attack.low || !target.crouching;
    if attack.active() && !attack.landed && in_reach && connects {
        attack.landed = true;
        target.health -= attack.strike.damage(attack.low) * damage / 100;
        target.stun = STUN_SECONDS;
        target.attack = None;
        target.x += facing * KNOCKBACK;
    }
    if attack.done() {
        attacker.attack = None;
    }
}

/// The fight's controls, translated from keys by the screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Stand,
    Crouch,
    Punch,
    Kick,
}

#[cfg(test)]
mod tests {
    use super::*;

    const FRAME: f32 = 1.0 / 60.0;

    fn run(fight: &mut Fight, seconds: f32, held: Option<Hop>) {
        for _ in 0..(seconds / FRAME) as usize {
            fight.tick(FRAME, held);
        }
    }

    /// A fight with a boss that never decides anything, so the player can be tested alone.
    fn still(stake: usize) -> Fight {
        let mut fight = Fight::new(stake, 7);
        fight.think = f32::INFINITY;
        fight
    }

    #[test]
    fn holding_left_and_right_walks_within_the_screen_without_passing_the_boss() {
        let mut fight = still(0);
        run(&mut fight, 5.0, Some(Hop::Left));
        assert_eq!(fight.player.x, HALF_BODY);

        run(&mut fight, 5.0, Some(Hop::Right));
        assert_eq!(fight.player.x, fight.boss.x - CLOSEST);

        fight.boss.x = WIDTH - HALF_BODY;
        run(&mut fight, 5.0, Some(Hop::Right));
        assert_eq!(fight.player.x, WIDTH - HALF_BODY - CLOSEST);
        assert!(fight.player.x < fight.boss.x);
    }

    #[test]
    fn crouching_stops_walking() {
        let mut fight = still(0);
        fight.press(Input::Crouch);
        run(&mut fight, 1.0, Some(Hop::Right));
        assert_eq!(fight.player.x, 160.0);

        fight.press(Input::Stand);
        run(&mut fight, 0.2, Some(Hop::Right));
        assert!(fight.player.x > 160.0);
    }

    /// The boss within reach of every attack, health tracked from full.
    fn close(boss_crouching: bool) -> Fight {
        let mut fight = still(3);
        fight.boss.x = fight.player.x + CLOSEST;
        fight.boss.crouching = boss_crouching;
        fight
    }

    #[test]
    fn standing_attacks_miss_a_crouching_target_and_low_attacks_hit_both() {
        let full = TIERS[3].health;
        for strike in [Input::Punch, Input::Kick] {
            let mut fight = close(true);
            fight.press(strike);
            run(&mut fight, 0.5, None);
            assert_eq!(fight.boss.health, full, "{strike:?} high on a crouch");

            for crouching in [false, true] {
                let mut fight = close(crouching);
                fight.press(Input::Crouch);
                fight.press(strike);
                run(&mut fight, 0.5, None);
                assert!(fight.boss.health < full, "{strike:?} low");
            }
        }
    }

    #[test]
    fn an_attack_lands_only_once_and_only_in_its_active_window() {
        let mut fight = close(false);
        fight.player.throw(Strike::Kick);
        run(&mut fight, 0.1, None);
        assert_eq!(fight.boss.health, TIERS[3].health, "still winding up");

        run(&mut fight, 0.5, None);
        assert_eq!(fight.boss.health, TIERS[3].health - 10);
        assert!(fight.player.attack.is_none());
    }

    #[test]
    fn attacks_out_of_reach_miss() {
        let mut fight = still(0);
        fight.boss.x = fight.player.x + Strike::Kick.reach() + 1.0;
        fight.press(Input::Kick);
        run(&mut fight, 0.6, None);
        assert_eq!(fight.boss.health, TIERS[0].health);
    }

    #[test]
    fn knocking_out_the_boss_wins_and_ends_the_round() {
        let mut fight = close(false);
        fight.boss.health = 1;
        fight.player.throw(Strike::Punch);
        run(&mut fight, 0.5, None);
        assert_eq!(fight.won, Some(true));

        fight.player.attack = None;
        fight.press(Input::Punch);
        assert!(fight.player.attack.is_none());
    }

    #[test]
    fn time_up_goes_to_the_healthier_fighter_and_a_tie_to_the_boss() {
        for (player, boss, won) in [(50, 40, true), (40, 50, false), (50, 50, false)] {
            let mut fight = still(1);
            fight.player.health = player;
            // Health compares as a share of each fighter's full health.
            fight.boss.health = boss * TIERS[1].health / 100;
            run(&mut fight, ROUND_SECONDS + 1.0, None);
            assert_eq!(fight.won, Some(won), "{player} vs {boss}");
        }
    }

    #[test]
    fn the_boss_beats_an_idle_player_at_every_tier_and_tougher_bosses_faster() {
        let mut times = Vec::new();
        for stake in 0..STAKES.len() {
            let mut fight = Fight::new(stake, 7);
            while fight.won.is_none() {
                fight.tick(FRAME, None);
            }
            assert_eq!(fight.won, Some(false), "tier {stake}");
            assert!(fight.player.health <= 0, "tier {stake} won by knockout");
            times.push(fight.time_left);
        }
        assert!(times.windows(2).all(|w| w[0] < w[1]), "{times:?}");
    }
}
