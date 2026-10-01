use font8x8::legacy::BASIC_LEGACY;
use macroquad::prelude::*;

use crate::audio::TRACKS;
use crate::game::{
    Action, COFFEE_CAPACITY, COFFEE_RUN_COST, GameState, IR_FEE, Item, Outcome, Profile, Reply,
    Role, TeamMember, money,
};
use crate::landmarks::Landmark;
use crate::street::{self, Leg, Street};
use crate::ui::{ActionMenu, CoffeeRun, Music, Screen, Shop, wrap};

pub const WIDTH: f32 = 640.0;
pub const HEIGHT: f32 = 480.0;

const BG: Color = Color::from_hex(0x000000);
const NAVY: Color = Color::from_hex(0x1d2b53);
const DIM: Color = Color::from_hex(0x5f574f);
const INK: Color = Color::from_hex(0xfff1e8);
const GREEN: Color = Color::from_hex(0x00e436);
const DARK_GREEN: Color = Color::from_hex(0x008751);
const CYAN: Color = Color::from_hex(0x29adff);
const AMBER: Color = Color::from_hex(0xffa300);
const RED: Color = Color::from_hex(0xff004d);
const LIGHT: Color = Color::from_hex(0xc2c3c7);
const BROWN: Color = Color::from_hex(0xab5236);
const YELLOW: Color = Color::from_hex(0xffec27);
const PEACH: Color = Color::from_hex(0xffccaa);

const GLYPH: f32 = 8.0;
const MARGIN: f32 = 16.0;
/// Left edge of the log column, right of the status panel.
const LOG_X: f32 = 264.0;
const LOG_LINES: usize = 29;
const WEEKDAYS: [&str; 7] = ["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];

/// 8x8 bitmap font rendered from a texture atlas of the 128 ASCII glyphs.
pub struct Font(Texture2D);

impl Font {
    pub fn load() -> Self {
        let mut image = Image::gen_image_color(128 * GLYPH as u16, GLYPH as u16, BLANK);
        for (code, rows) in BASIC_LEGACY.iter().enumerate() {
            for (y, row) in rows.iter().enumerate() {
                for x in 0..8 {
                    if row >> x & 1 == 1 {
                        image.set_pixel((code * 8 + x) as u32, y as u32, WHITE);
                    }
                }
            }
        }
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);
        Self(texture)
    }

    fn text(&self, text: &str, x: f32, y: f32, scale: f32, color: Color) {
        let size = GLYPH * scale;
        for (i, c) in text.chars().enumerate() {
            let code = if c.is_ascii() { c as u8 } else { b'?' };
            draw_texture_ex(
                &self.0,
                x + i as f32 * size,
                y,
                color,
                DrawTextureParams {
                    source: Some(Rect::new(code as f32 * GLYPH, 0.0, GLYPH, GLYPH)),
                    dest_size: Some(vec2(size, size)),
                    ..Default::default()
                },
            );
        }
    }

    fn centered(&self, text: &str, center_x: f32, y: f32, scale: f32, color: Color) {
        let x = (center_x - text.len() as f32 * GLYPH * scale / 2.0).floor();
        self.text(text, x, y, scale, color);
    }
}

pub fn screen(screen: &Screen, font: &Font, music: Music) {
    clear_background(BG);
    match screen {
        Screen::Title => title(font, music),
        Screen::Company(company) => {
            name_entry(font, "Name the company you are protecting:", company)
        }
        Screen::Lead { company, lead } => name_entry(
            font,
            &format!("{} needs an incident lead. Your name:", company.trim()),
            lead,
        ),
        Screen::Profile { company, .. } => profile_choice(font, company.trim()),
        Screen::Shop(shop) => vendor_hall(font, shop),
        Screen::Hire { shop, name } => {
            vendor_hall(font, shop);
            let level = if shop.item() == Some(Item::Senior) {
                "senior"
            } else {
                "junior"
            };
            name_popup(font, &format!("Name your new {level} analyst:"), name);
        }
        Screen::Actions(menu) => action_menu(font, menu),
        Screen::Recruit { menu, name } => {
            action_menu(font, menu);
            let level = if menu.action() == Some(Action::Recruit(Item::Senior)) {
                "senior"
            } else {
                "junior"
            };
            name_popup(font, &format!("Name the {level} analyst to recruit:"), name);
        }
        Screen::Play(game) => play(font, game, false),
        Screen::Travel(travel) => play(font, &travel.game, true),
        Screen::Team(game) => team_screen(font, game),
        Screen::Coffee(run) => coffee_run(font, run),
        Screen::Report(game) => after_action(font, game),
    }
}

fn blink() -> bool {
    get_time().fract() < 0.5
}

