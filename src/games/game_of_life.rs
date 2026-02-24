use crate::games::Game;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use std::io::{stdout, Write};
use std::time::Duration;
use std::{io, thread};

pub struct GameOfLife {
    width: usize,
    height: usize,
}

impl GameOfLife {
    pub fn new() -> Self {
        let (w, h) = crossterm::terminal::size().unwrap_or((80, 24));
        Self {
            width: w as usize,
            height: (h as usize).saturating_sub(1),
        }
    }

    fn random_grid(&self) -> Vec<Vec<bool>> {
        let mut rng = rand::thread_rng();
        let mut cells = vec![vec![false; self.width]; self.height];
        for row in &mut cells {
            for cell in row {
                *cell = rng.gen_bool(0.3);
            }
        }
        cells
    }

    fn count_live_neighbors(cells: &Vec<Vec<bool>>, width: usize, height: usize, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in [-1isize, 0, 1] {
            for dx in [-1isize, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (x as isize + dx).rem_euclid(width as isize) as usize;
                let ny = (y as isize + dy).rem_euclid(height as isize) as usize;
                if cells[ny][nx] {
                    count += 1;
                }
            }
        }
        count
    }

    fn cell_color_escape(alive: bool, neighbors: usize) -> &'static str {
        match (alive, neighbors) {
            (true, 0..=1) => "\x1b[41m ",
            (true, 2) => "\x1b[42m ",
            (true, 3) => "\x1b[102m ",
            (true, _) => "\x1b[45m ",
            (false, 3) => "\x1b[46m ",
            (false, 1..=2) => "\x1b[100m ",
            (false, _) => "\x1b[40m ",
        }
    }

    fn next_generation(cells: &Vec<Vec<bool>>, width: usize, height: usize) -> Vec<Vec<bool>> {
        let mut next = vec![vec![false; width]; height];
        for y in 0..height {
            for x in 0..width {
                let alive = cells[y][x];
                let n = Self::count_live_neighbors(cells, width, height, x, y);
                let next_state = matches!((alive, n), (true, 2 | 3) | (false, 3));
                next[y][x] = next_state;
            }
        }
        next
    }

    fn render(cells: &Vec<Vec<bool>>, width: usize, height: usize) {
        let mut stdout = stdout();
        let mut frame = String::with_capacity(width * height * 3);
        for y in 0..height {
            for x in 0..width {
                let alive = cells[y][x];
                let n = Self::count_live_neighbors(cells, width, height, x, y);
                frame.push_str(Self::cell_color_escape(alive, n));
            }
            frame.push('\n');
        }
        frame.push_str("\x1b[0m");
        execute!(stdout, MoveTo(0, 0), Print(&frame)).unwrap();
        stdout.flush().unwrap();
    }
}

impl Game for GameOfLife {
    fn name(&self) -> &'static str {
        "Game of Life"
    }

    fn run(&mut self) -> io::Result<()> {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, Hide)?;
        crossterm::terminal::enable_raw_mode()?;

        let mut cells = self.random_grid();
        let tick = Duration::from_secs(1);

        loop {
            let start = std::time::Instant::now();
            Self::render(&cells, self.width, self.height);
            cells = Self::next_generation(&cells, self.width, self.height);

            while poll(Duration::from_millis(0))? {
                if let Event::Key(k) = read()? {
                    if k.code == KeyCode::Char('q') {
                        execute!(stdout, Show, LeaveAlternateScreen)?;
                        crossterm::terminal::disable_raw_mode()?;
                        return Ok(());
                    }
                }
            }

            let elapsed = start.elapsed();
            if elapsed < tick {
                thread::sleep(tick - elapsed);
            }
        }
    }
}

pub fn boxed() -> Box<dyn Game> {
    Box::new(GameOfLife::new())
}
