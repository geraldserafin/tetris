pub const WIDTH: usize = 10;
pub const HEIGTH: usize = 24;
use crate::puzzle::Puzzle;
use opengl_graphics::GlGraphics;
use piston::input::*;
use rand::Rng;
pub struct Board {
    pub board: [[u8; WIDTH]; HEIGTH + 2],
    pub current_puzzle: Puzzle,
    pub puzzles: [Puzzle; 7],
    pub colors: [([f32; 4], i32); 7],
}

impl Board {
    pub fn tick(&mut self) {
        let pos_y = self.current_puzzle.pos_y as usize;
        let pos_x = self.current_puzzle.pos_x;
        let width = self.current_puzzle.puzzle[0].len();
        let length = self.current_puzzle.puzzle.len();
        let down = self.current_puzzle.offset.2;
        let mut can_fall = true;

        if pos_y + down + 1 < HEIGTH {
            'outer: for i in 0..length {
                for j in 0..width {
                    let q = pos_x as i64 + j as i64;
                    if self.current_puzzle.puzzle[i][j] == 1
                        && self.board[pos_y + i + 1][q as usize] > 0
                    {
                        can_fall = false;
                        break 'outer;
                    }
                }
            }
            if can_fall {
                self.current_puzzle.pos_y += 1;
            } else {
                self.place_puzzle();
            }
        } else {
            self.place_puzzle();
        }
    }
    pub fn place_puzzle(&mut self) {
        let pos_y = self.current_puzzle.pos_y;
        let pos_x = self.current_puzzle.pos_x;

        for i in 0..self.current_puzzle.puzzle.len() {
            for j in 0..self.current_puzzle.puzzle[0].len() {
                if self.current_puzzle.puzzle[i][j] > 0 {
                    let q = pos_x + j as i64;
                    self.board[pos_y as usize + i][q as usize] = self.current_puzzle.color.1 as u8;
                }
            }
        }
        let mut rng = rand::thread_rng();
        self.current_puzzle = self.puzzles[rng.gen_range(0..self.puzzles.len())].clone();
        self.clear_rows();
    }
    fn clear_rows(&mut self) {
        for (index, row) in self.board.clone().into_iter().enumerate() {
            let mut clear = true;

            for point in row {
                if point == 0 {
                    clear = false;
                    break;
                }
            }

            if clear {
                self.board[index].fill(0);
                self.board[..=index].rotate_right(1);
            }
        }
    }
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        gl.draw(args.viewport(), |c, gl| {
            for (i, row) in self.board.iter().enumerate() {
                for (j, point) in row.iter().enumerate() {
                    if *point > 0 {
                        let square = graphics::rectangle::square(
                            (j as usize * 20) as f64 + 1.0,
                            (i as usize * 20) as f64 + 1.0,
                            18_f64,
                        );
                        let mut color = self.colors[*point as usize - 1];
                        color.0[3] = 0.45;
                        graphics::rectangle(color.0, square, c.transform, gl);
                    }
                }
            }
        });
    }
    pub fn pressed(&mut self, btn: &Button) {
        let pos_x = self.current_puzzle.pos_x;
        let pos_y = self.current_puzzle.pos_y;
        let left = self.current_puzzle.offset.0 as i64;
        let right = self.current_puzzle.offset.1 as i64;

        self.current_puzzle.pos_x = match btn {
            &Button::Keyboard(Key::Left) if pos_x + left > 0 => {
                let mut can_move = true;
                'outer: for i in 0..self.current_puzzle.puzzle.len() as i64 {
                    for j in 0..self.current_puzzle.puzzle[0].len() as i64 {
                        if self.current_puzzle.puzzle[i as usize][j as usize] > 0 {
                            if self.board[(pos_y + i) as usize][(pos_x + j - 1) as usize] > 0 {
                                can_move = false;
                                break 'outer;
                            }
                        }
                    }
                }
                if can_move {
                    self.current_puzzle.pos_x - 1
                } else {
                    self.current_puzzle.pos_x
                }
            }
            &Button::Keyboard(Key::Right) if pos_x + right < WIDTH as i64 - 1 => {
                let mut can_move = true;
                'outer1: for i in 0..self.current_puzzle.puzzle.len() {
                    for j in 0..self.current_puzzle.puzzle[0].len() {
                        if self.current_puzzle.puzzle[i][j] > 0 {
                            if self.board[pos_y as usize + i][(pos_x + j as i64 + 1) as usize] > 0 {
                                can_move = false;
                                break 'outer1;
                            }
                        }
                    }
                }
                if can_move {
                    self.current_puzzle.pos_x + 1
                } else {
                    self.current_puzzle.pos_x
                }
            }
            _ => self.current_puzzle.pos_x,
        };
        match btn {
            &Button::Keyboard(Key::Up) => {
                self.current_puzzle.rotate_c();
                self.current_puzzle.rotate_test(self.board.clone());
            }
            &Button::Keyboard(Key::Down) => {
                self.current_puzzle.rotate_cc();
                self.current_puzzle.rotate_test(self.board.clone());
            }
            _ => {}
        }
    }
}
