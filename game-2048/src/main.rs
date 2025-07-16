use std::{
    fmt::Display,
    io::Write,
    ops::{Index, IndexMut},
};

use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    event::{read, Event, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use owo_colors::{DynColors, OwoColorize};
use rand::{rngs::ThreadRng, seq::IndexedRandom, Rng};

const fn get_color(x: u64) -> DynColors {
    match x {
        2 => DynColors::Rgb(238, 228, 222),
        4 => DynColors::Rgb(237, 224, 200),
        8 => DynColors::Rgb(242, 177, 120),
        16 => DynColors::Rgb(245, 149, 99),
        32 => DynColors::Rgb(246, 124, 95),
        64 => DynColors::Rgb(246, 94, 59),
        128 => DynColors::Rgb(237, 207, 114),
        256 => DynColors::Rgb(237, 204, 97),
        512 => DynColors::Rgb(237, 200, 80),
        1024 => DynColors::Rgb(237, 197, 63),
        2048 => DynColors::Rgb(237, 194, 46),
        4096 => DynColors::Rgb(238, 102, 106),
        8192 => DynColors::Rgb(238, 78, 90),
        16384 => DynColors::Rgb(241, 65, 65),
        32768 => DynColors::Rgb(114, 178, 212),
        65536 => DynColors::Rgb(88, 159, 226),
        131072 => DynColors::Rgb(23, 129, 204),
        _ => DynColors::Rgb(255, 255, 255),
    }
}

pub struct NZip<T> {
    n: usize,
    iters: Vec<T>,
}

impl<T: Iterator> Iterator for NZip<T> {
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = Vec::with_capacity(self.n);
        for iter in &mut self.iters {
            res.push(iter.next()?)
        }
        Some(res)
    }
}

impl<T: DoubleEndedIterator> DoubleEndedIterator for NZip<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut res = Vec::with_capacity(self.n);
        for iter in &mut self.iters {
            res.push(iter.next_back()?)
        }
        Some(res)
    }
}

#[derive(Clone)]
pub struct Game {
    size: usize,
    grid: Vec<u64>,
    rng: ThreadRng,
    score: u64,
}

impl Index<(usize, usize)> for Game {
    type Output = u64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.1 * self.size + index.0]
    }
}
impl IndexMut<(usize, usize)> for Game {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.1 * self.size + index.0]
    }
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            grid: vec![0; size * size],
            rng: rand::rng(),
            score: 0,
        }
    }
    pub fn init(size: usize) -> Self {
        let mut g = Self::new(size);
        g.add();
        g.add();
        g
    }
    pub fn add(&mut self) {
        let mut pos = Vec::new();
        for y in 0..self.size {
            for x in 0..self.size {
                if self[(x, y)] == 0 {
                    pos.push((x, y));
                }
            }
        }
        let pos = *pos.choose(&mut self.rng).unwrap();
        self[pos] = (self.rng.random_range(0..=1) + 1) * 2;
    }
    pub fn lost(&self) -> bool {
        let mut copy = self.clone();
        !(copy.left() || copy.down() || copy.right() || copy.up())
    }
    fn rotate(&mut self) {
        let iters = self
            .grid
            .chunks(self.size)
            .rev()
            .map(|x| x.iter().cloned())
            .collect();
        let iter = NZip {
            n: self.size,
            iters,
        };
        self.grid = iter.flatten().collect();
    }
    fn rotate_back(&mut self) {
        let iters = self
            .grid
            .chunks(self.size)
            .map(|x| x.iter().cloned())
            .collect();
        let iter = NZip {
            n: self.size,
            iters,
        };
        self.grid = iter.rev().flatten().collect();
    }
    pub fn left(&mut self) -> bool {
        let mut changed = false;
        for y in 0..self.size {
            let mut index = 0;
            let mut fixed = false;
            let mut zero_found = false;
            for x in 0..self.size {
                if self[(x, y)] == 0 {
                    zero_found = true;
                    continue;
                } else if zero_found {
                    changed = true;
                }
                if index != 0 && self[(index - 1, y)] == (self[(x, y)]) && !fixed {
                    self[(index - 1, y)] = self[(x, y)] * 2;
                    self[(x, y)] = 0;
                    self.score += self[(index - 1, y)];
                    fixed = true;
                    changed = true;
                } else {
                    self[(index, y)] = self[(x, y)];
                    if index != x {
                        self[(x, y)] = 0;
                    }
                    fixed = false;
                    index += 1;
                }
            }
        }
        changed
    }
    pub fn down(&mut self) -> bool {
        self.rotate();
        let res = self.left();
        self.rotate_back();
        res
    }
    pub fn right(&mut self) -> bool {
        self.rotate();
        self.rotate();
        let res = self.left();
        self.rotate_back();
        self.rotate_back();
        res
    }
    pub fn up(&mut self) -> bool {
        self.rotate_back();
        let res = self.left();
        self.rotate();
        res
    }
    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let mut lost = false;
        init_print()?;
        print!("{self}");
        loop {
            match read()? {
                Event::Key(event) => {
                    if match event.code {
                        crossterm::event::KeyCode::Left => self.left(),
                        crossterm::event::KeyCode::Right => self.right(),
                        crossterm::event::KeyCode::Down => self.down(),
                        crossterm::event::KeyCode::Up => self.up(),
                        crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Char('q') => {
                            break
                        }
                        crossterm::event::KeyCode::Char('c')
                            if event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            break
                        }
                        _ => {
                            continue;
                        }
                    } {
                        self.add();
                    }
                    print!("{self}");
                    if self.lost() {
                        lost = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        if lost {
            execute!(std::io::stdout(), MoveToNextLine(1))?;
            print!("Game Over !");
        }
        clean_print()?;
        Ok(())
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        execute!(std::io::stdout(), MoveTo(0, 0)).map_err(|_| std::fmt::Error)?;

        let show = |f: &mut std::fmt::Formatter,
                    start: &str,
                    c: &str,
                    c2: &str,
                    end: &str|
         -> std::fmt::Result {
            write!(f, "{start}{}{end}", {
                vec![c2.repeat(6); self.size].join(c)
            })?;
            execute!(std::io::stdout(), MoveToNextLine(1)).map_err(|_| std::fmt::Error)
        };

        for y in 0..self.size {
            if y == 0 {
                show(f, "┌", "┬", "─", "┐")?;
            } else {
                show(f, "├", "┼", "─", "┤")?;
            }
            show(f, "│", "│", " ", "│")?;
            for x in 0..self.size {
                if self[(x, y)] != 0 {
                    write!(f, "│{: ^6}", self[(x, y)].color(get_color(self[(x, y)])))?;
                } else {
                    write!(f, "│      ")?;
                }
            }
            write!(f, "│")?;
            execute!(std::io::stdout(), MoveToNextLine(1)).map_err(|_| std::fmt::Error)?;
            show(f, "│", "│", " ", "│")?;
        }
        show(f, "└", "┴", "─", "┘")?;
        execute!(std::io::stdout(), MoveToNextLine(2)).map_err(|_| std::fmt::Error)?;
        write!(f, "score: {}", self.score)?;
        std::io::stdout().flush().map_err(|_| std::fmt::Error)
    }
}

pub fn init_print() -> Result<(), std::io::Error> {
    execute!(std::io::stdout(), Clear(ClearType::All), Hide)?;
    enable_raw_mode()
}

pub fn clean_print() -> Result<(), std::io::Error> {
    execute!(std::io::stdout(), Show)?;
    disable_raw_mode()?;
    println!("");
    Ok(())
}

fn main() {
    let mut g = Game::init(4);
    g.run().unwrap();
}
