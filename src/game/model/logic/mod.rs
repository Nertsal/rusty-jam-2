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
        let mut attach_impl = || -> Option<()> {
            let _triangle = self
                .player_a
                .active_shapes
                .0
                .get(&triangle)
                .filter(|triangle| triangle.shape.0.len() == 1)?;
            let target = self
                .player_a
                .active_shapes
                .0
                .get_mut(&target)
                .filter(|target| target.shape.boundary().contains(&pos))?;

            target.shape.0.push(pos);
            self.player_a
                .active_shapes
                .0
                .remove(&triangle)
                .expect("Attached triangle disappeared");
            Some(())
        };
        attach_impl();
    }

    fn upgrade_plant(&mut self, source_shape: Id, target_plant: Id) {
        let mut upgrade_impl = || -> Option<()> {
            let source = self.player_a.remove_shape(source_shape)?;
            let plant = self.player_a.shape_farm.plants.get_mut(&target_plant)?;

            match source.0.len() {
                0 => return None,
                1 => {
                    // Increase efficiency
                    todo!()
                }
                _ => {
                    // Change shape
                    plant.shape = source;
                    plant.time_left = plant.cooldown;
                }
            }
            Some(())
        };
        upgrade_impl();
    }

    fn attack(&mut self, weapon: Id, target: Id) {}
}

impl Player {
    fn remove_shape(&mut self, id: Id) -> Option<Shape> {
        self.shape_buffer
            .0
            .remove(&id)
            .or_else(|| self.active_shapes.0.remove(&id))
            .map(|shape| shape.shape)
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
