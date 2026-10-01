use std::fmt;

/// The day the IPO bell rings, 26 weeks after day 1.
pub const IPO_DAY: u32 = 182;

/// Pots of coffee the break room can hold.
pub const COFFEE_CAPACITY: i32 = 36;
/// Cost in dollars of sending the intern across the street for coffee.
pub const COFFEE_RUN_COST: i64 = 10;
/// Pots of coffee the intern brings back.
const COFFEE_RUN_POTS: i32 = 12;
/// Extra burnout everyone gains each day the coffee is out.
const NO_COFFEE_BURNOUT: i32 = 2;
/// Burnout each analyst sheds each day before their share of the workload.
const DAILY_REST: i32 = 1;
/// Trust below this gets the CISO fired.
const FIRING_TRUST: i32 = 30;
/// Budget in dollars below which the SOC Manager is made redundant.
const REDUNDANCY_BUDGET: i64 = 50_000;
/// Posture each area loses every Monday as systems drift.
const POSTURE_DECAY: i32 = 2;
/// Brand loyalty below this drags the valuation down every Monday.
const BRAND_DRAG_LEVEL: i32 = 50;
/// Valuation below this percentage of the base valuation pulls the IPO.
const PULLED_PERCENT: i64 = 40;

/// What happens to an intern who loses to traffic, rotated by week.
const INTERN_FATES: [&str; 4] = [
    "The intern was flattened by a delivery truck. The $10 and the coffee order are gone.",
    "The intern became a hood ornament. Legal says interns are unpaid, so this is technically fine.",
    "The intern was hit by a rideshare driver who then rated them one star. No coffee today.",
    "The intern crossed on \"Don't Walk\". Traffic did not stop. HR is already posting the job ad.",
];

/// Ways a burned out team member leaves, rotated so departures read differently.
const BURNOUT_EXITS: [&str; 5] = [
    "burned out and quit to open a llama sanctuary.",
    "had a heart attack when the CFO asked for \"just a quick update\". Doctors prescribed zero email.",
    "had a stroke after triaging 40,000 alerts. Doctors blame the \"informational\" ones.",
    "has died of dysentery. The break room fridge is now a crime scene.",
    "rage-quit and changed their LinkedIn headline to \"Goat Farmer\".",
];

/// Formats dollars the way the game shows money: $10, $18K, $1.5M, $1.20B.
pub fn money(dollars: i64) -> String {
    let d = dollars as f64;
    match dollars.abs() {
        1_000_000_000.. => format!("${:.2}B", d / 1e9),
        1_000_000.. => format!("${:.1}M", d / 1e6),
        1_000.. => format!("${}K", dollars / 1_000),
        _ => format!("${dollars}"),
    }
}

/// How hard the SOC is working, the game's "pace" setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tempo {
    Relaxed,
    Steady,
    Crunch,
}

impl Tempo {
    pub fn next(self) -> Self {
        match self {
            Self::Relaxed => Self::Steady,
            Self::Steady => Self::Crunch,
            Self::Crunch => Self::Relaxed,
        }
    }

    /// Burnout the analysts share each day.
    fn workload(self) -> i32 {
        match self {
            Self::Relaxed => 2,
            Self::Steady => 6,
            Self::Crunch => 12,
        }
    }

    /// Pots of coffee the SOC drinks each day.
    fn coffee(self) -> i32 {
        match self {
            Self::Relaxed => 1,
            Self::Steady => 2,
            Self::Crunch => 4,
        }
    }
}

impl fmt::Display for Tempo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Relaxed => "Relaxed",
            Self::Steady => "Steady",
            Self::Crunch => "Crunch",
        })
    }
}

/// Hidden security posture areas, never shown to the player.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Area {
    Identity,
    Endpoint,
    People,
    Perimeter,
    Resilience,
    Detection,
}

