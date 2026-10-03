//! Balance tests: whole games played by simple strategies through the public game API.

use std::mem::discriminant;

use crate::conference::Track;
use crate::events::PET_PROJECT;
use crate::game::{
    Action, FUNDING_INTERVAL, GameState, IR_FEE, Item, Outcome, Profile, Reply, Tempo,
};
use crate::landmarks::Landmark;

const SEEDS: u32 = 100;

/// Something the sensible strategy can be told never to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Skip {
    Buy(Item),
    Start(Action),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Style {
    /// Always continue, ignore every alert, take the first option everywhere.
    Idle,
    /// Random choices everywhere.
    Random,
    /// What a thoughtful player would do, optionally never doing one thing.
    Sensible(Option<Skip>),
}

/// A small xorshift generator so strategies are repeatable per seed.
struct Rng(u64);

impl Rng {
    fn below(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % n as u64) as usize
    }

    fn chance(&mut self, percent: usize) -> bool {
        self.below(100) < percent
    }
}

fn allowed(style: Style, action: Action) -> bool {
    match style {
        Style::Sensible(Some(Skip::Start(skipped))) => {
            discriminant(&skipped) != discriminant(&action)
        }
        Style::Sensible(Some(Skip::Buy(item))) => action != Action::Deploy(item),
        _ => true,
    }
}

fn may_buy(style: Style, item: Item) -> bool {
    style != Style::Sensible(Some(Skip::Buy(item)))
}

fn analysts(game: &GameState) -> Vec<usize> {
    (0..game.team.len()).collect()
}

fn average_burnout(game: &GameState) -> i32 {
    let analysts = analysts(game);
    analysts.iter().map(|&i| game.team[i].burnout).sum::<i32>() / analysts.len().max(1) as i32
}

/// Money kept spare for incidents and surprises on top of every payday.
const CUSHION: i64 = 150_000;

/// The lowest the budget gets before the IPO if weekly costs stay the same and the board
/// keeps funding at today's trust: what can be spent now without missing a payday.
fn spare(game: &GameState) -> i64 {
    let (mut cash, mut lowest) = (game.budget, game.budget);
    let mut monday = game.day + 7 - game.weekday();
    while monday < game.ipo_day {
        if (monday - 1).is_multiple_of(FUNDING_INTERVAL) {
            cash += game.grant();
        }
        cash -= game.weekly_costs();
        lowest = lowest.min(cash);
        monday += 7;
    }
    lowest
}

/// Mondays left before IPO day, each one a payday.
fn paydays_left(game: &GameState) -> i64 {
    ((game.ipo_day - 1) / 7 - (game.day - 1) / 7) as i64
}

/// Whether spending `cost` now and `weekly` more each week still leaves the cushion.
fn affordable(game: &GameState, cost: i64, weekly: i64) -> bool {
    spare(game) - cost - weekly * paydays_left(game) > CUSHION
}

fn setup(game: &mut GameState, style: Style, rng: &mut Rng) {
    match style {
        Style::Idle => {
            for _ in 0..3 {
                game.hire(Item::Junior, "Idle");
            }
        }
        Style::Random => {
            for _ in 0..2 + rng.below(3) {
                let level = if rng.chance(30) {
                    Item::Senior
                } else {
                    Item::Junior
                };
                game.hire(level, "Random");
            }
            for item in &Item::ALL[2..] {
                if rng.chance(40) && game.can_buy(*item) {
                    game.buy(*item);
                }
            }
        }
        Style::Sensible(_) => {
            // A team the budget and the board's funding can carry to the IPO.
            let hireable = |g: &GameState, level: Item| {
                may_buy(style, level) && affordable(g, level.price(), level.weekly())
            };
            if hireable(game, Item::Senior) {
                game.hire(Item::Senior, "Sensible");
            }
            while analysts(game).len() < 4 && hireable(game, Item::Junior) {
                game.hire(Item::Junior, "Sensible");
            }
            if analysts(game).is_empty() {
                game.hire(Item::Senior, "Sensible");
            }
            shop(game, style);
        }
    }
}

