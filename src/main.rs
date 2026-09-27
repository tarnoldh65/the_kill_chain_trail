mod audio;
mod draw;
mod game;
mod ui;

use draw::{HEIGHT, WIDTH};
use macroquad::prelude::*;
use ui::{Input, Screen};

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
    inputs
}

#[macroquad::main(conf)]
async fn main() {
    let font = draw::Font::load();
    let target = render_target(WIDTH as u32, HEIGHT as u32);
    target.texture.set_filter(FilterMode::Nearest);
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, WIDTH, HEIGHT));
    camera.render_target = Some(target.clone());
    let audio = audio::Audio::load().await;
    audio.start_music();
    let mut screen = Screen::Title;

    loop {
        for input in inputs() {
            let next = screen.clone().update(input);
            if let Some(cue) = ui::cue(&screen, &next) {
                audio.play(cue);
            }
            screen = next;
        }

        set_camera(&camera);
        draw::screen(&screen, &font);

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
