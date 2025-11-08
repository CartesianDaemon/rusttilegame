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

#[derive(Clone, PartialEq, Debug)]
pub struct ProgpuzzCustomProps {
    pub ai: ProgpuzzAI,
    pub prog: Prog,
}

impl ProgpuzzCustomProps {
    pub fn new(ai: ProgpuzzAI) -> Self {
        Self {
            ai,
            ..Self::default()
        }
    }
}

impl BaseCustomProps for ProgpuzzCustomProps {
    fn default() -> Self {
        Self {
            ai: ProgpuzzAI::Stay,
            prog: Prog::default(),
        }
    }

    fn is_hero(self: &Self) -> bool {
        self.ai == ProgpuzzAI::Prog
    }
    fn is_any_mov(self: &Self) -> bool {
        self.ai != ProgpuzzAI::Stay
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum ProgpuzzAI {
    Stay, // No self movement. Engine doesn't track specially.
    Prog, // Controlled by program assembled by player.
}

pub struct ProgpuzzMovementLogic;

impl BaseMovementLogic for ProgpuzzMovementLogic
{
    type CustomProps = ProgpuzzCustomProps;

    fn move_mov(map: &mut Map<Self>, mov: RosterIndex, cmd: Cmd) -> PaneContinuation {
        match map[mov].logical_props.custom_props.ai {
            ProgpuzzAI::Prog => {
                // TODO make sure cmd makes sense as program instruction not key
                if cmd != Cmd::Stay {
                    let target_pos = map[mov].pos() + cmd.as_dir();
                    if passable(map, target_pos) {
                        map.move_obj_to(mov, target_pos);
                    }
                }
                // Check for goal
                return if map.any_effect(map[mov].pos(), Effect::Win) {
                    PaneContinuation::Break(PaneConclusion::ArenaWin)
                } else {
                    PaneContinuation::Continue(())
                }
            },
            ProgpuzzAI::Stay => {
                // Do nothing
            },
            }
        return PaneContinuation::Continue(());
    }
}