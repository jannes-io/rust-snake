extern crate rand;
extern crate sdl2;
use rand::Rng;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashSet;

use crate::snake::snake::*;

#[derive(PartialEq)]
pub enum TileType {
    Empty,
    Snake,
    Apple,
}

pub struct SnakeGame {
    pub snake: Snake,
    pub apple_pos: (i8, i8),
    pub field: Vec<TileType>,
}

impl SnakeGame {
    pub fn new() -> Self {
        let mut field: Vec<TileType> = vec![];
        let mut rng = rand::thread_rng();
        let tile_count: u16 = (FIELD_SIZE as u8).into();

        for _ in 0..(tile_count * tile_count) {
            field.push(TileType::Empty);
        }

        SnakeGame {
            snake: Snake::new(),
            apple_pos: (rng.gen_range(0, FIELD_SIZE), rng.gen_range(0, FIELD_SIZE)),
            field,
        }
    }

    pub fn tick(&mut self, keys: &HashSet<Keycode>) -> bool {
        self.snake.tick(keys);
        if !self.snake.alive {
            return false;
        }

        if *self.snake.body.last().unwrap() == self.apple_pos {
            self.snake.length += 1;
            let mut rng = rand::thread_rng();
            self.apple_pos = (rng.gen_range(0, FIELD_SIZE), rng.gen_range(0, FIELD_SIZE));
        }

        for y in 0..FIELD_SIZE {
            for x in 0..FIELD_SIZE {
                let idx: usize = (y as u16 * FIELD_SIZE as u16 + x as u16) as usize;
                self.field[idx as usize] = TileType::Empty;
                if (x as i8, y as i8) == self.apple_pos {
                    self.field[idx as usize] = TileType::Apple
                } else {
                    for pos in &self.snake.body {
                        if *pos == (x as i8, y as i8) {
                            self.field[idx as usize] = TileType::Snake
                        }
                    }
                }
            }
        }
        true
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for y in 0..FIELD_SIZE {
            for x in 0..FIELD_SIZE {
                let idx: usize = (y as u16 * FIELD_SIZE as u16 + x as u16) as usize;
                let tile_type = &self.field[idx];
                match *tile_type {
                    TileType::Apple => canvas.set_draw_color(Color::RGB(255, 0, 0)),
                    TileType::Snake => canvas.set_draw_color(Color::RGB(0, 255, 255)),
                    _ => continue,
                }
                canvas.fill_rect(Rect::new(x as i32 * 32, y as i32 * 32, 32, 32))?
            }
        }
        Ok(())
    }
}
