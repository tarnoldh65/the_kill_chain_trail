use crate::audio::TRACKS;
use crate::game::{GameState, Item, Outcome, Profile};
use crate::street::{Hop, Street};

const NAME_LIMIT: usize = 20;
/// Real seconds each day takes while the clock is running.
pub const DAY_SECONDS: f32 = 0.5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cue {
    Select,
    Alarm,
    Win,
    Lose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Music {
    Track(usize),
    Off,
}

/// Music picked by a number key on the title screen: a track, or off after the last track.
pub fn music_choice(screen: &Screen, input: Input) -> Option<Music> {
    let Input::Char(c @ '1'..='9') = input else {
        return None;
    };
    let i = c as usize - '1' as usize;
    match screen {
        Screen::Title if i < TRACKS.len() => Some(Music::Track(i)),
        Screen::Title if i == TRACKS.len() => Some(Music::Off),
        _ => None,
    }
}

/// Sound to play for a screen transition, if anything changed.
pub fn cue(before: &Screen, after: &Screen) -> Option<Cue> {
    if let (Screen::Coffee(old), Screen::Coffee(new)) = (before, after) {
        let (old, new) = (&old.street, &new.street);
        return if old.survived.is_none() && new.survived == Some(false) {
            Some(Cue::Lose)
        } else if old.survived.is_none() && new.survived == Some(true) || old.leg != new.leg {
            Some(Cue::Win)
        } else if (old.x, old.row) != (new.x, new.row) {
            Some(Cue::Select)
        } else {
            None
        };
    }
    if let (Some(old), Some(new)) = (before.game(), after.game()) {
        if old.outcome.is_none()
            && let Some(outcome) = new.outcome
        {
            return Some(if outcome == Outcome::Ipo {
                Cue::Win
            } else {
                Cue::Lose
            });
        }
        if new.team.len() < old.team.len() {
            return Some(Cue::Alarm);
        }
    }
    if let (Screen::Travel(_), Screen::Travel(_)) = (before, after) {
        return None;
    }
    (before != after).then_some(Cue::Select)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Char(char),
    Backspace,
    Enter,
    Arrow(Hop),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Title,
    Company(String),
    Lead {
        company: String,
        lead: String,
    },
    Profile {
        company: String,
        lead: String,
    },
    /// The Vendor Hall before day 1.
    Shop(Shop),
    /// Naming the analyst being hired at the Vendor Hall.
    Hire {
        shop: Shop,
        name: String,
    },
    /// The day menu, with the clock stopped.
    Play(GameState),
    /// Days passing on their own until something happens or a key is pressed.
    Travel(Travel),
    Team(GameState),
    Coffee(CoffeeRun),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Travel {
    pub game: GameState,
    /// Seconds since the last day passed, kept in milliseconds so screens stay comparable.
    elapsed_ms: u32,
}

/// The Vendor Hall's listing, with a cursor on one row of `Item::ALL` or on the exit below it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shop {
    pub game: GameState,
    pub cursor: usize,
}

impl Shop {
    /// The item under the cursor, or `None` on the exit row.
    pub fn item(&self) -> Option<Item> {
        Item::ALL.get(self.cursor).copied()
    }
}

/// The intern's trip across the street, with the game waiting on the result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoffeeRun {
    pub game: GameState,
    pub street: Street,
}

impl Screen {
    pub fn game(&self) -> Option<&GameState> {
        match self {
            Self::Play(game)
            | Self::Team(game)
            | Self::Shop(Shop { game, .. })
            | Self::Hire {
                shop: Shop { game, .. },
                ..
            }
            | Self::Travel(Travel { game, .. })
            | Self::Coffee(CoffeeRun { game, .. }) => Some(game),
            _ => None,
        }
    }

