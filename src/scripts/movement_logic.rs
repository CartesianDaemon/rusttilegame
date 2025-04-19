use crate::engine::scripting::*;
use super::obj_types::*;

pub fn passable(field: &Field, pos: MapCoord) -> bool {
    field.all_pass(pos, Pass::Empty)
}

pub fn impassable(field: &Field, pos: MapCoord) -> bool {
    !passable(field, pos)
}

// TODO: Could recombine into a single move function, with the engine or script logic deciding when
//       to call it for the hero obj and when to call it for the other objs
pub fn move_character_refactored(field: &mut Field, rich_hero: RichMapHandle, cmd: Cmd) -> SceneEnding {
    if cmd != Cmd::Stay {
        let target_pos = field.obj_pos(rich_hero) + cmd.as_dir();
        if passable(field, target_pos) {
            field.obj_move_to_refactored(rich_hero, target_pos);
        }
    }
    // TODO: Avoid needing to re-get the hero handle, make move function consume or update the rich_hero handle.
    return if field.any_effect(field.roster.hero.pos(), Effect::Win) {
        SceneEnding::NextScene(Continuation::PlayWin)
    } else {
        SceneEnding::ContinuePlaying
    }
    // TODO: Also check if hero died? Usually superfluous if we don't allow moving into death.
}

pub fn move_mov_refactored(field: &mut Field, rich_mov: RichMapHandle) -> SceneEnding {
    match field.obj_props(rich_mov).ai {
        AI::Stay => {
            // Do nothing
        },
        AI::Hero => {
            // Handled separately.
        },
        AI::Bounce => {
            // TODO: Simplify duplication in field.obj_at(rich_mov.ros_idx) throughout?

            // If moving would hit a wall, first reverse direction.
            // TODO: Consider adding field.try_move() fn.
            // TODO: Consider adding map_coord *= -1.
            let target_pos = field.obj_target_pos(rich_mov);
            if impassable(field, target_pos) {
                let objm = field.obj_props_m(rich_mov);
                objm.dir = CoordDelta::from_xy(-objm.dir.dx, -objm.dir.dy);
            }

            // Move. Provided next space is passable. If both sides are impassable, don't move.
            // TODO: Consider adding field.obj_try_move() function?
            let target_pos = field.obj_pos(rich_mov) + field.obj_props(rich_mov).dir;
            if passable(field, target_pos) {
                field.obj_move_to_refactored(rich_mov, target_pos);
            }

            // Hero dies if mov moves onto hero
            // TODO: Check at end of function? Or as part of obj?
            if field.obj_props(rich_mov).effect == Effect::Kill && field.obj_pos(rich_mov) == field.obj_pos(field.rich_hero()) {
                return SceneEnding::NextScene(Continuation::PlayDie);
            }
        },
        _ => {
        }
    }
    return SceneEnding::ContinuePlaying;
}

pub fn move_mov(map: &mut InternalMap, hero: &MapHandle, mov: &mut MapHandle) -> SceneEnding {
    match map[*mov].ai {
        AI::Stay => {
            // Do nothing
        },
        AI::Hero => {
            // Handled separately.
        },
        AI::Drift => {
            // TODO: Deal with collisions between movs

            let mut drift_dir = CoordDelta::from_xy(0, 0);
            // If hitting wall, reverse direction.
            if map.loc_at(*mov + map[*mov].dir).impassable() {
                map[*mov].dir = CoordDelta::from_xy(-map[*mov].dir.dx, -map[*mov].dir.dy);
                // If hero "visible" forward or sideways, move one sideways towards them, if passable.
                // TODO: Check for obstacles to vision.
                let hero_dir = CoordDelta::from_xy((hero.x - mov.x).signum(),(hero.y - mov.y).signum());
                if map[*mov].dir.dx == 0 {
                    if hero_dir.dy != -map[*mov].dir.dy {
                        drift_dir = CoordDelta::from_xy(hero_dir.dx, 0);
                    }
                } else if map[*mov].dir.dy == 0 {
                    if hero_dir.dx != -map[*mov].dir.dx {
                        drift_dir = CoordDelta::from_xy(0, hero_dir.dy);
                    }
                } else {
                    panic!("AI::Drift only implemented for orthogal movement");
                }
            }

            // Move. Provided next space is passable. If both sides are impassable, don't move.
            // TODO: Animation for turning? At least avoiding wall?
            let delta = map[*mov].dir + drift_dir;
            if map.loc_at(*mov + delta).passable() {
                map.obj_move_delta(mov, delta);
            }

            // Hero dies if mov moves onto hero
            if map[*mov].effect == Effect::Kill {
                if mov.x == hero.x && mov.y == hero.y {
                    return SceneEnding::NextScene(Continuation::PlayDie);
                }
            }
        },
        AI::Scuttle => {
            // If hitting wall, choose new direction.
            if map.loc_at(*mov + map[*mov].dir).impassable() {
                let dx_to_hero = hero.x - mov.x;
                let dy_to_hero = hero.y - mov.y;
                // Find whether x or y is more towards the hero
                let x_longer_than_y = match dx_to_hero.abs() - dy_to_hero.abs() {
                    num if num > 0 => true,
                    num if num < 0 => false,
                    _ => map[*mov].dir.dy.abs() < map[*mov].dir.dy.abs(),
                };
                // dlongcoord is the orthogonal direction most towards the hero. dshortcoord is the other best.
                let (dlongcoord, dshortcoord) = if x_longer_than_y {
                    (CoordDelta::from_xy(dx_to_hero.signum(), 0), CoordDelta::from_xy(0, dy_to_hero.signum()))
                } else {
                    (CoordDelta::from_xy(0, dy_to_hero.signum()), CoordDelta::from_xy(dx_to_hero.signum(), 0))
                };
                // Prefer the directions "most" towards the hero first
                let try_dirs = vec![dlongcoord, dshortcoord, -dshortcoord, -dlongcoord];
                // Try each direction in turn, use the first passable one.
                // Can't be the same as original direction because that was impassable.
                // If none are passable, stay in the same direction we started.
                if let Some(dir) = try_dirs.iter().find(|dir|
                    map.loc_at(*mov + **dir).passable()
                ) {
                    map[*mov].dir = *dir;
                }
            }

            // Move. Provided next space is passable. If all sides were impassable, don't move.
            if map.loc_at(*mov + map[*mov].dir).passable() {
                map.obj_move_delta(mov, map[*mov].dir);
            }

            // Hero dies if bot moves onto hero
            if map[*mov].effect == Effect::Kill {
                if mov.x == hero.x && mov.y == hero.y {
                    return SceneEnding::NextScene(Continuation::PlayDie);
                }
            }
        },
        _ => {}
    }
    return SceneEnding::ContinuePlaying;
}