/// Cheap deterministic pseudo-random bits for decoration.
fn noise(n: usize) -> bool {
    let x = (n as u32).wrapping_mul(0x9E37_79B9);
    (x ^ x >> 15).wrapping_mul(0x85EB_CA6B) >> 16 & 1 == 1
}

fn binary_band(font: &Font, y: f32) {
    let offset = (get_time() * 6.0) as usize;
    let bits: String = (0..80)
        .map(|i| if noise(i + offset) { '1' } else { '0' })
        .collect();
    font.text(&bits, 0.0, y, 1.0, DARK_GREEN);
}

fn server_racks(y: f32) {
    let tick = (get_time() * 4.0) as usize;
    for rack in 0..8 {
        let x = 36.0 + rack as f32 * 72.0;
        draw_rectangle(x, y, 40.0, 80.0, NAVY);
        draw_rectangle_lines(x, y, 40.0, 80.0, 2.0, DIM);
        for slot in 0..6 {
            let slot_y = y + 8.0 + slot as f32 * 12.0;
            draw_line(x + 4.0, slot_y + 6.0, x + 28.0, slot_y + 6.0, 2.0, DIM);
            let led = if noise(rack * 31 + slot * 7 + tick) {
                GREEN
            } else {
                RED
            };
            draw_rectangle(x + 32.0, slot_y + 4.0, 4.0, 4.0, led);
        }
    }
}

fn title(font: &Font, music: Music) {
    binary_band(font, 16.0);
    font.centered("THE KILL CHAIN", WIDTH / 2.0, 96.0, 4.0, GREEN);
    font.centered("TRAIL", WIDTH / 2.0, 136.0, 4.0, GREEN);
    font.centered(
        "A security operations survival game",
        WIDTH / 2.0,
        192.0,
        1.0,
        CYAN,
    );
    server_racks(248.0);
    music_menu(font, music);
    if blink() {
        font.centered("PRESS ENTER TO BEGIN", WIDTH / 2.0, 400.0, 2.0, AMBER);
    }
    binary_band(font, 456.0);
}

fn music_menu(font: &Font, music: Music) {
    font.centered("Choose music with 1-4", WIDTH / 2.0, 344.0, 1.0, DIM);
    let options: Vec<(String, Music)> = TRACKS
        .iter()
        .enumerate()
        .map(|(i, name)| (format!("{} {name}", i + 1), Music::Track(i)))
        .chain([(format!("{} Off", TRACKS.len() + 1), Music::Off)])
        .collect();
    let gap = 3;
    let len: usize = options
        .iter()
        .map(|(label, _)| label.len() + gap)
        .sum::<usize>()
        - gap;
    let mut x = ((WIDTH - len as f32 * GLYPH) / 2.0).floor();
    for (label, option) in options {
        let color = if option == music { AMBER } else { INK };
        font.text(&label, x, 360.0, 1.0, color);
        x += (label.len() + gap) as f32 * GLYPH;
    }
}

fn name_entry(font: &Font, prompt: &str, value: &str) {
    font.centered("THE KILL CHAIN TRAIL", WIDTH / 2.0, 48.0, 2.0, GREEN);
    font.centered(prompt, WIDTH / 2.0, 192.0, 1.0, INK);

    let box_width = 21.0 * GLYPH * 2.0 + 16.0;
    let box_x = ((WIDTH - box_width) / 2.0).floor();
    draw_rectangle(box_x, 216.0, box_width, 32.0, NAVY);
    draw_rectangle_lines(box_x, 216.0, box_width, 32.0, 2.0, CYAN);
    let cursor = if blink() { "_" } else { "" };
    font.text(&format!("{value}{cursor}"), box_x + 8.0, 224.0, 2.0, INK);

    font.centered("Press ENTER to continue", WIDTH / 2.0, 272.0, 1.0, DIM);
}

fn profile_choice(font: &Font, company: &str) {
    font.centered("THE KILL CHAIN TRAIL", WIDTH / 2.0, 48.0, 2.0, GREEN);
    font.centered(
        &format!("What kind of company is {company}?"),
        WIDTH / 2.0,
        104.0,
        1.0,
        INK,
    );
    for (i, profile) in Profile::ALL.iter().enumerate() {
        let y = 144.0 + i as f32 * 72.0;
        font.text(
            &format!("{}) {profile}", i + 1),
            MARGIN + 48.0,
            y,
            2.0,
            AMBER,
        );
        font.text(
            &format!(
                "Budget {}   Valuation {}   Score x{}",
                money(profile.budget()),
                money(profile.valuation()),
                profile.multiplier()
            ),
            MARGIN + 80.0,
            y + 24.0,
            1.0,
            INK,
        );
        font.text(profile.blurb(), MARGIN + 80.0, y + 38.0, 1.0, DIM);
    }
    font.centered("Press 1-3", WIDTH / 2.0, 400.0, 1.0, DIM);
}

