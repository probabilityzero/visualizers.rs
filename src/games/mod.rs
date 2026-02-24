use std::io;

pub trait Game {
    fn name(&self) -> &'static str;
    fn run(&mut self) -> io::Result<()>;
}

pub type GameFactory = fn() -> Box<dyn Game>;

pub mod game_of_life;
pub mod random_walk;
pub mod bouncing_ball;

pub fn available_games() -> Vec<GameFactory> {
    vec![game_of_life::boxed, random_walk::boxed, bouncing_ball::boxed]
}
