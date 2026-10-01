use crate::audio::TRACKS;
use crate::game::{Choice, GameState, Outcome, Setback};
use crate::street::{Hop, Street};

const NAME_LIMIT: usize = 20;

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
            return Some(if outcome == Outcome::Contained {
                Cue::Win
            } else {
                Cue::Lose
            });
        }
        if new.team.len() < old.team.len() {
            return Some(Cue::Alarm);
        }
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
    Lead { company: String, lead: String },
    Play(GameState),
    Report(Report),
    Coffee(CoffeeRun),
}

/// The intern's trip across the street, with the game waiting on the result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoffeeRun {
    pub game: GameState,
    pub street: Street,
}

/// What happened after a decision, shown before play continues.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub game: GameState,
    pub choice: Choice,
    pub setback: Option<Setback>,
    /// Index of the first log entry caused by the decision.
    pub news: usize,
}

impl Screen {
    pub fn game(&self) -> Option<&GameState> {
        match self {
            Self::Play(game)
            | Self::Report(Report { game, .. })
            | Self::Coffee(CoffeeRun { game, .. }) => Some(game),
            _ => None,
        }
    }

    /// Advances anything that moves on its own, like traffic.
    pub fn tick(self, dt: f32) -> Self {
        match self {
            Self::Coffee(mut run) => {
                run.street.tick(dt);
                Self::Coffee(run)
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
            (Self::Lead { company, lead }, Input::Enter) if !lead.trim().is_empty() => Self::Play(
                GameState::new(company.trim(), lead.trim(), macroquad::rand::rand()),
            ),
            (Self::Lead { company, lead }, input) => Self::Lead {
                company,
                lead: edit(lead, input),
            },
            (Self::Play(game), Input::Enter) if game.outcome.is_some() => Self::Title,
            (Self::Play(mut game), Input::Char(c @ '1'..='3')) if game.outcome.is_none() => {
                let choice = game.options()[c as usize - '1' as usize];
                if !game.can_afford(&choice) {
                    return Self::Play(game);
                }
                let news = game.log.len() + 1;
                let setback = game.play(&choice);
                Self::Report(Report {
                    game,
                    choice,
                    setback,
                    news,
                })
            }
            (Self::Report(report), Input::Enter) => Self::Play(report.game),
            (Self::Play(mut game), Input::Char('4')) if game.can_send_intern() => {
                game.send_intern();
                Self::Coffee(CoffeeRun {
                    game,
                    street: Street::new(),
                })
            }
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
    use crate::game::{Outcome, Stage};

    fn type_text(mut screen: Screen, text: &str) -> Screen {
        for c in text.chars() {
            screen = screen.update(Input::Char(c));
        }
        screen
    }

    fn new_game() -> Screen {
        let screen = Screen::Title.update(Input::Enter);
        let screen = type_text(screen, "Acme").update(Input::Enter);
        type_text(screen, "Alex").update(Input::Enter)
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
    fn title_leads_to_company_and_lead_entry_then_play() {
        let screen = new_game();

        assert_eq!(game(&screen).company, "Acme");
        assert_eq!(game(&screen).lead, "Alex");
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

    #[test]
    fn number_keys_play_the_matching_offered_choice() {
        let before = new_game();
        let second = game(&before).options()[1];
        let screen = before.update(Input::Char('2'));
        let Screen::Report(report) = &screen else {
            panic!("expected Report, got {screen:?}");
        };

        assert_eq!(report.choice, second);
        assert_eq!(game(&screen).stage, Stage::Weaponization);
    }

    #[test]
    fn a_decision_shows_its_report_until_enter() {
        let screen = new_game().update(Input::Char('1'));
        let Screen::Report(report) = &screen else {
            panic!("expected Report, got {screen:?}");
        };

        assert!(report.game.log[report.news - 1].contains(report.choice.label));
        assert!(report.game.log[report.news].starts_with("Meanwhile"));
        assert_eq!(screen.clone().update(Input::Char('1')), screen);
        assert_eq!(
            screen.clone().update(Input::Enter),
            Screen::Play(report.game.clone())
        );
    }

    fn street(screen: &mut Screen) -> &mut Street {
        match screen {
            Screen::Coffee(run) => &mut run.street,
            other => panic!("expected Coffee, got {other:?}"),
        }
    }

    #[test]
    fn four_sends_the_intern_across_the_street() {
        let mut screen = new_game().update(Input::Char('4'));

        assert_eq!(game(&screen).budget, 140);
        assert_eq!(street(&mut screen).row, 0);
        let mut screen = screen.update(Input::Arrow(Hop::Up));
        assert_eq!(street(&mut screen).row, 1);
    }

    #[test]
    fn enter_only_leaves_the_street_once_the_run_is_over() {
        let mut screen = new_game().update(Input::Char('4'));
        let running = screen.clone();
        assert_eq!(running.clone().update(Input::Enter), running);

        street(&mut screen).survived = Some(true);
        let screen = screen.update(Input::Enter);

        assert!(matches!(screen, Screen::Play(_)));
        assert_eq!(game(&screen).coffee, 36);
        assert_eq!(screen.clone().update(Input::Char('4')), screen);
    }

    #[test]
    fn ticks_only_move_the_street() {
        assert_eq!(Screen::Title.tick(1.0), Screen::Title);

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
    fn unaffordable_and_invalid_choices_are_ignored() {
        let screen = with_game(new_game(), |game| game.budget = 0);
        let paid = game(&screen)
            .options()
            .iter()
            .position(|c| c.cost > 0)
            .unwrap();

        let screen = screen
            .update(Input::Char(char::from(b'1' + paid as u8)))
            .update(Input::Char('7'));

        assert_eq!(game(&screen).stage, Stage::Reconnaissance);
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
        let after = before.clone().update(Input::Char('1'));
        assert_eq!(cue(&before, &after), Some(Cue::Select));
    }

    #[test]
    fn ignored_input_is_silent() {
        let before = Screen::Company(String::new());
        let after = before.clone().update(Input::Enter);

        assert_eq!(cue(&before, &after), None);
    }

    #[test]
    fn losing_a_team_member_cues_the_alarm() {
        let before = with_game(new_game(), |game| game.team[0].burnout = 120);
        let after = before.clone().update(Input::Char('1'));

        assert_eq!(cue(&before, &after), Some(Cue::Alarm));
    }

    #[test]
    fn outcomes_cue_win_or_lose() {
        let final_stage = |containment| {
            with_game(new_game(), |game| {
                game.stage = Stage::ActionsOnObjectives;
                game.containment = containment;
            })
        };
        let (won, lost) = (final_stage(100), final_stage(0));

        assert_eq!(
            cue(&won, &won.clone().update(Input::Char('1'))),
            Some(Cue::Win)
        );
        assert_eq!(
            cue(&lost, &lost.clone().update(Input::Char('1'))),
            Some(Cue::Lose)
        );
    }

    #[test]
    fn wrap_breaks_on_word_boundaries() {
        assert_eq!(wrap("the quick brown fox", 10), ["the quick", "brown fox"]);
        assert_eq!(wrap("short", 10), ["short"]);
    }
}
