use glutin_window::GlutinWindow;
use opengl_graphics::GlGraphics;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use rand::Rng;

const RESOLUTION: u32 = 20;

mod board;
mod puzzle;

fn main() {
    let colors = [
        ([0.0, 1.0, 1.0, 1.0], 1),
        ([1.0, 1.0, 0.0, 1.0], 2),
        ([0.5, 0.0, 0.5, 1.0], 3),
        ([0.0, 1.0, 0.0, 1.0], 4),
        ([1.0, 0.0, 0.0, 1.0], 5),
        ([1.0, 0.5, 0.0, 1.0], 6),
        ([0.5, 0.5, 0.5, 1.0], 7),
    ];
    let puzzles = [
        puzzle::Puzzle {
            pos_x: 3,
            pos_y: 0,
            puzzle: vec![
                vec![0, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
            color: colors[0],
            offset: (0, 3, 1),
            tests: [
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                [(0, 0), (-1, 0), (1, -1), (0, 2), (-1, 2)],
            ],
            test_num: 0,
        },
        puzzle::Puzzle {
            pos_x: 3,
            pos_y: 0,
            puzzle: vec![vec![1, 0, 0], vec![1, 1, 1], vec![0, 0, 0]],
            color: colors[1],
            offset: (0, 3, 1),
            tests: [
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                [(0, 0), (-1, 0), (1, -1), (0, 2), (-1, 2)],
            ],
            test_num: 0,
        },
        puzzle::Puzzle {
            pos_x: 3,
            pos_y: 0,
            puzzle: vec![vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]],
            color: colors[2],
            offset: (0, 3, 1),
            tests: [
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                [(0, 0), (-1, 0), (1, -1), (0, 2), (-1, 2)],
            ],
            test_num: 0,
        },
        puzzle::Puzzle {
            pos_x: 3,
            pos_y: 0,
            puzzle: vec![vec![1, 1], vec![1, 1]],
            color: colors[3],
            offset: (0, 1, 0),
            tests: [
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                [(0, 0), (-1, 0), (1, -1), (0, 2), (-1, 2)],
            ],
            test_num: 0,
        },
        puzzle::Puzzle {
            pos_x: 3,
            pos_y: 0,
            puzzle: vec![vec![0, 1, 1], vec![1, 1, 0], vec![0, 0, 0]],
            color: colors[4],
            offset: (0, 3, 1),
            tests: [
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                [(0, 0), (-1, 0), (1, -1), (0, 2), (-1, 2)],
            ],
            test_num: 0,
        },
        puzzle::Puzzle {
            pos_x: 3,
            pos_y: 0,
            puzzle: vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 0, 0]],
            color: colors[5],
            offset: (0, 3, 1),
            tests: [
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                [(0, 0), (-1, 0), (1, -1), (0, 2), (-1, 2)],
            ],
            test_num: 0,
        },
        puzzle::Puzzle {
            pos_x: 3,
            pos_y: 0,
            puzzle: vec![vec![1, 1, 0], vec![0, 1, 1], vec![0, 0, 0]],
            color: colors[6],
            offset: (0, 3, 1),
            tests: [
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                [(0, 0), (-1, 0), (1, -1), (0, 2), (-1, 2)],
            ],
            test_num: 0,
        },
    ];

    let mut rng = rand::thread_rng();
    let mut board = board::Board {
        board: [[0; board::WIDTH]; board::HEIGTH + 2],
        current_puzzle: puzzles[rng.gen_range(0..puzzles.len())].clone(),
        puzzles: puzzles,
        colors: colors,
    };
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new(
        "Tetris",
        [
            RESOLUTION * board::WIDTH as u32,
            RESOLUTION * board::HEIGTH as u32,
        ],
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();
    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new()).ups(6);
    while let Some(e) = events.next(&mut window) {
        // rendering graphics
        if let Some(r) = e.render_args() {
            let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

            // draw black background
            gl.draw(r.viewport(), |_c, gl| {
                graphics::clear(black, gl);
            });

            // draw puzzles
            board.current_puzzle.render(&mut gl, &r);
            board.render(&mut gl, &r);
        }

        // game ticks
        if let Some(_u) = e.update_args() {
            board.tick();
        }

        // handling inputs
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                board.pressed(&k.button);
            }
        }
    }
}
