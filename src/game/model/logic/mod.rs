use super::*;

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

    pub fn to_vertices(&self) -> [Vec2<R32>; 3] {
        // Calculate position for the triangle pointing up
        let side_length = r32(1.0);
        let dx = side_length * r32(0.5);
        let dy_low = side_length * r32(3.0).sqrt() / r32(6.0);
        let dy_high = dy_low * r32(2.0);

        let angle = if self.is_upside_down() {
            // Rotate upside down
            R32::PI
        } else {
            R32::ZERO
        };
        let pos = self.to_cartesian();
        [
            vec2(dx, -dy_low),
            vec2(-dx, -dy_low),
            vec2(R32::ZERO, dy_high),
        ]
        .map(|p| p.rotate(angle) + pos)
    }
}

impl Shape {
    pub fn contains(&self, pos: Vec2<R32>) -> bool {
        self.0
            .iter()
            .any(|tri_pos| inside_triangle(pos, tri_pos.to_vertices()))
    }
}

pub fn inside_triangle(pos: Vec2<R32>, [a, b, c]: [Vec2<R32>; 3]) -> bool {
    todo!()
}
