// TODO: Add these types to a Script struct?
use crate::engine::scripting::*;
use super::obj_types::*;

pub fn passable(field: &Field, pos: MapCoord) -> bool {
    field.all_pass(pos, Pass::Empty)
}

pub fn impassable(field: &Field, pos: MapCoord) -> bool {
    !passable(field, pos)
}

pub fn move_mov(field: &mut Field, hdl: RosterHandle, cmd: Cmd) -> SceneEnding {
    match field.obj(hdl).ai {
        AI::Stay => {
            // Do nothing
        },
        AI::Hero => {
            if cmd != Cmd::Stay {
                let target_pos = field.obj_pos(hdl) + cmd.as_dir();
                if passable(field, target_pos) {
                    field.move_obj_to(hdl, target_pos);
                }
            }
            // TODO: Avoid needing to re-get the hero handle, make move function consume or update the rich_mov handle.
            return if field.any_effect(field.hero_pos(), Effect::Win) {
                SceneEnding::NextScene(Continuation::PlayWin)
            } else {
                SceneEnding::ContinuePlaying
            }
            // TODO: Also check if hero died? Usually superfluous if we don't allow moving into death.
        }
        AI::Bounce => {
            // TODO: Simplify duplication in field.obj_at(rich_mov.ros_idx) throughout?

            // If moving would hit a wall, first reverse direction.
            // TODO: Consider adding "can_move" function. Field would need movement_logic "passable(obj, tile)" dependency injection.
            // TODO: Consider adding field.try_move() fn.
            // TODO: Consider adding map_coord *= -1.
            let target_pos = field.obj_target_pos(hdl);
            if impassable(field, target_pos) {
                field.objm(hdl).dir.reverse();
            }

            // Move. Provided next space is passable. If both sides are impassable, don't move.
            // TODO: Consider adding field.obj_try_move() function?
            let target_pos = field.obj_pos(hdl) + field.obj(hdl).dir;
            if passable(field, target_pos) {
                field.move_obj_to(hdl, target_pos);
            }

            // Hero dies if mov moves onto hero
            // TODO: Check at end of function? Or as part of obj?
            if field.obj(hdl).effect == Effect::Kill && field.obj_pos(hdl) == field.hero_pos() {
                return SceneEnding::NextScene(Continuation::PlayDie);
            }
        },
        AI::Drift => {
            // TODO: Deal with collisions between movs

            let mut drift_dir = CoordDelta::from_xy(0, 0);

            // If hitting wall, reverse direction.
            if impassable(field, field.obj_target_pos(hdl)) {
                field.objm(hdl).dir.reverse();

                // And if hero "visible" forward or sideways, move one sideways towards them, if passable.
                // TODO: Check for obstacles to vision.
                let hero_dir = field.obj_pos(hdl).dir_to(field.hero_pos());
                if field.obj(hdl).dir.dx == 0 {
                    if hero_dir.dy != -field.obj(hdl).dir.dy {
                        drift_dir = CoordDelta::from_xy(hero_dir.dx, 0);
                    }
                } else if field.obj(hdl).dir.dy == 0 {
                    if hero_dir.dx != -field.obj(hdl).dir.dx {
                        drift_dir = CoordDelta::from_xy(0, hero_dir.dy);
                    }
                } else {
                    panic!("AI::Drift only implemented for orthogal movement");
                }
            }

            // Move. Provided next space is passable. If both sides are impassable, don't move.
            // TODO: Animation for turning? At least avoiding wall?
            let delta = field.obj(hdl).dir + drift_dir;
            if passable(field, field.obj_pos(hdl) + delta) {
                field.move_obj_to(hdl, field.obj_pos(hdl) + delta);
            }

            // Hero dies if mov moves onto hero
            if field.obj(hdl).effect == Effect::Kill && field.obj_pos(hdl) == field.hero_pos() {
                return SceneEnding::NextScene(Continuation::PlayDie);
            }
        },
        AI::Scuttle => {
            // If hitting wall, choose new direction.
            if impassable(field, field.obj_target_pos(hdl)) {
                let hero_dir = field.obj_pos(hdl).dir_to(field.hero_pos());
                let hero_delta = field.obj_pos(hdl).delta_to(field.hero_pos());
                // Find whether x or y is more towards the hero
                let x_longer_than_y = match hero_delta.dx.abs() - hero_delta.dy.abs() {
                    num if num > 0 => true,
                    num if num < 0 => false,
                    _ => field.obj(hdl).dir.dy.abs() < field.obj(hdl).dir.dy.abs(),
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
                    passable(field, field.obj_pos(hdl) + **dir)
                ) {
                    field.objm(hdl).dir = *dir;
                }
            }

            // Move. Provided next space is passable. If all sides were impassable, don't move.
            if passable(field, field.obj_target_pos(hdl)) {
                field.move_obj_to(hdl, field.obj_pos(hdl) + field.obj(hdl).dir);
            }

            // Hero dies if bot moves onto hero
            if field.obj(hdl).effect == Effect::Kill && field.obj_pos(hdl) == field.hero_pos() {
                return SceneEnding::NextScene(Continuation::PlayDie);
            }
        },
    }
    return SceneEnding::ContinuePlaying;
}
