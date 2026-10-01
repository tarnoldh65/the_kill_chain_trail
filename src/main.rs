mod attack;
mod audio;
mod draw;
mod events;
mod game;
mod landmarks;
mod street;
mod ui;

use audio::Audio;
use draw::{HEIGHT, WIDTH};
use macroquad::prelude::*;
use street::Hop;
use ui::{Cue, Input, Music, Screen};

fn conf() -> Conf {
    Conf {
        window_title: "The Kill Chain Trail".to_string(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: true,
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

/// Plays the sound for a screen change and swaps music when entering or leaving the street.
fn transition(audio: &Audio, music: Music, before: &Screen, after: &Screen) {
    if let Some(cue) = ui::cue(before, after) {
        audio.play(cue);
    }
    let in_traffic = |screen: &Screen| matches!(screen, Screen::Coffee(_));
    match (in_traffic(before), in_traffic(after)) {
        (false, true) => audio.play_rush_hour(music),
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
    let audio = Audio::load().await;
    let mut music = Music::Track(0);
    audio.play_music(music);
    let mut screen = Screen::Title;

    loop {
        for input in inputs() {
            if let Some(choice) = ui::music_choice(&screen, input) {
                music = choice;
                audio.play_music(music);
                audio.play(Cue::Select);
            }
            let next = screen.clone().update(input);
            transition(&audio, music, &screen, &next);
            screen = next;
        }
        // Cap the step so a slow frame cannot carry a car straight through the intern.
        let next = screen.clone().tick(get_frame_time().min(0.05));
        transition(&audio, music, &screen, &next);
        screen = next;

        set_camera(&camera);
        draw::screen(&screen, &font, music);

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
    }
}
