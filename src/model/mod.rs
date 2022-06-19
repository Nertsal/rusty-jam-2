use super::*;

pub type Time = R32;
pub type Turns = u64;
pub type Id = u64;

#[derive(Debug)]
pub struct IdGenerator(Id);

impl IdGenerator {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn next(&mut self) -> Id {
        let id = self.0;
        self.0 += 1;
        id
    }
}

#[derive(Debug)]
pub struct Model {
    pub id_gen: IdGenerator,
    pub player_a: Player,
    pub player_b: Player,
}

/// A position in a triangular grid
#[derive(Debug, Clone, Copy)]
pub struct TriPos {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
pub struct Player {
    pub shape_buffer: ShapeBuffer,
    pub farm: ShapeFarm,
    pub active_shapes: ActiveShapes,
}

/// A shape is basically formed from cells in a triangular grid.
#[derive(Debug, Clone)]
pub struct Shape(pub Vec<TriPos>);

#[derive(Debug, Clone)]
pub struct AliveShape {
    pub id: Id,
    pub shape: Shape,
}

#[derive(Debug)]
pub struct ShapeBuffer(pub Vec<AliveShape>);

#[derive(Debug)]
pub struct ShapeFarm {
    pub plants: Vec<Plant>,
}

#[derive(Debug)]
pub struct ActiveShapes(pub Vec<AliveShape>);

#[derive(Debug, Clone)]
pub struct Plant {
    pub shape: Shape,
    pub cooldown: Turns,
    pub time_left: Turns,
}

impl Model {
    pub fn new() -> Self {
        Self {
            id_gen: IdGenerator::new(),
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
        Self(vec![])
    }
}

impl ShapeFarm {
    pub fn new() -> Self {
        Self {
            plants: vec![Plant::new(Shape(vec![TriPos { x: 0, y: 0 }]), 1)],
        }
    }
}

impl ActiveShapes {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl Plant {
    pub fn new(shape: Shape, cooldown: Turns) -> Self {
        Self {
            time_left: cooldown,
            shape,
            cooldown,
        }
    }

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
