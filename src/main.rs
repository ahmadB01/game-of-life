use std::time::{Duration, Instant};

use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event::run;
use ggez::ContextBuilder;
use ggez::GameResult;
use ggez::{event::EventHandler, graphics};
use graphics::{DrawMode, DrawParam, Mesh, Rect};
use rand::{thread_rng, Rng};

const CELL_WIDTH: f32 = 20.0;
const CELL_HEIGHT: f32 = 20.0;

const GRID_COLS: usize = 40;
const GRID_LINES: usize = 30;

const WIDTH: f32 = CELL_WIDTH * GRID_COLS as f32;
const HEIGHT: f32 = CELL_HEIGHT * GRID_LINES as f32;

const INIT_PROB: f64 = 0.15;

// in ms
const UPDATE_TIME: u64 = 250;

fn main() -> GameResult {
    let (ctx, events_loop) = &mut ContextBuilder::new("game-of-life", "ahmad")
        .window_mode(w_mode())
        .window_setup(WindowSetup::default().title("Game of Life"))
        .build()?;

    let state = &mut Game::new();

    run(ctx, events_loop, state)
}

#[derive(Debug)]
struct Game {
    grid: Vec<bool>,
    next: Vec<bool>,
    last_update: Instant,
}

fn random_grid() -> Vec<bool> {
    let mut rng = thread_rng();
    let mut out = vec![false; GRID_LINES * GRID_COLS];
    for f in &mut out {
        *f = rng.gen_bool(INIT_PROB);
    }
    out
}

impl Game {
    fn new() -> Self {
        let r = random_grid();
        Self {
            grid: r.clone(),
            next: r,
            last_update: Instant::now(),
        }
    }
}

// ugly but stfu
fn neighbours(i: usize, j: usize, grid: &Vec<bool>) -> [Option<bool>; 8] {
    [
        if i == 0 || j == 0 {
            None
        } else {
            grid.get((i - 1) * GRID_COLS + j - 1).cloned()
        },
        if i == 0 {
            None
        } else {
            grid.get((i - 1) * GRID_COLS + j).cloned()
        },
        if i == 0 {
            None
        } else {
            grid.get((i - 1) * GRID_COLS + j + 1).cloned()
        },
        if j == 0 {
            None
        } else {
            grid.get(i * GRID_COLS + j - 1).cloned()
        },
        grid.get(i * GRID_COLS + j + 1).cloned(),
        if j == 0 {
            None
        } else {
            grid.get((i + 1) * GRID_COLS + j - 1).cloned()
        },
        grid.get((i + 1) * GRID_COLS + j).cloned(),
        grid.get((i + 1) * GRID_COLS + j + 1).cloned(),
    ]
}

fn update(grid: &mut Vec<bool>, next: &mut Vec<bool>) {
    for i in 0..GRID_LINES {
        for j in 0..GRID_COLS {
            let neigh = neighbours(i, j, grid)
                .iter()
                .map(|c| c.unwrap_or(false))
                .filter(|c| *c)
                .count();
            if !grid[i * GRID_COLS + j] && neigh == 3 {
                next[i * GRID_COLS + j] = true;
            } else if grid[i * GRID_COLS + j] && (neigh < 2 || neigh > 3) {
                next[i * GRID_COLS + j] = false;
            }
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> std::result::Result<(), ggez::GameError> {
        if Instant::now() - self.last_update >= Duration::from_millis(UPDATE_TIME) {
            update(&mut self.grid, &mut self.next);
            self.grid = self.next.clone();
            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> std::result::Result<(), ggez::GameError> {
        graphics::clear(ctx, [0.05, 0.05, 0.05, 1.0].into());

        for i in 0..GRID_LINES {
            for j in 0..GRID_COLS {
                if self.grid[i * GRID_COLS as usize + j] {
                    let rect = Rect::new(
                        j as f32 * CELL_WIDTH,
                        i as f32 * CELL_HEIGHT,
                        CELL_WIDTH,
                        CELL_HEIGHT,
                    );
                    let rect = Mesh::new_rectangle(
                        ctx,
                        DrawMode::fill(),
                        rect,
                        [0.8, 0.8, 0.8, 1.0].into(),
                    )?;
                    graphics::draw(ctx, &rect, DrawParam::default())?;
                }
            }
        }

        graphics::present(ctx)
    }
}

fn w_mode() -> WindowMode {
    WindowMode {
        borderless: true,
        width: WIDTH,
        height: HEIGHT,
        ..Default::default()
    }
}
