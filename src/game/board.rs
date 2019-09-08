use std::collections::LinkedList;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::{ Point as sdlPoint };

use crate::{TILE_SIZE, FIELD_OFFSET_LEFT, FIELD_OFFSET_TOP};
use crate::game::point::Point;
use crate::game::{Coordinates, Segment};

enum Action {
    Connect,
    Disconnect,
    None,
}

#[derive(Debug, PartialEq)]
pub struct Board {
    field: Vec<Vec<Option<Point>>>,
    segments: LinkedList<Segment>,
    closed_path: bool,
    current_color: Option<Color>,
}

impl Board {
    pub fn new() -> Self {
        let mut field = Vec::with_capacity(5);

        for x in 0..5 {
            let mut column: Vec<Option<Point>> = Vec::with_capacity(5);
            for y in 0..5 {
                let coords = Board::index_to_coordinates(x, y);
                column.push(Some(Point::new(coords)));
            }

            field.push(column)
        }

        Board {
            field,
            segments: LinkedList::new(),
            closed_path: false,
            current_color: None,
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        if let Some(color) = self.current_color {
            canvas.set_draw_color(color);
        } else {
            canvas.set_draw_color(Color::RGB(90, 90, 90));
        }

        self.segments.iter().for_each(|segment| {
            let from = sdlPoint::new(segment.from.x, segment.from.y);
            let to = sdlPoint::new(segment.to.x, segment.to.y);
            canvas.draw_line(from, to).unwrap();
        });

        self.field.iter_mut().flatten().for_each(|point| {
            if let Some(point) = point {
                point.render(canvas);
            }
        });
    }

    pub fn handle_click(&mut self, x: i32, y: i32) {
        if let Some(point) = self.get_point_mut(x, y) {
            point.connect();

            let center = point.center;
            let color = point.color;

            self.current_color = Some(color);

            self.segments.push_front(Segment {
                from: center,
                to: Coordinates { x, y },
            });

        }
    }

    pub fn handle_move(&mut self, x: i32, y: i32) {
        if self.closed_path {
            let mut segment = self.segments.front_mut().unwrap();
            segment.to = segment.from;
        }

        if let Some(segment) = self.segments.front_mut() {
            if !self.closed_path {
                segment.to.x = x;
                segment.to.y = y;
            }
        }

        let point = self.get_point(x, y);

        if None == point {
            return;
        }

        let point = point.unwrap();
        let center = point.center;

        let segment = self.segments.front().unwrap();
        let prev_point = self.get_point(segment.from.x, segment.from.y).unwrap();
        let prev_point_center = prev_point.center;

        let action = self.resolve_action(prev_point, point);

        match action {
            Action::Connect => self.connect(center),
            Action::Disconnect => self.disconnect(prev_point_center),
            Action::None => (),
        };
    }

    pub fn handle_raise(&mut self) {
        if self.segments.len() < 2 {
            self.segments.clear();

            self.field.iter_mut().flatten().for_each(|point| {
                if let Some(point) = point {
                    point.disconnect();
                }
            });

            return;
        }

        self.segments.clear();

        let count = self.clear_points();

        self.closed_path = false;
        self.current_color = None;
    }

    fn clear_points(&mut self) -> i32 {
        let mut count = 0;

        self.field = self.field.iter_mut().enumerate().map(|(x, column)| {
            let mut new_column: Vec<Option<Point>> = vec![None; 5];

            let points = column.iter_mut().filter(|point| !point.unwrap().connected);

            points.rev().enumerate()
                .for_each(|(y, point)| {
                    let mut point = point.unwrap();
                    let coords = Board::index_to_coordinates(x,  4 - y);
                    point.move_to(coords);
                    new_column[4 - y] = Some(point);
                });

            for i in 0..5 {
                if let None = new_column[i] {
                    let coords = Board::index_to_coordinates(x,  i);
                    new_column[i] = Some(Point::new(coords));
                    count += 1;
                }
            }

            new_column
        }).collect();

        count
    }

    fn get_point(&self, x: i32, y: i32) -> Option<&Point> {
        if let Some((x, y)) = Board::coordinates_to_index(Coordinates { x, y }) {
            let column = self.field.get(x).unwrap();
            if let Some(point) = column.get(y).unwrap() {
                return Some(point);
            }
        }

        None
    }

    fn get_point_mut(&mut self, x: i32, y: i32) -> Option<&mut Point> {
        if let Some((x, y)) = Board::coordinates_to_index(Coordinates { x, y }) {
            let column = self.field.get_mut(x).unwrap();
            if let Some(point) = column.get_mut(y).unwrap() {
                return Some(point);
            }
        }

        None
    }

    fn resolve_action(&self, prev: &Point, next: &Point) -> Action {
        if prev.center == next.center {
            return Action::None;
        }

        if prev.color != next.color {
            return Action::None;
        }

        if Board::is_siblings(prev, next) && !self.is_previous(next) {
            return Action::Connect;
        }

        if Board::is_siblings(prev, next) && self.is_previous(next) {
            return Action::Disconnect;
        }

        Action::None
    }

    fn connect(&mut self, next: Coordinates) {
        if self.closed_path {
            return;
        }

        let segment = self.segments.front_mut().unwrap();
        segment.to.x = next.x;
        segment.to.y = next.y;

        self.segments.push_front(Segment {
            from: next,
            to: next,
        });

        let point = self.get_point(next.x, next.y).unwrap();

        if point.connected {
            let color = point.color;
            self.connect_all(color);

            self.closed_path = true;
        }

        let point = self.get_point_mut(next.x, next.y).unwrap();

        point.connect();
    }

    fn disconnect(&mut self, next: Coordinates) {
        if self.segments.len() < 2 {
            return;
        }

        let from;
        let to;

        let front = self.segments.pop_front();
        to = front.unwrap().to;
        let front = self.segments.pop_front();
        from = front.unwrap().from;

        self.segments.push_front(Segment {
            from,
            to,
        });


        if self.closed_path {
            let color = self.get_point(next.x, next.y).unwrap().color;
            self.disconnect_all(color);

            self.closed_path = false;
        } else {
            let point = self.get_point_mut(next.x, next.y).unwrap();
            point.disconnect();
        }
    }

    fn connect_all(&mut self, color: Color) {
        self.field.iter_mut().flatten().for_each(|point| {
            if let Some(point) = point {
                if point.color == color {
                    point.connect();
                }
            }
        });
    }

    fn disconnect_all(&mut self, color: Color) {
        let connected_by_segments: Vec<Point> = self.field.iter().flatten()
            .filter(|point| {
                self.segments.iter().any(|segment| segment.from == point.unwrap().center)
            })
            .map(|p| {
                p.unwrap()
            })
            .collect();

        self.field.iter_mut().flatten().for_each(|point| {
            if let Some(point) = point {
                if point.color == color && !connected_by_segments.contains(point) {
                    point.disconnect();
                }
            }
        });
    }

    fn is_previous(&self, point: &Point) -> bool {
        let second_index;
        if self.segments.len() < 2 {
            second_index = 0
        } else {
            second_index = 1;
        }

        let from = self.segments.iter().nth(second_index).unwrap();

        if from.from == point.center {
            return true;
        }

        false
    }

    fn is_siblings(p1: &Point, p2: &Point) -> bool {
        let (x1, y1) = Board::coordinates_to_index(p1.center).unwrap();
        let (x2, y2) = Board::coordinates_to_index(p2.center).unwrap();

        if x1 == x2 && (y1 as i32 - y2 as i32).abs() == 1 {
            return true;
        }

        if y1 == y2 && (x1 as i32 - x2 as i32).abs() == 1 {
            return true;
        }

        false
    }

    fn index_to_coordinates(x: usize, y: usize) -> Coordinates {
        let x: i32 = x as i32 * TILE_SIZE + TILE_SIZE / 2 + FIELD_OFFSET_LEFT;
        let y: i32 = y as i32 * TILE_SIZE + TILE_SIZE / 2 + FIELD_OFFSET_TOP;

        Coordinates { x, y }
    }

    fn coordinates_to_index(coordinates: Coordinates) -> Option<(usize, usize)> {
        let x = ((coordinates.x - FIELD_OFFSET_LEFT) as f32 / TILE_SIZE as f32).floor() as i32;
        let y = ((coordinates.y - FIELD_OFFSET_TOP) as f32 / TILE_SIZE as f32).floor() as i32;

        if x < 0 || y < 0 || x > 4 || y > 4 {
            return None;
        }

        Some((x as usize, y as usize))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn index_to_coordinates() {
        let p1 = Board::index_to_coordinates(0, 0);
        assert_eq!(p1, Coordinates { x: 80, y: 130 });

        let p2 = Board::index_to_coordinates(2, 3);
        assert_eq!(p2, Coordinates { x: 200, y: 310 });

        let p2 = Board::index_to_coordinates(2, 2);
        assert_eq!(p2, Coordinates { x: 200, y: 250 });

        let p2 = Board::index_to_coordinates(4, 4);
        assert_eq!(p2, Coordinates { x: 320, y: 370 });
    }

    #[test]
    fn coordinates_to_index() {
        let p1 = Board::coordinates_to_index(Coordinates { x: 80, y: 130 }).unwrap();
        assert_eq!(p1 , (0, 0));

        let c2 = prepare_coordinates(3, 4, 10);
        let p2 = Board::coordinates_to_index(c2).unwrap();
        assert_eq!(p2, (3, 4));

        let c3 = prepare_coordinates(4, 3, 10);
        let p3 = Board::coordinates_to_index(c3).unwrap();
        assert_eq!(p3, (4, 3));

        let c4 = prepare_coordinates(0, 0, 10);
        let p4 = Board::coordinates_to_index(c4).unwrap();
        assert_eq!(p4, (0, 0));

        let c5 = prepare_coordinates(7, 3, 10);
        let p5 = Board::coordinates_to_index(c5);
        assert_eq!(p5, None);

        let c6 = prepare_coordinates(2, 3, 10);
        let p6 = Board::coordinates_to_index(c6);
        assert_eq!(p6, None);
    }

    fn prepare_coordinates(x: usize, y: usize, error: i32) -> Coordinates {
        let c = Board::index_to_coordinates(x, y);
        Coordinates {
            x: c.x + error,
            y: c.y + error,
        }
    }
}
