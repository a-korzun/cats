use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{ Point as sdlPoint, Rect };

use rand::Rng;
use rand::seq::SliceRandom;

use crate::game::color::{BLUE, RED, PURPLE, GREEN, ORANGE};
use crate::game::color::{DARK_BLUE, DARK_RED, DARK_PURPLE, DARK_GREEN, DARK_ORANGE};
use crate::game::Coordinates;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub color: Color,
    dark_color: Color,
    pub center: Coordinates,
    pub connected: bool,
    prev_center: Coordinates,
}

impl Point {
    pub fn new(coordinates: Coordinates) -> Point {
        let (dark_color, color) = Point::random_color();

        Point {
            color,
            dark_color,
            center: coordinates,
            prev_center: coordinates,
            connected: false,
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        let center;
        if self.prev_center != self.center {
            self.prev_center.y += 20;
            center = self.prev_center;
        } else {
            center = self.center;
        }

        if self.connected {
            canvas.set_draw_color(self.color);
            Point::fill_cat(canvas, center.x, center.y);

            canvas.set_draw_color(self.dark_color);
            Point::draw_cat(canvas, center.x, center.y);
            Point::draw_cat_face(canvas, center.x, center.y);

        } else {
            canvas.set_draw_color(self.color);
            Point::draw_cat(canvas, center.x, center.y);
            Point::draw_cat_face(canvas, center.x, center.y);
            Point::fill_cat_transparent(canvas, center.x, center.y);
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

    fn random_color() -> (Color, Color) {
        let dark_colors = [DARK_BLUE, DARK_RED, DARK_PURPLE, DARK_GREEN, DARK_ORANGE];
        let colors = [BLUE, RED, PURPLE, GREEN, ORANGE];

        let mut rng = rand::thread_rng();

        let index = rng.gen_range(0, 4);

        let dc = dark_colors[index];
        let c = colors[index];

        (Color::RGB(dc.0, dc.1, dc.2), Color::RGB(c.0, c.1, c.2))
    }

    fn draw_cat(canvas: &mut Canvas<Window>, cx: i32, cy: i32) {
        let points = [
            // countour
            sdlPoint::new(cx + 0, cy - 6),
            sdlPoint::new(cx + 1, cy - 6),
            sdlPoint::new(cx + 2, cy - 7),
            sdlPoint::new(cx + 3, cy - 8),
            sdlPoint::new(cx + 4, cy - 9),
            sdlPoint::new(cx + 5, cy - 10),
            sdlPoint::new(cx + 6, cy - 10),
            sdlPoint::new(cx + 7, cy - 9),
            sdlPoint::new(cx + 8, cy - 8),
            sdlPoint::new(cx + 8, cy - 7),
            sdlPoint::new(cx + 9, cy - 6),
            sdlPoint::new(cx + 9, cy - 5),
            sdlPoint::new(cx + 10, cy - 4),
            sdlPoint::new(cx + 10, cy - 3),
            sdlPoint::new(cx + 10, cy - 2),
            sdlPoint::new(cx + 11, cy - 1),
            sdlPoint::new(cx + 11, cy),
            sdlPoint::new(cx + 11, cy + 1),
            sdlPoint::new(cx + 11, cy + 2),
            sdlPoint::new(cx + 11, cy + 3),
            sdlPoint::new(cx + 11, cy + 4),
            sdlPoint::new(cx + 11, cy + 5),
            sdlPoint::new(cx + 11, cy + 6),
            sdlPoint::new(cx + 11, cy + 7),
            sdlPoint::new(cx + 10, cy + 8),
            sdlPoint::new(cx + 9, cy + 9),
            sdlPoint::new(cx + 8, cy + 10),
            sdlPoint::new(cx + 7, cy + 10),
            sdlPoint::new(cx + 6, cy + 11),
            sdlPoint::new(cx + 5, cy + 11),
            sdlPoint::new(cx + 4, cy + 11),
            sdlPoint::new(cx + 3, cy + 11),
            sdlPoint::new(cx + 2, cy + 11),
            sdlPoint::new(cx + 1, cy + 11),
            sdlPoint::new(cx + 0, cy + 11),
            sdlPoint::new(cx - 1, cy + 11),
            sdlPoint::new(cx - 2, cy + 11),
            sdlPoint::new(cx - 3, cy + 11),
            sdlPoint::new(cx - 4, cy + 11),
            sdlPoint::new(cx - 5, cy + 11),
            sdlPoint::new(cx - 6, cy + 11),
            sdlPoint::new(cx - 7, cy + 10),
            sdlPoint::new(cx - 8, cy + 10),
            sdlPoint::new(cx - 9, cy + 9),
            sdlPoint::new(cx - 10, cy + 8),
            sdlPoint::new(cx - 11, cy + 7),
            sdlPoint::new(cx - 11, cy + 6),
            sdlPoint::new(cx - 11, cy + 5),
            sdlPoint::new(cx - 11, cy + 4),
            sdlPoint::new(cx - 11, cy + 3),
            sdlPoint::new(cx - 11, cy + 2),
            sdlPoint::new(cx - 11, cy + 1),
            sdlPoint::new(cx - 11, cy),
            sdlPoint::new(cx - 11, cy - 1),
            sdlPoint::new(cx - 10, cy - 2),
            sdlPoint::new(cx - 10, cy - 3),
            sdlPoint::new(cx - 10, cy - 4),
            sdlPoint::new(cx - 9, cy - 5),
            sdlPoint::new(cx - 9, cy - 6),
            sdlPoint::new(cx - 8, cy - 7),
            sdlPoint::new(cx - 8, cy - 8),
            sdlPoint::new(cx - 7, cy - 9),
            sdlPoint::new(cx - 6, cy - 10),
            sdlPoint::new(cx - 5, cy - 10),
            sdlPoint::new(cx - 4, cy - 9),
            sdlPoint::new(cx - 3, cy - 8),
            sdlPoint::new(cx - 2, cy - 7),
            sdlPoint::new(cx - 1, cy - 6),
        ];

        for point in points.iter() {
            canvas.draw_point(*point).unwrap();
        }
    }

    fn draw_cat_face(canvas: &mut Canvas<Window>, cx: i32, cy: i32) {
        let points = [
            //left eye
            sdlPoint::new(cx - 6, cy - 1),
            sdlPoint::new(cx - 5, cy - 1),
            sdlPoint::new(cx - 7, cy),
            sdlPoint::new(cx - 5, cy),
            sdlPoint::new(cx - 4, cy),
            sdlPoint::new(cx - 7, cy + 1),
            sdlPoint::new(cx - 6, cy + 1),
            sdlPoint::new(cx - 5, cy + 1),
            sdlPoint::new(cx - 4, cy + 1),
            sdlPoint::new(cx - 6, cy + 2),
            sdlPoint::new(cx - 5, cy + 2),
            //right eye
            sdlPoint::new(cx + 5, cy - 1),
            sdlPoint::new(cx + 6, cy - 1),
            sdlPoint::new(cx + 4, cy),
            sdlPoint::new(cx + 6, cy),
            sdlPoint::new(cx + 7, cy),
            sdlPoint::new(cx + 4, cy + 1),
            sdlPoint::new(cx + 5, cy + 1),
            sdlPoint::new(cx + 6, cy + 1),
            sdlPoint::new(cx + 7, cy + 1),
            sdlPoint::new(cx + 5, cy + 2),
            sdlPoint::new(cx + 6, cy + 2),
            //mouth
            sdlPoint::new(cx, cy + 4),
            sdlPoint::new(cx, cy + 6),
            sdlPoint::new(cx - 2, cy + 7),
            sdlPoint::new(cx - 1, cy + 7),
            sdlPoint::new(cx + 1, cy + 7),
            sdlPoint::new(cx + 2, cy + 7),
        ];

        for point in points.iter() {
            canvas.draw_point(*point).unwrap();
        }
    }

    fn fill_cat_transparent(canvas: &mut Canvas<Window>, cx: i32, cy: i32) {
        let color = canvas.draw_color();
        let alpha_color = Color::RGBA(color.r, color.g, color.b, 20);
        canvas.set_draw_color(alpha_color);

        Point::fill_cat(canvas, cx, cy);
    }

    fn fill_cat(canvas: &mut Canvas<Window>, cx: i32, cy: i32) {
        canvas.fill_rect(Rect::new(cx - 8, cy - 6, 17, 17)).unwrap();

        canvas.fill_rect(Rect::new(cx - 10, cy - 4, 2, 13)).unwrap();
        canvas.fill_rect(Rect::new(cx + 9, cy - 4, 2, 13)).unwrap();

        canvas.fill_rect(Rect::new(cx - 7, cy - 9, 4, 3)).unwrap();
        canvas.fill_rect(Rect::new(cx + 4, cy - 9, 4, 3)).unwrap();

        canvas.draw_point(sdlPoint::new(cx - 3, cy - 7)).unwrap();
        canvas.draw_point(sdlPoint::new(cx + 3, cy - 7)).unwrap();
    }
}