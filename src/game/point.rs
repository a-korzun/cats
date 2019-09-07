use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{ Point as sdlPoint };

use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::game::color::{BLUE, RED, PURPLE, GREEN, ORANGE};
use crate::game::Coordinates;

static RADIUS: i32 = 10;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub color: Color,
    pub center: Coordinates,
    pub connected: bool,
}

impl Point {
    pub fn new(Coordinates{ x, y }: Coordinates) -> Point {
        Point {
            color: Point::random_color(),
            center: Coordinates { x, y },
            connected: false,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);

        Point::draw_circle(canvas, self.center.x, self.center.y, RADIUS);

        if self.connected {
            Point::fill_circle(canvas, self.center.x, self.center.y, RADIUS);
        }
    }

    pub fn connect(&mut self) {
        self.connected = true;
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    fn random_color() -> Color {
        let colors = [BLUE, RED, PURPLE, GREEN, ORANGE];

        let mut rng = thread_rng();

        let (r, g, b) = *colors.choose(&mut rng).unwrap();
        // let (r, g, b) = colors[0];

        Color::RGB(r, g, b)
    }

    fn draw_circle(canvas: &mut Canvas<Window>, cx: i32, cy: i32, radius: i32) {
        let mut x = radius;
        let mut y = 0;

        let mut error = -radius;

        while x >= y {
            canvas.draw_point(sdlPoint::new(cx + x, cy + y)).unwrap();
            canvas.draw_point(sdlPoint::new(cx + y, cy + x)).unwrap();

            if x != 0 {
                canvas.draw_point(sdlPoint::new(cx - x, cy + y)).unwrap();
                canvas.draw_point(sdlPoint::new(cx + y, cy - x)).unwrap();
            }

            if y != 0 {
                canvas.draw_point(sdlPoint::new(cx + x, cy - y)).unwrap();
                canvas.draw_point(sdlPoint::new(cx - y, cy + x)).unwrap();
            }

            if x != 0 && y != 0 {
                canvas.draw_point(sdlPoint::new(cx - x, cy - y)).unwrap();
                canvas.draw_point(sdlPoint::new(cx - y, cy - x)).unwrap();
            }

            error += y;
            y += 1;
            error += y;

            if error >= 0 {
                x -= 1;
                error -= x;
                error -= x;
            }
        }
    }

    fn fill_circle(canvas: &mut Canvas<Window>, cx: i32, cy: i32, radius: i32) {
        (1..=radius).for_each(|dy| {
            let dx: i32 = ((2.0 * radius as f32 * dy as f32) - (dy as f32 * dy as f32)).sqrt().floor() as i32;

            canvas.draw_line(
                sdlPoint::new(cx - dx, cy + dy - radius),
                sdlPoint::new(cx + dx, cy + dy - radius)
            ).unwrap();
            canvas.draw_line(
                sdlPoint::new(cx - dx, cy - dy + radius),
                sdlPoint::new(cx + dx, cy - dy + radius)
            ).unwrap();
        });
    }
}
