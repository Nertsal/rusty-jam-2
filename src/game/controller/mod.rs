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
        render: &mut Render,
        event: geng::Event,
    ) -> Vec<PlayerAction> {
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::Enter => vec![PlayerAction::EndTurn],
                _ => vec![],
            },
            geng::Event::MouseDown { position, button } => match button {
                geng::MouseButton::Left => self.mouse_left_down(model, render, position),
                _ => vec![],
            },
            geng::Event::MouseMove { position, .. } => self.mouse_move(render, position),
            geng::Event::MouseUp { button, .. } => match button {
                geng::MouseButton::Left => self.mouse_left_up(),
                _ => vec![],
            },
            _ => vec![],
        }
    }

    fn mouse_left_down(
        &mut self,
        model: &Model,
        render: &mut Render,
        position: Vec2<f64>,
    ) -> Vec<PlayerAction> {
        let mouse_world_pos = render.screen_to_world(position);
        for shape in &model.player_a.shape_buffer.0 {
            if let Some(&shape_pos) = render.positions.get(shape.id) {
                if shape.shape.contains(mouse_world_pos - shape_pos) {
                    self.dragging = Some(Dragging::Shape(shape.id));
                    return vec![];
                }
            }
        }
        vec![]
    }

    fn mouse_move(&mut self, render: &mut Render, position: Vec2<f64>) -> Vec<PlayerAction> {
        let dragging = match &mut self.dragging {
            Some(d) => d,
            None => return vec![],
        };
        let mouse_world_pos = render.screen_to_world(position);
        match dragging {
            &mut Dragging::Shape(shape_id) => {
                // Move the shape
                let current_pos = match render.positions.get_mut(shape_id) {
                    Some(pos) => pos,
                    None => {
                        self.dragging = None;
                        return vec![];
                    }
                };
                // let (pos, area) = render.clamp_shape_pos(mouse_world_pos);
                // *current_pos = pos;
                // match area {

                // }
                vec![]
            }
        }
    }

    fn mouse_left_up(&mut self) -> Vec<PlayerAction> {
        self.dragging.take();
        vec![]
    }
}
