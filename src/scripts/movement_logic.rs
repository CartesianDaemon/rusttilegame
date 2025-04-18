use crate::engine::scripting::*;
use super::obj_types::*;

pub fn move_character_refactored(rich_hero: RichMapHandle, field: &mut Field, cmd: Cmd) -> SceneEnding {
    if cmd != Cmd::Stay {
        let dir = cmd.as_dir();
        if field.obj_can_move_refactored(rich_hero, dir) {
            field.obj_move_delta_refactored(rich_hero, dir);
            // TODO: Avoid needing to re-get the hero handle, make move function consume or update the rich_hero handle.
            if field.any_effect(field.roster.hero.as_pos(), Effect::Win) {
                return SceneEnding::NextScene(Continuation::PlayWin);
            }
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
        AI::Snake => {
            // if mov on same row xor column as hero, change dir to face hero
            if (mov.x == hero.x) != (mov.y == hero.y) {
                let new_dir = CoordDelta::from_xy((hero.x - mov.x).signum(),(hero.y - mov.y).signum());
                map[*mov].dir = new_dir;
            }

            // NOTE: When mov goes out of bounds is placeholder for real win condition.
            if !(0..map.w() as i16).contains(&(mov.x + map[*mov].dir.dx)) ||
                !(0..map.h() as i16).contains(&(mov.y + map[*mov].dir.dy))
            {
                return SceneEnding::NextScene(Continuation::PlayWin);
            }
            else
            {
                // move mov to new location
                // TODO: Have a "move_dir" fn.
                let dir = map[*mov].dir;
                map.obj_move_delta(mov, dir);
            }

            // Die if mov moves onto hero
            if mov.x == hero.x && mov.y == hero.y {
                return SceneEnding::NextScene(Continuation::PlayDie);
            }
        },
        AI::Bounce => {
            // TODO: Make a Map:: fn for "at pos + dir, or appropriate default if off map"

            // If hitting wall, reverse direction.
            if map.loc_at(*mov + map[*mov].dir).impassable() {
                map[*mov].dir = CoordDelta::from_xy(-map[*mov].dir.dx, -map[*mov].dir.dy);
            }

            // Move. Provided next space is passable. If both sides are impassable, don't
            // move.
            if map.loc_at(*mov + map[*mov].dir).passable() {
                map.obj_move_delta(mov, map[*mov].dir);
            }

            // Hero dies if mov moves onto hero
            if map[*mov].effect == Effect::Kill {
                if mov.x == hero.x && mov.y == hero.y {
                    return SceneEnding::NextScene(Continuation::PlayDie);
                }
            }
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
    }
    return SceneEnding::ContinuePlaying;
}
