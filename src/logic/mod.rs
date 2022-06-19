use super::*;

use model::*;

impl Model {
    pub fn update(&mut self, _delta_time: Time) {}

    pub fn next_turn(&mut self) {
        for plant in &mut self.player_a.farm.plants {
            if plant.tick() {
                self.player_a.shape_buffer.0.push(AliveShape {
                    id: self.id_gen.next(),
                    shape: plant.shape.clone(),
                });
            }
        }
    }
}
