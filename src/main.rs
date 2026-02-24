mod games;

use std::io::{self, stdout, Write};
use crossterm::event::{read, Event, KeyCode};

fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

fn main() -> io::Result<()> {
    loop {
        clear_screen();
        println!("Visualisers — Select a game to run:\n");

        let factories = games::available_games();
        for (i, factory) in factories.iter().enumerate() {
            let g = factory();
            println!("  {}) {}", i + 1, g.name());
        }

        println!("\n  q) Quit\n");
        println!("Press a number key to run its game.");

        // Wait for a single key event
        if let Event::Key(k) = read()? {
            match k.code {
                KeyCode::Char('q') => {
                    println!("Exiting.");
                    break;
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    let idx = (c as u8 - b'0') as usize;
                    if idx >= 1 && idx <= factories.len() {
                        let mut game = (factories[idx - 1])();
                        let _ = game.run();
                    }
                }
                _ => {}
            }
        }

        let _ = stdout().flush();
    }

    Ok(())
}