impl Area {
    pub const ALL: [Area; 6] = [
        Self::Identity,
        Self::Endpoint,
        Self::People,
        Self::Perimeter,
        Self::Resilience,
        Self::Detection,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Analyst,
    Manager,
    Ciso,
    Cio,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Analyst => "Analyst",
            Self::Manager => "Manager",
            Self::Ciso => "CISO",
            Self::Cio => "CIO",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamMember {
    pub name: String,
    pub role: Role,
    pub burnout: i32,
    /// Weekly pay from the SOC budget; leadership is paid by the company.
    pub salary: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Ipo,
    Pulled,
    ShutDown,
    Fired,
    TeamCollapsed,
    Bankrupt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub company: String,
    pub lead: String,
    /// Day 1 is a Monday; the game ends as an IPO on `IPO_DAY`.
    pub day: u32,
    pub base_valuation: i64,
    pub valuation: i64,
    pub trust: i32,
    pub brand: i32,
    pub budget: i64,
    pub coffee: i32,
    pub tempo: Tempo,
    /// Indexed by `Area`; kept private so it can never be drawn.
    posture: [i32; Area::ALL.len()],
    pub team: Vec<TeamMember>,
    pub log: Vec<String>,
    pub outcome: Option<Outcome>,
    /// Whether the intern has already been sent for coffee this week.
    pub coffee_run_made: bool,
}

impl GameState {
    pub fn new(company: &str, lead: &str) -> Self {
        let member = |name: &str, role, burnout, salary| TeamMember {
            name: name.to_string(),
            role,
            burnout,
            salary,
        };
        Self {
            company: company.to_string(),
            lead: lead.to_string(),
            day: 1,
            base_valuation: 1_000_000_000,
            valuation: 1_000_000_000,
            trust: 60,
            brand: 60,
            budget: 500_000,
            coffee: 24,
            tempo: Tempo::Steady,
            posture: [30; Area::ALL.len()],
            team: vec![
                member("Maya", Role::Analyst, 20, 6_000),
                member("Dev", Role::Analyst, 15, 6_000),
                member("Sam", Role::Analyst, 10, 6_000),
                member("Jules", Role::Manager, 20, 0),
                member("Ravi", Role::Ciso, 15, 0),
                member("Dana", Role::Cio, 10, 0),
            ],
            log: vec![format!(
                "{company} goes public in 26 weeks. {lead} takes command of the SOC."
            )],
            outcome: None,
            coffee_run_made: false,
        }
    }

    pub fn analysts(&self) -> i32 {
        self.team.iter().filter(|m| m.role == Role::Analyst).count() as i32
    }

    /// Days since the last Monday: 0 is Monday.
    pub fn weekday(&self) -> u32 {
        (self.day - 1) % 7
    }

    pub fn days_to_ipo(&self) -> u32 {
        IPO_DAY - self.day
    }

    fn payroll(&self) -> i64 {
        self.team.iter().map(|m| m.salary).sum()
    }

    /// The final score, only for reaching the IPO.
    pub fn score(&self) -> Option<i64> {
        (self.outcome == Some(Outcome::Ipo)).then_some(self.valuation)
    }

    pub fn coffee_full(&self) -> bool {
        self.coffee >= COFFEE_CAPACITY
    }

    pub fn can_send_intern(&self) -> bool {
        self.outcome.is_none()
            && !self.coffee_run_made
            && !self.coffee_full()
            && self.budget >= COFFEE_RUN_COST
    }

    /// Pays for a coffee run; the intern still has to survive the street.
    pub fn send_intern(&mut self) {
        self.budget -= COFFEE_RUN_COST;
        self.coffee_run_made = true;
    }

    pub fn intern_returns(&mut self, survived: bool) {
        if survived {
            self.coffee = (self.coffee + COFFEE_RUN_POTS).min(COFFEE_CAPACITY);
            self.log.push(format!(
                "The intern made it back with {COFFEE_RUN_POTS} pots of coffee and only minor tire marks."
            ));
        } else {
            let week = (self.day - 1) / 7;
            self.log
                .push(INTERN_FATES[week as usize % INTERN_FATES.len()].to_string());
        }
    }

    /// Advances one day and returns whether something happened that should stop the clock.
    pub fn advance(&mut self) -> bool {
        self.day += 1;
        let mut stop = false;
        if self.weekday() == 0 {
            stop |= self.monday();
        }
        if self.outcome.is_none() {
            stop |= self.drink_coffee();
            stop |= self.work();
            stop |= self.leadership_changes();
            self.check_outcome();
        }
        stop || self.outcome.is_some()
    }

    /// Weekly upkeep: posture drift, brand drag, and payroll.
    fn monday(&mut self) -> bool {
        self.coffee_run_made = false;
        for value in &mut self.posture {
            *value = (*value - POSTURE_DECAY).clamp(0, 100);
        }
        if self.brand < BRAND_DRAG_LEVEL {
            let drag = self.base_valuation * (BRAND_DRAG_LEVEL - self.brand) as i64 / 1000;
            self.valuation -= drag;
            self.log.push(format!(
                "Weak brand loyalty knocked {} off the valuation.",
                money(drag)
            ));
        }

        let mut stop = false;
        while self.payroll() > self.budget && self.analysts() > 1 {
            let (i, _) = self
                .team
                .iter()
                .enumerate()
                .filter(|(_, m)| m.role == Role::Analyst)
                .max_by_key(|(_, m)| m.salary)
                .unwrap();
            let analyst = self.team.remove(i);
            self.log.push(format!(
                "Payroll came up short. {} the Analyst was laid off and walked out holding a cardboard box.",
                analyst.name
            ));
            stop = true;
        }
        if self.payroll() > self.budget {
            self.outcome = Some(Outcome::Bankrupt);
            return true;
        }
        self.budget -= self.payroll();
        self.log
            .push(format!("Payday: {} in salaries.", money(self.payroll())));
        stop
    }

    fn drink_coffee(&mut self) -> bool {
        let had_coffee = self.coffee > 0;
        self.coffee = (self.coffee - self.tempo.coffee()).max(0);
        if self.coffee > 0 {
            return false;
        }
        for member in &mut self.team {
            member.burnout += NO_COFFEE_BURNOUT;
        }
        if had_coffee {
            self.log
                .push("The coffee ran out. Tempers are short.".to_string());
        }
        had_coffee
    }

    /// Analysts share the day's workload; anyone fully burned out leaves.
    fn work(&mut self) -> bool {
        let analysts = self.analysts();
        if analysts > 0 {
            let share = self.tempo.workload() / analysts - DAILY_REST;
            for member in self.team.iter_mut().filter(|m| m.role == Role::Analyst) {
                member.burnout = (member.burnout + share).max(0);
            }
        }

        let mut exit = self.day as usize;
        let log = &mut self.log;
        let before = self.team.len();
        self.team.retain(|m| {
            if m.burnout >= 100 {
                let reason = BURNOUT_EXITS[exit % BURNOUT_EXITS.len()];
                log.push(format!("{} the {} {reason}", m.name, m.role));
                exit += 1;
            }
            m.burnout < 100
        });
        self.team.len() < before
    }

    fn leadership_changes(&mut self) -> bool {
        let mut changed = false;
        if self.budget < REDUNDANCY_BUDGET
            && let Some(i) = self.team.iter().position(|m| m.role == Role::Manager)
        {
            let manager = self.team.remove(i);
            self.log.push(format!(
                "Finance made {} the Manager redundant to save money. The consultants who recommended it billed $80K.",
                manager.name
            ));
            changed = true;
        }
        if self.trust < FIRING_TRUST
            && let Some(i) = self.team.iter().position(|m| m.role == Role::Ciso)
        {
            let ciso = self.team.remove(i);
            self.log.push(format!(
                "The board fired {} the CISO over the SOC's performance.",
                ciso.name
            ));
            changed = true;
        }
        changed
    }

    fn check_outcome(&mut self) {
        self.outcome = if self.trust <= 0 {
            Some(Outcome::Fired)
        } else if self.brand <= 0 {
            Some(Outcome::ShutDown)
        } else if self.valuation < self.base_valuation * PULLED_PERCENT / 100 {
            Some(Outcome::Pulled)
        } else if self.analysts() == 0 {
            Some(Outcome::TeamCollapsed)
        } else if self.day >= IPO_DAY {
            Some(Outcome::Ipo)
        } else {
            None
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> GameState {
        GameState::new("Acme", "Alex")
    }

    /// Advances until the given day, keeping the coffee topped up.
    fn advance_to(game: &mut GameState, day: u32) {
        while game.day < day && game.outcome.is_none() {
            game.coffee = COFFEE_CAPACITY;
            game.advance();
        }
    }

    fn analyst(name: &str, burnout: i32, salary: i64) -> TeamMember {
        TeamMember {
            name: name.to_string(),
            role: Role::Analyst,
            burnout,
            salary,
        }
    }

    /// Each analyst's burnout change and the coffee drunk over one quiet day.
    fn one_day(tempo: Tempo, analysts: usize) -> (i32, i32) {
        let mut game = game();
        game.day = 2;
        game.tempo = tempo;
        game.coffee = COFFEE_CAPACITY;
        game.team = (0..analysts).map(|_| analyst("A", 50, 0)).collect();
        game.advance();
        (game.team[0].burnout - 50, COFFEE_CAPACITY - game.coffee)
    }

    #[test]
    fn new_game_starts_on_day_one_with_full_roster() {
        let game = game();

        assert_eq!(game.day, 1);
        assert_eq!(game.weekday(), 0);
        assert_eq!(game.days_to_ipo(), IPO_DAY - 1);
        assert_eq!(game.analysts(), 3);
        assert!(game.team.iter().any(|m| m.role == Role::Manager));
        assert!(game.team.iter().any(|m| m.role == Role::Ciso));
        assert!(game.team.iter().any(|m| m.role == Role::Cio));
        assert!(game.log[0].contains("Acme") && game.log[0].contains("Alex"));
        assert_eq!(game.outcome, None);
    }

    #[test]
    fn advancing_moves_exactly_one_day() {
        let mut game = game();

        game.advance();
        assert_eq!(game.day, 2);
        game.advance();
        assert_eq!(game.day, 3);
    }

    #[test]
    fn a_quiet_game_ends_as_an_ipo_on_ipo_day() {
        let mut game = game();
        game.tempo = Tempo::Relaxed;

        advance_to(&mut game, IPO_DAY - 1);
        assert_eq!(game.outcome, None);
        assert_eq!(game.score(), None);

        assert!(game.advance());
        assert_eq!(game.day, IPO_DAY);
        assert_eq!(game.outcome, Some(Outcome::Ipo));
        assert_eq!(game.score(), Some(game.valuation));
    }

    #[test]
    fn payroll_is_taken_only_on_mondays() {
        let mut game = game();
        let mut paydays = Vec::new();

        while game.day < 22 {
            let budget = game.budget;
            game.advance();
            if game.budget != budget {
                paydays.push(game.day);
                assert_eq!(budget - game.budget, 18_000);
            }
        }

        assert_eq!(paydays, [8, 15, 22]);
        assert!(game.log.iter().any(|e| e == "Payday: $18K in salaries."));
    }

    #[test]
    fn a_short_budget_lays_off_the_most_expensive_analyst() {
        let mut game = game();
        game.day = 7;
        game.team[1].salary = 9_000;
        game.budget = 13_000;

        assert!(game.advance());

        assert_eq!(game.analysts(), 2);
        assert!(game.team.iter().all(|m| m.name != "Dev"));
        assert_eq!(game.budget, 1_000);
        assert!(
            game.log
                .iter()
                .any(|e| e.contains("Dev the Analyst was laid off"))
        );
        assert_eq!(game.outcome, None);
    }

    #[test]
    fn a_budget_that_cannot_pay_one_analyst_is_bankrupt() {
        let mut game = game();
        game.day = 7;
        game.budget = 5_000;

        assert!(game.advance());

        assert_eq!(game.analysts(), 1);
        assert_eq!(game.outcome, Some(Outcome::Bankrupt));
        assert_eq!(game.score(), None);
    }

    #[test]
    fn harder_tempos_burn_out_analysts_and_drink_coffee_faster() {
        let (relaxed, steady, crunch) = (
            one_day(Tempo::Relaxed, 3),
            one_day(Tempo::Steady, 3),
            one_day(Tempo::Crunch, 3),
        );

        assert!(relaxed.0 < 0, "Relaxed recovers");
        assert!(relaxed.0 < steady.0 && steady.0 < crunch.0);
        assert!(relaxed.1 < steady.1 && steady.1 < crunch.1);
    }

    #[test]
    fn more_analysts_share_the_burnout() {
        for tempo in [Tempo::Steady, Tempo::Crunch] {
            assert!(one_day(tempo, 6).0 < one_day(tempo, 3).0);
        }
    }

    #[test]
    fn tempo_cycles_through_all_three() {
        assert_eq!(Tempo::Relaxed.next(), Tempo::Steady);
        assert_eq!(Tempo::Steady.next(), Tempo::Crunch);
        assert_eq!(Tempo::Crunch.next(), Tempo::Relaxed);
    }

    #[test]
    fn running_out_of_coffee_burns_everyone_and_stops_the_clock() {
        let mut game = game();
        game.coffee = 1;

        assert!(game.advance());
        assert_eq!(game.coffee, 0);
        assert_eq!(game.team[5].burnout, 10 + NO_COFFEE_BURNOUT);
        assert!(game.log.iter().any(|e| e.contains("coffee ran out")));

        assert!(!game.advance(), "only running out stops the clock");
        assert_eq!(game.team[5].burnout, 10 + 2 * NO_COFFEE_BURNOUT);
    }

    #[test]
    fn posture_decays_every_monday_and_stays_in_range() {
        let mut game = game();

        advance_to(&mut game, 7);
        assert_eq!(game.posture, [30; 6]);
        advance_to(&mut game, 8);
        assert_eq!(game.posture, [30 - POSTURE_DECAY; 6]);

        game.tempo = Tempo::Relaxed;
        while game.outcome.is_none() {
            game.coffee = COFFEE_CAPACITY;
            game.advance();
            assert!(game.posture.iter().all(|p| (0..=100).contains(p)));
        }
        assert_eq!(game.posture, [0; 6]);
    }

    #[test]
    fn weak_brand_drags_the_valuation_on_mondays() {
        let mut game = game();
        game.brand = 30;

        advance_to(&mut game, 7);
        assert_eq!(game.valuation, 1_000_000_000);
        advance_to(&mut game, 8);

        assert_eq!(game.valuation, 1_000_000_000 - 20_000_000);
        assert!(game.log.iter().any(|e| e.contains("knocked $20.0M off")));
    }

    #[test]
    fn burned_out_members_leave_with_a_message() {
        let mut game = game();
        game.day = 4;
        game.team[0].burnout = 99;

        assert!(game.advance());

        assert_eq!(game.analysts(), 2);
        assert!(
            game.log
                .iter()
                .any(|e| e == "Maya the Analyst burned out and quit to open a llama sanctuary.")
        );
    }

    #[test]
    fn departures_rotate_through_burnout_exits() {
        let mut game = game();
        game.team[0].burnout = 99;
        game.team[1].burnout = 99;

        game.advance();

        assert!(
            game.log
                .iter()
                .any(|e| e.starts_with("Maya the Analyst had a stroke"))
        );
        assert!(
            game.log
                .iter()
                .any(|e| e.starts_with("Dev the Analyst has died of dysentery"))
        );
    }

    #[test]
    fn low_budget_makes_the_manager_redundant() {
        let mut game = game();
        game.budget = REDUNDANCY_BUDGET - 1;

        assert!(game.advance());

        assert!(game.team.iter().all(|m| m.role != Role::Manager));
        assert!(
            game.log
                .iter()
                .any(|e| e.contains("made Jules the Manager redundant"))
        );
    }

    #[test]
    fn low_trust_gets_the_ciso_fired() {
        let mut game = game();
        game.trust = FIRING_TRUST - 1;

        assert!(game.advance());

        assert!(game.team.iter().all(|m| m.role != Role::Ciso));
        assert!(game.log.iter().any(|e| e.contains("fired Ravi the CISO")));
    }

    #[test]
    fn each_ending_has_its_trigger_and_only_an_ipo_scores() {
        for outcome in [
            Outcome::Fired,
            Outcome::ShutDown,
            Outcome::Pulled,
            Outcome::TeamCollapsed,
            Outcome::Ipo,
        ] {
            let mut game = game();
            match outcome {
                Outcome::Fired => game.trust = 0,
                Outcome::ShutDown => game.brand = 0,
                Outcome::Pulled => game.valuation = 399_999_999,
                Outcome::TeamCollapsed => game.team.iter_mut().for_each(|m| m.burnout = 99),
                Outcome::Ipo => game.day = IPO_DAY - 1,
                Outcome::Bankrupt => unreachable!("tested with payroll"),
            }

            assert!(game.advance());

            assert_eq!(game.outcome, Some(outcome));
            assert_eq!(game.score().is_some(), outcome == Outcome::Ipo);
        }
    }

    #[test]
    fn a_coffee_run_costs_money_once_per_week() {
        let mut game = game();

        game.send_intern();

        assert_eq!(game.budget, 500_000 - COFFEE_RUN_COST);
        assert!(!game.can_send_intern());
        advance_to(&mut game, 7);
        game.coffee = 0;
        assert!(!game.can_send_intern());
        advance_to(&mut game, 8);
        game.coffee = 0;
        assert!(game.can_send_intern());
        game.budget = COFFEE_RUN_COST - 1;
        assert!(!game.can_send_intern());
    }

    #[test]
    fn a_surviving_intern_restocks_the_coffee_up_to_capacity() {
        let mut game = game();

        game.intern_returns(true);
        assert_eq!(game.coffee, 24 + COFFEE_RUN_POTS);
        assert!(game.log.last().unwrap().contains("made it back"));

        game.intern_returns(true);
        assert_eq!(game.coffee, COFFEE_CAPACITY);
    }

    #[test]
    fn the_intern_is_not_sent_when_coffee_is_full() {
        let mut game = game();
        game.coffee = COFFEE_CAPACITY;

        assert!(!game.can_send_intern());
        game.coffee -= 1;
        assert!(game.can_send_intern());
    }

    #[test]
    fn a_flattened_intern_brings_no_coffee() {
        let mut game = game();

        game.intern_returns(false);

        assert_eq!(game.coffee, 24);
        assert!(game.log.last().unwrap().contains("delivery truck"));
    }

    #[test]
    fn money_is_shown_in_short_units() {
        assert_eq!(money(10), "$10");
        assert_eq!(money(18_000), "$18K");
        assert_eq!(money(1_500_000), "$1.5M");
        assert_eq!(money(1_200_000_000), "$1.20B");
    }
}
