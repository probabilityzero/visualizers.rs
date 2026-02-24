use crate::games::Game;
use crossterm::{execute, queue, style::Print, terminal::EnterAlternateScreen, terminal::LeaveAlternateScreen, cursor::{Hide, MoveTo, Show}, event::{poll, read, Event, KeyCode}};
use std::io::{stdout, Write};
use std::{io, thread};
use std::time::Duration;

pub struct BouncingBall {
    width: u16,
    height: u16,
}

impl BouncingBall {
    pub fn new() -> Self {
        let (w, h) = crossterm::terminal::size().unwrap_or((80, 24));
        Self { width: w, height: h.saturating_sub(1) }
    }
}

impl Game for BouncingBall {
    fn name(&self) -> &'static str { "Bouncing Ball" }

    fn run(&mut self) -> io::Result<()> {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, Hide)?;
        crossterm::terminal::enable_raw_mode()?;

        let mut x = self.width as i32 / 2;
        let mut y = self.height as i32 / 2;
        let mut vx = 1i32;
        let mut vy = 1i32;

        loop {
            queue!(stdout, MoveTo(0, 0), Print("\n"))?;
            queue!(stdout, MoveTo(x as u16, y as u16), Print("o"))?;
            stdout.flush()?;

            if poll(Duration::from_millis(30))? {
                if let Event::Key(k) = read()? {
                    if k.code == KeyCode::Char('q') {
                        execute!(stdout, Show, LeaveAlternateScreen)?;
                        crossterm::terminal::disable_raw_mode()?;
                        return Ok(());
                    }
                }
            }

            x += vx;
            y += vy;
            if x <= 0 || x as u16 >= self.width - 1 { vx = -vx; }
            if y <= 0 || y as u16 >= self.height - 1 { vy = -vy; }

            thread::sleep(Duration::from_millis(30));
        }
    }
}

pub fn boxed() -> Box<dyn Game> { Box::new(BouncingBall::new()) }
