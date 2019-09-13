pub(crate) mod board;
pub(crate) mod color;
pub(crate) mod point;

use sdl2::render::{Canvas, Texture};
use sdl2::video::{Window, WindowContext};
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;

use std::path::Path;
use std::rc::Rc;

use board::Board;
use crate::{TILE_SIZE, FIELD_OFFSET_LEFT, FIELD_OFFSET_TOP};

const SCORE_LIMIT: i32 = 9999;

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Clone, Copy)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

pub struct Game {
    score: i32,
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0,
            board: Board::new(),
        }
    }

    pub fn update(&mut self) {
        self.board.update();
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        self.board.render(canvas);
        self.render_score(canvas)
    }

    pub fn handle_click(&mut self, x: i32, y: i32) {
        self.board.handle_click(x, y);
    }

    pub fn handle_move(&mut self, x: i32, y: i32) {
        self.board.handle_move(x, y);
    }

    pub fn handle_raise(&mut self) {
        let amount = self.board.handle_raise();
        self.update_score(amount);
    }

    fn update_score(&mut self, amount: i32) {
        if self.score == SCORE_LIMIT {
            return;
        }

        if self.score + amount >= SCORE_LIMIT {
            self.score = SCORE_LIMIT;
        }

        self.score += amount;
    }

    fn render_score(&mut self, canvas: &mut Canvas<Window>) {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new("assets/numbers.png")).unwrap();

        self.score.to_string().as_str().chars().enumerate().for_each(|(index, symbol)| {
            let offset = symbol.to_digit(10).unwrap() as i32;
            let src = Rect::new(16 * offset, 0, 16, 16);
            let target = Rect::new(16 * index as i32 + FIELD_OFFSET_LEFT, 20, 16, 16);
            canvas.copy(&texture, src, target).unwrap();
        });
    }
}