/// Price column text: the recruiter's fee plus weekly salary for analysts.
fn price_text(item: Item) -> String {
    match item.salary() {
        0 => money(item.price()),
        salary => format!("{} + {}/wk", money(item.price()), money(salary)),
    }
}

fn vendor_hall(font: &Font, shop: &Shop) {
    let game = &shop.game;
    font.text("THE VENDOR HALL", MARGIN, 8.0, 2.0, AMBER);
    right(font, &game.company, 12.0, CYAN);
    font.text(
        &format!(
            "Budget {}   Payroll {}/wk, {} to IPO",
            money(game.budget),
            money(game.payroll()),
            money(game.payroll() * game.paydays_left())
        ),
        MARGIN,
        36.0,
        1.0,
        INK,
    );
    divider(52.0);

    for (i, item) in Item::ALL.iter().enumerate() {
        let y = 64.0 + i as f32 * 16.0;
        let owned = game.owned.contains(item);
        let color = if owned {
            GREEN
        } else if game.can_buy(*item) {
            INK
        } else {
            DIM
        };
        let status = match item {
            _ if owned => "OWNED".to_string(),
            Item::Coffee => format!("{}/{COFFEE_CAPACITY} pots", game.coffee),
            _ => String::new(),
        };
        font.text(
            &format!("{:<28}{:<18}{status}", item.label(), price_text(*item)),
            MARGIN + 24.0,
            y,
            1.0,
            color,
        );
    }
    let exit_y = 64.0 + Item::ALL.len() as f32 * 16.0;
    let can_open = game.analysts() > 0;
    let exit = if game.day == 1 {
        "Open for business"
    } else {
        "Leave the Vendor Hall"
    };
    font.text(
        exit,
        MARGIN + 24.0,
        exit_y,
        1.0,
        if can_open { AMBER } else { DIM },
    );
    font.text(
        ">",
        MARGIN + 8.0,
        64.0 + shop.cursor as f32 * 16.0,
        1.0,
        AMBER,
    );
    divider(exit_y + 20.0);

    let note = match shop.item() {
        Some(item) => item.description(),
        None if can_open => "Start day 1. Leftover budget pays the weekly payroll.",
        None => "Hire at least one analyst first.",
    };
    font.text(note, MARGIN, exit_y + 32.0, 1.0, CYAN);

    let analysts: Vec<String> = game
        .team
        .iter()
        .filter(|m| m.role.is_analyst())
        .map(|m| format!("{} ({})", m.name, short_role(m.role)))
        .collect();
    let roster = format!("Analysts: {}", analysts.join(", "));
    for (i, line) in wrap(&roster, 76).iter().take(4).enumerate() {
        font.text(line, MARGIN, exit_y + 56.0 + i as f32 * 12.0, 1.0, INK);
    }
    font.text("UP/DOWN to choose, ENTER to buy.", MARGIN, 460.0, 1.0, DIM);
}

fn name_popup(font: &Font, prompt: &str, name: &str) {
    let (x, y, w, h) = (96.0, 160.0, WIDTH - 192.0, 112.0);
    draw_rectangle(x, y, w, h, NAVY);
    draw_rectangle_lines(x, y, w, h, 2.0, CYAN);
    font.centered(prompt, WIDTH / 2.0, y + 16.0, 1.0, INK);
    let cursor = if blink() { "_" } else { "" };
    font.text(&format!("{name}{cursor}"), x + 24.0, y + 44.0, 2.0, AMBER);
    font.centered(
        "ENTER to confirm, or ENTER with no name to cancel",
        WIDTH / 2.0,
        y + 84.0,
        1.0,
        DIM,
    );
}

fn action_menu(font: &Font, menu: &ActionMenu) {
    let game = &menu.game;
    timeline(font, game);
    font.text("TAKE AN ACTION", MARGIN, 72.0, 2.0, AMBER);
    right(font, &format!("Budget {}", money(game.budget)), 76.0, INK);
    font.text(
        &format!("{:<38}{:<8}COST", "ACTION", "DAYS"),
        MARGIN + 24.0,
        100.0,
        1.0,
        CYAN,
    );
    let actions = game.actions();
    let row_y = |i: usize| 116.0 + i as f32 * 16.0;
    for (i, action) in actions.iter().enumerate() {
        let cost = match action.cost() {
            0 => "-".to_string(),
            cost => money(cost),
        };
        font.text(
            &format!("{:<38}{:<8}{cost}", action.label(), game.duration(*action)),
            MARGIN + 24.0,
            row_y(i),
            1.0,
            INK,
        );
    }
    font.text("Back", MARGIN + 24.0, row_y(actions.len()), 1.0, AMBER);
    font.text(">", MARGIN + 8.0, row_y(menu.cursor), 1.0, AMBER);
    divider(384.0);
    let note = menu
        .action()
        .map_or("Return to the day menu.", |a| a.description());
    font.text(note, MARGIN, 396.0, 1.0, CYAN);
    font.text(
        "UP/DOWN to choose, ENTER to start. Days pass while the SOC works.",
        MARGIN,
        460.0,
        1.0,
        DIM,
    );
}

