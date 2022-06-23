use super::*;

pub mod logic;

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

pub enum PlayerAction {
    ActivateShape(Id),
    DeactivateShape(Id),
    AttachShape {
        triangle: Id,
        target: Id,
        pos: TriPos,
    },
    UpgradePlant {
        source_shape: Id,
        target_plant: Id,
    },
    EndTurn,
}

#[derive(Debug)]
pub struct Model {
    id_gen: IdGenerator,
    pub player_a: Player,
    pub player_b: Player,
}

#[derive(Debug, Clone)]
pub struct GrabbedShape {
    pub shape: AliveShape,
}

/// A position in a triangular grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
pub struct ShapeBuffer(pub Collection<AliveShape>);

#[derive(Debug)]
pub struct ShapeFarm {
    pub plants: Collection<Plant>,
}

#[derive(Debug)]
pub struct ActiveShapes(pub Collection<AliveShape>);

#[derive(Debug, Clone)]
pub struct Plant {
    pub id: Id,
    pub shape: Shape,
    pub cooldown: Turns,
    pub time_left: Turns,
}

impl Model {
    pub fn new() -> Self {
        let mut id_gen = IdGenerator::new();
        Self {
            player_a: Player::new(&mut id_gen),
            player_b: Player::new(&mut id_gen),
            id_gen,
        }
    }
}

impl Player {
    pub fn new(id_gen: &mut IdGenerator) -> Self {
        Self {
            shape_buffer: ShapeBuffer::new(),
            farm: ShapeFarm::new(id_gen),
            active_shapes: ActiveShapes::new(),
        }
    }
}

impl ShapeBuffer {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl ShapeFarm {
    pub fn new(id_gen: &mut IdGenerator) -> Self {
        let mut plants = Collection::new();
        plants.insert(Plant::new(
            id_gen.next(),
            Shape(vec![TriPos { x: 0, y: 0 }]),
            1,
        ));
        Self { plants }
    }
}

impl ActiveShapes {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl Plant {
    pub fn new(id: Id, shape: Shape, cooldown: Turns) -> Self {
        Self {
            time_left: cooldown,
            id,
            shape,
            cooldown,
        }
    }
}

impl HasId for AliveShape {
    type Id = Id;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl HasId for Plant {
    type Id = Id;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
