// TODO: Add these types to BaseScripts struct??
use crate::engine::for_scripting::*;

pub fn passable<MovementLogic: BaseMovementLogic>(map: &Map<MovementLogic>, pos: MapCoord) -> bool {
    map.all_pass(pos, Pass::Empty)
}

#[allow(dead_code)]
pub fn impassable<MovementLogic: BaseMovementLogic>(map: &Map<MovementLogic>, pos: MapCoord) -> bool {
    !passable(map, pos)
}

pub struct ProgpuzzMovementLogic;

impl BaseMovementLogic for ProgpuzzMovementLogic
{
    type CustomProps = super::super::simple_custom_props::SimpleCustomProps;

    fn move_mov(map: &mut Map<Self>, mov: RosterIndex, cmd: Cmd) -> PaneContinuation {
        match map[mov].logical_props.ai {
            SimpleAI::Hero => {
                // TODO make sure cmd makes sense as program instruction not key
                if cmd != Cmd::Stay {
                    let target_pos = map[mov].pos() + cmd.as_dir();
                    if passable(map, target_pos) {
                        map.move_obj_to(mov, target_pos);
                    }
                }
                // Check for goal
                return if map.any_effect(map[mov].pos(), Effect::Win) {
                    PaneContinuation::Break(PaneEnding::PlayWin)
                } else {
                    PaneContinuation::Continue(())
                }
            },
            SimpleAI::Stay => {
                // Do nothing
            },
            SimpleAI::Bounce => {
                // ???? TODO: Remove. TODO combine two match branches.
            },
            SimpleAI::Drift => {
                // ????
            },
            SimpleAI::Scuttle => {
                // ????
            },
            }
        return PaneContinuation::Continue(());
    }
}