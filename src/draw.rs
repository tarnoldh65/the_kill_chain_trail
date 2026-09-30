use font8x8::legacy::BASIC_LEGACY;
use macroquad::prelude::*;

use crate::art::{self, Sprite};
use crate::audio::TRACKS;
use crate::game::{COFFEE_CAPACITY, COFFEE_RUN_COST, GameState, Outcome, Scene, Setback, Stage};
use crate::street::{self, Leg, Street};
use crate::ui::{CoffeeRun, Music, Report, Screen, wrap};

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
const LOG_LINES: usize = 14;

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
        Screen::Play(game) => play(font, game),
        Screen::Report(report) => {
            play(font, &report.game);
            report_popup(font, report);
        }
        Screen::Coffee(run) => coffee_run(font, run),
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

fn burnout_color(value: i32) -> Color {
    match value {
        75.. => RED,
        50.. => AMBER,
        _ => GREEN,
    }
}

fn divider(y: f32) {
    draw_line(MARGIN, y, WIDTH - MARGIN, y, 1.0, DIM);
}

fn play(font: &Font, game: &GameState) {
    header(font, game);
    trail(font, game.stage);
    divider(88.0);
    company_panel(font, game);
    team_panel(font, game);
    divider(204.0);
    event_log(font, game);
    divider(372.0);
    match game.outcome {
        None => choices(font, game),
        Some(outcome) => ending(font, game, outcome),
    }
}

fn header(font: &Font, game: &GameState) {
    font.text(
        &game.stage.to_string().to_uppercase(),
        MARGIN,
        8.0,
        2.0,
        AMBER,
    );
    let lead = format!("Lead: {}", game.lead);
    for (i, line) in [game.company.as_str(), lead.as_str()].iter().enumerate() {
        let x = WIDTH - MARGIN - line.len() as f32 * GLYPH;
        font.text(line, x, 6.0 + i as f32 * 10.0, 1.0, CYAN);
    }
}

fn trail(font: &Font, current: Stage) {
    let node_x = |i: usize| 50.0 + i as f32 * 90.0;
    let y = 48.0;
    let current = current as usize;
    for (i, stage) in Stage::ALL.iter().enumerate() {
        let color = match i.cmp(&current) {
            std::cmp::Ordering::Less => GREEN,
            std::cmp::Ordering::Equal => AMBER,
            std::cmp::Ordering::Greater => DIM,
        };
        if i > 0 {
            let line_color = if i <= current { GREEN } else { DIM };
            draw_line(node_x(i - 1) + 6.0, y, node_x(i) - 6.0, y, 2.0, line_color);
        }
        draw_rectangle(node_x(i) - 6.0, y - 6.0, 12.0, 12.0, color);
        font.centered(trail_label(*stage), node_x(i), y + 14.0, 1.0, color);
    }
    if blink() {
        font.centered("v", node_x(current), y - 18.0, 1.0, AMBER);
    }
}

/// Short stage names that fit between trail nodes.
fn trail_label(stage: Stage) -> &'static str {
    match stage {
        Stage::Reconnaissance => "RECON",
        Stage::Weaponization => "WEAPONIZE",
        Stage::Delivery => "DELIVERY",
        Stage::Exploitation => "EXPLOIT",
        Stage::Installation => "INSTALL",
        Stage::CommandAndControl => "C2",
        Stage::ActionsOnObjectives => "ACTIONS",
    }
}

