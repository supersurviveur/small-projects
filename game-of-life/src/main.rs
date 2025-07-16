use rand::Rng;
use std::fmt::Write;
use std::thread::sleep;
use std::time::Duration;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(PartialEq)]
pub enum GameMode {
    Dead,
    Alive,
    Portal,
}

#[derive(Clone)]
pub struct Grid {
    grid: Vec<bool>,

    height: usize,
    width: usize,
}
impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            grid: vec![false; height * width],
        }
    }
    pub fn random(height: usize, width: usize) -> Self {
        let mut grid = Self::new(height, width);
        let mut rng = rand::rng();
        for y in 0..height {
            for x in 0..width {
                if rng.random_bool(0.2) {
                    grid[(x, y)] = true;
                }
            }
        }
        grid
    }
}

pub struct GameOfLife {
    height: usize,
    width: usize,
    grid: Grid,
    mode: GameMode,
}

impl GameOfLife {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            grid: Grid::new(height, width),
            mode: GameMode::Portal,
        }
    }
    pub fn random(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            grid: Grid::random(height, width),
            mode: GameMode::Portal,
        }
    }
    pub fn step(&mut self) {
        let copied = self.grid.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let nb = self.count_neighbours(&copied, x, y);
                self.grid[(x, y)] = if self.grid[(x, y)] {
                    nb >= 2 && nb <= 3
                } else {
                    nb == 3
                }
            }
        }
    }

    pub fn steps(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step()
        }
    }

    fn count_neighbours(&self, grid: &Grid, x: usize, y: usize) -> u8 {
        let mut nb = 0;

        for i in 0..=2 {
            for j in 0..=2 {
                if i == 1 && j == 1 {
                    continue;
                }
                nb += match (
                    i == 0 && x == 0,
                    j == 0 && y == 0,
                    i == 2 && x == self.width - 1,
                    j == 2 && y == self.height - 1,
                ) {
                    (true, _, _, _) | (_, true, _, _) | (_, _, true, _) | (_, _, _, true)
                        if self.mode != GameMode::Portal =>
                    {
                        self.mode == GameMode::Alive
                    }
                    (true, true, _, _) if self.mode == GameMode::Portal => {
                        grid[(self.width - x - 1, self.height - y - 1)]
                    }
                    (_, _, true, true) if self.mode == GameMode::Portal => grid[(0, 0)],
                    (true, false, _, false) if self.mode == GameMode::Portal => {
                        grid[(self.width - x - 1, y + j - 1)]
                    }
                    (true, false, _, true) if self.mode == GameMode::Portal => {
                        grid[(self.width - x - 1, 0)]
                    }
                    (false, true, false, _) if self.mode == GameMode::Portal => {
                        grid[(x + i - 1, self.height - y - 1)]
                    }
                    (false, true, true, _) if self.mode == GameMode::Portal => {
                        grid[(0, self.height - y - 1)]
                    }
                    (false, false, true, false) if self.mode == GameMode::Portal => {
                        grid[(0, y + j - 1)]
                    }
                    (false, false, false, true) if self.mode == GameMode::Portal => {
                        grid[(x + i - 1, 0)]
                    }
                    (false, false, false, false) => grid[(x + i - 1, y + j - 1)],
                    _ => unreachable!(),
                } as u8;
            }
        }
        nb
    }
}

impl Display for GameOfLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        write!(res, "\x1b[1;1H")?;
        for y in (0..self.height).step_by(2) {
            for x in (0..self.width).step_by(2) {
                match (
                    self.grid[(x, y)],
                    self.grid[(x + 1, y)],
                    self.grid[(x, y + 1)],
                    self.grid[(x + 1, y + 1)],
                ) {
                    (true, true, true, true) => write!(res, "█")?,
                    (true, true, true, false) => write!(res, "▛")?,
                    (true, true, false, true) => write!(res, "▜")?,
                    (true, false, true, true) => write!(res, "▙")?,
                    (false, true, true, true) => write!(res, "▟")?,
                    (true, true, false, false) => write!(res, "▀")?,
                    (true, false, true, false) => write!(res, "▌")?,
                    (false, true, true, false) => write!(res, "▞")?,
                    (true, false, false, true) => write!(res, "▚")?,
                    (false, true, false, true) => write!(res, "▐")?,
                    (false, false, true, true) => write!(res, "▄")?,
                    (true, false, false, false) => write!(res, "▘")?,
                    (false, true, false, false) => write!(res, "▝")?,
                    (false, false, true, false) => write!(res, "▖")?,
                    (false, false, false, true) => write!(res, "▗")?,
                    (false, false, false, false) => write!(res, " ")?,
                }
            }
            writeln!(res, "")?;
        }
        write!(f, "{res}")
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.1 * self.width + index.0]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.1 * self.width + index.0]
    }
}

fn hide_cursor() {
    print!("\x1b[?25l")
}
fn show_cursor() {
    print!("\x1b[?25h")
}

pub fn main() {
    hide_cursor();
    let mut g = GameOfLife::random(100, 114);
    // let mut g = GameOfLife::new(80, 100);

    // g.grid[(5, 5)] = true;
    // g.grid[(5, 6)] = true;
    // g.grid[(5, 7)] = true;
    loop {
        print!("{g}");
        g.step();
        // sleep(Duration::from_millis(200))
    }
    
}
