use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Area(pub AABB<f32>);

#[derive(Debug, Clone)]
pub struct RelativeLayout(Layout);

#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub shape_buffer_a: Area,
    pub active_shapes_a: Area,
    pub shape_buffer_b: Area,
    pub active_shapes_b: Area,
}

impl RelativeLayout {
    pub fn new() -> Self {
        // Relative values
        const DANGER_ZONE: f32 = 0.2;
        Self(Layout {
            shape_buffer_a: Area::new((0.05, 0.3), (0.2, 0.7)),
            active_shapes_a: Area::new((0.5 - DANGER_ZONE, 0.3), (0.45, 0.7)),
            shape_buffer_b: Area::new((0.8, 0.3), (0.95, 0.7)),
            active_shapes_b: Area::new((0.55, 0.3), (0.5 + DANGER_ZONE, 0.7)),
        })
    }

    pub fn adapt(&self, target: AABB<f32>) -> Layout {
        let layout = &self.0;
        let adapt = |area: Area| area.adapt(target);
        Layout {
            shape_buffer_a: adapt(layout.shape_buffer_a),
            active_shapes_a: adapt(layout.active_shapes_a),
            shape_buffer_b: adapt(layout.shape_buffer_b),
            active_shapes_b: adapt(layout.active_shapes_b),
        }
    }
}

impl Area {
    pub fn new((x_min, y_min): (f32, f32), (x_max, y_max): (f32, f32)) -> Self {
        Self(AABB {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }

    pub fn adapt(self, target: AABB<f32>) -> Self {
        Self(AABB {
            x_min: self.0.x_min * target.width() + target.x_min,
            x_max: self.0.x_max * target.width() + target.x_min,
            y_min: self.0.y_min * target.height() + target.y_min,
            y_max: self.0.y_max * target.height() + target.y_min,
        })
    }

    pub fn point(&self, x: f32, y: f32) -> Vec2<f32> {
        vec2(self.x(x), self.y(y))
    }

    pub fn x(&self, x: f32) -> f32 {
        x * self.0.width() + self.0.x_min
    }

    pub fn y(&self, y: f32) -> f32 {
        y * self.0.height() + self.0.y_min
    }

    pub fn join(&self, other: &Self) -> Self {
        Self(AABB {
            x_min: self.0.x_min.min(other.0.x_min),
            x_max: self.0.x_max.max(other.0.x_max),
            y_min: self.0.y_min.min(other.0.y_min),
            y_max: self.0.y_max.max(other.0.y_max),
        })
    }
}