/// Role names that fit the narrow status panel.
fn short_role(role: Role) -> String {
    match role {
        Role::Junior => "Junior".to_string(),
        Role::Senior => "Senior".to_string(),
        role => role.to_string(),
    }
}

fn bar(x: f32, y: f32, width: f32, value: i32, color: Color) {
    draw_rectangle(x, y, width, 8.0, NAVY);
    draw_rectangle(x, y, width * value.clamp(0, 100) as f32 / 100.0, 8.0, color);
}

fn health_color(value: i32) -> Color {
    match value {
        50.. => GREEN,
        25.. => AMBER,
        _ => RED,
    }
}

/// Matches `condition`: Good is green, Fair amber, Poor and worse red.
fn burnout_color(value: i32) -> Color {
    match value {
        50.. => RED,
        25.. => AMBER,
        _ => GREEN,
    }
}

fn divider(y: f32) {
    draw_line(MARGIN, y, WIDTH - MARGIN, y, 1.0, DIM);
}

/// Text right-aligned to the screen margin.
fn right(font: &Font, text: &str, y: f32, color: Color) {
    font.text(
        text,
        WIDTH - MARGIN - text.len() as f32 * GLYPH,
        y,
        1.0,
        color,
    );
}

/// How a burnout level reads on the roster, Oregon Trail style.
fn condition(burnout: i32) -> &'static str {
    match burnout {
        75.. => "Very poor",
        50.. => "Poor",
        25.. => "Fair",
        _ => "Good",
    }
}

fn play(font: &Font, game: &GameState, traveling: bool) {
    timeline(font, game);
    status_panel(font, game);
    draw_line(LOG_X - 12.0, 64.0, LOG_X - 12.0, 384.0, 1.0, DIM);
    event_log(font, game);
    divider(384.0);
    match game.outcome {
        Some(outcome) => ending(font, game, outcome),
        None if traveling => {
            font.text("Days are passing...", MARGIN, 400.0, 1.0, INK);
            if blink() {
                font.text("Press any key to stop", MARGIN, 456.0, 1.0, AMBER);
            }
        }
        None if game.incident.is_some() => incident_popup(font, game),
        None if game.event.is_some() => event_menu(font, game),
        None if game.pending_alert().is_none() && game.stop.is_some() => landmark_popup(font, game),
        None if game.pending_alert().is_some() => alert_menu(font, game),
        None => day_menu(font, game),
    }
}

fn landmark_popup(font: &Font, game: &GameState) {
    let stop = game.stop.as_ref().expect("a landmark");
    let landmark = stop.landmark;
    let (x, y, w, h) = (40.0, 72.0, WIDTH - 80.0, 312.0);
    draw_rectangle(x, y, w, h, NAVY);
    draw_rectangle_lines(x, y, w, h, 2.0, CYAN);
    font.centered(
        &landmark.to_string().to_uppercase(),
        WIDTH / 2.0,
        y + 12.0,
        2.0,
        AMBER,
    );
    let width = ((w - 32.0) / GLYPH) as usize;
    let mut line_y = y + 40.0;
    for line in wrap(landmark.intro(), width) {
        font.text(&line, x + 16.0, line_y, 1.0, INK);
        line_y += 12.0;
    }
    line_y += 8.0;
    if landmark == Landmark::PenTest {
        for (area, grade) in game.report_card() {
            let color = match grade {
                'A' | 'B' => GREEN,
                'C' => AMBER,
                _ => RED,
            };
            font.text(&format!("{area:<12}"), x + 48.0, line_y, 1.0, INK);
            font.text(&grade.to_string(), x + 160.0, line_y, 1.0, color);
            line_y += 12.0;
        }
        line_y += 8.0;
    }
    let options: Vec<(String, bool)> = if landmark.is_fort() {
        vec![
            ("Visit the Vendor Hall".to_string(), true),
            ("Rest (no time passes)".to_string(), !stop.rested),
            ("Move on".to_string(), true),
        ]
    } else {
        landmark
            .crossings()
            .iter()
            .enumerate()
            .map(|(i, c)| (c.label.to_string(), game.can_cross(i)))
            .collect()
    };
    for (i, (label, available)) in options.iter().enumerate() {
        let color = if *available { AMBER } else { DIM };
        font.text(&format!("{}) {label}", i + 1), x + 16.0, line_y, 1.0, color);
        line_y += 16.0;
    }
    font.text(
        &format!("Press 1-{}. Grey choices are unavailable.", options.len()),
        x + 16.0,
        y + h - 20.0,
        1.0,
        DIM,
    );
}

