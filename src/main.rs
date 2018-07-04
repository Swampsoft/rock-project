extern crate ggez;

use ggez::{conf, event::EventHandler, graphics, Context, GameResult};

struct MainState {}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<Self> {
        let s = MainState {};
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::present(ctx);
        Ok(())
    }
}

fn main() -> GameResult<()> {
    let c = conf::Conf {
        window_mode: conf::WindowMode::default().dimensions(1024, 768),
        window_setup: conf::WindowSetup::default().title("Rock Project"),
        backend: conf::Backend::OpenGL { major: 3, minor: 2 },
    };

    let ctx = &mut ggez::Context::load_from_conf("rock-project", "Swampsoft Games", c).unwrap();

    let main_state = &mut MainState::new(ctx)?;

    ggez::event::run(ctx, main_state)
}
