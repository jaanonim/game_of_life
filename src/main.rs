use block::Block;
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::input::{Key, MouseButton};
use tetra::math::Vec2;
use tetra::{input, Context, ContextBuilder, State};
pub mod block;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const SCALE: f32 = 16.0;
const MOVEMENT_SPEED: f32 = 4.0;

struct GameState {
    texture: Texture,
    mouse_position: Vec2<f32>,
    blocks: Vec<Block>,
    mouse_lock: bool,
    space_counter: u8,
    camera_pos: Vec2<f32>,
}

struct NeighborsInfo {
    pub count: i32,
    pub positions: Vec<Vec2<f32>>,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            texture: Texture::new(ctx, "./resources/pixel.png")?,
            mouse_position: Vec2::new(0.0, 0.0),
            blocks: vec![],
            mouse_lock: false,
            space_counter: 0,
            camera_pos: Vec2::new(0.0, 0.0),
        })
    }

    fn update(&mut self) {
        println!("Update {}", self.blocks.len());
        let mut to_del: Vec<Vec2<f32>> = vec![];
        let mut to_spawn: Vec<Vec2<f32>> = vec![];
        for block in self.blocks.clone() {
            let pos = block.pos;

            let nb = self.get_neighbors(pos);

            if match nb.count {
                1 => true,
                2 | 3 => false,
                _ => true,
            } {
                to_del.push(pos);
            }

            for pos in nb.positions {
                if self.get_neighbors(pos).count == 3 && !to_spawn.contains(&pos) {
                    to_spawn.push(pos)
                }
            }
        }

        for pos in to_del {
            if let Some(idx) = self
                .blocks
                .iter()
                .position(|item| item.pos.x == pos.x && item.pos.y == pos.y)
            {
                self.blocks.remove(idx);
            }
        }
        for pos in to_spawn {
            self.blocks.push(Block::new(pos.x, pos.y));
        }
    }

    fn get_neighbors(&mut self, pos: Vec2<f32>) -> NeighborsInfo {
        let dirs: Vec<Vec2<f32>> = vec![
            Vec2::new(1.0, 0.0),
            Vec2::new(-1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(0.0, -1.0),
            Vec2::new(1.0, -1.0),
            Vec2::new(-1.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(-1.0, -1.0),
        ];

        let mut positions: Vec<Vec2<f32>> = vec![];
        let mut count: usize = 0;

        for dir in dirs {
            let x = pos.x + dir.x;
            let y = pos.y + dir.y;

            if let Some(_b) = self
                .blocks
                .iter()
                .find(|item| item.pos.x == x && item.pos.y == y)
            {
                count += 1;
            } else {
                positions.push(Vec2::new(x, y))
            }
        }
        NeighborsInfo {
            count: count as i32,
            positions,
        }
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), tetra::TetraError> {
        if input::is_key_down(ctx, Key::W) {
            self.camera_pos.y -= MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::S) {
            self.camera_pos.y += MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::A) {
            self.camera_pos.x -= MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::D) {
            self.camera_pos.x += MOVEMENT_SPEED;
        }

        self.mouse_position = input::get_mouse_position(ctx).round();
        if input::is_mouse_button_down(ctx, MouseButton::Left) && !self.mouse_lock {
            self.mouse_lock = true;
            let x = ((self.mouse_position.x + self.camera_pos.x) / SCALE).floor();
            let y = ((self.mouse_position.y + self.camera_pos.y) / SCALE).floor();

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

        if input::is_key_down(ctx, input::Key::Space) {
            if self.space_counter == 0 {
                self.space_counter = 5;
                self.update();
            } else {
                self.space_counter -= 1;
            }
        } else {
            self.space_counter = 0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.1, 0.1, 0.1));

        let range_x = (self.camera_pos.x / SCALE) as i32 - 1
            ..((WINDOW_WIDTH + self.camera_pos.x) / SCALE) as i32 + 1;

        for x in range_x {
            let range_y = (self.camera_pos.y / SCALE) as i32 - 1
                ..((WINDOW_HEIGHT + self.camera_pos.y) / SCALE) as i32 + 1;

            for y in range_y {
                let screen_x = x as f32 * SCALE - self.camera_pos.x;
                let screen_y = y as f32 * SCALE - self.camera_pos.y;

                if self.blocks.contains(&Block::new(x as f32, y as f32)) {
                    self.texture.draw(
                        ctx,
                        DrawParams::new()
                            .position(Vec2::new(screen_x, screen_y))
                            .origin(Vec2::new(0.0, 0.0))
                            .scale(Vec2::new(SCALE - 1.0, SCALE - 1.0)),
                    );
                } else {
                    self.texture.draw(
                        ctx,
                        DrawParams::new()
                            .position(Vec2::new(screen_x, screen_y))
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
    ContextBuilder::new("Game of Life", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}
