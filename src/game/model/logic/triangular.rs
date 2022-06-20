use super::*;

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
    fn sign(p: Vec2<R32>, a: Vec2<R32>, b: Vec2<R32>) -> R32 {
        (p.x - b.x) * (a.y - b.y) - (a.x - b.x) * (p.y - b.y)
    }

    let d1 = sign(pos, a, b);
    let d2 = sign(pos, b, c);
    let d3 = sign(pos, c, a);

    let has_neg = (d1 < R32::ZERO) || (d2 < R32::ZERO) || (d3 < R32::ZERO);
    let has_pos = (d1 > R32::ZERO) || (d2 > R32::ZERO) || (d3 > R32::ZERO);

    !(has_neg && has_pos)
}
