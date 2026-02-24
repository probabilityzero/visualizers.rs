use crate::games::Game;
use crossterm::{execute, queue, style::Print, terminal::EnterAlternateScreen, terminal::LeaveAlternateScreen, cursor::{Hide, MoveTo, Show}, event::{poll, read, Event, KeyCode}};
use std::io::{stdout, Write};
use std::{io, thread};
use std::time::Duration;
use rand::Rng;

pub struct RandomWalk {
    width: u16,
    height: u16,
}

impl RandomWalk {
    pub fn new() -> Self {
        let (w, h) = crossterm::terminal::size().unwrap_or((80, 24));
        Self { width: w, height: h.saturating_sub(1) }
    }
}

impl Game for RandomWalk {
    fn name(&self) -> &'static str { "Random Walk" }

    fn run(&mut self) -> io::Result<()> {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, Hide)?;
        crossterm::terminal::enable_raw_mode()?;

        let mut x = self.width / 2;
        let mut y = self.height / 2;
        let mut rng = rand::thread_rng();

        loop {
            queue!(stdout, MoveTo(0, 0), Print("\n"))?;
            queue!(stdout, MoveTo(x, y), Print("*"))?;
            stdout.flush()?;

            if poll(Duration::from_millis(50))? {
                if let Event::Key(k) = read()? {
                    if k.code == KeyCode::Char('q') {
                        execute!(stdout, Show, LeaveAlternateScreen)?;
                        crossterm::terminal::disable_raw_mode()?;
                        return Ok(());
                    }
                }
            }

            match rng.gen_range(0..4) {
                0 => if x > 0 { x -= 1; } else { x = self.width - 1; }
                1 => if x < self.width - 1 { x += 1; } else { x = 0; }
                2 => if y > 0 { y -= 1; } else { y = self.height - 1; }
                _ => if y < self.height - 1 { y += 1; } else { y = 0; }
            }

            thread::sleep(Duration::from_millis(30));
        }
    }
}

pub fn boxed() -> Box<dyn Game> { Box::new(RandomWalk::new()) }
