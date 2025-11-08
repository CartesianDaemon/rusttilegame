// TODO: Add these types to a Script struct?
use tile_engine::for_scripting::*;

#[derive(Clone, Debug)]
pub struct PushpuzzMovementLogic;

impl BaseMovementLogic for PushpuzzMovementLogic {
    type CustomProps = tile_engine::simple_custom_props::SimpleCustomProps;

    // Would be nice for these to be a function of an enum/trait impls
    fn move_mov(map: &mut Map<Self>, mov: RosterIndex, cmd: Cmd) -> PaneContinuation {
        let hero = map.hero();
        match map[mov].logical_props.custom_props.ai {
            SimpleAI::Stay => {
                // Do nothing
            },
            SimpleAI::Hero => {
                if cmd != Cmd::Stay {
                    let target_pos = map[mov].pos() + cmd.as_dir();
                    if passable(map, target_pos) {
                        map.move_obj_to(mov, target_pos);
                    }
                }
                // TODO: Avoid needing to re-get the hero handle, make move function consume or update the rich_mov handle.
                return if map.any_effect(map[mov].pos(), Effect::Win) {
                    PaneContinuation::Break(PaneConclusion::ArenaWin)
                } else {
                    PaneContinuation::Continue(())
                }
                // TODO: Also check if hero died? Usually superfluous if we don't allow moving into death.
            }
            SimpleAI::Bounce => {
                // TODO: Simplify duplication in map.obj_at(rich_mov.ros_idx) throughout?

                // If moving would hit a wall, first reverse direction.
                // TODO: Consider adding "can_move" function. Map would need movement_logic "passable(obj, tile)" dependency injection.
                // TODO: Consider adding map.try_move() fn.
                // TODO: Consider adding map_coord *= -1.
                let target_pos = map.obj_target_pos(mov);
                if impassable(map, target_pos) {
                    map[mov].logical_props.dir.reverse();
                }

                // Move. Provided next space is passable. If both sides are impassable, don't move.
                // TODO: Consider adding map.obj_try_move() function?
                let target_pos = map[mov].pos() + map[mov].logical_props.dir;
                if passable(map, target_pos) {
                    map.move_obj_to(mov, target_pos);
                }

                // Hero dies if mov moves onto hero
                // TODO: Check at end of function? Or as part of obj?
                if map[mov].logical_props.effect == Effect::Kill && map[mov].pos() == map[hero].pos() {
                    return PaneContinuation::Break(PaneConclusion::ArenaDie);
                }
            },
            SimpleAI::Drift => {
                // TODO: Deal with collisions between movs

                let mut drift_dir = CoordDelta::from_xy(0, 0);

                // If hitting wall, reverse direction.
                if impassable(map, map.obj_target_pos(mov)) {
                    map[mov].logical_props.dir.reverse();

                    // And if hero "visible" forward or sideways, move one sideways towards them, if passable.
                    // TODO: Check for obstacles to vision.
                    let hero_dir = map[mov].pos().dir_to(map[hero].pos());
                    if map[mov].logical_props.dir.dx == 0 {
                        if hero_dir.dy != -map[mov].logical_props.dir.dy {
                            drift_dir = CoordDelta::from_xy(hero_dir.dx, 0);
                        }
                    } else if map[mov].logical_props.dir.dy == 0 {
                        if hero_dir.dx != -map[mov].logical_props.dir.dx {
                            drift_dir = CoordDelta::from_xy(0, hero_dir.dy);
                        }
                    } else {
                        panic!("SimpleAI::Drift only implemented for orthogal movement");
                    }
                }

                // Move. Provided next space is passable. If both sides are impassable, don't move.
                // TODO: Animation for turning? At least avoiding wall?
                let delta = map[mov].logical_props.dir + drift_dir;
                if passable(map, map[mov].pos() + delta) {
                    map.move_obj_to(mov, map[mov].pos() + delta);
                }

                // Hero dies if mov moves onto hero
                if map[mov].logical_props.effect == Effect::Kill && map[mov].pos() == map[hero].pos() {
                    return PaneContinuation::Break(PaneConclusion::ArenaDie);
                }
            },
            SimpleAI::Scuttle => {
                // If hitting wall, choose new direction.
                if impassable(map, map.obj_target_pos(mov)) {
                    let hero_dir = map[mov].pos().dir_to(map[hero].pos());
                    let hero_delta = map[mov].pos().delta_to(map[hero].pos());
                    // Find whether x or y is more towards the hero
                    let x_longer_than_y = match hero_delta.dx.abs() - hero_delta.dy.abs() {
                        num if num > 0 => true,
                        num if num < 0 => false,
                        _ => map[mov].logical_props.dir.dy.abs() < map[mov].logical_props.dir.dy.abs(),
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
                        passable(map, map[mov].pos() + **dir)
                    ) {
                        map[mov].logical_props.dir = *dir;
                    }
                }

                // Move. Provided next space is passable. If all sides were impassable, don't move.
                if passable(map, map.obj_target_pos(mov)) {
                    map.move_obj_to(mov, map[mov].pos() + map[mov].logical_props.dir);
                }

                // Hero dies if bot moves onto hero
                if map[mov].logical_props.effect == Effect::Kill && map[mov].pos() == map[hero].pos() {
                    return PaneContinuation::Break(PaneConclusion::ArenaDie);
                }
            },
        }
        return PaneContinuation::Continue(());
    }
}