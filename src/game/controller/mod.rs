use super::*;

use model::*;

pub struct Controller {
    dragging: Option<Dragging>,
}

pub enum Dragging {
    Shape(Id),
}

impl Controller {
    pub fn new() -> Self {
        Self { dragging: None }
    }

    pub fn handle_event(
        &mut self,
        model: &Model,
        render: &Render,
        event: geng::Event,
    ) -> Option<PlayerAction> {
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::Enter => Some(PlayerAction::EndTurn),
                _ => None,
            },
            geng::Event::MouseDown { position, button } => match button {
                geng::MouseButton::Left => self.mouse_left_down(model, render, position),
                _ => None,
            },
            _ => None,
        }
    }

    fn mouse_left_down(
        &mut self,
        model: &Model,
        render: &Render,
        position: Vec2<f64>,
    ) -> Option<PlayerAction> {
        let mouse_world_pos = render
            .camera
            .screen_to_world(render.framebuffer_size, position.map(|x| x as _))
            .map(|x| r32(x));
        for shape in &model.player_a.shape_buffer.0 {
            if let Some(&shape_pos) = render.positions.get(shape.id) {
                if shape.shape.contains(mouse_world_pos - shape_pos) {
                    self.dragging = Some(Dragging::Shape(shape.id));
                    return Some(PlayerAction::GrabShape(shape.id));
                }
            }
        }
        None
    }
}