fn event_menu(font: &Font, game: &GameState) {
    font.text("What will you do?", MARGIN, 390.0, 1.0, AMBER);
    let choices = game.event_choices();
    for (i, choice) in choices.iter().enumerate() {
        font.text(
            &format!("{}) {}", i + 1, choice.label),
            MARGIN + 16.0,
            402.0 + i as f32 * 11.0,
            1.0,
            INK,
        );
    }
    font.text(
        &format!("Press 1-{}. What happened is in the log.", choices.len()),
        MARGIN,
        462.0,
        1.0,
        DIM,
    );
}

fn alert_menu(font: &Font, game: &GameState) {
    font.text("ALERT! What will you do?", MARGIN, 390.0, 1.0, RED);
    let investigate = format!("Investigate ({} days)", game.investigation_days());
    let ir = format!("Call the IR firm ({})", money(IR_FEE));
    let options = [
        (investigate.as_str(), Reply::Investigate),
        (ir.as_str(), Reply::CallIr),
        ("Ignore it", Reply::Ignore),
    ];
    for (i, (label, reply)) in options.iter().enumerate() {
        let color = if game.can_reply(*reply) { INK } else { DIM };
        font.text(
            &format!("{}) {label}", i + 1),
            MARGIN + 16.0,
            402.0 + i as f32 * 11.0,
            1.0,
            color,
        );
    }
    font.text(
        "Press 1-3. The alert is in the log. It may be nothing.",
        MARGIN,
        462.0,
        1.0,
        DIM,
    );
}

fn incident_popup(font: &Font, game: &GameState) {
    let actor = game.incident.expect("an incident");
    let (x, y, w, h) = (40.0, 96.0, WIDTH - 80.0, 288.0);
    draw_rectangle(x, y, w, h, NAVY);
    draw_rectangle_lines(x, y, w, h, 2.0, RED);
    font.centered("INCIDENT", WIDTH / 2.0, y + 12.0, 2.0, RED);
    let width = ((w - 32.0) / GLYPH) as usize;
    let mut line_y = y + 40.0;
    for line in wrap(actor.incident(), width) {
        font.text(&line, x + 16.0, line_y, 1.0, INK);
        line_y += 12.0;
    }
    line_y += 12.0;
    for (i, response) in actor.responses().iter().enumerate() {
        let color = if game.can_respond(response) {
            AMBER
        } else {
            DIM
        };
        font.text(
            &format!("{}) {}", i + 1, response.label),
            x + 16.0,
            line_y,
            1.0,
            color,
        );
        line_y += 16.0;
    }
    let condition = actor.condition();
    font.text(
        &format!("Some choices leave {}:", condition.tag()),
        x + 16.0,
        y + h - 52.0,
        1.0,
        CYAN,
    );
    font.text(condition.effect(), x + 16.0, y + h - 40.0, 1.0, CYAN);
    font.text(
        &format!(
            "Press 1-{}. Grey choices are unavailable.",
            actor.responses().len()
        ),
        x + 16.0,
        y + h - 20.0,
        1.0,
        DIM,
    );
}

fn after_action(font: &Font, game: &GameState) {
    font.text("AFTER-ACTION REPORT", MARGIN, 12.0, 2.0, AMBER);
    font.text(
        "What the attackers were really doing:",
        MARGIN,
        36.0,
        1.0,
        INK,
    );
    let width = ((WIDTH - 2.0 * MARGIN) / GLYPH) as usize;
    for (i, line) in game.after_action().iter().take(38).enumerate() {
        let color = match line.as_str() {
            "ATTACKERS" | "ALERTS" => CYAN,
            l if l.contains(" REAL ") => RED,
            l if l.contains(" FALSE ") => DIM,
            _ => INK,
        };
        let line: String = line.chars().take(width).collect();
        font.text(&line, MARGIN, 56.0 + i as f32 * 10.0, 1.0, color);
    }
    if blink() {
        font.text(
            "Press ENTER to return to the title",
            MARGIN,
            460.0,
            1.0,
            AMBER,
        );
    }
}

