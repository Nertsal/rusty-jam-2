use super::*;

use model::*;

pub struct Controller {
    state: State,
}

#[derive(Debug)]
enum State {
    Idle,
    DraggingShape { shape_id: Id, start_pos: Vec2<f64> },
    SelectingAttackTarget { weapon_id: Id },
}

struct Context<'a> {
    model: &'a Model,
    render: &'a mut Render,
    event: geng::Event,
}

impl Controller {
    pub fn new() -> Self {
        Self { state: State::Idle }
    }

    pub fn handle_event(
        &mut self,
        model: &Model,
        render: &mut Render,
        event: geng::Event,
    ) -> Vec<PlayerAction> {
        let context = Context {
            model,
            render,
            event,
        };
        let state = std::mem::replace(&mut self.state, State::Idle);
        let (new_state, actions) = state.handle_event(context);
        self.state = new_state;
        actions
    }
}

impl State {
    pub fn handle_event<'a>(self, context: Context<'a>) -> (Self, Vec<PlayerAction>) {
        match self {
            Self::Idle => handle_idle(context),
            Self::DraggingShape {
                shape_id,
                start_pos,
            } => handle_drag_shape(shape_id, start_pos, context),
            Self::SelectingAttackTarget { weapon_id } => {
                handle_select_attack_target(weapon_id, context)
            }
        }
    }
}

fn handle_idle<'a>(ctx: Context<'a>) -> (State, Vec<PlayerAction>) {
    match ctx.event {
        geng::Event::KeyDown { key } => match key {
            geng::Key::Enter => (State::Idle, vec![PlayerAction::EndTurn]),
            _ => (State::Idle, vec![]),
        },
        geng::Event::MouseDown {
            position,
            button: geng::MouseButton::Left,
        } => {
            let mouse_world_pos = ctx.render.screen_to_world(position);
            for shape in ctx
                .model
                .player_a
                .shape_buffer
                .0
                .iter()
                .chain(&ctx.model.player_a.active_shapes.0)
            {
                if let Some(&shape_pos) = ctx.render.positions.get(shape.id) {
                    if shape.shape.contains(mouse_world_pos - shape_pos) {
                        return (
                            State::DraggingShape {
                                shape_id: shape.id,
                                start_pos: position,
                            },
                            vec![],
                        );
                    }
                }
            }
            (State::Idle, vec![])
        }
        _ => (State::Idle, vec![]),
    }
}

fn handle_drag_shape<'a>(
    shape_id: Id,
    start_pos: Vec2<f64>,
    ctx: Context<'a>,
) -> (State, Vec<PlayerAction>) {
    match ctx.event {
        geng::Event::MouseUp {
            button: geng::MouseButton::Left,
            position,
        } => {
            if start_pos == position {
                (
                    State::SelectingAttackTarget {
                        weapon_id: shape_id,
                    },
                    vec![],
                )
            } else {
                (State::Idle, vec![])
            }
        }
        geng::Event::MouseMove { position, .. } => {
            let mouse_world_pos = ctx.render.screen_to_world(position);
            // Move the shape
            let bounds = ctx
                .render
                .layout
                .shape_buffer_a
                .join(&ctx.render.layout.active_shapes_a)
                .join(&ctx.render.layout.shape_farm_a);
            let pos = bounds.clamp_point(mouse_world_pos.map(|x| x.as_f32()));
            let actions = {
                if ctx.render.layout.shape_buffer_a.contains(pos) {
                    vec![PlayerAction::DeactivateShape(shape_id)]
                } else if ctx.render.layout.active_shapes_a.contains(pos) {
                    vec![PlayerAction::ActivateShape(shape_id)]
                } else if ctx.render.layout.shape_farm_a.contains(pos) {
                    let mut upgradable_plants = ctx
                        .model
                        .player_a
                        .shape_farm
                        .plants
                        .iter()
                        .filter_map(|plant| {
                            ctx.render.positions.get(plant.id).and_then(|pos| {
                                ctx.render
                                    .scales
                                    .get(plant.id)
                                    .map(|scale| (pos, scale, plant))
                            })
                        })
                        .filter(|(pos, scale, plant)| {
                            plant.shape.contains((mouse_world_pos - **pos) / **scale)
                        })
                        .map(|(_, _, plant)| plant.id);
                    upgradable_plants
                        .next()
                        .map(|plant_id| {
                            vec![PlayerAction::UpgradePlant {
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

            let mut attachments = ctx
                .model
                .player_a
                .active_shapes
                .0
                .iter()
                .filter(|shape| shape.id != shape_id)
                .filter_map(|shape| {
                    ctx.render
                        .positions
                        .get(shape.id)
                        .and_then(|&shape_pos| try_attach(pos, &shape.shape, shape_pos))
                        .map(|pos| (shape.id, pos))
                });
            if let Some((target_id, attach_pos)) = attachments.next() {
                return (
                    State::DraggingShape {
                        shape_id,
                        start_pos,
                    },
                    vec![PlayerAction::AttachShape {
                        triangle: shape_id,
                        target: target_id,
                        pos: attach_pos,
                    }],
                );
            }

            let current_pos = match ctx.render.positions.get_mut(shape_id) {
                Some(pos) => pos,
                None => {
                    return (State::Idle, vec![]);
                }
            };
            *current_pos = pos;
            (
                State::DraggingShape {
                    shape_id,
                    start_pos,
                },
                actions,
            )
        }
        _ => (
            State::DraggingShape {
                shape_id,
                start_pos,
            },
            vec![],
        ),
    }
}

fn handle_select_attack_target<'a>(weapon_id: Id, ctx: Context<'a>) -> (State, Vec<PlayerAction>) {
    match ctx.event {
        geng::Event::MouseDown {
            position,
            button: geng::MouseButton::Left,
        } => {
            let mouse_world_pos = ctx.render.screen_to_world(position);
            for (id, scale, pos, shape) in ctx
                .model
                .player_b
                .active_shapes
                .0
                .iter()
                .filter_map(|shape| {
                    ctx.render
                        .positions
                        .get(shape.id)
                        .map(|pos| (shape.id, r32(1.0), *pos, &shape.shape))
                })
                .chain(
                    ctx.model
                        .player_b
                        .shape_farm
                        .plants
                        .iter()
                        .filter_map(|plant| {
                            ctx.render.positions.get(plant.id).and_then(|pos| {
                                ctx.render
                                    .scales
                                    .get(plant.id)
                                    .map(|scale| (plant.id, *scale, *pos, &plant.shape))
                            })
                        }),
                )
            {
                if shape.contains((mouse_world_pos - pos) / scale) {
                    return (
                        State::Idle,
                        vec![PlayerAction::Attack {
                            weapon: weapon_id,
                            target: id,
                        }],
                    );
                }
            }
            (State::Idle, vec![])
        }
        _ => (State::SelectingAttackTarget { weapon_id }, vec![]),
    }
}

fn try_attach(center: Vec2<R32>, shape: &Shape, shape_pos: Vec2<R32>) -> Option<TriPos> {
    shape
        .boundary()
        .find(|pos| logic::inside_triangle(center, pos.to_vertices().map(|pos| pos + shape_pos)))
}