    /// Advances anything that moves on its own, like traffic and the calendar.
    pub fn tick(self, dt: f32) -> Self {
        match self {
            Self::Coffee(mut run) => {
                run.street.tick(dt);
                Self::Coffee(run)
            }
            Self::Travel(Travel {
                mut game,
                mut elapsed_ms,
            }) => {
                elapsed_ms += (dt * 1000.0) as u32;
                let day_ms = (DAY_SECONDS * 1000.0) as u32;
                while elapsed_ms >= day_ms {
                    elapsed_ms -= day_ms;
                    if game.advance() {
                        return Self::Play(game);
                    }
                }
                Self::Travel(Travel { game, elapsed_ms })
            }
            screen => screen,
        }
    }

    pub fn update(self, input: Input) -> Self {
        match (self, input) {
            (Self::Title, Input::Enter) => Self::Company(String::new()),
            (Self::Company(company), Input::Enter) if !company.trim().is_empty() => Self::Lead {
                company,
                lead: String::new(),
            },
            (Self::Company(company), input) => Self::Company(edit(company, input)),
            (Self::Lead { company, lead }, Input::Enter) if !lead.trim().is_empty() => {
                Self::Profile { company, lead }
            }
            (Self::Lead { company, lead }, input) => Self::Lead {
                company,
                lead: edit(lead, input),
            },
            (Self::Profile { company, lead }, Input::Char(c @ '1'..='3')) => {
                let profile = Profile::ALL[c as usize - '1' as usize];
                Self::Shop(Shop {
                    game: GameState::new(company.trim(), lead.trim(), profile),
                    cursor: 0,
                })
            }
            (Self::Shop(mut shop), Input::Arrow(Hop::Up)) => {
                shop.cursor = shop.cursor.saturating_sub(1);
                Self::Shop(shop)
            }
            (Self::Shop(mut shop), Input::Arrow(Hop::Down)) => {
                shop.cursor = (shop.cursor + 1).min(Item::ALL.len());
                Self::Shop(shop)
            }
            (Self::Shop(mut shop), Input::Enter) => match shop.item() {
                None if shop.game.analysts() > 0 => Self::Play(shop.game),
                Some(item) if shop.game.can_buy(item) => match item {
                    Item::Junior | Item::Senior => Self::Hire {
                        shop,
                        name: String::new(),
                    },
                    _ => {
                        shop.game.buy(item);
                        Self::Shop(shop)
                    }
                },
                _ => Self::Shop(shop),
            },
            (Self::Hire { mut shop, name }, Input::Enter) => {
                if !name.trim().is_empty() {
                    let item = shop.item().expect("hiring from an item row");
                    shop.game.hire(item, name.trim());
                }
                Self::Shop(shop)
            }
            (Self::Hire { shop, name }, input) => Self::Hire {
                shop,
                name: edit(name, input),
            },
            (Self::Play(game), Input::Enter) if game.outcome.is_some() => Self::Title,
            (Self::Play(game), _) if game.outcome.is_some() => Self::Play(game),
            (Self::Play(game), Input::Char('1')) => Self::Travel(Travel {
                game,
                elapsed_ms: 0,
            }),
            (Self::Play(game), Input::Char('2')) => Self::Team(game),
            (Self::Play(mut game), Input::Char('3')) => {
                game.tempo = game.tempo.next();
                Self::Play(game)
            }
            (Self::Play(mut game), Input::Char('4')) if game.can_send_intern() => {
                game.send_intern();
                Self::Coffee(CoffeeRun {
                    game,
                    street: Street::new(),
                })
            }
            (Self::Travel(travel), _) => Self::Play(travel.game),
            (Self::Team(game), Input::Enter) => Self::Play(game),
            (Self::Coffee(mut run), Input::Arrow(hop)) => {
                run.street.hop(hop);
                Self::Coffee(run)
            }
            (Self::Coffee(mut run), Input::Enter) if run.street.survived.is_some() => {
                run.game.intern_returns(run.street.survived == Some(true));
                Self::Play(run.game)
            }
            (screen, _) => screen,
        }
    }
}

