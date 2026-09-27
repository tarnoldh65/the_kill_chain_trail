use crate::game::{GameState, Outcome};

const NAME_LIMIT: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cue {
    Select,
    Alarm,
    Win,
    Lose,
}

/// Sound to play for a screen transition, if anything changed.
pub fn cue(before: &Screen, after: &Screen) -> Option<Cue> {
    if let (Screen::Play(old), Screen::Play(new)) = (before, after) {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Title,
    Company(String),
    Lead { company: String, lead: String },
    Play(GameState),
}

impl Screen {
    pub fn update(self, input: Input) -> Self {
        match (self, input) {
            (Self::Title, Input::Enter) => Self::Company(String::new()),
            (Self::Company(company), Input::Enter) if !company.trim().is_empty() => Self::Lead {
                company,
                lead: String::new(),
            },
            (Self::Company(company), input) => Self::Company(edit(company, input)),
            (Self::Lead { company, lead }, Input::Enter) if !lead.trim().is_empty() => {
                Self::Play(GameState::new(company.trim(), lead.trim()))
            }
            (Self::Lead { company, lead }, input) => Self::Lead {
                company,
                lead: edit(lead, input),
            },
            (Self::Play(game), Input::Enter) if game.outcome.is_some() => Self::Title,
            (Self::Play(mut game), Input::Char(c @ '1'..='3')) if game.outcome.is_none() => {
                let choice = game.stage.choices()[c as usize - '1' as usize];
                if game.can_afford(&choice) {
                    game.play(&choice);
                }
                Self::Play(game)
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

    fn game(screen: &Screen) -> &GameState {
        match screen {
            Screen::Play(game) => game,
            other => panic!("expected Play, got {other:?}"),
        }
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
    fn number_keys_play_the_matching_choice() {
        let screen = new_game().update(Input::Char('2'));

        assert_eq!(game(&screen).stage, Stage::Weaponization);
        assert_eq!(game(&screen).budget, 120);
    }

    #[test]
    fn unaffordable_and_invalid_choices_are_ignored() {
        let mut screen = new_game();
        if let Screen::Play(game) = &mut screen {
            game.budget = 0;
        }

        let screen = screen.update(Input::Char('2')).update(Input::Char('7'));

        assert_eq!(game(&screen).stage, Stage::Reconnaissance);
    }

    #[test]
    fn enter_after_the_outcome_returns_to_title() {
        let screen = type_text(new_game(), "333");
        assert_eq!(game(&screen).outcome, Some(Outcome::Fired));

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
        let mut before = new_game();
        if let Screen::Play(game) = &mut before {
            game.team[0].burnout = 95;
        }
        let after = before.clone().update(Input::Char('1'));

        assert_eq!(cue(&before, &after), Some(Cue::Alarm));
    }

    #[test]
    fn outcomes_cue_win_or_lose() {
        let won = type_text(new_game(), "312312");
        let lost = type_text(new_game(), "33");

        assert_eq!(cue(&won, &type_text(won.clone(), "2")), Some(Cue::Win));
        assert_eq!(cue(&lost, &type_text(lost.clone(), "3")), Some(Cue::Lose));
    }

    #[test]
    fn wrap_breaks_on_word_boundaries() {
        assert_eq!(wrap("the quick brown fox", 10), ["the quick", "brown fox"]);
        assert_eq!(wrap("short", 10), ["short"]);
    }
}
