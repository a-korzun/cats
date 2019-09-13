use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::rect::Rect;

use rand::Rng;

use crate::game::color::{PURPLE, BLUE, GREEN, RED, CYAN};
use crate::game::Coordinates;

const VELOCITY: i32 = 2;

#[derive(PartialEq, Copy, Clone)]
pub struct Point {
    pub color: Color,
    pub center: Coordinates,
    pub connected: bool,
    texture_offset: i32,
    transition_center: Coordinates,
    speed: i32,
}

impl Point {
    pub fn new(coordinates: Coordinates) -> Self {
        let (color, offset) = Point::random_color();

        Point {
            color,
            texture_offset: offset as i32,
            center: coordinates,
            transition_center: Coordinates { x: coordinates.x, y: coordinates.y - 400 },
            connected: false,
            speed: 0,
        }
    }

    pub fn update(&mut self) {
        let step = self.speed;
        let diff = self.center.y - self.transition_center.y;

        if diff < step {
            self.transition_center.y = self.center.y;
            self.speed = 0;
            return;
        }

        if diff > 0 {
            self.transition_center.y += step;
            self.speed += VELOCITY;
        }
    }

    pub fn render<'a>(&mut self, canvas: &mut Canvas<Window>, texture: &'a Texture) {
        let center = self.transition_center;
        let texture_size = 32;

        if self.connected {
            let src = Rect::new(
                self.texture_offset * texture_size,
                texture_size,
                texture_size as u32,
                texture_size as u32
            );
            let target = Rect::new(
                center.x - texture_size / 2,
                center.y - texture_size / 2,
                texture_size as u32,
                texture_size as u32
            );
            canvas.copy(&texture, src, target).unwrap();
        } else {
            let src = Rect::new(
                self.texture_offset * texture_size,
                0,
                texture_size as u32,
                texture_size as u32
            );
            let target = Rect::new(
                center.x - texture_size / 2,
                center.y - texture_size / 2,
                texture_size as u32,
                texture_size as u32
            );
            canvas.copy(&texture, src, target).unwrap();
        }
    }

    pub fn move_to(&mut self, coordinates: Coordinates) {
        if coordinates == self.center {
            return;
        }

        self.center.y = coordinates.y;
    }

    pub fn connect(&mut self) {
        self.connected = true;
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    fn random_color() -> (Color, usize) {
        let colors = [PURPLE, BLUE, GREEN, RED, CYAN];

        let mut rng = rand::thread_rng();

        let offset = rng.gen_range(0, 5);

        let color = colors[offset];

        (color, offset)
    }
}