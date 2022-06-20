use super::*;

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

    pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        self.0.get_mut(&id)
    }

    pub fn get_or_default(&mut self, id: Id, default: T) -> &T {
        self.0.entry(id).or_insert(default)
    }
}

pub struct Render {
    geng: Geng,
    assets: Rc<Assets>,
    pub positions: Storage<Vec2<Coord>>,
    pub camera: Camera2d,
    pub framebuffer_size: Vec2<f32>,
}

impl Render {
    pub fn screen_to_world(&self, screen_pos: Vec2<f64>) -> Vec2<R32> {
        self.camera
            .screen_to_world(self.framebuffer_size, screen_pos.map(|x| x as _))
            .map(|x| r32(x))
    }

    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            positions: Storage::new(),
            camera: Camera2d {
                center: vec2(0.0, 0.0),
                rotation: 0.0,
                fov: 20.0,
            },
            framebuffer_size: vec2(1.0, 1.0),
        }
    }

    pub fn draw(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        self.framebuffer_size = framebuffer_size;
        let camera_center = Vec2::ZERO;
        let camera_width = self.camera.fov / framebuffer_size.y * framebuffer_size.x;
        let bounds =
            AABB::point(camera_center).extend_symmetric(vec2(camera_width, self.camera.fov) / 2.0);

        let relative_v2 = |x, y| vec2(x, y) * bounds.size() + bounds.bottom_left();

        const DANGER_ZONE: f32 = 0.2;
        draw_2d::Segment::new(
            Segment::new(
                relative_v2(0.5 - DANGER_ZONE, 0.0),
                relative_v2(0.5 - DANGER_ZONE, 1.0),
            ),
            0.01 * bounds.width(),
            Color::GRAY,
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);
        draw_2d::Segment::new(
            Segment::new(relative_v2(0.5, 0.0), relative_v2(0.5, 1.0)),
            0.02 * bounds.width(),
            Color::rgb(0.2, 0.2, 0.2),
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);
        draw_2d::Segment::new(
            Segment::new(
                relative_v2(0.5 + DANGER_ZONE, 0.0),
                relative_v2(0.5 + DANGER_ZONE, 1.0),
            ),
            0.01 * bounds.width(),
            Color::GRAY,
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);

        draw_farm(
            &model.player_a.farm,
            bounds,
            &self.camera,
            &self.geng,
            framebuffer,
        );

        let buffer_bounds = AABB::from_corners(
            vec2(0.05, 0.3) * bounds.size(),
            vec2(0.2, 0.7) * bounds.size(),
        )
        .translate(bounds.bottom_left());
        for shape in &model.player_a.shape_buffer.0 {
            let random_pos = vec2(
                global_rng().gen_range(buffer_bounds.x_min..=buffer_bounds.x_max),
                global_rng().gen_range(buffer_bounds.y_min..=buffer_bounds.y_max),
            )
            .map(|x| r32(x));
            let position = *self.positions.get_or_default(shape.id, random_pos);
            draw_shape(
                position.map(|x| x.as_f32()),
                &shape.shape.0,
                1.0,
                &self.camera,
                &self.geng,
                framebuffer,
            );
        }

        let active_bounds =
            AABB::from_corners(relative_v2(0.5 - DANGER_ZONE, 0.3), relative_v2(0.45, 0.7));
        for shape in &model.player_a.active_shapes.0 {
            let random_pos = vec2(
                global_rng().gen_range(active_bounds.x_min..=active_bounds.x_max),
                global_rng().gen_range(active_bounds.y_min..=active_bounds.y_max),
            )
            .map(|x| r32(x));
            let position = *self.positions.get_or_default(shape.id, random_pos);
            draw_shape(
                position.map(|x| x.as_f32()),
                &shape.shape.0,
                1.0,
                &self.camera,
                &self.geng,
                framebuffer,
            );
        }
    }
}

pub fn draw_farm(
    farm: &ShapeFarm,
    bounds: AABB<f32>,
    camera: &Camera2d,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
) {
    for (index, plant) in farm.plants.iter().enumerate() {
        let bounding_box =
            AABB::points_bounding_box(plant.shape.0.iter().map(|pos| pos.to_cartesian())); // TODO: avoid panick when shape has no points
        let scale = r32(1.0)
            / bounding_box
                .width()
                .max(bounding_box.height())
                .max(r32(1.0));
        let draw_count = ((1.0
            - (plant.time_left as f32 / plant.cooldown as f32) * plant.shape.0.len() as f32)
            .floor() as usize)
            .max(1);
        draw_shape(
            bounds.bottom_left() + vec2(0.1 * (index as f32 + 1.0), 0.1) * bounds.size(),
            plant.shape.0.iter().take(draw_count),
            scale.as_f32(),
            camera,
            geng,
            framebuffer,
        );
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
        let vertices = tri_pos
            .to_vertices()
            .into_iter()
            .map(|pos| pos.map(|x| x.as_f32()) * scale + offset)
            .collect();
        draw_2d::Polygon::new(vertices, Color::GREEN).draw_2d(geng, framebuffer, camera);
    }
}