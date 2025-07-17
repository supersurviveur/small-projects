use crossterm::cursor::{Hide, MoveTo, MoveToNextLine, Show};
use crossterm::event::{poll, read, Event, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use rand::Rng;
use std::time::Duration;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameMode {
    Dead,
    Alive,
    Portal,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DisplayMode {
    Text1x1,
    Text1x2,
    Text2x2,
    Text2x3,
    Text2x4,
}

const TEXT_1X1: [char; 2] = [' ', '█'];
const TEXT_1X2: [char; 4] = [' ', '▀', '▄', '█'];
const TEXT_2X2: [char; 16] = [
    ' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█',
];
const TEXT_2X3: [char; 64] = [
    ' ', '🬀', '🬁', '🬂', '🬃', '🬄', '🬅', '🬆', '🬇', '🬈', '🬉', '🬊', '🬋', '🬌', '🬍', '🬎', '🬏', '🬐', '🬑',
    '🬒', '🬓', '▌', '🬔', '🬕', '🬖', '🬗', '🬘', '🬙', '🬚', '🬛', '🬜', '🬝', '🬞', '🬟', '🬠', '🬡', '🬢', '🬣',
    '🬤', '🬥', '🬦', '🬧', '▐', '🬨', '🬩', '🬪', '🬫', '🬬', '🬭', '🬮', '🬯', '🬰', '🬱', '🬲', '🬳', '🬴', '🬵',
    '🬶', '🬷', '🬸', '🬹', '🬺', '🬻', '█',
];
const TEXT_2X4: [char; 256] = [
    ' ', '𜺨', '𜺫', '🮂', '𜴀', '▘', '𜴁', '𜴂', '𜴃', '𜴄', '▝', '𜴅', '𜴆', '𜴇', '𜴈', '▀', '𜴉', '𜴊', '𜴋',
    '𜴌', '🯦', '𜴍', '𜴎', '𜴏', '𜴐', '𜴑', '𜴒', '𜴓', '𜴔', '𜴕', '𜴖', '𜴗', '𜴘', '𜴙', '𜴚', '𜴛', '𜴜', '𜴝',
    '𜴞', '𜴟', '🯧', '𜴠', '𜴡', '𜴢', '𜴣', '𜴤', '𜴥', '𜴦', '𜴧', '𜴨', '𜴩', '𜴪', '𜴫', '𜴬', '𜴭', '𜴮', '𜴯',
    '𜴰', '𜴱', '𜴲', '𜴳', '𜴴', '𜴵', '🮅', '𜺣', '𜴶', '𜴷', '𜴸', '𜴹', '𜴺', '𜴻', '𜴼', '𜴽', '𜴾', '𜴿', '𜵀',
    '𜵁', '𜵂', '𜵃', '𜵄', '▖', '𜵅', '𜵆', '𜵇', '𜵈', '▌', '𜵉', '𜵊', '𜵋', '𜵌', '▞', '𜵍', '𜵎', '𜵏', '𜵐',
    '▛', '𜵑', '𜵒', '𜵓', '𜵔', '𜵕', '𜵖', '𜵗', '𜵘', '𜵙', '𜵚', '𜵛', '𜵜', '𜵝', '𜵞', '𜵟', '𜵠', '𜵡', '𜵢',
    '𜵣', '𜵤', '𜵥', '𜵦', '𜵧', '𜵨', '𜵩', '𜵪', '𜵫', '𜵬', '𜵭', '𜵮', '𜵯', '𜵰', '𜺠', '𜵱', '𜵲', '𜵳', '𜵴',
    '𜵵', '𜵶', '𜵷', '𜵸', '𜵹', '𜵺', '𜵻', '𜵼', '𜵽', '𜵾', '𜵿', '𜶀', '𜶁', '𜶂', '𜶃', '𜶄', '𜶅', '𜶆', '𜶇',
    '𜶈', '𜶉', '𜶊', '𜶋', '𜶌', '𜶍', '𜶎', '𜶏', '▗', '𜶐', '𜶑', '𜶒', '𜶓', '▚', '𜶔', '𜶕', '𜶖', '𜶗', '▐',
    '𜶘', '𜶙', '𜶚', '𜶛', '▜', '𜶜', '𜶝', '𜶞', '𜶟', '𜶠', '𜶡', '𜶢', '𜶣', '𜶤', '𜶥', '𜶦', '𜶧', '𜶨', '𜶩',
    '𜶪', '𜶫', '▂', '𜶬', '𜶭', '𜶮', '𜶯', '𜶰', '𜶱', '𜶲', '𜶳', '𜶴', '𜶵', '𜶶', '𜶷', '𜶸', '𜶹', '𜶺', '𜶻',
    '𜶼', '𜶽', '𜶾', '𜶿', '𜷀', '𜷁', '𜷂', '𜷃', '𜷄', '𜷅', '𜷆', '𜷇', '𜷈', '𜷉', '𜷊', '𜷋', '𜷌', '𜷍', '𜷎',
    '𜷏', '𜷐', '𜷑', '𜷒', '𜷓', '𜷔', '𜷕', '𜷖', '𜷗', '𜷘', '𜷙', '𜷚', '▄', '𜷛', '𜷜', '𜷝', '𜷞', '▙', '𜷟',
    '𜷠', '𜷡', '𜷢', '▟', '𜷣', '▆', '𜷤', '𜷥', '█',
];

#[derive(Clone)]
pub struct Grid {
    grid: Box<[bool]>,

    height: usize,
    width: usize,
}
impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            grid: vec![false; height * width].into_boxed_slice(),
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
    display_mode: DisplayMode,
}

impl GameOfLife {
    pub fn with_grid(grid: Grid, display_mode: DisplayMode) -> Self {
        Self {
            height: grid.height,
            width: grid.width,
            grid,
            mode: GameMode::Portal,
            display_mode,
        }
    }
    pub fn new(height: usize, width: usize, display_mode: DisplayMode) -> Self {
        Self::with_grid(Grid::new(height, width), display_mode)
    }
    pub fn random(height: usize, width: usize, display_mode: DisplayMode) -> Self {
        Self::with_grid(Grid::random(height, width), display_mode)
    }
    pub fn random_full(display_mode: DisplayMode) -> Result<Self, std::io::Error> {
        let (mut width, mut height) = crossterm::terminal::size()?;
        match display_mode {
            DisplayMode::Text1x1 => {}
            DisplayMode::Text1x2 => height *= 2,
            DisplayMode::Text2x2 => {
                width *= 2;
                height *= 2;
            }
            DisplayMode::Text2x3 => {
                width *= 2;
                height *= 3;
            }
            DisplayMode::Text2x4 => {
                width *= 2;
                height *= 4;
            }
        }
        Ok(Self::random(height as usize, width as usize, display_mode))
    }
    pub fn step(&mut self) {
        let copied = self.grid.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let nb = self.count_neighbours(&copied, x, y);
                self.grid[(x, y)] = if self.grid[(x, y)] {
                    (2..=3).contains(&nb)
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
        execute!(std::io::stdout(), MoveTo(0, 0)).map_err(|_| std::fmt::Error)?;
        let (patterns, width, height) = match self.display_mode {
            DisplayMode::Text1x1 => (TEXT_1X1.as_slice(), 1, 1),
            DisplayMode::Text1x2 => (TEXT_1X2.as_slice(), 1, 2),
            DisplayMode::Text2x2 => (TEXT_2X2.as_slice(), 2, 2),
            DisplayMode::Text2x3 => (TEXT_2X3.as_slice(), 2, 3),
            DisplayMode::Text2x4 => (TEXT_2X4.as_slice(), 2, 4),
        };
        for y in (0..self.height).step_by(height) {
            for x in (0..self.width).step_by(width) {
                let mut code = 0;
                for i in (0..height).rev() {
                    for j in (0..width).rev() {
                        code <<= 1;
                        if (0..self.height).contains(&(i + y))
                            && (0..self.width).contains(&(j + x))
                            && self.grid[(x + j, y + i)]
                        {
                            code |= 1;
                        }
                    }
                }
                write!(f, "{}", patterns[code])?;
            }
            execute!(std::io::stdout(), MoveToNextLine(1)).map_err(|_| std::fmt::Error)?;
        }
        Ok(())
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

impl Index<(usize, usize)> for GameOfLife {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index]
    }
}

impl IndexMut<(usize, usize)> for GameOfLife {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index]
    }
}

pub fn init_print() -> Result<(), std::io::Error> {
    execute!(std::io::stdout(), Clear(ClearType::All), Hide)?;
    enable_raw_mode()
}

pub fn clean_print() -> Result<(), std::io::Error> {
    execute!(std::io::stdout(), Show)?;
    disable_raw_mode()?;
    println!();
    Ok(())
}

pub fn main() {
    init_print().unwrap();
    let mut g = GameOfLife::random_full(DisplayMode::Text2x4).unwrap();
    // let mut g = GameOfLife::new(80, 100);

    loop {
        print!("{g}");
        g.step();
        // sleep(Duration::from_millis(200))

        if poll(Duration::from_secs(0)).unwrap() {
            if let Event::Key(event) = read().unwrap() {
                match event.code {
                    crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Char('q') => break,
                    crossterm::event::KeyCode::Char('c')
                        if event.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        break
                    }
                    _ => {}
                }
            }
        }
    }
    clean_print().unwrap();
}
