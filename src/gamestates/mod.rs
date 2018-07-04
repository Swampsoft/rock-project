
pub mod hello;

use ggez::{Context, GameResult};

pub trait GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;
}
