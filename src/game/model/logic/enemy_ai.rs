use super::*;

pub fn enemy_ai(model: &Model) -> Vec<PlayerAction> {
    let player = &model.player_b;
    let target_shape =
        [(0, 0), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)].map(|(x, y)| TriPos { x, y });

    let mut actions = Vec::new();
    let mut build = Vec::new();
    for shape in &player.shape_buffer.0 {
        if shape.shape.0.is_empty() {
            continue;
        }
        if shape.shape.0.len() == 1 {
            build.push(shape.id);
        } else {
            actions.push(PlayerAction::ActivateShape(shape.id));
        }
    }

    if player.active_shapes.0.is_empty() {
        if let Some(id) = build.pop() {
            actions.push(PlayerAction::ActivateShape(id));
        }
    }

    for (base_id, base_size) in player
        .active_shapes
        .0
        .iter()
        .map(|shape| (shape.id, shape.shape.0.len()))
        .filter(|(_, len)| *len < 6)
    {
        let id = match build.pop() {
            Some(id) => id,
            None => break,
        };
        let pos = *match target_shape.get(base_size) {
            Some(pos) => pos,
            None => {
                error!(
                    "Failed to find an appropriate position for the build (base_id: {base_id:?}, base_size: {base_size:?}, target_shape: {target_shape:?})"
                );
                break;
            }
        };
        actions.push(PlayerAction::AttachShape {
            triangle: id,
            target: base_id,
            pos,
        });
    }

    actions
}
