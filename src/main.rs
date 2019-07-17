extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod snake;
use crate::snake::snake::SnakeGame;

const WIN_SIZE: u32 = 640;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Snake", WIN_SIZE, WIN_SIZE)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(21, 21, 21));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut snake_game = SnakeGame::new();
    let mut snake_alive: bool;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        let keys = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        snake_alive = snake_game.tick(&keys);
        if !snake_alive {
            println!("Snake died :(");
            break 'running;
        }
        canvas.set_draw_color(Color::RGB(21, 21, 21));
        canvas.clear();
        snake_game.draw(&mut canvas)?;
        canvas.present();

        ::std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
