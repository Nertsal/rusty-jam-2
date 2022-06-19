use super::*;

use geng::{Camera2d, Draw2d};
use model::*;

pub type Coord = R32;

pub struct Storage<T>(HashMap<Id, T>);

impl<T> Storage<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, id: Id) -> Option<&T> {
        self.0.get(&id)
    }

    pub fn get_or_default(&mut self, id: Id, default: T) -> &T {
        self.0.entry(id).or_insert(default)
    }

    pub fn set(&mut self, id: Id, value: T) {
        self.0.insert(id, value);
    }
}

pub struct Render {
    geng: Geng,
    assets: Rc<Assets>,
    pub positions: Storage<Vec2<Coord>>,
}

impl Render {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            positions: Storage::new(),
        }
    }

    pub fn draw(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        let camera_center = Vec2::ZERO;
        let camera_height = 50.0;
        let camera = &Camera2d {
            center: vec2(0.0, 0.0),
            rotation: 0.0,
            fov: camera_height,
        };
        let camera_width = camera.fov / framebuffer_size.y * framebuffer_size.x;
        let bounds =
            AABB::point(camera_center).extend_symmetric(vec2(camera_width, camera_height) / 2.0);

        for (index, plant) in model.player_a.farm.plants.iter().enumerate() {
            let bounding_box =
                AABB::points_bounding_box(plant.shape.0.iter().map(|pos| pos.to_cartesian())); // TODO: avoid panick when shape has no points
            let scale = r32(10.0)
                / bounding_box
                    .width()
                    .max(bounding_box.height())
                    .max(r32(1.0));
            let draw_count = ((1.0
                - (plant.time_left as f32 / plant.cooldown as f32) * plant.shape.0.len() as f32)
                .floor() as usize)
                .max(1);
            draw_shape(
                bounds.bottom_left() + vec2(10.0 * (index as f32 + 1.0), 10.0),
                plant.shape.0.iter().take(draw_count),
                scale.as_f32(),
                camera,
                &self.geng,
                framebuffer,
            );
        }

        for shape in &model.player_a.shape_buffer.0 {
            let position = *self
                .positions
                .get_or_default(shape.id, vec2(r32(0.0), r32(0.0)));
            draw_shape(
                position.map(|x| x.as_f32()),
                &shape.shape.0,
                1.0,
                camera,
                &self.geng,
                framebuffer,
            );
        }
    }
}

pub fn draw_shape<'a>(
    offset: Vec2<f32>,
    shape: impl IntoIterator<Item = &'a TriPos>,
    scale: f32,
    camera: &Camera2d,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
) {
    for tri_pos in shape {
        let angle = if tri_pos.is_upside_down() {
            f32::PI
        } else {
            0.0
        };
        let pos = tri_pos.to_cartesian().map(|x| x.as_f32());
        draw_triangle(
            pos * scale + offset,
            angle,
            Color::GREEN,
            scale * 0.7,
            camera,
            geng,
            framebuffer,
        )
    }
}

pub fn draw_triangle(
    pos: Vec2<f32>,
    angle: f32,
    color: Color<f32>,
    side_length: f32,
    camera: &Camera2d,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
) {
    // Calculate relative position for the triangle pointing up
    let dx = side_length * 0.5;
    let dy_low = side_length * 3.0.sqrt() / 6.0;
    let dy_high = dy_low * 2.0;
    let positions = [vec2(dx, -dy_low), vec2(-dx, -dy_low), vec2(0.0, dy_high)]
        .into_iter()
        .map(|position| position.rotate(angle) + pos)
        .collect();
    draw_2d::Polygon::new(positions, color).draw_2d(geng, framebuffer, camera);
}
