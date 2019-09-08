pub(crate) mod board;
pub(crate) mod color;
pub(crate) mod point;

use board::Board;

use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Clone, Copy)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub struct Segment {
    pub from: Coordinates,
    pub to: Coordinates,
}

pub struct Game {
    pub score: u32,
    pub board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            score: 0,
            board: Board::new(),
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        self.board.render(canvas);
    }

    pub fn handle_click(&mut self, x: i32, y: i32) {
        self.board.handle_click(x, y);
    }

    pub fn handle_move(&mut self, x: i32, y: i32) {
        self.board.handle_move(x, y);
    }

    pub fn handle_raise(&mut self) {
        self.board.handle_raise();
    }
}