use font8x8::legacy::BASIC_LEGACY;
use macroquad::prelude::*;

use crate::game::{GameState, Outcome, Stage};
use crate::ui::{Screen, wrap};

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

pub fn screen(screen: &Screen, font: &Font) {
    clear_background(BG);
    match screen {
        Screen::Title => title(font),
        Screen::Company(company) => {
            name_entry(font, "Name the company you are protecting:", company)
        }
        Screen::Lead { company, lead } => name_entry(
            font,
            &format!("{} needs an incident lead. Your name:", company.trim()),
            lead,
        ),
        Screen::Play(game) => play(font, game),
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

fn title(font: &Font) {
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
    if blink() {
        font.centered("PRESS ENTER TO BEGIN", WIDTH / 2.0, 384.0, 2.0, AMBER);
    }
    binary_band(font, 456.0);
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
        &format!("{} pots", game.coffee),
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
    for (i, choice) in game.stage.choices().iter().enumerate() {
        let color = if game.can_afford(choice) { INK } else { DIM };
        let line = format!("{}) {:<40} ${}", i + 1, choice.label, choice.cost);
        font.text(&line, MARGIN + 16.0, 400.0 + i as f32 * 16.0, 1.0, color);
    }
    font.text(
        "Press 1-3. Grey choices are over budget.",
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
