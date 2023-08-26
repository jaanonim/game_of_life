use tetra::math::Vec2;

#[derive(PartialEq, Debug)]
pub struct Block {
    pub is_alive: bool,
    pub pos: Vec2<f32>,
}

impl Block {
    pub fn new(x: f32, y: f32) -> Block {
        Block {
            is_alive: true,
            pos: Vec2::new(x, y),
        }
    }
}
