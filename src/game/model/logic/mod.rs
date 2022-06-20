use super::*;

mod triangular;

pub use triangular::*;

impl Model {
    pub fn update(&mut self, _delta_time: Time) {}

    pub fn handle_player_action(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::EndTurn => self.tick(),
            PlayerAction::ActivateShape(shape_id) => self.activate_shape(shape_id),
            PlayerAction::DeactivateShape(shape_id) => self.deactivate_shape(shape_id),
        }
    }

    fn tick(&mut self) {
        for plant in &mut self.player_a.farm.plants {
            if plant.tick() {
                self.player_a.shape_buffer.0.insert(AliveShape {
                    id: self.id_gen.next(),
                    shape: plant.shape.clone(),
                });
            }
        }
    }

    fn activate_shape(&mut self, shape_id: Id) {}

    fn deactivate_shape(&mut self, shape_id: Id) {}
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
