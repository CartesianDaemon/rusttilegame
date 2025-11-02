// TODO: Add these types to a Script struct?
use crate::engine::for_scripting::*;

#[derive(Clone, Debug)]
pub struct PushpuzzScripts {
}

impl BaseScripts for PushpuzzScripts {
    fn move_mov(field: &mut Map, mov: RosterIndex, cmd: Cmd) -> SceneContinuation {
        move_mov(field, mov, cmd)
    }
}

impl PushpuzzScripts {
}

pub fn passable(field: &Map, pos: MapCoord) -> bool {
    field.all_pass(pos, Pass::Empty)
}

pub fn impassable(field: &Map, pos: MapCoord) -> bool {
    !passable(field, pos)
}

// Would be nice for these to be a function of an enum/trait impls
pub fn move_mov(field: &mut Map, mov: RosterIndex, cmd: Cmd) -> SceneContinuation {
    let hero = field.hero();
    match field[mov].props.ai {
        AI::Stay => {
            // Do nothing
        },
        AI::Hero => {
            if cmd != Cmd::Stay {
                let target_pos = field[mov].pos() + cmd.as_dir();
                if passable(field, target_pos) {
                    field.move_obj_to(mov, target_pos);
                }
            }
            // TODO: Avoid needing to re-get the hero handle, make move function consume or update the rich_mov handle.
            return if field.any_effect(field[mov].pos(), Effect::Win) {
                SceneContinuation::Break(SceneEnding::PlayWin)
            } else {
                SceneContinuation::Continue(())
            }
            // TODO: Also check if hero died? Usually superfluous if we don't allow moving into death.
        }
        AI::Bounce => {
            // TODO: Simplify duplication in field.obj_at(rich_mov.ros_idx) throughout?

            // If moving would hit a wall, first reverse direction.
            // TODO: Consider adding "can_move" function. Map would need movement_logic "passable(obj, tile)" dependency injection.
            // TODO: Consider adding field.try_move() fn.
            // TODO: Consider adding map_coord *= -1.
            let target_pos = field.obj_target_pos(mov);
            if impassable(field, target_pos) {
                field[mov].props.dir.reverse();
            }

            // Move. Provided next space is passable. If both sides are impassable, don't move.
            // TODO: Consider adding field.obj_try_move() function?
            let target_pos = field[mov].pos() + field[mov].props.dir;
            if passable(field, target_pos) {
                field.move_obj_to(mov, target_pos);
            }

            // Hero dies if mov moves onto hero
            // TODO: Check at end of function? Or as part of obj?
            if field[mov].props.effect == Effect::Kill && field[mov].pos() == field[hero].pos() {
                return SceneContinuation::Break(SceneEnding::PlayDie);
            }
        },
        AI::Drift => {
            // TODO: Deal with collisions between movs

            let mut drift_dir = CoordDelta::from_xy(0, 0);

            // If hitting wall, reverse direction.
            if impassable(field, field.obj_target_pos(mov)) {
                field[mov].props.dir.reverse();

                // And if hero "visible" forward or sideways, move one sideways towards them, if passable.
                // TODO: Check for obstacles to vision.
                let hero_dir = field[mov].pos().dir_to(field[hero].pos());
                if field[mov].props.dir.dx == 0 {
                    if hero_dir.dy != -field[mov].props.dir.dy {
                        drift_dir = CoordDelta::from_xy(hero_dir.dx, 0);
                    }
                } else if field[mov].props.dir.dy == 0 {
                    if hero_dir.dx != -field[mov].props.dir.dx {
                        drift_dir = CoordDelta::from_xy(0, hero_dir.dy);
                    }
                } else {
                    panic!("AI::Drift only implemented for orthogal movement");
                }
            }

            // Move. Provided next space is passable. If both sides are impassable, don't move.
            // TODO: Animation for turning? At least avoiding wall?
            let delta = field[mov].props.dir + drift_dir;
            if passable(field, field[mov].pos() + delta) {
                field.move_obj_to(mov, field[mov].pos() + delta);
            }

            // Hero dies if mov moves onto hero
            if field[mov].props.effect == Effect::Kill && field[mov].pos() == field[hero].pos() {
                return SceneContinuation::Break(SceneEnding::PlayDie);
            }
        },
        AI::Scuttle => {
            // If hitting wall, choose new direction.
            if impassable(field, field.obj_target_pos(mov)) {
                let hero_dir = field[mov].pos().dir_to(field[hero].pos());
                let hero_delta = field[mov].pos().delta_to(field[hero].pos());
                // Find whether x or y is more towards the hero
                let x_longer_than_y = match hero_delta.dx.abs() - hero_delta.dy.abs() {
                    num if num > 0 => true,
                    num if num < 0 => false,
                    _ => field[mov].props.dir.dy.abs() < field[mov].props.dir.dy.abs(),
                };
                // dlongcoord is the orthogonal direction most towards the hero. dshortcoord is the other best.
                let (dlongcoord, dshortcoord) = if x_longer_than_y {
                    (CoordDelta::from_xy(hero_dir.dx, 0), CoordDelta::from_xy(0, hero_dir.dy))
                } else {
                    (CoordDelta::from_xy(0, hero_dir.dy), CoordDelta::from_xy(hero_dir.dx, 0))
                };
                // Prefer the directions "most" towards the hero first
                let try_dirs = vec![dlongcoord, dshortcoord, -dshortcoord, -dlongcoord];
                // Try each direction in turn, use the first passable one.
                // Can't be the same as original direction because that was impassable.
                // If none are passable, stay in the same direction we started.
                if let Some(dir) = try_dirs.iter().find(|dir|
                    passable(field, field[mov].pos() + **dir)
                ) {
                    field[mov].props.dir = *dir;
                }
            }

            // Move. Provided next space is passable. If all sides were impassable, don't move.
            if passable(field, field.obj_target_pos(mov)) {
                field.move_obj_to(mov, field[mov].pos() + field[mov].props.dir);
            }

            // Hero dies if bot moves onto hero
            if field[mov].props.effect == Effect::Kill && field[mov].pos() == field[hero].pos() {
                return SceneContinuation::Break(SceneEnding::PlayDie);
            }
        },
    }
    return SceneContinuation::Continue(());
}
