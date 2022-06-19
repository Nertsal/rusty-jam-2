use super::*;

use geng::{Camera2d, Draw2d};
use model::*;

pub struct Render {
    geng: Geng,
    assets: Rc<Assets>,
}

impl Render {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
        }
    }

    pub fn draw(&mut self, _model: &Model, framebuffer: &mut ugli::Framebuffer) {
        let shape = Shape(vec![
            TriPos { x: 0, y: 0 },
            TriPos { x: 1, y: 0 },
            TriPos { x: 0, y: 1 },
            TriPos { x: -1, y: 0 },
            TriPos { x: 0, y: -1 },
            TriPos { x: 1, y: 1 },
            TriPos { x: -1, y: 1 },
            TriPos { x: 1, y: -1 },
            TriPos { x: -1, y: -1 },
        ]);
        let camera = &Camera2d {
            center: vec2(0.0, 0.0),
            rotation: 0.0,
            fov: 10.0,
        };
        self.draw_shape(&shape, camera, framebuffer);
    }

    pub fn draw_shape(
        &self,
        shape: &Shape,
        camera: &Camera2d,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        for tri_pos in &shape.0 {
            let angle = if tri_pos.is_upside_down() {
                f32::PI
            } else {
                0.0
            };
            let pos = tri_pos.to_cartesian().map(|x| x.as_f32());
            println!("Triangle at {tri_pos:?} (cartesian: {pos:?}) at angle {angle}");
            self.draw_triangle(pos, angle, Color::GREEN, 0.7, camera, framebuffer)
        }
    }

    pub fn draw_triangle(
        &self,
        pos: Vec2<f32>,
        angle: f32,
        color: Color<f32>,
        side_length: f32,
        camera: &Camera2d,
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
        draw_2d::Polygon::new(dbg!(positions), color).draw_2d(&self.geng, framebuffer, camera);
    }
}
