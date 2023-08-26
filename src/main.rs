use block::Block;
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::input::MouseButton;
use tetra::math::Vec2;
use tetra::{input, Context, ContextBuilder, State};
pub mod block;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const SCALE: f32 = 16.0;

struct GameState {
    texture: Texture,
    mouse_position: Vec2<f32>,
    blocks: Vec<Block>,
    mouse_lock: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            texture: Texture::new(ctx, "./resources/pixel.png")?,
            mouse_position: Vec2::new(0.0, 0.0),
            blocks: vec![],
            mouse_lock: false,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        self.mouse_position = input::get_mouse_position(ctx).round();
        if input::is_mouse_button_down(ctx, MouseButton::Left) && !self.mouse_lock {
            self.mouse_lock = true;
            let x = (self.mouse_position.x as i32 / SCALE as i32) as f32;
            let y = (self.mouse_position.y as i32 / SCALE as i32) as f32;

            if let Some(idx) = self
                .blocks
                .iter()
                .position(|item| item.pos.x == x && item.pos.y == y)
            {
                self.blocks.remove(idx);
            } else {
                self.blocks.push(Block::new(x, y));
            }
        } else if input::is_mouse_button_up(ctx, MouseButton::Left) {
            self.mouse_lock = false;
        }

        graphics::clear(ctx, Color::rgb(0.1, 0.1, 0.1));

        for x in (0..(WINDOW_WIDTH as i32)).step_by(SCALE as usize) {
            for y in (0..(WINDOW_HEIGHT as i32)).step_by(SCALE as usize) {
                if self.blocks.contains(&Block::new(
                    (x / SCALE as i32) as f32,
                    (y / SCALE as i32) as f32,
                )) {
                    self.texture.draw(
                        ctx,
                        DrawParams::new()
                            .position(Vec2::new(x as f32, y as f32))
                            .origin(Vec2::new(0.0, 0.0))
                            .scale(Vec2::new(SCALE - 1.0, SCALE - 1.0)),
                    );
                } else {
                    self.texture.draw(
                        ctx,
                        DrawParams::new()
                            .position(Vec2::new(x as f32, y as f32))
                            .origin(Vec2::new(0.0, 0.0))
                            .scale(Vec2::new(SCALE - 1.0, SCALE - 1.0))
                            .color(Color::rgb(0.0, 0.0, 0.0)),
                    );
                }
            }
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}
