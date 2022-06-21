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
            geng::Event::MouseMove { position, .. } => self.mouse_move(model, render, position),
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

    fn mouse_move(
        &mut self,
        model: &Model,
        render: &mut Render,
        position: Vec2<f64>,
    ) -> Vec<PlayerAction> {
        let dragging = match &mut self.dragging {
            Some(d) => d,
            None => return vec![],
        };
        let mouse_world_pos = render.screen_to_world(position);
        match dragging {
            &mut Dragging::Shape(shape_id) => {
                // Move the shape
                let bounds = render
                    .layout
                    .shape_buffer_a
                    .join(&render.layout.active_shapes_a)
                    .join(&render.layout.shape_farm_a);
                let pos = bounds.clamp_point(mouse_world_pos.map(|x| x.as_f32()));
                let actions = {
                    if render.layout.shape_buffer_a.contains(pos) {
                        vec![PlayerAction::DeactivateShape(shape_id)]
                    } else if render.layout.active_shapes_a.contains(pos) {
                        vec![PlayerAction::ActivateShape(shape_id)]
                    } else if render.layout.shape_farm_a.contains(pos) {
                        let mut upgradable_plants = model
                            .player_a
                            .farm
                            .plants
                            .iter()
                            .filter_map(|plant| {
                                render.positions.get(plant.id).and_then(|pos| {
                                    render.scales.get(plant.id).map(|scale| (pos, scale, plant))
                                })
                            })
                            .filter(|(pos, scale, plant)| {
                                plant.shape.contains((mouse_world_pos - **pos) / **scale)
                            })
                            .map(|(_, _, plant)| plant.id);
                        upgradable_plants
                            .next()
                            .map(|plant_id| {
                                vec![PlayerAction::UpgradeFarm {
                                    source_shape: shape_id,
                                    target_plant: plant_id,
                                }]
                            })
                            .unwrap_or(vec![])
                    } else {
                        vec![]
                    }
                };
                let pos = pos.map(r32);

                let mut attachments = model
                    .player_a
                    .active_shapes
                    .0
                    .iter()
                    .filter(|shape| shape.id != shape_id)
                    .filter_map(|shape| {
                        render
                            .positions
                            .get(shape.id)
                            .and_then(|&shape_pos| try_attach(pos, &shape.shape, shape_pos))
                            .map(|pos| (shape.id, pos))
                    });
                if let Some((target_id, attach_pos)) = attachments.next() {
                    return vec![PlayerAction::AttachShape {
                        triangle: shape_id,
                        target: target_id,
                        pos: attach_pos,
                    }];
                }

                let current_pos = match render.positions.get_mut(shape_id) {
                    Some(pos) => pos,
                    None => {
                        self.dragging = None;
                        return vec![];
                    }
                };
                *current_pos = pos;
                actions
            }
        }
    }

    fn mouse_left_up(&mut self) -> Vec<PlayerAction> {
        self.dragging.take();
        vec![]
    }
}

fn try_attach(center: Vec2<R32>, shape: &Shape, shape_pos: Vec2<R32>) -> Option<TriPos> {
    shape
        .boundary()
        .find(|pos| logic::inside_triangle(center, pos.to_vertices().map(|pos| pos + shape_pos)))
}
