// TODO: Add these types to BaseScripts struct??
use crate::engine::for_scripting::*;
use crate::engine::for_gamedata::*;

pub fn passable<MovementLogic: BaseMovementLogic>(map: &Map<MovementLogic>, pos: MapCoord) -> bool {
    map.all_pass(pos, Pass::Empty)
}

#[allow(dead_code)]
pub fn impassable<MovementLogic: BaseMovementLogic>(map: &Map<MovementLogic>, pos: MapCoord) -> bool {
    !passable(map, pos)
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ProgpuzzCustomProps {
    pub ai: ProgpuzzAI,
}

impl BaseCustomProps for ProgpuzzCustomProps {
    fn default() -> Self {
        Self {
            ai: ProgpuzzAI::Stay,
        }
    }

    fn is_hero(props: Self) -> bool {
        props.ai == ProgpuzzAI::Hero
    }
    fn is_any_mov(props: Self) -> bool {
        props.ai != ProgpuzzAI::Stay
    }
}

// Types of movement-control logic ents can use
// TODO: Make a copy for each game specialisatoin with different types.
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum ProgpuzzAI {
    Stay, // No self movement. Not added to Roster's list of movs.
    Hero, // Controlled by keys. Assume only one hero, added to Roster's hero entry.
    // Everything else may spontaneously move or need to be enumerated, ie needs to be added to roster.
    Bounce, // Move in direction, reverse direction at walls.
    Drift, // Move in direction, reverse direction at walls, move diagonally towards hero at reversal.
    Scuttle, // Move in direction, when hit wall change to move orthogonally towards hero.
}

pub struct ProgpuzzMovementLogic;

impl BaseMovementLogic for ProgpuzzMovementLogic
{
    type CustomProps = super::super::simple_custom_props::SimpleCustomProps;

    fn move_mov(map: &mut Map<Self>, mov: RosterIndex, cmd: Cmd) -> PaneContinuation {
        match map[mov].logical_props.custom_props.ai {
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