/// The calendar bar from day 1 to IPO day, on every gameplay screen.
fn timeline(font: &Font, game: &GameState) {
    font.text(&game.company, MARGIN, 6.0, 1.0, CYAN);
    right(font, &format!("Lead: {}", game.lead), 6.0, CYAN);

    let (left, end, y) = (MARGIN + 4.0, WIDTH - MARGIN - 36.0, 30.0);
    let x_of = |day: u32| left + (end - left) * (day - 1) as f32 / (game.ipo_day - 1) as f32;
    draw_line(left, y, end, y, 2.0, DIM);
    for monday in (8..game.ipo_day).step_by(7) {
        draw_line(x_of(monday), y - 3.0, x_of(monday), y + 3.0, 1.0, DIM);
    }
    let now = x_of(game.day);
    draw_line(left, y, now, y, 2.0, GREEN);
    for (i, (landmark, &day)) in Landmark::ALL.iter().zip(&game.landmark_days).enumerate() {
        let color = if i < game.next_landmark { GREEN } else { CYAN };
        draw_rectangle(x_of(day) - 3.0, y - 3.0, 6.0, 6.0, color);
        font.centered(landmark.short(), x_of(day), y - 13.0, 1.0, color);
    }
    draw_rectangle(end - 4.0, y - 4.0, 8.0, 8.0, AMBER);
    font.text("IPO", end + 8.0, y - 4.0, 1.0, AMBER);
    draw_rectangle(
        now - 3.0,
        y - 7.0,
        6.0,
        14.0,
        if blink() { AMBER } else { INK },
    );

    font.text(
        &format!("DAY {} {}", game.day, WEEKDAYS[game.weekday() as usize]),
        MARGIN,
        44.0,
        1.0,
        INK,
    );
    right(
        font,
        &format!("{} DAYS TO IPO", game.days_to_ipo()),
        44.0,
        AMBER,
    );
    let tags: Vec<_> = game.conditions.iter().map(|c| c.tag()).collect();
    font.centered(&tags.join(" "), WIDTH / 2.0, 44.0, 1.0, RED);
    divider(60.0);
}

fn status_panel(font: &Font, game: &GameState) {
    let x = MARGIN;
    let value_x = x + 88.0;
    let row = |i: usize| 88.0 + i as f32 * 14.0;
    font.text("STATUS", x, 72.0, 1.0, CYAN);

    font.text("Valuation", x, row(0), 1.0, INK);
    font.text(&money(game.valuation), value_x, row(0), 1.0, INK);
    for (i, (label, value)) in [("Trust", game.trust), ("Brand", game.brand)]
        .into_iter()
        .enumerate()
    {
        font.text(label, x, row(i + 1), 1.0, INK);
        bar(value_x, row(i + 1), 96.0, value, health_color(value));
        font.text(&value.to_string(), value_x + 104.0, row(i + 1), 1.0, INK);
    }
    font.text("Budget", x, row(3), 1.0, INK);
    font.text(&money(game.budget), value_x, row(3), 1.0, INK);
    font.text("Coffee", x, row(4), 1.0, INK);
    let coffee_color = match game.coffee {
        0 => RED,
        1..10 => AMBER,
        _ => INK,
    };
    font.text(
        &format!("{}/{COFFEE_CAPACITY} pots", game.coffee),
        value_x,
        row(4),
        1.0,
        coffee_color,
    );
    font.text("Tempo", x, row(5), 1.0, INK);
    font.text(&game.tempo.to_string(), value_x, row(5), 1.0, INK);
    font.text("Task", x, row(6), 1.0, INK);
    let task = match &game.task {
        Some(task) => format!("{:.15} {}d", task.action.label(), task.days_left),
        None => "-".to_string(),
    };
    font.text(&task, value_x, row(6), 1.0, AMBER);
    font.text("Probe", x, row(7), 1.0, INK);
    let probe = match &game.investigation {
        Some(investigation) => format!("Digging {}d", investigation.days_left),
        None => "-".to_string(),
    };
    font.text(&probe, value_x, row(7), 1.0, AMBER);

    font.text("TEAM", x, 206.0, 1.0, CYAN);
    for (i, member) in game.team.iter().enumerate() {
        let y = 222.0 + i as f32 * 14.0;
        let name: String = member.name.chars().take(10).collect();
        font.text(
            &format!("{name:<10} {}", short_role(member.role)),
            x,
            y,
            1.0,
            INK,
        );
        font.text(
            condition(member.burnout),
            x + 160.0,
            y,
            1.0,
            burnout_color(member.burnout),
        );
    }
}

fn event_log(font: &Font, game: &GameState) {
    font.text("LOG", LOG_X, 72.0, 1.0, CYAN);
    let width = ((WIDTH - MARGIN - LOG_X) / GLYPH) as usize;
    let lines: Vec<String> = game
        .log
        .iter()
        .flat_map(|entry| wrap(entry, width))
        .collect();
    let start = lines.len().saturating_sub(LOG_LINES);
    for (i, line) in lines[start..].iter().enumerate() {
        font.text(line, LOG_X, 88.0 + i as f32 * 10.0, 1.0, INK);
    }
}

