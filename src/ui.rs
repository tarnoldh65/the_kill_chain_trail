use crate::audio::TRACKS;
use crate::conference::Track;
use crate::game::{Action, Area, Entry, GameState, Item, Outcome, Profile, Reply};
use crate::street::{Hop, Street};

const NAME_LIMIT: usize = 20;
pub const WEEKDAYS: [&str; 7] = ["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];
/// Pixels per line of log text, plus the gaps that separate entries and days.
pub const LOG_LINE: f32 = 10.0;
const ENTRY_GAP: f32 = 4.0;
const DAY_GAP: f32 = 8.0;
/// Characters per line and pixels of height on a page of the full log.
pub const PAGE_WIDTH: usize = 76;
pub const PAGE_HEIGHT: f32 = 400.0;

/// Real seconds each day takes while the clock is running.
pub const DAY_SECONDS: f32 = 0.5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cue {
    Select,
    Alarm,
    Win,
    Lose,
    Tick,
    Payday,
    Klaxon,
    Sting,
    Fanfare,
    Bell,
}

impl Cue {
    pub const ALL: [Cue; 10] = [
        Self::Select,
        Self::Alarm,
        Self::Win,
        Self::Lose,
        Self::Tick,
        Self::Payday,
        Self::Klaxon,
        Self::Sting,
        Self::Fanfare,
        Self::Bell,
    ];
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
                Cue::Bell
            } else {
                Cue::Lose
            });
        }
        if new.incident.is_some() && old.incident.is_none() {
            return Some(Cue::Sting);
        }
        if new.pending_alert().is_some() && old.pending_alert().is_none() {
            return Some(Cue::Klaxon);
        }
        let arrived = new.stop.is_some() && old.stop.is_none();
        if arrived && new.day > old.day {
            return Some(Cue::Fanfare);
        }
        if new.team.len() < old.team.len() || new.event.is_some() && old.event.is_none() {
            return Some(Cue::Alarm);
        }
        if new.day > old.day {
            return Some(if new.weekday() == 0 {
                Cue::Payday
            } else {
                Cue::Tick
            });
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
    /// Procurement before day 1.
    Shop(Shop),
    /// Naming the analyst being hired through Procurement.
    Hire {
        shop: Shop,
        name: String,
    },
    /// Picking something for the SOC to spend its days on.
    Actions(ActionMenu),
    /// Picking who goes to the conference, as team indices.
    Attendees {
        game: GameState,
        picked: Vec<usize>,
    },
    /// Picking a track for each attendee in turn.
    Tracks {
        game: GameState,
        picked: Vec<usize>,
        tracks: Vec<Track>,
    },
    /// The day menu, with the clock stopped.
    Play(GameState),
    /// Days passing on their own until something happens or a key is pressed.
    Travel(Travel),
    /// The roster, with a cursor for choosing someone to let go.
    Team {
        game: GameState,
        cursor: usize,
        /// Waiting for Y or N before firing the member under the cursor.
        confirming: bool,
    },
    Coffee(CoffeeRun),
    /// The six defense areas, what helps each, and the pen test grades.
    Defenses(GameState),
    /// The whole log, one page at a time; page 0 is the oldest.
    Log {
        game: GameState,
        page: usize,
    },
    /// The after-action report once the game is over.
    Report(GameState),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Travel {
    pub game: GameState,
    /// Seconds since the last day passed, kept in milliseconds so screens stay comparable.
    elapsed_ms: u32,
}

/// Procurement tabs: staff, one per defense area, then services.
pub const TABS: usize = 8;

/// The tab an item is listed under.
fn tab_of(item: Item) -> usize {
    match (item, item.category()) {
        (Item::Junior | Item::Senior, _) => 0,
        (_, Some(area)) => 1 + area as usize,
        (_, None) => TABS - 1,
    }
}

/// The items listed under a tab, in catalog order.
pub fn tab_items(tab: usize) -> Vec<Item> {
    Item::ALL
        .into_iter()
        .filter(|&i| tab_of(i) == tab)
        .collect()
}

pub fn tab_name(tab: usize) -> String {
    match tab {
        0 => "Staff".to_string(),
        t if t == TABS - 1 => "Services".to_string(),
        t => Area::ALL[t - 1].to_string(),
    }
}

/// Procurement, open on one tab with a cursor on one of its items or on the exit below.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shop {
    pub game: GameState,
    pub tab: usize,
    pub cursor: usize,
}

impl Shop {
    pub fn new(game: GameState) -> Self {
        Self {
            game,
            tab: 0,
            cursor: 0,
        }
    }

    /// The item under the cursor, or `None` on the exit row.
    pub fn item(&self) -> Option<Item> {
        tab_items(self.tab).get(self.cursor).copied()
    }
}

/// The available actions, with a cursor on one of them or on the way back below them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionMenu {
    pub game: GameState,
    pub cursor: usize,
}

