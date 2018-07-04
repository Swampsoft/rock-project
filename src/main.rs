extern crate ggez;

mod gamestates;

use ggez::{conf, event::EventHandler, Context, GameResult};

use gamestates::{GameState, hello::HelloState};

struct StateManager {
    states: Vec<Box<GameState>>,
}

impl StateManager {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let s = StateManager {
            states: vec![Box::new(HelloState::new(ctx)?)]
        };
        Ok(s)
    }
}

impl EventHandler for StateManager {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // only update top state
        match self.states.last_mut() {
            Some(state) => state.update(ctx),
            None => ctx.quit(),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // draw all states from bottom up
        // this should allow to show e.g. a game menu on top of the (paused) game
        for state in self.states.iter_mut() {
            state.draw(ctx)?
        }
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

    let main_state = &mut StateManager::new(ctx)?;

    ggez::event::run(ctx, main_state)
}
