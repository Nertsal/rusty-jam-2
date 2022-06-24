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

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.values_mut()
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

    pub fn insert(&mut self, id: Id, value: T) {
        self.0.insert(id, value);
    }
}

pub struct Render {
    geng: Geng,
    assets: Rc<Assets>,
    relative_layout: RelativeLayout,
    pub layout: Layout,
    pub positions: Storage<Vec2<Coord>>,
    pub scales: Storage<Coord>,
    camera: Camera2d,
    framebuffer_size: Vec2<f32>,
}

impl Render {
    pub fn screen_to_world(&self, screen_pos: Vec2<f64>) -> Vec2<Coord> {
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
            scales: Storage::new(),
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
        if self.framebuffer_size != framebuffer_size {
            self.positions
                .iter_mut()
                .for_each(|pos| *pos *= (framebuffer_size / self.framebuffer_size).map(r32));
        }
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

        let plants_a = model
            .player_a
            .shape_farm
            .plants
            .iter()
            .map(|plant| (plant, Color::BLUE, layout.shape_farm_a.0));
        let plants_b = model
            .player_b
            .shape_farm
            .plants
            .iter()
            .map(|plant| (plant, Color::RED, layout.shape_farm_b.0));
        let plants = plants_a.chain(plants_b).map(|(plant, color, layout)| {
            let random_pos = random_point_in(layout).map(r32);
            let position = *self.positions.get_or_default(plant.id, random_pos);
            let bounding_box =
                AABB::points_bounding_box(plant.shape.0.iter().map(|pos| pos.to_cartesian())); // TODO: avoid panic when shape has no points
            let scale = r32(1.0)
                / bounding_box
                    .width()
                    .max(bounding_box.height())
                    .max(r32(1.0));
            self.scales.insert(plant.id, scale);
            let draw_count = (((1.0 - plant.time_left as f32 / plant.cooldown as f32)
                * plant.shape.0.len() as f32)
                .ceil() as usize)
                .max(1);
            (
                position.map(|x| x.as_f32()),
                plant.shape.0.iter().take(draw_count),
                scale.as_f32(),
                color,
            )
        });

        draw_shapes(plants, &self.camera, &self.geng, framebuffer);

        let buffer_a = model
            .player_a
            .shape_buffer
            .0
            .iter()
            .map(|shape| (shape, Color::GRAY, layout.shape_buffer_a.0));
        let buffer_b = model
            .player_b
            .shape_buffer
            .0
            .iter()
            .map(|shape| (shape, Color::GRAY, layout.shape_buffer_b.0));
        let active_a = model
            .player_a
            .active_shapes
            .0
            .iter()
            .map(|shape| (shape, Color::BLUE, layout.active_shapes_a.0));
        let active_b = model
            .player_b
            .active_shapes
            .0
            .iter()
            .map(|shape| (shape, Color::RED, layout.active_shapes_b.0));
        let shapes = buffer_a
            .chain(buffer_b)
            .chain(active_a)
            .chain(active_b)
            .map(|(shape, color, layout)| {
                let random_pos = random_point_in(layout).map(r32);
                let position = *self.positions.get_or_default(shape.id, random_pos);
                (position.map(|x| x.as_f32()), &shape.shape.0, 1.0, color)
            });

        draw_shapes(shapes, &self.camera, &self.geng, framebuffer);
    }
}

pub fn draw_shapes<'a>(
    shapes: impl IntoIterator<
        Item = (
            Vec2<f32>,
            impl IntoIterator<Item = &'a TriPos>,
            f32,
            Color<f32>,
        ),
    >,
    camera: &'a Camera2d,
    geng: &'a Geng,
    framebuffer: &'a mut ugli::Framebuffer,
) {
    for (pos, shape, scale, color) in shapes {
        draw_shape(pos, shape, scale, color, camera, geng, framebuffer);
    }
}

pub fn random_point_in(aabb: AABB<f32>) -> Vec2<f32> {
    vec2(
        global_rng().gen_range(aabb.x_min..=aabb.x_max),
        global_rng().gen_range(aabb.y_min..=aabb.y_max),
    )
}

pub fn draw_shape<'a>(
    offset: Vec2<f32>,
    shape: impl IntoIterator<Item = &'a TriPos>,
    scale: f32,
    color: Color<f32>,
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
        draw_2d::Polygon::new(vertices, color).draw_2d(geng, framebuffer, camera);
    }
}