fn company_panel(font: &Font, game: &GameState) {
    let x = MARGIN;
    let bar_x = x + 96.0;
    let row = |i: usize| 112.0 + i as f32 * 14.0;
    font.text("COMPANY", x, 96.0, 1.0, CYAN);

    font.text("Budget", x, row(0), 1.0, INK);
    font.text(&format!("${}", game.budget), bar_x, row(0), 1.0, INK);
    font.text("Coffee", x, row(1), 1.0, INK);
    let coffee_color = if game.coffee < game.team.len() as i32 {
        RED
    } else {
        INK
    };
    font.text(
        &format!("{}/{COFFEE_CAPACITY} pots", game.coffee),
        bar_x,
        row(1),
        1.0,
        coffee_color,
    );

    for (i, (label, value)) in [("Trust", game.trust), ("Brand", game.brand)]
        .into_iter()
        .enumerate()
    {
        font.text(label, x, row(i + 2), 1.0, INK);
        bar(bar_x, row(i + 2), 160.0, value, health_color(value));
        font.text(&value.to_string(), bar_x + 168.0, row(i + 2), 1.0, INK);
    }

    let target = game.stage.target();
    font.text("Contain", x, row(4), 1.0, INK);
    bar(bar_x, row(4), 160.0, game.containment, CYAN);
    draw_rectangle(
        bar_x + 160.0 * target as f32 / 100.0,
        row(4) - 2.0,
        2.0,
        12.0,
        INK,
    );
    font.text(
        &format!("{}/{}", game.containment, target),
        bar_x + 168.0,
        row(4),
        1.0,
        INK,
    );
}

fn team_panel(font: &Font, game: &GameState) {
    let x = 344.0;
    font.text("TEAM              BURNOUT", x, 96.0, 1.0, CYAN);
    for (i, member) in game.team.iter().enumerate() {
        let y = 112.0 + i as f32 * 14.0;
        font.text(
            &format!("{:<6} {}", member.name, member.role),
            x,
            y,
            1.0,
            INK,
        );
        bar(
            x + 144.0,
            y,
            120.0,
            member.burnout,
            burnout_color(member.burnout),
        );
    }
}

fn event_log(font: &Font, game: &GameState) {
    font.text("LOG", MARGIN, 212.0, 1.0, CYAN);
    let width = ((WIDTH - 2.0 * MARGIN) / GLYPH) as usize;
    let lines: Vec<String> = game
        .log
        .iter()
        .flat_map(|entry| wrap(entry, width))
        .collect();
    let start = lines.len().saturating_sub(LOG_LINES);
    for (i, line) in lines[start..].iter().enumerate() {
        font.text(line, MARGIN, 228.0 + i as f32 * 10.0, 1.0, INK);
    }
}

fn choices(font: &Font, game: &GameState) {
    font.text("What will you do?", MARGIN, 382.0, 1.0, AMBER);
    // The coffee run is not offered while the break room is full.
    let intern = (!game.coffee_full()).then_some((
        "Send the intern for coffee (no turn)",
        COFFEE_RUN_COST,
        game.can_send_intern(),
    ));
    let options: Vec<_> = game
        .stage
        .choices()
        .map(|c| (c.label, c.cost, game.can_afford(&c)))
        .into_iter()
        .chain(intern)
        .collect();
    for (i, (label, cost, available)) in options.iter().enumerate() {
        let color = if *available { INK } else { DIM };
        let line = format!("{}) {label:<40} ${cost}", i + 1);
        font.text(&line, MARGIN + 16.0, 396.0 + i as f32 * 14.0, 1.0, color);
    }
    font.text(
        &format!("Press 1-{}. Grey choices are unavailable.", options.len()),
        MARGIN,
        456.0,
        1.0,
        DIM,
    );
}

