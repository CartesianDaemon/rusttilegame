use crate::engine::for_scripting::*;
use crate::engine::for_gamedata::*;

// NB Breadcrumb: Move these fns to a struct in simple props.
pub fn passable<MovementLogic: BaseMovementLogic>(map: &Map<MovementLogic>, pos: MapCoord) -> bool {
    map.all_pass(pos, Pass::Empty)
}

#[allow(dead_code)]
pub fn impassable<MovementLogic: BaseMovementLogic>(map: &Map<MovementLogic>, pos: MapCoord) -> bool {
    !passable(map, pos)
}

// NB: Custom props could be enum we need different data for different object types.
// Would need to figure out which types can have which AIs.
#[derive(Clone, PartialEq, Debug)]
pub struct ProgpuzzCustomProps {
    pub ai: ProgpuzzAI,
    pub prog: Prog,
    // Next instruction to execute as index into vec.
    // Later will need handle into branching object.
    pub ip: usize,
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
            ip: 0,
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
        let props = &map[mov].logical_props.custom_props;
        match props.ai {
            ProgpuzzAI::Prog => {
                // NB: For now mostly ignoring input cmd. Need to revisit.
                match props.prog.instrs.get(props.ip) {
                    // Conclude pane with failure if we reach the end of the program.
                    None => return PaneContinuation::Break(PaneConclusion::ArenaDie),

                    // Move forward
                    Some(Instr::F) => {
                        // NB Breadcrumb: Move to an attempt_action fn in simple_props.
                        let target_pos = map[mov].pos() + cmd.as_dir();
                        if passable(map, target_pos) {
                            map.move_obj_to(mov, target_pos);
                        }
                    },
                    // Rotate L
                    Some(Instr::L) => {
                        map[mov].logical_props.dir.rotate_l();
                    },
                    // Rotate R
                    Some(Instr::R) => {
                        map[mov].logical_props.dir.rotate_r();
                    },
                    // Loop through contained instructions. NB: Placeholder.
                    Some(Instr::Loop(_)) => {
                        unimplemented!();
                    },
                }

                // Advance to next instr for next time.
                map[mov].logical_props.custom_props.ip +=1;

                // Conclude pane successfully if hero finds with goal.
                if map.any_effect(map[mov].pos(), Effect::Win) {
                    return PaneContinuation::Break(PaneConclusion::ArenaWin)
                }

                // Continue pane without concluding.
                return PaneContinuation::Continue(());
            },
            ProgpuzzAI::Stay => {
                // Do nothing
            },
            }
        return PaneContinuation::Continue(());
    }
}