fn edit(mut text: String, input: Input) -> String {
    match input {
        Input::Char(c) if text.len() < NAME_LIMIT && (c.is_ascii_graphic() || c == ' ') => {
            text.push(c)
        }
        Input::Backspace => {
            text.pop();
        }
        _ => {}
    }
    text
}

/// Word-wraps text into lines of at most `width` characters.
pub fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = vec![String::new()];
    for word in text.split_whitespace() {
        let line = lines.last_mut().unwrap();
        if !line.is_empty() && line.len() + 1 + word.len() > width {
            lines.push(word.to_string());
        } else {
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{COFFEE_RUN_COST, IPO_DAY, Role, Tempo};

    fn type_text(mut screen: Screen, text: &str) -> Screen {
        for c in text.chars() {
            screen = screen.update(Input::Char(c));
        }
        screen
    }

    /// The Vendor Hall for a Healthtech company.
    fn vendor_hall() -> Screen {
        let screen = Screen::Title.update(Input::Enter);
        let screen = type_text(screen, "Acme").update(Input::Enter);
        type_text(screen, "Alex")
            .update(Input::Enter)
            .update(Input::Char('2'))
    }

    fn arrows(mut screen: Screen, hop: Hop, times: usize) -> Screen {
        for _ in 0..times {
            screen = screen.update(Input::Arrow(hop));
        }
        screen
    }

    /// Hires a junior analyst from the top row and returns to the top.
    fn hire_junior(screen: Screen, name: &str) -> Screen {
        let screen = arrows(screen, Hop::Up, Item::ALL.len()).update(Input::Enter);
        type_text(screen, name).update(Input::Enter)
    }

    /// Leaves the Vendor Hall through the exit row.
    fn open_for_business(screen: Screen) -> Screen {
        arrows(screen, Hop::Down, Item::ALL.len()).update(Input::Enter)
    }

    /// Day 1 with two junior analysts hired.
    fn new_game() -> Screen {
        open_for_business(hire_junior(hire_junior(vendor_hall(), "Maya"), "Dev"))
    }

    fn shop(screen: &Screen) -> &Shop {
        match screen {
            Screen::Shop(shop) => shop,
            other => panic!("expected Shop, got {other:?}"),
        }
    }

    /// Changes the game behind a play screen.
    fn with_game(mut screen: Screen, change: impl FnOnce(&mut GameState)) -> Screen {
        if let Screen::Play(game) = &mut screen {
            change(game);
        }
        screen
    }

    fn game(screen: &Screen) -> &GameState {
        screen.game().expect("expected a game screen")
    }

    #[test]
    fn title_leads_through_names_profile_and_vendor_hall_to_day_one() {
        let screen = vendor_hall();
        assert_eq!(shop(&screen).game.profile, Profile::Healthtech);
        assert_eq!(shop(&screen).game.company, "Acme");
        assert_eq!(shop(&screen).game.lead, "Alex");

        let screen = new_game();
        assert!(matches!(screen, Screen::Play(_)));
        assert_eq!(game(&screen).day, 1);
    }

    #[test]
    fn number_keys_pick_the_profile() {
        let profile = Screen::Profile {
            company: "Acme".to_string(),
            lead: "Alex".to_string(),
        };
        for (key, expected) in ['1', '2', '3'].into_iter().zip(Profile::ALL) {
            let screen = profile.clone().update(Input::Char(key));
            assert_eq!(shop(&screen).game.profile, expected);
        }
        assert_eq!(profile.clone().update(Input::Char('4')), profile);
    }

    #[test]
    fn arrows_move_the_cursor_within_the_listing_and_exit() {
        let screen = vendor_hall().update(Input::Arrow(Hop::Up));
        assert_eq!(shop(&screen).cursor, 0);

        let screen = arrows(screen, Hop::Down, 3);
        assert_eq!(shop(&screen).item(), Some(Item::ALL[3]));

        let screen = arrows(screen, Hop::Down, 20);
        assert_eq!(shop(&screen).cursor, Item::ALL.len());
        assert_eq!(shop(&screen).item(), None);
    }

    #[test]
    fn enter_buys_the_item_under_the_cursor() {
        let screen = arrows(vendor_hall(), Hop::Down, 3).update(Input::Enter);
        let item = Item::ALL[3];

        assert_eq!(shop(&screen).game.owned, [item]);
        assert_eq!(
            shop(&screen).game.budget,
            Profile::Healthtech.budget() - item.price()
        );
        assert_eq!(screen.clone().update(Input::Enter), screen, "bought once");
    }

    #[test]
    fn unaffordable_items_are_not_bought() {
        let mut screen = arrows(vendor_hall(), Hop::Down, 3);
        if let Screen::Shop(shop) = &mut screen {
            shop.game.budget = 0;
        }

        assert_eq!(screen.clone().update(Input::Enter), screen);
    }

    #[test]
    fn hiring_asks_for_a_name() {
        let screen = arrows(vendor_hall(), Hop::Down, 1).update(Input::Enter);
        assert!(matches!(screen, Screen::Hire { .. }));

        let screen = type_text(screen, "Marcus").update(Input::Enter);
        let hired = shop(&screen).game.team.last().unwrap();
        assert_eq!(hired.name, "Marcus");
        assert_eq!(hired.role, Role::Senior);
    }

    #[test]
    fn an_empty_name_cancels_the_hire() {
        let before = vendor_hall();
        let screen = before.clone().update(Input::Enter).update(Input::Enter);

        assert_eq!(screen, before);
    }

    #[test]
    fn the_vendor_hall_needs_an_analyst_before_opening() {
        let screen = open_for_business(vendor_hall());
        assert!(matches!(screen, Screen::Shop(_)));

        let screen = open_for_business(hire_junior(vendor_hall(), "Maya"));
        assert!(matches!(screen, Screen::Play(_)));
    }

    #[test]
    fn leftover_budget_carries_into_day_one() {
        let screen = new_game();

        assert_eq!(
            game(&screen).budget,
            Profile::Healthtech.budget() - 2 * Item::Junior.price()
        );
    }

    #[test]
    fn empty_names_are_not_accepted() {
        let screen = Screen::Company(String::new()).update(Input::Enter);

        assert_eq!(screen, Screen::Company(String::new()));
    }

    #[test]
    fn name_entry_supports_backspace_and_length_limit() {
        let screen = type_text(Screen::Company(String::new()), "Acmx").update(Input::Backspace);
        assert_eq!(screen, Screen::Company("Acm".to_string()));

        let screen = type_text(Screen::Company(String::new()), &"x".repeat(30));
        assert_eq!(screen, Screen::Company("x".repeat(NAME_LIMIT)));
    }

    #[test]
    fn name_entry_ignores_control_characters() {
        let screen = type_text(Screen::Company(String::new()), "A\r\u{8}");

        assert_eq!(screen, Screen::Company("A".to_string()));
    }

    /// Starts the clock and runs it for `seconds`.
    fn travel(screen: Screen, seconds: f32) -> Screen {
        screen.update(Input::Char('1')).tick(seconds)
    }

    #[test]
    fn continue_runs_the_clock_one_day_per_tick() {
        let screen = new_game().update(Input::Char('1'));
        assert!(matches!(screen, Screen::Travel(_)));
        assert_eq!(game(&screen).day, 1);

        let screen = screen.tick(DAY_SECONDS / 2.0);
        assert_eq!(game(&screen).day, 1);
        let screen = screen.tick(DAY_SECONDS / 2.0);
        assert_eq!(game(&screen).day, 2);
        let screen = screen.tick(DAY_SECONDS * 3.0);
        assert_eq!(game(&screen).day, 5);
        assert!(matches!(screen, Screen::Travel(_)));
    }

    #[test]
    fn any_key_stops_the_clock() {
        for input in [Input::Char('x'), Input::Enter, Input::Arrow(Hop::Up)] {
            let screen = travel(new_game(), DAY_SECONDS).update(input);

            assert!(matches!(screen, Screen::Play(_)));
            assert_eq!(game(&screen).day, 2);
        }
    }

    #[test]
    fn something_happening_stops_the_clock() {
        let screen = travel(
            with_game(new_game(), |game| game.coffee = 1),
            DAY_SECONDS * 5.0,
        );

        assert!(matches!(screen, Screen::Play(_)));
        assert_eq!(game(&screen).day, 2);
        assert!(game(&screen).log.last().unwrap().contains("coffee ran out"));
    }

    #[test]
    fn the_clock_stops_at_the_end_of_the_game() {
        let screen = with_game(new_game(), |game| game.day = IPO_DAY - 1);
        let screen = travel(screen, DAY_SECONDS * 5.0);

        assert!(matches!(screen, Screen::Play(_)));
        assert_eq!(game(&screen).outcome, Some(Outcome::Ipo));
        assert_eq!(screen.clone().update(Input::Char('1')), screen);
    }

    #[test]
    fn two_checks_the_team_until_enter() {
        let screen = new_game().update(Input::Char('2'));
        assert!(matches!(screen, Screen::Team(_)));
        assert_eq!(screen.clone().update(Input::Char('1')), screen);

        assert_eq!(screen.update(Input::Enter), new_game());
    }

    #[test]
    fn three_cycles_the_tempo() {
        let screen = new_game().update(Input::Char('3'));
        assert_eq!(game(&screen).tempo, Tempo::Crunch);

        let screen = screen.update(Input::Char('3'));
        assert_eq!(game(&screen).tempo, Tempo::Relaxed);
    }

    fn street(screen: &mut Screen) -> &mut Street {
        match screen {
            Screen::Coffee(run) => &mut run.street,
            other => panic!("expected Coffee, got {other:?}"),
        }
    }

    #[test]
    fn four_sends_the_intern_across_the_street() {
        let before = new_game();
        let mut screen = before.clone().update(Input::Char('4'));

        assert_eq!(game(&screen).budget, game(&before).budget - COFFEE_RUN_COST);
        assert_eq!(street(&mut screen).row, 0);
        let mut screen = screen.update(Input::Arrow(Hop::Up));
        assert_eq!(street(&mut screen).row, 1);
    }

    #[test]
    fn enter_only_leaves_the_street_once_the_run_is_over() {
        let coffee = game(&new_game()).coffee;
        let mut screen = new_game().update(Input::Char('4'));
        let running = screen.clone();
        assert_eq!(running.clone().update(Input::Enter), running);

        street(&mut screen).survived = Some(true);
        let screen = screen.update(Input::Enter);

        assert!(matches!(screen, Screen::Play(_)));
        assert_eq!(game(&screen).coffee, coffee + 12);
        assert_eq!(screen.clone().update(Input::Char('4')), screen);
    }

    #[test]
    fn ticks_only_move_the_street_and_the_clock() {
        assert_eq!(Screen::Title.tick(1.0), Screen::Title);
        assert_eq!(new_game().tick(1.0), new_game());

        let mut screen = new_game().update(Input::Char('4'));
        let before = street(&mut screen).lanes.clone();
        let mut screen = screen.tick(0.1);
        assert_ne!(street(&mut screen).lanes, before);
    }

    #[test]
    fn street_cues_hops_escapes_and_crashes_but_not_traffic() {
        let mut before = new_game().update(Input::Char('4'));
        street(&mut before).row = 5;
        let mut hopped = before.clone();
        street(&mut hopped).row = 4;
        let mut crashed = before.clone();
        street(&mut crashed).survived = Some(false);
        let mut home = before.clone();
        street(&mut home).survived = Some(true);

        assert_eq!(cue(&before, &before.clone().tick(0.1)), None);
        assert_eq!(cue(&before, &hopped), Some(Cue::Select));
        assert_eq!(cue(&before, &crashed), Some(Cue::Lose));
        assert_eq!(cue(&before, &home), Some(Cue::Win));
    }

    #[test]
    fn number_keys_pick_music_only_on_the_title() {
        assert_eq!(
            music_choice(&Screen::Title, Input::Char('1')),
            Some(Music::Track(0))
        );
        assert_eq!(
            music_choice(&Screen::Title, Input::Char('3')),
            Some(Music::Track(2))
        );
        assert_eq!(
            music_choice(&Screen::Title, Input::Char('4')),
            Some(Music::Off)
        );
        assert_eq!(music_choice(&Screen::Title, Input::Char('5')), None);
        assert_eq!(music_choice(&Screen::Title, Input::Enter), None);
        assert_eq!(music_choice(&new_game(), Input::Char('1')), None);
    }

    #[test]
    fn the_intern_goes_once_per_week() {
        let screen = new_game().update(Input::Char('4'));
        let screen = match screen {
            Screen::Coffee(mut run) => {
                run.street.survived = Some(false);
                Screen::Coffee(run).update(Input::Enter)
            }
            other => panic!("expected Coffee, got {other:?}"),
        };

        assert_eq!(screen.clone().update(Input::Char('4')), screen);
    }

    #[test]
    fn invalid_keys_are_ignored() {
        let screen = new_game();

        assert_eq!(screen.clone().update(Input::Char('7')), screen);
    }

    #[test]
    fn enter_after_the_outcome_returns_to_title() {
        let screen = with_game(new_game(), |game| game.outcome = Some(Outcome::Fired));

        assert_eq!(screen.update(Input::Enter), Screen::Title);
    }

    #[test]
    fn typing_and_choosing_cue_a_select_sound() {
        let before = Screen::Company(String::new());
        let after = before.clone().update(Input::Char('A'));
        assert_eq!(cue(&before, &after), Some(Cue::Select));

        let before = new_game();
        let after = before.clone().update(Input::Char('3'));
        assert_eq!(cue(&before, &after), Some(Cue::Select));
    }

    #[test]
    fn ignored_input_and_quiet_days_are_silent() {
        let before = Screen::Company(String::new());
        let after = before.clone().update(Input::Enter);
        assert_eq!(cue(&before, &after), None);

        let before = new_game().update(Input::Char('1'));
        let after = before.clone().tick(DAY_SECONDS);
        assert_eq!(game(&after).day, 2);
        assert_eq!(cue(&before, &after), None);
    }

    #[test]
    fn losing_a_team_member_cues_the_alarm() {
        let before =
            with_game(new_game(), |game| game.team[3].burnout = 99).update(Input::Char('1'));
        let after = before.clone().tick(DAY_SECONDS);

        assert_eq!(cue(&before, &after), Some(Cue::Alarm));
    }

    #[test]
    fn outcomes_cue_win_only_for_an_ipo() {
        let last_day = |trust| {
            with_game(new_game(), |game| {
                game.day = IPO_DAY - 1;
                game.trust = trust;
            })
            .update(Input::Char('1'))
        };
        let (won, lost) = (last_day(60), last_day(0));

        assert_eq!(cue(&won, &won.clone().tick(DAY_SECONDS)), Some(Cue::Win));
        assert_eq!(cue(&lost, &lost.clone().tick(DAY_SECONDS)), Some(Cue::Lose));
    }

    #[test]
    fn wrap_breaks_on_word_boundaries() {
        assert_eq!(wrap("the quick brown fox", 10), ["the quick", "brown fox"]);
        assert_eq!(wrap("short", 10), ["short"]);
    }
}
