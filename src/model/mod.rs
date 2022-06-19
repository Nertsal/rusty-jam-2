use super::*;

pub type Time = R32;
pub type Turns = u64;

pub struct Model {
    pub player_a: Player,
    pub player_b: Player,
}

/// A position in a triangular grid
#[derive(Debug, Clone, Copy)]
pub struct TriPos {
    pub x: i64,
    pub y: i64,
}

pub struct Player {
    pub shape_buffer: ShapeBuffer,
    pub farm: ShapeFarm,
    pub active_shapes: ActiveShapes,
}

/// A shape is basically formed from cells in a triangular grid.
pub struct Shape(pub Vec<TriPos>);

pub struct ShapeBuffer {}

pub struct ShapeFarm {}

pub struct ActiveShapes {}

pub struct Plant {
    pub cooldown: Turns,
    pub time_left: Turns,
}

impl Model {
    pub fn new() -> Self {
        Self {
            player_a: Player::new(),
            player_b: Player::new(),
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            shape_buffer: ShapeBuffer::new(),
            farm: ShapeFarm::new(),
            active_shapes: ActiveShapes::new(),
        }
    }
}

impl ShapeBuffer {
    pub fn new() -> Self {
        Self {}
    }
}

impl ShapeFarm {
    pub fn new() -> Self {
        Self {}
    }
}

impl ActiveShapes {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plant {
    pub fn tick(&mut self) -> bool {
        if self.time_left <= 0 {
            self.time_left = self.cooldown;
            return true;
        }
        self.time_left -= 1;
        false
    }
}

impl TriPos {
    pub fn to_cartesian(&self) -> Vec2<R32> {
        let side = r32(1.0);
        let root_3 = r32(3.0).sqrt();
        vec2(
            side * r32(0.5) * r32(self.x as _),
            side * root_3 * r32(0.5) * r32(self.y as _)
                + if self.is_upside_down() {
                    r32(1.0)
                } else {
                    r32(-1.0)
                } * side
                    * root_3
                    / r32(12.0),
        )
    }

    /// Whether a triangle in that position points down
    pub fn is_upside_down(&self) -> bool {
        (self.x + self.y) % 2 != 0
    }
}
