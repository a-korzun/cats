extern crate sdl2;
extern crate rand;

pub(crate) mod game;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::BlendMode;
use sdl2::image::InitFlag;

use game::Game;
use game::color::BACKGROUND;

pub const FIELD_OFFSET_LEFT: i32 = 50;
pub const FIELD_OFFSET_TOP: i32 = 100;
pub const TILE_SIZE: i32 = 60;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

    let window = video_subsystem.window("cats", 400, 420)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .accelerated()
        .build()
        .unwrap();

    canvas.set_blend_mode(BlendMode::Blend);
    canvas.set_draw_color(BACKGROUND);
    canvas.clear();
    canvas.present();

    let mut game = Game::new();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        game.handle_click(x, y);
                    }
                },
                Event::MouseMotion {x, y, mousestate, ..} => {
                    if mousestate.is_mouse_button_pressed(MouseButton::Left) {
                        game.handle_move(x, y);
                    }
                },
                Event::MouseButtonUp { .. } => {
                    game.handle_raise();
                },
                _ => ()
            }
        }

        canvas.set_draw_color(BACKGROUND);
        canvas.clear();

        game.update();
        game.render(&mut canvas);

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}