/// Buys what a sensible player wants while keeping enough for payroll.
fn shop(game: &mut GameState, style: Style) {
    // Cheap tools in every area first, then the mid tier, then premium as money allows.
    let wanted = [
        Item::CoffeeSubscription,
        Item::PasswordManager,
        Item::Antivirus,
        Item::Posters,
        Item::VulnScanner,
        Item::Runbooks,
        Item::LogCollection,
        Item::MfaTokens,
        Item::EmailGateway,
        Item::Backups,
        Item::Waf,
        Item::IrRetainer,
        Item::Edr,
        Item::Allowlisting,
        Item::TrainingPlatform,
        Item::Siem,
        Item::Insurance,
        Item::Asm,
        Item::Pam,
        Item::DrSite,
        Item::Mdr,
    ];
    for item in wanted {
        if may_buy(style, item)
            && game.can_buy(item)
            && affordable(game, game.price(item), item.weekly())
        {
            game.buy(item);
        }
    }
    while may_buy(style, Item::Coffee) && game.can_buy(Item::Coffee) {
        game.buy(Item::Coffee);
    }
}

/// Handles whatever is waiting on the player before the next day.
fn resolve(game: &mut GameState, style: Style, rng: &mut Rng) {
    while game.outcome.is_none() {
        if !game.cards.is_empty() {
            game.reveal_next();
        } else if let Some(actor) = game.incident {
            let possible: Vec<usize> = (0..actor.responses().len())
                .filter(|&i| game.can_respond(&actor.responses()[i]))
                .collect();
            let pick = match style {
                Style::Idle => possible[0],
                Style::Random => possible[rng.below(possible.len())],
                Style::Sensible(_) => *possible
                    .iter()
                    .min_by_key(|&&i| {
                        let r = actor.responses()[i];
                        let cost = game.incident_cost(&r);
                        let broke = cost > 0 && !affordable(game, cost, 0);
                        r.valuation * 10 - r.trust as i64 * 3 - r.brand as i64 * 3
                            + cost / 10_000
                            + if r.lingers { 30 } else { 0 }
                            + if broke { 1_000 } else { 0 }
                    })
                    .unwrap(),
            };
            game.respond(pick);
        } else if let Some(event) = &game.event {
            let choices = game.event_choices().len();
            let pick = match style {
                Style::Idle => 0,
                Style::Random => rng.below(choices),
                // Every first choice is the safe one, except going along with the CIO.
                Style::Sensible(_) => (event.index == PET_PROJECT) as usize,
            };
            game.choose(pick);
        } else if game.pending_alert().is_some() {
            let reply = match style {
                Style::Idle => Reply::Ignore,
                Style::Random => [Reply::Investigate, Reply::CallIr, Reply::Ignore][rng.below(3)],
                Style::Sensible(_) => {
                    if game.can_reply(Reply::Investigate) {
                        Reply::Investigate
                    } else if game.can_reply(Reply::CallIr) && affordable(game, IR_FEE, 0) {
                        Reply::CallIr
                    } else {
                        Reply::Ignore
                    }
                }
            };
            let reply = if game.can_reply(reply) {
                reply
            } else {
                Reply::Ignore
            };
            game.reply(reply);
        } else if let Some(stop) = game.stop.clone() {
            landmark(game, style, rng, stop.landmark, stop.attended);
        } else {
            return;
        }
    }
}

fn landmark(game: &mut GameState, style: Style, rng: &mut Rng, at: Landmark, attended: bool) {
    if at == Landmark::Conference && !attended {
        match style {
            Style::Idle => game.skip_conference(),
            Style::Random => {
                let mut picks = Vec::new();
                for i in analysts(game) {
                    if rng.chance(50) {
                        picks.push((i, Track::ALL[rng.below(4)]));
                    }
                }
                if game.conference_cost(picks.len()) <= game.budget {
                    game.attend(&picks);
                } else {
                    game.skip_conference();
                }
            }
            Style::Sensible(_) => {
                // Everyone but one goes to the talks; one holds the fort.
                let picks: Vec<_> = analysts(game)
                    .into_iter()
                    .skip(1)
                    .map(|i| (i, Track::Talks))
                    .collect();
                game.attend(&picks);
            }
        }
        return;
    }
    if at.is_fort() {
        if let Style::Sensible(_) = style {
            game.rest_at_fort();
            while analysts(game).len() < 4
                && may_buy(style, Item::Junior)
                && affordable(game, game.price(Item::Junior), Item::Junior.weekly())
            {
                game.hire(Item::Junior, "Sensible");
            }
            shop(game, style);
        }
        game.leave_fort();
        return;
    }
    let ways: Vec<usize> = (0..at.crossings().len())
        .filter(|&i| game.can_cross(i))
        .collect();
    let way = match style {
        Style::Idle => ways[0],
        Style::Random => ways[rng.below(ways.len())],
        Style::Sensible(_) => *ways
            .iter()
            .max_by_key(|&&i| (game.crossing_odds(&at.crossings()[i].odds), usize::MAX - i))
            .unwrap(),
    };
    game.cross(way);
}