fn ending(font: &Font, game: &GameState, outcome: Outcome) {
    let (message, color) = match outcome {
        Outcome::Contained => (
            format!(
                "{} contained the attack. The board sends thanks and more coffee.",
                game.company
            ),
            GREEN,
        ),
        Outcome::Breached => (
            format!(
                "The attackers escaped with {}'s data. Expect headlines.",
                game.company
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
                "{} is out of money. The SOC is replaced by a free antivirus trial.",
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
    };
    for (i, line) in wrap(&message, 38).iter().enumerate() {
        font.text(line, MARGIN, 384.0 + i as f32 * 20.0, 2.0, color);
    }
    if blink() {
        font.text("Press ENTER to play again", MARGIN, 456.0, 1.0, AMBER);
    }
}

fn scene_sprite(scene: Scene) -> &'static Sprite {
    match scene {
        Scene::Hunt => &art::HUNT,
        Scene::Spend => &art::SPEND,
        Scene::Sleep => &art::SLEEP,
        Scene::Patch => &art::PATCH,
        Scene::Phish => &art::PHISH,
        Scene::Coffee => &art::COFFEE,
        Scene::Block => &art::BLOCK,
        Scene::Isolate => &art::ISOLATE,
        Scene::Unplug => &art::UNPLUG,
        Scene::Press => &art::PRESS,
    }
}

fn setback_sprite(setback: Setback) -> &'static Sprite {
    match setback {
        Setback::Behind => &art::SKULL,
        Setback::Fired => &art::BOX,
        Setback::Departed => &art::TOMBSTONE,
    }
}

/// Draws a sprite on a black panel centered at `center_x`, one block per pixel.
fn sprite_panel(sprite: &Sprite, center_x: f32, y: f32) {
    let scale = 7.0;
    let width = art::WIDTH as f32 * scale;
    let x = (center_x - width / 2.0).floor();
    draw_rectangle(x - 8.0, y - 8.0, width + 16.0, 16.0 * scale + 16.0, BG);
    draw_rectangle_lines(
        x - 8.0,
        y - 8.0,
        width + 16.0,
        16.0 * scale + 16.0,
        2.0,
        DIM,
    );
    for (row, pixels) in sprite.iter().enumerate() {
        for (col, pixel) in pixels.chars().enumerate() {
            let color = match pixel {
                'k' => BG,
                'n' => NAVY,
                'd' => DIM,
                'l' => LIGHT,
                'w' => INK,
                'g' => GREEN,
                'G' => DARK_GREEN,
                'c' => CYAN,
                'a' => AMBER,
                'r' => RED,
                'b' => BROWN,
                'y' => YELLOW,
                _ => continue,
            };
            draw_rectangle(
                x + col as f32 * scale,
                y + row as f32 * scale,
                scale,
                scale,
                color,
            );
        }
    }
}

fn report_popup(font: &Font, report: &Report) {
    let (x, y, w, h) = (40.0, 24.0, WIDTH - 80.0, HEIGHT - 48.0);
    draw_rectangle(x, y, w, h, NAVY);
    draw_rectangle_lines(x, y, w, h, 2.0, CYAN);
    font.centered("INCIDENT REPORT", WIDTH / 2.0, y + 12.0, 2.0, AMBER);
    font.centered(report.choice.label, WIDTH / 2.0, y + 36.0, 1.0, CYAN);

    let sprite_y = y + 64.0;
    let activity = scene_sprite(report.choice.scene);
    match report.setback {
        Some(setback) => {
            sprite_panel(activity, WIDTH / 2.0 - 112.0, sprite_y);
            font.centered("->", WIDTH / 2.0, sprite_y + 48.0, 2.0, AMBER);
            sprite_panel(setback_sprite(setback), WIDTH / 2.0 + 112.0, sprite_y);
        }
        None => sprite_panel(activity, WIDTH / 2.0, sprite_y),
    }

    let width = ((w - 32.0) / GLYPH) as usize;
    let lines = wrap(report.choice.text, width)
        .into_iter()
        .map(|line| (line, INK))
        .chain(
            report.game.log[report.news..]
                .iter()
                .flat_map(|entry| wrap(entry, width))
                .map(|line| (line, RED)),
        );
    for (i, (line, color)) in lines.take(15).enumerate() {
        font.text(&line, x + 16.0, 216.0 + i as f32 * 12.0, 1.0, color);
    }
    if blink() {
        font.centered(
            "Press ENTER to continue",
            WIDTH / 2.0,
            y + h - 20.0,
            1.0,
            AMBER,
        );
    }
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
