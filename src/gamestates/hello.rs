
use ggez::{graphics::{self, Image, Point2, Rect, spritebatch::SpriteBatch}, Context, GameResult};

use super::GameState;

pub struct HelloState {
    text: graphics::TextCached,
    batch: SpriteBatch,
}

impl HelloState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let image = Image::new(ctx, "/tiles.png")?;
        let mut batch = SpriteBatch::new(image);

        batch.set_filter(graphics::FilterMode::Nearest);

        batch.add(graphics::DrawParam {
            src: Rect::new(37.0 / 256.0, 1.0 / 256.0, 16.0 / 256.0, 12.0 / 256.0),
            dest: Point2::new(0.0, 0.0),
            scale: Point2::new(5.0, 5.0),
            .. Default::default()
        });

        batch.add(graphics::DrawParam {
            src: Rect::new(37.0 / 256.0, 1.0 / 256.0, 16.0 / 256.0, 12.0 / 256.0),
            dest: Point2::new(13.0 * 5.0, 6.0 * 5.0),
            scale: Point2::new(5.0, 5.0),
            .. Default::default()
        });

        batch.add(graphics::DrawParam {
            src: Rect::new(37.0 / 256.0, 1.0 / 256.0, 16.0 / 256.0, 12.0 / 256.0),
            dest: Point2::new(13.0 * 5.0, -6.0 * 5.0),
            scale: Point2::new(5.0, 5.0),
            .. Default::default()
        });


        let s = HelloState {
            text: graphics::TextCached::new("Hello!")?,
            batch,
        };
        Ok(s)
    }
}

impl GameState for HelloState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::draw(ctx, &self.batch, Point2::new(100.0, 100.0), 0.0)?;
        graphics::draw(ctx, &self.text, Point2::new(512.0, 384.0), 0.0)?;
        graphics::present(ctx);
        Ok(())
    }
}