/// The day's standing decisions: tempo, coffee, and what to work on.
fn plan(game: &mut GameState, style: Style, rng: &mut Rng) {
    match style {
        Style::Idle => {}
        Style::Random => {
            if rng.chance(5) {
                game.tempo = game.tempo.next();
            }
            if game.can_send_intern() && rng.chance(30) {
                game.send_intern();
                game.intern_returns(rng.chance(60));
            }
            let actions = game.actions();
            if !actions.is_empty() && rng.chance(10) {
                let action = actions[rng.below(actions.len())];
                game.start(action);
            }
            // An impulse buy through Procurement now and then.
            let item = Item::ALL[rng.below(Item::ALL.len())];
            if rng.chance(3) && game.can_buy(item) {
                match item {
                    Item::Junior | Item::Senior => game.hire(item, "Random"),
                    _ => game.buy(item),
                }
            }
        }
        Style::Sensible(_) => {
            let tired = average_burnout(game);
            game.tempo = if tired >= 45 {
                Tempo::Relaxed
            } else {
                Tempo::Steady
            };
            if game.can_send_intern() && game.coffee < 24 {
                game.send_intern();
                game.intern_returns(true);
            }
            // A weekly trip to Procurement for anything missing, and a new hire if short.
            if game.weekday() == 0 {
                shop(game, style);
                let short = analysts(game).len() + game.searches.len() < 3;
                if short
                    && may_buy(style, Item::Junior)
                    && game.can_buy(Item::Junior)
                    && affordable(game, game.price(Item::Junior), Item::Junior.weekly())
                {
                    game.hire(Item::Junior, "Sensible");
                }
            }
            if let Some(action) = sensible_action(game, style, tired) {
                game.start(action);
            }
        }
    }
}

fn sensible_action(game: &GameState, style: Style, tired: i32) -> Option<Action> {
    let actions: Vec<Action> = game
        .actions()
        .into_iter()
        .filter(|&a| allowed(style, a))
        .collect();
    let offered = |wanted: Action| actions.contains(&wanted).then_some(wanted);
    let first = |pick: fn(&Action) -> bool| actions.iter().copied().find(pick);

    first(|a| matches!(a, Action::Clear(_)))
        .or_else(|| (tired >= 55).then(|| offered(Action::DayOff)).flatten())
        // Trust pays for itself now that the board funds by it.
        .or_else(|| {
            (game.trust < 50)
                .then(|| {
                    offered(Action::BriefLeadership)
                        .or_else(|| offered(Action::Operate(Item::Runbooks)))
                })
                .flatten()
        })
        .or_else(|| first(|a| matches!(a, Action::Deploy(_))))
        .or_else(|| first(|a| matches!(a, Action::Maintain(_))))
        .or_else(|| {
            // Push harder in the four weeks before the pen test, so its boosts are fresh.
            let pen_test = game.landmark_days[Landmark::PenTest as usize];
            let cramming = game.day + 28 >= pen_test && game.day < pen_test;
            if tired >= if cramming { 60 } else { 35 } {
                return None;
            }
            let rotation: Vec<Action> = actions
                .iter()
                .copied()
                .filter(|a| matches!(a, Action::Operate(_)))
                .chain([Action::BriefLeadership])
                .collect();
            // Each start is logged, so counting them walks the rotation.
            let started = game
                .log
                .iter()
                .filter(|l| l.starts_with("Started:"))
                .count();
            let start = started % rotation.len();
            (0..rotation.len()).find_map(|i| offered(rotation[(start + i) % rotation.len()]))
        })
}

