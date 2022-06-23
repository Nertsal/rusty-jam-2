use super::*;
use geng::prelude::itertools::Itertools;

mod triangular;

pub use triangular::*;

impl Model {
    pub fn update(&mut self, _delta_time: Time) {}

    pub fn handle_player_action(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::EndTurn => self.tick(),
            PlayerAction::ActivateShape(shape_id) => self.activate_shape(shape_id),
            PlayerAction::DeactivateShape(shape_id) => self.deactivate_shape(shape_id),
            PlayerAction::AttachShape {
                triangle,
                target,
                pos,
            } => self.attach_shape(triangle, target, pos),
            PlayerAction::UpgradePlant {
                source_shape,
                target_plant,
            } => self.upgrade_plant(source_shape, target_plant),
            PlayerAction::Attack { weapon, target } => self.attack(weapon, target),
        }
    }

    fn tick(&mut self) {
        for plant in &mut self.player_a.shape_farm.plants {
            if plant.tick() {
                self.player_a.shape_buffer.0.insert(AliveShape {
                    id: self.id_gen.next(),
                    shape: plant.shape.clone(),
                });
            }
        }
    }

    fn activate_shape(&mut self, shape_id: Id) {
        if let Some(shape) = self.player_a.shape_buffer.0.remove(&shape_id) {
            self.player_a.active_shapes.0.insert(shape);
        }
    }

    fn deactivate_shape(&mut self, shape_id: Id) {
        if let Some(shape) = self.player_a.active_shapes.0.remove(&shape_id) {
            self.player_a.shape_buffer.0.insert(shape);
        }
    }

    fn attach_shape(&mut self, triangle: Id, target: Id, pos: TriPos) {
        if self
            .player_a
            .active_shapes
            .0
            .get(&triangle)
            .filter(|triangle| triangle.shape.0.len() == 1)
            .is_none()
        {
            return;
        }
        let target = match self
            .player_a
            .active_shapes
            .0
            .get_mut(&target)
            .filter(|target| target.shape.boundary().contains(&pos))
        {
            Some(target) => target,
            None => return,
        };

        target.shape.0.push(pos);
        self.player_a
            .active_shapes
            .0
            .remove(&triangle)
            .expect("Attached triangle disappeared");
    }

    fn upgrade_plant(&mut self, source_shape: Id, target_plant: Id) {
        let source = match self.player_a.shape_buffer.0.remove(&source_shape) {
            Some(source) => source,
            None => return,
        };
        let plant = match self.player_a.shape_farm.plants.get_mut(&target_plant) {
            Some(farm) => farm,
            None => return,
        };

        match source.shape.0.len() {
            0 => return,
            1 => {
                // Increase efficiency
                todo!()
            }
            _ => {
                // Change shape
                plant.shape = source.shape.clone();
                plant.time_left = plant.cooldown;
            }
        }
    }

    fn attack(&mut self, weapon: Id, target: Id) {}
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