impl ActionMenu {
    /// The action under the cursor, or `None` on the way back.
    pub fn action(&self) -> Option<Action> {
        self.game.actions().get(self.cursor).copied()
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
            | Self::Team { game, .. }
            | Self::Report(game)
            | Self::Log { game, .. }
            | Self::Defenses(game)
            | Self::Attendees { game, .. }
            | Self::Tracks { game, .. }
            | Self::Shop(Shop { game, .. })
            | Self::Actions(ActionMenu { game, .. })
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
                Self::Shop(Shop::new(GameState::new(
                    company.trim(),
                    lead.trim(),
                    profile,
                    macroquad::rand::rand(),
                )))
            }
            (Self::Shop(mut shop), Input::Arrow(Hop::Up)) => {
                shop.cursor = shop.cursor.saturating_sub(1);
                Self::Shop(shop)
            }
            (Self::Shop(mut shop), Input::Arrow(Hop::Down)) => {
                shop.cursor = (shop.cursor + 1).min(tab_items(shop.tab).len());
                Self::Shop(shop)
            }
            (Self::Shop(mut shop), Input::Arrow(hop @ (Hop::Left | Hop::Right))) => {
                shop.tab = match hop {
                    Hop::Left => shop.tab.saturating_sub(1),
                    _ => (shop.tab + 1).min(TABS - 1),
                };
                shop.cursor = 0;
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
            (Self::Play(game), Input::Char('d' | 'D')) => Self::Defenses(game),
            (Self::Defenses(game), Input::Enter) => Self::Play(game),
            (Self::Play(game), Input::Char('l' | 'L')) => {
                let page = log_pages(&game.log).len() - 1;
                Self::Log { game, page }
            }
            (Self::Play(game), Input::Enter) if game.outcome.is_some() => Self::Report(game),
            (Self::Play(game), _) if game.outcome.is_some() => Self::Play(game),
            (Self::Report(_), Input::Enter) => Self::Title,
            (Self::Play(mut game), input) if !game.cards.is_empty() => {
                if input == Input::Enter {
                    game.reveal_next();
                }
                Self::Play(game)
            }
            (Self::Play(mut game), input) if game.incident.is_some() => {
                let responses = game.incident.unwrap().responses();
                if let Input::Char(c @ '1'..='9') = input
                    && let Some(response) = responses.get(c as usize - '1' as usize)
                    && game.can_respond(response)
                {
                    game.respond(c as usize - '1' as usize);
                }
                Self::Play(game)
            }
            (Self::Play(mut game), input) if game.event.is_some() => {
                if let Input::Char(c @ '1'..='9') = input
                    && (c as usize - '1' as usize) < game.event_choices().len()
                {
                    game.choose(c as usize - '1' as usize);
                }
                Self::Play(game)
            }
            (Self::Play(mut game), input)
                if game.pending_alert().is_none() && game.stop.is_some() =>
            {
                let stop = game.stop.as_ref().unwrap();
                let fort = stop.landmark.is_fort();
                if stop.landmark == crate::landmarks::Landmark::Conference && !stop.attended {
                    match input {
                        Input::Char('1') => {
                            return Self::Attendees {
                                game,
                                picked: Vec::new(),
                            };
                        }
                        Input::Char('2') => game.skip_conference(),
                        _ => {}
                    }
                    return Self::Play(game);
                }
                match input {
                    Input::Char('1') if fort => return Self::Shop(Shop::new(game)),
                    Input::Char('2') if fort => game.rest_at_fort(),
                    Input::Char('3') if fort => game.leave_fort(),
                    Input::Char(c @ '1'..='9')
                        if !fort && game.can_cross(c as usize - '1' as usize) =>
                    {
                        game.cross(c as usize - '1' as usize)
                    }
                    _ => {}
                }
                Self::Play(game)
            }
            (Self::Attendees { game, mut picked }, Input::Char(c @ '1'..='9')) => {
                let i = c as usize - '1' as usize;
                if i < game.team.len() {
                    if let Some(at) = picked.iter().position(|&p| p == i) {
                        picked.remove(at);
                    } else if game.conference_cost(picked.len() + 1) <= game.budget {
                        picked.push(i);
                    }
                }
                Self::Attendees { game, picked }
            }
            (Self::Attendees { mut game, picked }, Input::Enter) if picked.is_empty() => {
                game.attend(&[]);
                Self::Travel(Travel {
                    game,
                    elapsed_ms: 0,
                })
            }
            (Self::Attendees { game, picked }, Input::Enter) => Self::Tracks {
                game,
                picked,
                tracks: Vec::new(),
            },
            (
                Self::Tracks {
                    mut game,
                    picked,
                    mut tracks,
                },
                Input::Char(c @ '1'..='4'),
            ) => {
                tracks.push(Track::ALL[c as usize - '1' as usize]);
                if tracks.len() < picked.len() {
                    return Self::Tracks {
                        game,
                        picked,
                        tracks,
                    };
                }
                let picks: Vec<_> = picked.into_iter().zip(tracks).collect();
                game.attend(&picks);
                Self::Travel(Travel {
                    game,
                    elapsed_ms: 0,
                })
            }
            (Self::Play(mut game), input) if game.pending_alert().is_some() => {
                let reply = match input {
                    Input::Char('1') => Some(Reply::Investigate),
                    Input::Char('2') => Some(Reply::CallIr),
                    Input::Char('3') => Some(Reply::Ignore),
                    _ => None,
                };
                if let Some(reply) = reply
                    && game.can_reply(reply)
                {
                    game.reply(reply);
                }
                Self::Play(game)
            }
            (Self::Play(game), Input::Char('1')) => Self::Travel(Travel {
                game,
                elapsed_ms: 0,
            }),
            (Self::Play(game), Input::Char('2')) if !game.actions().is_empty() => {
                Self::Actions(ActionMenu { game, cursor: 0 })
            }
            (Self::Play(game), Input::Char('3')) => Self::Team {
                game,
                cursor: 0,
                confirming: false,
            },
            (Self::Play(mut game), Input::Char('4')) => {
                game.tempo = game.tempo.next();
                Self::Play(game)
            }
            (Self::Actions(mut menu), Input::Arrow(Hop::Up)) => {
                menu.cursor = menu.cursor.saturating_sub(1);
                Self::Actions(menu)
            }
            (Self::Actions(mut menu), Input::Arrow(Hop::Down)) => {
                menu.cursor = (menu.cursor + 1).min(menu.game.actions().len());
                Self::Actions(menu)
            }
            (Self::Actions(mut menu), Input::Enter) => match menu.action() {
                None => Self::Play(menu.game),
                Some(action) => {
                    menu.game.start(action);
                    Self::Travel(Travel {
                        game: menu.game,
                        elapsed_ms: 0,
                    })
                }
            },
            (Self::Log { game, page }, Input::Arrow(Hop::Left)) => Self::Log {
                game,
                page: page.saturating_sub(1),
            },
            (Self::Log { game, page }, Input::Arrow(Hop::Right)) => {
                let last = log_pages(&game.log).len() - 1;
                Self::Log {
                    game,
                    page: (page + 1).min(last),
                }
            }
            (Self::Log { game, .. }, Input::Enter) => Self::Play(game),
            (Self::Play(mut game), Input::Char('5')) if game.can_send_intern() => {
                game.send_intern();
                Self::Coffee(CoffeeRun {
                    game,
                    street: Street::new(),
                })
            }
            (Self::Play(game), Input::Char('6')) => Self::Shop(Shop::new(game)),
            (Self::Travel(travel), _) => Self::Play(travel.game),
            (
                Self::Team {
                    mut game,
                    cursor,
                    confirming: true,
                },
                input,
            ) => {
                if let Input::Char('y' | 'Y') = input {
                    game.fire(cursor);
                }
                let cursor = cursor.min(game.team.len() - 1);
                Self::Team {
                    game,
                    cursor,
                    confirming: false,
                }
            }
            (Self::Team { game, .. }, Input::Enter) => Self::Play(game),
            (Self::Team { game, cursor, .. }, Input::Char('f' | 'F')) => Self::Team {
                confirming: game.can_fire(cursor),
                game,
                cursor,
            },
            (Self::Team { game, cursor, .. }, Input::Arrow(hop)) => {
                let cursor = match hop {
                    Hop::Up => cursor.saturating_sub(1),
                    Hop::Down => (cursor + 1).min(game.team.len() - 1),
                    _ => cursor,
                };
                Self::Team {
                    game,
                    cursor,
                    confirming: false,
                }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineKind {
    Day,
    Alert,
    Text,
}

/// A line of the log as laid out: its text, what it is, and the space above it.
#[derive(Debug, Clone, PartialEq)]
pub struct LogLine {
    pub text: String,
    pub kind: LineKind,
    pub gap: f32,
}

fn day_header(day: u32, continued: bool) -> String {
    let more = if continued { " (cont.)" } else { "" };
    format!("DAY {day} {}{more}", WEEKDAYS[(day as usize - 1) % 7])
}

/// One entry's lines, led by a day header when it starts a new day.
fn entry_lines(entry: &Entry, width: usize, header: bool, first: bool) -> Vec<LogLine> {
    let mut lines = Vec::new();
    if header {
        lines.push(LogLine {
            text: day_header(entry.day, false),
            kind: LineKind::Day,
            gap: if first { 0.0 } else { DAY_GAP },
        });
    }
    let kind = if entry.starts_with("ALERT") || entry.starts_with("INCIDENT") {
        LineKind::Alert
    } else {
        LineKind::Text
    };
    for (i, text) in wrap(entry, width).into_iter().enumerate() {
        let gap = if i == 0 { ENTRY_GAP } else { 0.0 };
        lines.push(LogLine { text, kind, gap });
    }
    lines
}

/// The whole log as lines of `width`, grouped under a header for each day.
pub fn log_lines(entries: &[Entry], width: usize) -> Vec<LogLine> {
    let mut lines = Vec::new();
    for (i, entry) in entries.iter().enumerate() {
        let header = i == 0 || entries[i - 1].day != entry.day;
        lines.extend(entry_lines(entry, width, header, i == 0));
    }
    lines
}

/// Height of lines drawn top-down, ignoring the gap above the first.
fn height(lines: &[LogLine]) -> f32 {
    lines.iter().skip(1).map(|l| l.gap).sum::<f32>() + LOG_LINE * lines.len() as f32
}

/// The whole log split into pages that fit the full-log screen, oldest first.
/// An entry is never split, and a day continued from the last page repeats its header.
pub fn log_pages(entries: &[Entry]) -> Vec<Vec<LogLine>> {
    let mut pages: Vec<Vec<LogLine>> = vec![Vec::new()];
    for (i, entry) in entries.iter().enumerate() {
        let new_day = i == 0 || entries[i - 1].day != entry.day;
        let page = pages.last_mut().unwrap();
        let mut lines = entry_lines(entry, PAGE_WIDTH, new_day, page.is_empty());
        let mut grown = page.clone();
        grown.extend(lines.iter().cloned());
        if height(&grown) <= PAGE_HEIGHT || page.is_empty() {
            *page = grown;
            continue;
        }
        if !new_day {
            lines.insert(
                0,
                LogLine {
                    text: day_header(entry.day, true),
                    kind: LineKind::Day,
                    gap: 0.0,
                },
            );
        } else {
            lines[0].gap = 0.0;
        }
        pages.push(lines);
    }
    pages
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

    /// Procurement for a Healthtech company.
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

    /// Leaves Procurement through the exit row.
    fn open_for_business(screen: Screen) -> Screen {
        arrows(screen, Hop::Down, Item::ALL.len()).update(Input::Enter)
    }

    /// Day 1 with two junior analysts hired, on a seed where nothing stops the clock for five days.
    fn new_game() -> Screen {
        let screen = open_for_business(hire_junior(hire_junior(vendor_hall(), "Maya"), "Dev"));
        with_game(screen, |game| {
            game.seed = (0..)
                .find(|&seed| {
                    let mut quiet = game.clone();
                    quiet.seed = seed;
                    (0..5).all(|_| !quiet.advance())
                })
                .unwrap();
        })
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
        assert_eq!(shop(&screen).item(), Some(Item::Junior));

        let screen = arrows(screen, Hop::Down, 20);
        assert_eq!(shop(&screen).cursor, 2, "stops on the exit below the staff");
        assert_eq!(shop(&screen).item(), None);

        let screen = screen.update(Input::Arrow(Hop::Right));
        assert_eq!(shop(&screen).cursor, 0, "a new tab starts at the top");
        assert_eq!(shop(&screen).item(), Some(Item::PasswordManager));
        let screen = arrows(screen, Hop::Right, 20);
        assert_eq!(tab_name(shop(&screen).tab), "Services");
        let screen = arrows(screen, Hop::Left, 20);
        assert_eq!(shop(&screen).tab, 0);
    }

    #[test]
    fn enter_buys_the_item_under_the_cursor() {
        let screen = arrows(vendor_hall(), Hop::Right, 1);
        let screen = arrows(screen, Hop::Down, 1).update(Input::Enter);
        let item = Item::MfaTokens;

        assert_eq!(shop(&screen).game.owned, [item]);
        assert_eq!(
            shop(&screen).game.budget,
            Profile::Healthtech.budget() - item.price()
        );
        assert_eq!(screen.clone().update(Input::Enter), screen, "bought once");
    }

    #[test]
    fn unaffordable_items_are_not_bought() {
        let mut screen = arrows(vendor_hall(), Hop::Right, 1);
        if let Screen::Shop(shop) = &mut screen {
            shop.game.budget = 0;
        }

        assert_eq!(screen.clone().update(Input::Enter), screen);
    }

    #[test]
    fn every_item_sits_on_exactly_one_tab() {
        let listed: Vec<Item> = (0..TABS).flat_map(tab_items).collect();
        assert_eq!(listed.len(), Item::ALL.len());
        for item in Item::ALL {
            assert!(listed.contains(&item), "{item:?} is unreachable");
        }
        for tab in 1..TABS - 1 {
            assert!(
                tab_items(tab)
                    .iter()
                    .all(|i| i.category() == Some(Area::ALL[tab - 1])),
                "{}",
                tab_name(tab)
            );
        }
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
    fn three_checks_the_team_until_enter() {
        let start = new_game();
        let screen = start.clone().update(Input::Char('3'));
        assert!(matches!(screen, Screen::Team { .. }));
        assert_eq!(screen.clone().update(Input::Char('1')), screen);

        assert_eq!(screen.update(Input::Enter), start);
    }

    fn action_menu(screen: &Screen) -> &ActionMenu {
        match screen {
            Screen::Actions(menu) => menu,
            other => panic!("expected Actions, got {other:?}"),
        }
    }

    /// Opens the action menu and moves the cursor to `action`.
    fn choose(screen: Screen, action: Action) -> Screen {
        let screen = screen.update(Input::Char('2'));
        let index = action_menu(&screen)
            .game
            .actions()
            .iter()
            .position(|a| *a == action)
            .unwrap();
        arrows(screen, Hop::Down, index)
    }

    #[test]
    fn two_lists_actions_and_back_returns_to_the_day_menu() {
        let start = new_game();
        let screen = start.clone().update(Input::Char('2'));
        let count = action_menu(&screen).game.actions().len();
        assert_eq!(action_menu(&screen).action(), Some(Action::PatchSprint));

        let screen = arrows(screen, Hop::Down, count + 3);
        assert_eq!(action_menu(&screen).cursor, count);
        assert_eq!(screen.update(Input::Enter), start);
    }

    #[test]
    fn choosing_an_action_starts_it_and_runs_the_clock() {
        let screen = choose(new_game(), Action::PatchSprint).update(Input::Enter);

        let Screen::Travel(travel) = &screen else {
            panic!("expected Travel, got {screen:?}");
        };
        assert_eq!(
            travel.game.task.as_ref().unwrap().action,
            Action::PatchSprint
        );

        let screen = screen.tick(DAY_SECONDS * 10.0);
        assert!(
            matches!(screen, Screen::Play(_)),
            "finishing stops the clock"
        );
        assert_eq!(game(&screen).task, None);
        assert_eq!(game(&screen).day, 5);
    }

    #[test]
    fn no_new_action_while_one_is_under_way() {
        let screen = choose(new_game(), Action::PatchSprint)
            .update(Input::Enter)
            .update(Input::Char('x'));
        assert!(matches!(screen, Screen::Play(_)));

        assert_eq!(screen.clone().update(Input::Char('2')), screen);
        assert!(
            matches!(screen.update(Input::Char('1')), Screen::Travel(_)),
            "resume"
        );
    }

    #[test]
    fn six_opens_the_vendor_hall_any_day() {
        let start = with_game(new_game(), |game| game.day = 20);
        let screen = start.clone().update(Input::Char('6'));
        assert!(matches!(screen, Screen::Shop(_)));

        let screen = arrows(screen, Hop::Right, 1).update(Input::Enter);
        assert_eq!(
            game(&screen).owned,
            [Item::PasswordManager],
            "bought mid-game"
        );

        let screen = arrows(screen, Hop::Left, 1).update(Input::Enter);
        let screen = type_text(screen, "Priya").update(Input::Enter);
        assert_eq!(
            game(&screen).searches[0].name,
            "Priya",
            "a search, not a hire"
        );
        assert_eq!(game(&screen).analysts(), 2);

        let screen = arrows(screen, Hop::Down, 20).update(Input::Enter);
        assert!(matches!(screen, Screen::Play(_)), "back to the day menu");
    }

    #[test]
    fn the_vendor_hall_waits_while_something_is_pending() {
        let screen = alerted();

        assert_eq!(screen.clone().update(Input::Char('6')), screen);
    }

    fn page(screen: &Screen) -> usize {
        match screen {
            Screen::Log { page, .. } => *page,
            other => panic!("expected Log, got {other:?}"),
        }
    }

    /// A log of `days` days, each with an entry long enough to wrap.
    fn long_log(days: u32) -> Vec<Entry> {
        (1..=days)
            .flat_map(|day| {
                [
                    Entry {
                        day,
                        text: format!("Day {day} happened."),
                    },
                    Entry {
                        day,
                        text: "A long entry that goes on and on about a very busy day in the SOC, \
                               long enough to wrap onto a second line of the page."
                            .to_string(),
                    },
                ]
            })
            .collect()
    }

    #[test]
    fn the_log_pages_hold_every_entry_once_in_order() {
        let log = long_log(40);
        let pages = log_pages(&log);
        assert!(pages.len() > 1);

        let text: Vec<String> = pages
            .iter()
            .flatten()
            .filter(|l| l.kind != LineKind::Day)
            .map(|l| l.text.clone())
            .collect();
        let expected: Vec<String> = log.iter().flat_map(|e| wrap(e, PAGE_WIDTH)).collect();
        assert_eq!(text, expected);
        for page in &pages {
            assert!(height(page) <= PAGE_HEIGHT);
            assert_eq!(
                page[0].kind,
                LineKind::Day,
                "every page starts with its day"
            );
            assert_eq!(page[0].gap, 0.0);
        }
    }

    #[test]
    fn a_day_split_across_pages_repeats_its_header() {
        let log: Vec<Entry> = (0..60)
            .map(|i| Entry {
                day: 3,
                text: format!("Entry {i}"),
            })
            .collect();
        let pages = log_pages(&log);

        assert!(pages.len() > 1);
        assert_eq!(pages[0][0].text, "DAY 3 WED");
        assert_eq!(pages[1][0].text, "DAY 3 WED (cont.)");
    }

    #[test]
    fn l_opens_the_newest_page_and_arrows_turn_pages() {
        let start = with_game(new_game(), |game| game.log = long_log(40));
        let last = log_pages(&game(&start).log).len() - 1;
        let screen = start.clone().update(Input::Char('l'));
        assert_eq!(page(&screen), last, "starts on the newest page");

        let screen = screen.update(Input::Arrow(Hop::Right));
        assert_eq!(page(&screen), last, "no page past the newest");
        let screen = arrows(screen, Hop::Left, 2);
        assert_eq!(page(&screen), last - 2);
        let screen = arrows(screen, Hop::Left, 100);
        assert_eq!(page(&screen), 0, "no page before the oldest");
        let screen = screen.update(Input::Arrow(Hop::Right));
        assert_eq!(page(&screen), 1);

        assert_eq!(screen.update(Input::Enter), start);
    }

    #[test]
    fn d_opens_the_defenses_from_any_play_screen() {
        let screen = alerted().update(Input::Char('d'));
        assert!(matches!(screen, Screen::Defenses(_)));
        assert_eq!(screen.clone().update(Input::Char('1')), screen);

        let screen = screen.update(Input::Enter);
        assert!(
            game(&screen).pending_alert().is_some(),
            "the alert still waits"
        );
    }

    #[test]
    fn the_full_log_opens_even_while_something_is_waiting() {
        let screen = alerted().update(Input::Char('L'));
        assert!(matches!(screen, Screen::Log { .. }));

        let screen = screen.update(Input::Enter);
        assert!(
            game(&screen).pending_alert().is_some(),
            "the alert still waits"
        );
    }

    #[test]
    fn finished_actions_show_a_result_card() {
        let screen = choose(new_game(), Action::Tabletop).update(Input::Enter);
        let screen = screen.tick(DAY_SECONDS * 2.0);

        let card = &game(&screen).cards[0];
        assert_eq!(card.title, "TABLETOP EXERCISE");
        assert!(card.effect.contains("Trust +2"));
        assert!(card.effect.contains("Resilience improved"));
        assert_eq!(
            screen.clone().update(Input::Char('1')),
            screen,
            "ENTER first"
        );
        assert!(game(&screen.update(Input::Enter)).cards.is_empty());
    }

    fn team(screen: &Screen) -> (usize, bool) {
        match screen {
            Screen::Team {
                cursor, confirming, ..
            } => (*cursor, *confirming),
            other => panic!("expected Team, got {other:?}"),
        }
    }

    #[test]
    fn analysts_are_fired_from_the_team_screen_after_confirming() {
        let screen = new_game().update(Input::Char('3'));
        assert_eq!(team(&screen), (0, false));

        let screen = screen.update(Input::Char('f'));
        assert_eq!(team(&screen), (0, true), "asks first");
        let kept = screen.clone().update(Input::Char('n'));
        assert_eq!(team(&kept), (0, false));
        assert_eq!(game(&kept).analysts(), 2);

        let screen = screen.update(Input::Char('y'));
        assert_eq!(game(&screen).analysts(), 1);
        assert!(game(&screen).team.iter().all(|m| m.name != "Maya"));

        let screen = screen.update(Input::Char('f'));
        assert_eq!(team(&screen), (0, false), "not the last analyst");
        let screen = arrows(screen, Hop::Down, 10);
        assert_eq!(team(&screen).0, 0, "cursor stays on the roster");
    }

    #[test]
    fn four_cycles_the_tempo() {
        let screen = new_game().update(Input::Char('4'));
        assert_eq!(game(&screen).tempo, Tempo::Crunch);

        let screen = screen.update(Input::Char('4'));
        assert_eq!(game(&screen).tempo, Tempo::Relaxed);
    }

    fn street(screen: &mut Screen) -> &mut Street {
        match screen {
            Screen::Coffee(run) => &mut run.street,
            other => panic!("expected Coffee, got {other:?}"),
        }
    }

    #[test]
    fn five_sends_the_intern_across_the_street() {
        let before = new_game();
        let mut screen = before.clone().update(Input::Char('5'));

        assert_eq!(game(&screen).budget, game(&before).budget - COFFEE_RUN_COST);
        assert_eq!(street(&mut screen).row, 0);
        let mut screen = screen.update(Input::Arrow(Hop::Up));
        assert_eq!(street(&mut screen).row, 1);
    }

    #[test]
    fn enter_only_leaves_the_street_once_the_run_is_over() {
        let coffee = game(&new_game()).coffee;
        let mut screen = new_game().update(Input::Char('5'));
        let running = screen.clone();
        assert_eq!(running.clone().update(Input::Enter), running);

        street(&mut screen).survived = Some(true);
        let screen = screen.update(Input::Enter);

        assert!(matches!(screen, Screen::Play(_)));
        assert_eq!(game(&screen).coffee, coffee + 6);
        assert_eq!(game(&screen).cards[0].title, "COFFEE RUN");
        let screen = screen.update(Input::Enter);
        assert_eq!(
            screen.clone().update(Input::Char('5')),
            screen,
            "once a week"
        );
    }

    #[test]
    fn ticks_only_move_the_street_and_the_clock() {
        assert_eq!(Screen::Title.tick(1.0), Screen::Title);
        let start = new_game();
        assert_eq!(start.clone().tick(1.0), start);

        let mut screen = new_game().update(Input::Char('5'));
        let before = street(&mut screen).lanes.clone();
        let mut screen = screen.tick(0.1);
        assert_ne!(street(&mut screen).lanes, before);
    }

    #[test]
    fn street_cues_hops_escapes_and_crashes_but_not_traffic() {
        let mut before = new_game().update(Input::Char('5'));
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
        let screen = new_game().update(Input::Char('5'));
        let screen = match screen {
            Screen::Coffee(mut run) => {
                run.street.survived = Some(false);
                Screen::Coffee(run).update(Input::Enter)
            }
            other => panic!("expected Coffee, got {other:?}"),
        };

        assert_eq!(screen.clone().update(Input::Char('5')), screen);
    }

    #[test]
    fn invalid_keys_are_ignored() {
        let screen = new_game();

        assert_eq!(screen.clone().update(Input::Char('7')), screen);
    }

    #[test]
    fn enter_after_the_outcome_shows_the_report_then_the_title() {
        let screen = with_game(new_game(), |game| game.outcome = Some(Outcome::Fired));

        let screen = screen.update(Input::Enter);
        assert!(matches!(screen, Screen::Report(_)));
        assert_eq!(screen.clone().update(Input::Char('1')), screen);
        assert_eq!(screen.update(Input::Enter), Screen::Title);
    }

    fn alerted() -> Screen {
        with_game(new_game(), |game| game.raise_false_alarm())
    }

    #[test]
    fn an_alert_takes_over_the_day_menu_until_answered() {
        let screen = alerted();
        assert_eq!(screen.clone().update(Input::Char('5')), screen);
        assert_eq!(
            screen.clone().update(Input::Char('2')),
            screen,
            "no IR retainer"
        );

        let screen = screen.update(Input::Char('3'));
        assert_eq!(game(&screen).pending_alert(), None);
        assert!(matches!(screen.update(Input::Char('1')), Screen::Travel(_)));
    }

    #[test]
    fn one_investigates_an_alert() {
        let screen = alerted().update(Input::Char('1'));

        assert!(game(&screen).investigation.is_some());
        assert_eq!(game(&screen).pending_alert(), None);
    }

    #[test]
    fn an_incident_waits_for_a_possible_response() {
        let screen = with_game(new_game(), |game| {
            game.incident = Some(crate::attack::Actor::Ransomware)
        });
        assert_eq!(
            screen.clone().update(Input::Char('1')),
            screen,
            "no backups"
        );
        assert_eq!(screen.clone().update(Input::Char('9')), screen);

        let screen = screen.update(Input::Char('3'));

        assert_eq!(game(&screen).incident, None);
        assert!(!game(&screen).conditions.is_empty());
    }

    #[test]
    fn an_event_waits_for_a_choice() {
        let screen = with_game(new_game(), |game| {
            game.event = Some(crate::game::PendingEvent {
                index: crate::events::PET_PROJECT,
                patient: 3,
            })
        });
        let trust = game(&screen).trust;
        assert_eq!(screen.clone().update(Input::Char('3')), screen);
        assert_eq!(screen.clone().update(Input::Enter), screen);

        let screen = screen.update(Input::Char('2'));

        assert_eq!(game(&screen).event, None);
        assert_eq!(game(&screen).trust, trust - 5);
    }

    fn at(landmark: crate::landmarks::Landmark) -> Screen {
        with_game(new_game(), |game| {
            game.stop = Some(crate::game::Stop {
                landmark,
                rested: false,
                attended: true,
            })
        })
    }

    #[test]
    fn forts_offer_the_vendor_hall_rest_and_the_road() {
        let fort = at(crate::landmarks::Landmark::Conference);
        assert_eq!(fort.clone().update(Input::Char('4')), fort);

        let hall = fort.clone().update(Input::Char('1'));
        assert!(matches!(hall, Screen::Shop(_)));
        let back = open_for_business(hall);
        assert!(game(&back).stop.is_some(), "back at the fort");

        let rested = fort.clone().update(Input::Char('2'));
        assert!(game(&rested).stop.as_ref().unwrap().rested);

        let moved_on = fort.update(Input::Char('3'));
        assert_eq!(game(&moved_on).stop, None);
        assert!(matches!(
            moved_on.update(Input::Char('1')),
            Screen::Travel(_)
        ));
    }

    /// The conference fort on arrival, before anyone has picked who goes.
    fn conference() -> Screen {
        with_game(new_game(), |game| {
            game.stop = Some(crate::game::Stop {
                landmark: crate::landmarks::Landmark::Conference,
                rested: false,
                attended: false,
            })
        })
    }

    #[test]
    fn the_conference_starts_with_picking_who_goes() {
        let screen = conference().update(Input::Char('1'));
        assert!(matches!(screen, Screen::Attendees { .. }));

        let skipped = conference().update(Input::Char('2'));
        assert!(game(&skipped).stop.as_ref().unwrap().attended);
    }

    #[test]
    fn number_keys_toggle_attendees_within_the_budget() {
        let screen = conference().update(Input::Char('1'));
        let picked = |s: &Screen| match s {
            Screen::Attendees { picked, .. } => picked.clone(),
            other => panic!("expected Attendees, got {other:?}"),
        };

        let screen = screen.update(Input::Char('2'));
        assert_eq!(picked(&screen), [1], "Dev is the second analyst");
        let screen = screen.update(Input::Char('1')).update(Input::Char('2'));
        assert_eq!(picked(&screen), [0]);
        assert_eq!(picked(&screen.clone().update(Input::Char('9'))), [0]);

        let broke = match conference().update(Input::Char('1')) {
            Screen::Attendees { mut game, picked } => {
                game.budget = game.conference_cost(1);
                Screen::Attendees { game, picked }
            }
            other => panic!("expected Attendees, got {other:?}"),
        };
        let broke = broke.update(Input::Char('1')).update(Input::Char('2'));
        assert_eq!(picked(&broke).len(), 1, "cannot afford a second ticket");
    }

    #[test]
    fn each_attendee_picks_a_track_then_the_clock_runs() {
        let screen = conference()
            .update(Input::Char('1'))
            .update(Input::Char('1'))
            .update(Input::Char('2'))
            .update(Input::Enter);
        assert!(matches!(screen, Screen::Tracks { .. }));

        let screen = screen.update(Input::Char('1'));
        assert!(matches!(screen, Screen::Tracks { .. }));
        let screen = screen.update(Input::Char('4'));

        let Screen::Travel(travel) = &screen else {
            panic!("expected Travel, got {screen:?}");
        };
        assert_eq!(travel.game.team[0].track, Some(Track::Talks));
        assert_eq!(travel.game.team[1].track, Some(Track::Hallway));
    }

    #[test]
    fn going_alone_skips_the_tracks() {
        let screen = conference().update(Input::Char('1')).update(Input::Enter);

        assert!(matches!(screen, Screen::Travel(_)));
        assert!(game(&screen).conference_days > 0);
    }

    #[test]
    fn conference_cards_are_revealed_with_enter() {
        let screen = conference().update(Input::Char('1')).update(Input::Enter);
        let screen = screen.tick(DAY_SECONDS * 3.0);
        assert!(matches!(screen, Screen::Play(_)));
        assert_eq!(game(&screen).cards.len(), 1, "the lead's card");
        assert_eq!(screen.clone().update(Input::Char('1')), screen);

        let screen = screen.update(Input::Enter);

        assert!(game(&screen).cards.is_empty());
        assert!(game(&screen).stop.as_ref().unwrap().attended);
    }

    #[test]
    fn rivers_are_crossed_with_a_number_key() {
        let river = at(crate::landmarks::Landmark::BoardBriefing);
        assert!(matches!(
            river.clone().update(Input::Char('1')),
            Screen::Play(_)
        ));
        assert_eq!(river.clone().update(Input::Char('4')), river);

        let crossed = river.update(Input::Char('2'));

        assert_eq!(game(&crossed).stop, None);
    }

    #[test]
    fn alerts_incidents_events_and_landmarks_each_have_a_sound() {
        let before = new_game();
        let incident = with_game(new_game(), |game| {
            game.incident = Some(crate::attack::Actor::Hacktivists)
        });
        let event = with_game(new_game(), |game| {
            game.event = Some(crate::game::PendingEvent {
                index: 1,
                patient: 3,
            })
        });

        assert_eq!(cue(&before, &alerted()), Some(Cue::Klaxon));
        assert_eq!(cue(&before, &incident), Some(Cue::Sting));
        assert_eq!(cue(&before, &event), Some(Cue::Alarm));

        let traveling = with_game(new_game(), |game| {
            game.next_landmark = 0;
            game.day = crate::landmarks::Landmark::BoardBriefing.day() - 1;
        })
        .update(Input::Char('1'));
        let arrived = traveling.clone().tick(DAY_SECONDS);
        assert!(game(&arrived).stop.is_some());
        assert_eq!(cue(&traveling, &arrived), Some(Cue::Fanfare));
    }

    #[test]
    fn typing_and_choosing_cue_a_select_sound() {
        let before = Screen::Company(String::new());
        let after = before.clone().update(Input::Char('A'));
        assert_eq!(cue(&before, &after), Some(Cue::Select));

        let before = new_game();
        let after = before.clone().update(Input::Char('4'));
        assert_eq!(cue(&before, &after), Some(Cue::Select));
    }

    #[test]
    fn ignored_input_and_waiting_are_silent() {
        let before = Screen::Company(String::new());
        let after = before.clone().update(Input::Enter);
        assert_eq!(cue(&before, &after), None);

        let before = new_game().update(Input::Char('1'));
        let after = before.clone().tick(DAY_SECONDS / 2.0);
        assert_eq!(cue(&before, &after), None);
    }

    #[test]
    fn days_tick_and_mondays_pay() {
        let before = new_game().update(Input::Char('1'));
        let after = before.clone().tick(DAY_SECONDS);
        assert_eq!(game(&after).day, 2);
        assert_eq!(cue(&before, &after), Some(Cue::Tick));

        let sunday = with_game(new_game(), |game| game.day = 7).update(Input::Char('1'));
        let monday = sunday.clone().tick(DAY_SECONDS);
        assert_eq!(cue(&sunday, &monday), Some(Cue::Payday));
    }

    #[test]
    fn losing_a_team_member_cues_the_alarm() {
        let before =
            with_game(new_game(), |game| game.team[0].burnout = 99).update(Input::Char('1'));
        let after = before.clone().tick(DAY_SECONDS);

        assert_eq!(cue(&before, &after), Some(Cue::Alarm));
    }

    #[test]
    fn the_ipo_rings_the_bell_and_other_endings_lose() {
        let last_day = |trust| {
            with_game(new_game(), |game| {
                game.day = IPO_DAY - 1;
                game.trust = trust;
            })
            .update(Input::Char('1'))
        };
        let (won, lost) = (last_day(60), last_day(0));

        assert_eq!(cue(&won, &won.clone().tick(DAY_SECONDS)), Some(Cue::Bell));
        assert_eq!(cue(&lost, &lost.clone().tick(DAY_SECONDS)), Some(Cue::Lose));
    }

    #[test]
    fn wrap_breaks_on_word_boundaries() {
        assert_eq!(wrap("the quick brown fox", 10), ["the quick", "brown fox"]);
        assert_eq!(wrap("short", 10), ["short"]);
    }
}
