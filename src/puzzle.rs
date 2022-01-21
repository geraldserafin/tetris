use crate::board::{HEIGTH, WIDTH};
use opengl_graphics::GlGraphics;
use piston::input::*;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
const RESOLUTION: u32 = 20;

pub struct Puzzle {
    pub puzzle: Vec<Vec<u8>>,
    pub pos_x: i64,
    pub pos_y: i64,
    pub color: [f32; 4],
    pub offset: (usize, usize, usize),
    pub tests: [[(i8, i8); 5]; 4],
    pub test_num: i32,
}

impl Puzzle {
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        gl.draw(args.viewport(), |c, gl| {
            let heigth = self.puzzle.len();
            let width = self.puzzle[0].len();
            for i in 0..heigth {
                for j in 0..width {
                    if self.puzzle[i][j] > 0 {
                        let square = graphics::rectangle::square(
                            ((self.pos_x + j as i64) * (RESOLUTION) as i64) as f64 + 1.0,
                            ((self.pos_y + i as i64) * (RESOLUTION) as i64) as f64 + 1.0,
                            18_f64,
                        );
                        graphics::rectangle(self.color, square, c.transform, gl);
                    }
                }
            }
        });
    }

    pub fn rotate_c(&mut self) {
        self.puzzle.reverse();
        self.puzzle = transpose(self.puzzle.clone());
        if self.test_num < 3 {
            self.test_num += 1;
        } else {
            self.test_num = 0;
        }
        self.get_offsets();
        self.get_kicks();
    }

    pub fn rotate_cc(&mut self) {
        self.puzzle = transpose(self.puzzle.clone());
        self.puzzle.reverse();
        if self.test_num > 0 {
            self.test_num -= 1;
        } else {
            self.test_num = 0;
        }
        self.get_offsets();
        self.get_kicks();
    }

    fn get_offsets(&mut self) {
        // left & right
        let mut left = 4;
        let mut right = 0;
        for (i, row) in transpose(self.puzzle.clone()).iter().enumerate() {
            let sum: u8 = row.iter().sum();
            if sum > 0 && i < left {
                left = i;
            }
            if sum > 0 && i > right {
                right = i;
            }
        }
        // down
        let mut down = 0;
        for (i, row) in self.puzzle.clone().iter().enumerate() {
            let sum: u8 = row.iter().sum();
            if sum > 0 && i > down {
                down = i;
            }
        }
        self.offset = (left, right, down);
    }

    fn get_kicks(&mut self) {
        // wall kicks
        if self.pos_x + self.offset.1 as i64 >= WIDTH as i64 {
            self.pos_x -= self.pos_x + self.offset.1 as i64 + 1 - WIDTH as i64;
        }
        if 0 > self.pos_x + self.offset.0 as i64 {
            self.pos_x -= self.pos_x + self.offset.0 as i64;
        }
        // floor kick
        if self.pos_y + self.offset.2 as i64 >= HEIGTH as i64 {
            self.pos_y -= self.pos_y + self.offset.2 as i64 + 1 - HEIGTH as i64;
        }
    }

    pub fn rotate_test(&mut self, board: [[u8; WIDTH]; HEIGTH + 2]) {
        let pos_x = self.pos_x;
        let pos_y = self.pos_y;
        let left = self.offset.0 as i64;
        let right = self.offset.1 as i64;

        for test in self.tests[self.test_num as usize] {
            let t1 = test.0 as i64;
            let t2 = test.1 as i64;

            let mut failed = false;
            'inner: for i in 0..self.puzzle.len() {
                for j in 0..self.puzzle[0].len() {
                    let x = pos_y + t1 + i as i64;
                    let y = pos_x + left + t2 + j as i64;

                    println!("x:{x} y:{y}");

                    if board[x as usize][y as usize] > 0 && self.puzzle[i][j] > 0 {
                        failed = true;
                        break 'inner;
                    }
                }
            }
            if !failed {
                self.pos_y += t1 as i64;
                self.pos_x += t2 as i64;
                break;
            }
        }
    }
}