fn day_menu(font: &Font, game: &GameState) {
    font.text("What will you do?", MARGIN, 390.0, 1.0, AMBER);
    let resume = match &game.task {
        Some(task) => format!(
            "Continue ({}, {} days left)",
            task.action.label(),
            task.days_left
        ),
        None => "Continue".to_string(),
    };
    let tempo = format!("Set the tempo (now {})", game.tempo);
    let intern = format!(
        "Send the intern for coffee ({}, once a week)",
        money(COFFEE_RUN_COST)
    );
    let options = [
        (resume.as_str(), true),
        ("Take an action", !game.actions().is_empty()),
        ("Check the team", true),
        (tempo.as_str(), true),
        (intern.as_str(), game.can_send_intern()),
    ];
    // The coffee run is not offered while the break room is full.
    let shown = if game.coffee_full() { 4 } else { 5 };
    for (i, (label, available)) in options[..shown].iter().enumerate() {
        let color = if *available { INK } else { DIM };
        font.text(
            &format!("{}) {label}", i + 1),
            MARGIN + 16.0,
            402.0 + i as f32 * 11.0,
            1.0,
            color,
        );
    }
    font.text(
        &format!("Press 1-{shown}. Grey choices are unavailable."),
        MARGIN,
        462.0,
        1.0,
        DIM,
    );
}

fn ending(font: &Font, game: &GameState, outcome: Outcome) {
    let (message, color) = match outcome {
        Outcome::Ipo => (
            format!(
                "{} rang the opening bell at {}! Score: {}.",
                game.company,
                money(game.valuation),
                game.score().unwrap_or_default()
            ),
            GREEN,
        ),
        Outcome::Pulled => (
            format!(
                "The board pulled {}'s IPO. The bankers stopped returning calls.",
                game.company
            ),
            RED,
        ),
        Outcome::ShutDown => (
            format!(
                "Customers abandoned {}. The lights are off for good.",
                game.company
            ),
            RED,
        ),
        Outcome::Fired => (
            format!(
                "The board has lost faith. {}, you have been fired.",
                game.lead
            ),
            RED,
        ),
        Outcome::TeamCollapsed => (
            format!(
                "No analysts left: quit, sick, or worse. {} is defenseless.",
                game.company
            ),
            RED,
        ),
        Outcome::Bankrupt => (
            format!(
                "The SOC cannot make payroll. {} replaces it with a free antivirus trial.",
                game.company
            ),
            RED,
        ),
    };
    for (i, line) in wrap(&message, 38).iter().take(3).enumerate() {
        font.text(line, MARGIN, 394.0 + i as f32 * 20.0, 2.0, color);
    }
    if blink() {
        font.text("Press ENTER to play again", MARGIN, 460.0, 1.0, AMBER);
    }
}

fn team_screen(font: &Font, game: &GameState) {
    timeline(font, game);
    font.text("THE TEAM", MARGIN, 72.0, 2.0, AMBER);
    font.text(
        &format!("{:<21}{:<16}{:<10}BURNOUT", "NAME", "ROLE", "SALARY"),
        MARGIN,
        100.0,
        1.0,
        CYAN,
    );
    for (i, member) in game.team.iter().enumerate() {
        team_row(font, member, 116.0 + i as f32 * 16.0);
    }
    divider(384.0);
    font.text(
        &format!(
            "Weekly payroll: {}",
            money(game.team.iter().map(|m| m.salary).sum())
        ),
        MARGIN,
        396.0,
        1.0,
        INK,
    );
    if blink() {
        font.text("Press ENTER to return", MARGIN, 460.0, 1.0, AMBER);
    }
}

fn team_row(font: &Font, member: &TeamMember, y: f32) {
    let salary = if member.salary > 0 {
        format!("{}/wk", money(member.salary))
    } else {
        "-".to_string()
    };
    font.text(
        &format!(
            "{:<20} {:<15} {salary:<7}",
            member.name,
            member.role.to_string()
        ),
        MARGIN,
        y,
        1.0,
        INK,
    );
    let color = burnout_color(member.burnout);
    bar(MARGIN + 376.0, y, 120.0, member.burnout, color);
    font.text(condition(member.burnout), MARGIN + 504.0, y, 1.0, color);
}

/// Top of a street row on screen; row 0 is the office sidewalk at the bottom.
fn row_y(row: usize) -> f32 {
    344.0 - 48.0 * row as f32
}

