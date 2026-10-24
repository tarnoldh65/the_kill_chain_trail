mod art;
mod attack;
mod audio;
#[cfg(test)]
mod balance;
mod casino;
mod conference;
mod draw;
mod events;
mod fight;
mod game;
mod landmarks;
mod manual;
mod operations;
mod options;
mod scores;
mod storage;
mod street;
mod ui;

use audio::Audio;
use draw::{HEIGHT, WIDTH};
use macroquad::prelude::*;
use std::time::{Duration, Instant};
use street::Hop;
use ui::{Cue, Input, Music, Screen};

const FRAME: Duration = Duration::from_micros(16_667);

fn conf() -> Conf {
    Conf {
        window_title: "The Kill Chain Trail".to_string(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: true,
        platform: miniquad::conf::Platform {
            swap_interval: Some(1),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn inputs() -> Vec<Input> {
    let mut inputs: Vec<Input> = std::iter::from_fn(get_char_pressed)
        .map(Input::Char)
        .collect();
    if is_key_pressed(KeyCode::Backspace) {
        inputs.push(Input::Backspace);
    }
    if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
        inputs.push(Input::Enter);
    }
    for (key, hop) in [
        (KeyCode::Up, Hop::Up),
        (KeyCode::Down, Hop::Down),
        (KeyCode::Left, Hop::Left),
        (KeyCode::Right, Hop::Right),
    ] {
        if is_key_pressed(key) {
            inputs.push(Input::Arrow(hop));
        }
    }
    inputs
}

/// Plays the sounds for a screen change, swaps music when entering or leaving the street or
/// a fight, and records the score when a game ends.
fn transition(
    audio: &Audio,
    music: Music,
    scores: &mut Vec<scores::Score>,
    before: &Screen,
    after: &Screen,
) {
    for cue in ui::cues(before, after) {
        audio.play(cue);
    }
    if let (Some(old), Some(new)) = (before.game(), after.game())
        && old.outcome.is_none()
        && new.outcome.is_some()
        && new.score().is_some()
    {
        scores::record(scores, new);
        scores::save(scores);
    }
    let in_traffic = |screen: &Screen| matches!(screen, Screen::Coffee(_));
    let in_fight = |screen: &Screen| matches!(screen, Screen::Fight(_));
    match (in_traffic(before), in_traffic(after)) {
        (false, true) => audio.play_rush_hour(music),
        (true, false) => audio.play_music(music),
        _ => {}
    }
    match (in_fight(before), in_fight(after)) {
        (false, true) => audio.play_fight(music),
        (true, false) => audio.play_music(music),
        _ => {}
    }
}

#[macroquad::main(conf)]
async fn main() {
    let font = draw::Font::load();
    let target = render_target(WIDTH as u32, HEIGHT as u32);
    target.texture.set_filter(FilterMode::Nearest);
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, WIDTH, HEIGHT));
    camera.render_target = Some(target.clone());
    macroquad::rand::srand(miniquad::date::now() as u64);
    let mut audio = Audio::load().await;
    let mut options = options::load();
    audio.set_volumes(options);
    audio.play_music(options.music);
    let mut screen = Screen::Title;
    let mut scores = scores::load();
    let mut last_frame = Instant::now();

    loop {
        for input in inputs() {
            if let Some(changed) = ui::options_change(&screen, input, options) {
                if changed.music != options.music {
                    audio.play_music(changed.music);
                }
                options = changed;
                audio.set_volumes(options);
                options::save(options);
                audio.play(Cue::Select);
            }
            let next = screen.clone().update(input);
            transition(&audio, options.music, &mut scores, &screen, &next);
            screen = next;
        }
        // Cap the step so a slow frame cannot carry a car straight through the intern.
        let held = if is_key_down(KeyCode::Left) {
            Some(Hop::Left)
        } else if is_key_down(KeyCode::Right) {
            Some(Hop::Right)
        } else {
            None
        };
        let next = screen.clone().tick(get_frame_time().min(0.05), held);
        transition(&audio, options.music, &mut scores, &screen, &next);
        screen = next;

        set_camera(&camera);
        draw::screen(&screen, &font, options, &scores);

        // Scale the 640x480 frame to the window in whole steps when it fits, letterboxed.
        set_default_camera();
        clear_background(BLACK);
        let fit = (screen_width() / WIDTH).min(screen_height() / HEIGHT);
        let scale = if fit >= 1.0 { fit.floor() } else { fit };
        draw_texture_ex(
            &target.texture,
            ((screen_width() - WIDTH * scale) / 2.0).floor(),
            ((screen_height() - HEIGHT * scale) / 2.0).floor(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(WIDTH * scale, HEIGHT * scale)),
                flip_y: true,
                ..Default::default()
            },
        );

        next_frame().await;
        // Hold to 60 fps when the driver ignores vsync, so the loop never spins a core flat out.
        std::thread::sleep(FRAME.saturating_sub(last_frame.elapsed()));
        last_frame = Instant::now();
    }
}