fn play(profile: Profile, seed: u32, style: Style) -> GameState {
    let mut game = GameState::new("Acme", "Alex", profile, seed);
    let mut rng = Rng(seed as u64 * 2_654_435_761 + 1);
    setup(&mut game, style, &mut rng);
    while game.outcome.is_none() {
        resolve(&mut game, style, &mut rng);
        if game.outcome.is_some() {
            break;
        }
        plan(&mut game, style, &mut rng);
        game.advance();
    }
    game
}

/// Percent of seeds that reach the IPO for a profile and style.
fn ipo_rate(profile: Profile, style: Style) -> usize {
    let wins = (0..SEEDS)
        .filter(|&seed| play(profile, seed, style).outcome == Some(Outcome::Ipo))
        .count();
    wins * 100 / SEEDS as usize
}

#[test]
fn idle_play_almost_never_reaches_the_ipo() {
    for profile in Profile::ALL {
        let rate = ipo_rate(profile, Style::Idle);
        assert!(rate < 10, "{profile}: {rate}%");
    }
}

#[test]
fn random_play_usually_loses() {
    for profile in Profile::ALL {
        let rate = ipo_rate(profile, Style::Random);
        assert!(rate < 25, "{profile}: {rate}%");
    }
}

#[test]
fn sensible_play_usually_reaches_the_ipo() {
    for profile in Profile::ALL {
        let rate = ipo_rate(profile, Style::Sensible(None));
        assert!(rate > 70, "{profile}: {rate}%");
    }
}

#[test]
fn every_ending_happens_to_some_strategy() {
    // A pulled IPO is rare (about 1 in 70 random Fintech games), so this looks wider.
    let mut endings = Vec::new();
    for profile in Profile::ALL {
        for style in [Style::Idle, Style::Random, Style::Sensible(None)] {
            for seed in 0..3 * SEEDS {
                endings.push(play(profile, seed, style).outcome.unwrap());
            }
        }
    }

    for outcome in [
        Outcome::Ipo,
        Outcome::Pulled,
        Outcome::ShutDown,
        Outcome::Fired,
        Outcome::TeamCollapsed,
        Outcome::Bankrupt,
    ] {
        assert!(endings.contains(&outcome), "{outcome:?} never happens");
    }
}

#[test]
fn no_single_action_or_purchase_is_required_to_win() {
    let skips = Item::ALL.map(Skip::Buy).into_iter().chain(
        [
            // Skipping one operation skips them all.
            Action::Operate(Item::Siem),
            Action::DayOff,
            Action::BriefLeadership,
            Action::Maintain(crate::game::Area::Detection),
            Action::Clear(crate::attack::Condition::SystemsDown),
        ]
        .map(Skip::Start),
    );
    for skip in skips {
        let wins = Profile::ALL
            .iter()
            .flat_map(|&profile| (0..20).map(move |seed| (profile, seed)))
            .any(|(profile, seed)| {
                play(profile, seed, Style::Sensible(Some(skip))).outcome == Some(Outcome::Ipo)
            });
        assert!(wins, "skipping {skip:?} never wins");
    }
}

#[test]
fn sensible_play_earns_passable_pen_test_grades() {
    // Average grade points over every area and seed, in hundredths: A is 400, F is 0.
    // Gaming can only afford the cheap tools, so it stays near a D.
    let floors = [
        (Profile::Fintech, 190),
        (Profile::Healthtech, 130),
        (Profile::Gaming, 70),
    ];
    for (profile, floor) in floors {
        let points: Vec<u32> = (0..SEEDS)
            .filter_map(|seed| play(profile, seed, Style::Sensible(None)).pen_test)
            .flat_map(|(_, grades)| grades.map(|(_, grade)| "FDCBA".find(grade).unwrap() as u32))
            .collect();
        let average = points.iter().sum::<u32>() * 100 / points.len() as u32;
        assert!(average >= floor, "{profile}: {average}");
    }
}