fn coffee_run(font: &Font, run: &CoffeeRun) {
    let street = &run.street;
    let goal = match street.leg {
        Leg::ToShop => "Reach the door of JAVA THE HUT",
        Leg::ToOffice => "Coffee acquired! Get back to the office door",
    };
    font.text(&format!("COFFEE RUN: {goal}"), MARGIN, 8.0, 1.0, AMBER);

    // Coffee shop across the street.
    draw_rectangle(0.0, 24.0, WIDTH, 80.0, BROWN);
    font.text("JAVA THE HUT", 400.0, 36.0, 2.0, YELLOW);
    for x in [48.0, 176.0, 304.0, 560.0] {
        draw_rectangle(x, 56.0, 48.0, 32.0, CYAN);
    }
    door(street::SHOP_DOOR, 72.0, street.leg == Leg::ToShop);

    // Office building on this side.
    draw_rectangle(0.0, 392.0, WIDTH, 88.0, NAVY);
    font.text(
        &format!("{} SOC", run.game.company.to_uppercase()),
        240.0,
        440.0,
        2.0,
        INK,
    );
    door(street::OFFICE_DOOR, 392.0, street.leg == Leg::ToOffice);

    for row in [0, street::SHOP_ROW] {
        draw_rectangle(0.0, row_y(row), WIDTH, 48.0, DIM);
        for x in (0..WIDTH as usize).step_by(32) {
            draw_line(
                x as f32,
                row_y(row),
                x as f32,
                row_y(row) + 48.0,
                1.0,
                LIGHT,
            );
        }
    }
    draw_rectangle(
        0.0,
        row_y(street::LANES),
        WIDTH,
        48.0 * street::LANES as f32,
        BG,
    );
    for x in (0..WIDTH as usize).step_by(32) {
        for row in [2, 4] {
            draw_rectangle(x as f32, row_y(row) + 47.0, 16.0, 2.0, INK);
        }
    }
    draw_rectangle(0.0, row_y(3) + 45.0, WIDTH, 2.0, YELLOW);
    draw_rectangle(0.0, row_y(3) + 49.0, WIDTH, 2.0, YELLOW);

    let colors = [RED, CYAN, AMBER, GREEN];
    for (i, lane) in street.lanes.iter().enumerate() {
        for car in &lane.cars {
            vehicle(
                car.x,
                row_y(i + 1) + 8.0,
                car.length,
                lane.speed > 0.0,
                colors[i],
            );
        }
    }

    intern(street);

    if let Some(survived) = street.survived {
        let message = if survived {
            "The intern made it back with the coffee!"
        } else {
            "SPLAT! The intern did not make it."
        };
        draw_rectangle(80.0, 196.0, WIDTH - 160.0, 72.0, NAVY);
        draw_rectangle_lines(80.0, 196.0, WIDTH - 160.0, 72.0, 2.0, CYAN);
        font.centered(
            message,
            WIDTH / 2.0,
            212.0,
            1.0,
            if survived { GREEN } else { RED },
        );
        if blink() {
            font.centered("Press ENTER to continue", WIDTH / 2.0, 244.0, 1.0, AMBER);
        }
    }
}

/// A door whose frame blinks while it is the intern's goal.
fn door(x: f32, y: f32, goal: bool) {
    let frame = if goal && blink() { AMBER } else { LIGHT };
    draw_rectangle(x, y, street::DOOR_WIDTH, 32.0, frame);
    draw_rectangle(x + 4.0, y + 4.0, street::DOOR_WIDTH - 8.0, 28.0, BG);
}

/// A car or truck seen from above, facing the way it drives.
fn vehicle(x: f32, y: f32, length: f32, rightward: bool, color: Color) {
    let front = if rightward { x + length - 12.0 } else { x };
    for wheel_x in [x + 6.0, x + length - 14.0] {
        draw_rectangle(wheel_x, y - 2.0, 8.0, 36.0, DIM);
    }
    draw_rectangle(x, y, length, 32.0, color);
    if length > 50.0 {
        let cargo_x = if rightward { x } else { x + 16.0 };
        draw_rectangle(cargo_x, y + 2.0, length - 16.0, 28.0, LIGHT);
    }
    draw_rectangle(front + 2.0, y + 4.0, 8.0, 24.0, NAVY);
    let light_x = if rightward { x + length - 2.0 } else { x };
    for light_y in [y + 2.0, y + 26.0] {
        draw_rectangle(light_x, light_y, 2.0, 4.0, YELLOW);
    }
}

fn intern(street: &Street) {
    let (x, y) = (street.x, row_y(street.row) + 12.0);
    if street.survived == Some(false) {
        for (dx, dy, w, h) in [
            (-6.0, 8.0, 28.0, 8.0),
            (2.0, 0.0, 12.0, 24.0),
            (-2.0, 4.0, 20.0, 16.0),
        ] {
            draw_rectangle(x + dx, y + dy, w, h, RED);
        }
        return;
    }
    draw_rectangle(x + 4.0, y, 8.0, 8.0, PEACH);
    draw_rectangle(x + 2.0, y + 8.0, 12.0, 10.0, GREEN);
    draw_rectangle(x + 3.0, y + 18.0, 4.0, 6.0, NAVY);
    draw_rectangle(x + 9.0, y + 18.0, 4.0, 6.0, NAVY);
    if street.leg == Leg::ToOffice {
        draw_rectangle(x + 13.0, y + 9.0, 5.0, 7.0, INK);
        draw_rectangle(x + 13.0, y + 9.0, 5.0, 2.0, BROWN);
    }
}
