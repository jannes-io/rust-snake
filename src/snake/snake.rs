extern crate sdl2;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;

pub const FIELD_SIZE: i8 = 20;

pub struct Snake {
    pub body: Vec<(i8, i8)>,
    pub length: usize,
    pub velocity: (i8, i8),
    pub alive: bool,
}

impl Snake {
    pub fn new() -> Self {
        let mut body: Vec<(i8, i8)> = vec![];
        let mut x: i8 = FIELD_SIZE / 2;
        let y: i8 = x;

        body.push((x, y));
        x += 1;
        body.push((x, y));
        x += 1;
        body.push((x, y));

        Snake {
            body,
            length: 3,
            velocity: (1, 0),
            alive: true,
        }
    }

    pub fn tick(&mut self, keys: &HashSet<Keycode>) -> () {
        if keys.contains(&Keycode::Up) && self.velocity != (0, 1) {
            self.velocity = (0, -1)
        }
        if keys.contains(&Keycode::Down) && self.velocity != (0, -1) {
            self.velocity = (0, 1)
        }
        if keys.contains(&Keycode::Left) && self.velocity != (1, 0) {
            self.velocity = (-1, 0)
        }
        if keys.contains(&Keycode::Right) && self.velocity != (-1, 0) {
            self.velocity = (1, 0)
        }

        if self.body.len() >= self.length {
            self.body = self.body[1..].to_vec();
        }

        let new_pos = self.body.last();
        match new_pos {
            Some((mut x, mut y)) => {
                x += self.velocity.0;
                y += self.velocity.1;

                self.alive = !(x < 0 || x >= FIELD_SIZE || y < 0 || y >= FIELD_SIZE);
                for body_pos in &self.body {
                    if *body_pos == (x, y) {
                        self.alive = false;
                    }
                }
                if self.alive {
                    self.body.push((x, y));
                }
            }
            None => {}
        }
    }
}