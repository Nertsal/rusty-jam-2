use super::*;

mod triangular;

pub use triangular::*;

impl Model {
    pub fn update(&mut self, _delta_time: Time) {}

    pub fn tick(&mut self) {
        for plant in &mut self.player_a.farm.plants {
            if plant.tick() {
                self.player_a.shape_buffer.0.push(AliveShape {
                    id: self.id_gen.next(),
                    shape: plant.shape.clone(),
                });
            }
        }
    }

    pub fn handle_player_action(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::GrabShape(_) => todo!(),
            PlayerAction::ReleaseGrabbed => todo!(),
            PlayerAction::EndTurn => self.tick(),
        }
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
