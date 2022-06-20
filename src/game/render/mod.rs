use super::*;

use model::*;

mod layout;

use layout::*;

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
    relative_layout: RelativeLayout,
    pub layout: Layout,
    pub positions: Storage<Vec2<Coord>>,
    camera: Camera2d,
    framebuffer_size: Vec2<f32>,
}

impl Render {
    pub fn screen_to_world(&self, screen_pos: Vec2<f64>) -> Vec2<R32> {
        self.camera
            .screen_to_world(self.framebuffer_size, screen_pos.map(|x| x as _))
            .map(r32)
    }

    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            relative_layout: RelativeLayout::new(),
            layout: RelativeLayout::new().adapt(AABB::ZERO.extend_uniform(1.0)),
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

        let bounds = AABB::point(self.camera.center).extend_symmetric(
            vec2(
                self.camera.fov / framebuffer_size.y * framebuffer_size.x,
                self.camera.fov,
            ) / 2.0,
        );

        self.layout = self.relative_layout.adapt(bounds);
        let layout = &self.layout;

        let active_shapes = layout.active_shapes_a.join(&layout.active_shapes_b);
        draw_2d::Segment::new(
            Segment::new(active_shapes.point(0.0, 0.0), active_shapes.point(0.0, 1.0)),
            0.01 * bounds.width(),
            Color::GRAY,
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);
        draw_2d::Segment::new(
            Segment::new(active_shapes.point(0.5, 0.0), active_shapes.point(0.5, 1.0)),
            0.02 * bounds.width(),
            Color::rgb(0.2, 0.2, 0.2),
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);
        draw_2d::Segment::new(
            Segment::new(active_shapes.point(1.0, 0.0), active_shapes.point(1.0, 1.0)),
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

        for shape in &model.player_a.shape_buffer.0 {
            let random_pos = random_point_in(layout.shape_buffer_a.0).map(r32);
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

        for shape in &model.player_a.active_shapes.0 {
            let random_pos = random_point_in(layout.active_shapes_a.0).map(r32);
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

pub fn random_point_in(aabb: AABB<f32>) -> Vec2<f32> {
    vec2(
        global_rng().gen_range(aabb.x_min..=aabb.x_max),
        global_rng().gen_range(aabb.y_min..=aabb.y_max),
    )
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
            AABB::points_bounding_box(plant.shape.0.iter().map(|pos| pos.to_cartesian())); // TODO: avoid panic when shape has no points
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
