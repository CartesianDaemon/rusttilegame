use tile_engine::for_gamedata::*;

// NB: Custom props could be enum we need different data for different object types.
// Would need to figure out which types can have which AIs.
#[derive(Clone, Debug)]
pub struct ProgpuzzCustomProps {
    pub ai: ProgpuzzAI,
    pub prog: Prog,
    pub about_to_exec_init_instr: bool,
}

impl ProgpuzzCustomProps {
    pub fn new(ai: ProgpuzzAI) -> Self {
        Self {
            ai,
            ..Self::default()
        }
    }
}

// Fuzzy matching
impl PartialEq for ProgpuzzCustomProps {
    fn eq(&self, other: &Self) -> bool {
        self.ai == other.ai
    }
}

impl BaseCustomProps for ProgpuzzCustomProps {
    fn default() -> Self {
        Self {
            ai: ProgpuzzAI::Stay,
            prog: Prog::default(),
            about_to_exec_init_instr: true,
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

#[derive(Clone)]
pub struct ProgpuzzGameLogic;

impl BaseGameLogic for ProgpuzzGameLogic
{
    type CustomProps = ProgpuzzCustomProps;

    fn harmonise(coding_arena: &mut CodingArena<Self>) {
        // Set progbot's prog to the user-assembled prog.
        let bot = coding_arena.curr_arena.as_ref().unwrap().hero();
        coding_arena.curr_arena.as_mut().unwrap()[bot].logical_props.custom_props.prog = coding_arena.coding.prog.clone();
    }

    fn get_active_idx(coding_arena: &CodingArena<Self>) -> Option<usize> {
        if let Some(arena) = &coding_arena.curr_arena {
            Some(arena[arena.hero()].logical_props.custom_props.prog.curr_ip)
        } else {
            None
        }
    }

    fn move_mov(map: &mut Arena<Self>, mov: RosterIndex, _cmd: InputCmd) -> WidgetContinuation {
        let props = &mut map[mov].logical_props.custom_props;
        match props.ai {
            ProgpuzzAI::Prog => {
                if props.about_to_exec_init_instr {
                    props.about_to_exec_init_instr = false;
                } else {
                    props.prog.advance_next_instr();

                    if props.prog.finished() {
                        log::debug!("Bot reached end of program.");
                        return WidgetContinuation::Break(WidgetConclusion::Die);
                    }
                }

                match props.prog.curr_op_mut() {
                    None => {
                        log::debug!("Bot reached empty parent instr.");
                        return WidgetContinuation::Break(WidgetConclusion::Die);
                    }
                    Some(Instr::Action(action_op, action_data)) => {
                        action_data.blocked = false;
                        match action_op {
                            // Move forward
                            ActionOpcode::F => {
                                let target_pos = map[mov].pos() + map[mov].logical_props.dir;
                                if map.passable(target_pos) {
                                    log::debug!("Bot move F. {} -> {}", map[mov].pos(), target_pos);
                                    map.move_obj_to(mov, target_pos);
                                } else {
                                    log::debug!("Bot blocked F. {} -/-> {}", map[mov].pos(), target_pos);
                                    map[mov].logical_props.custom_props.prog.curr_op_mut().unwrap().as_action_data().blocked = true;
                                }
                            },
                            ActionOpcode::L => {
                                map[mov].logical_props.dir.rotate_l();
                                log::debug!("Bot rotate L. {} -> {}", map[mov].logical_props.prev_dir , map[mov].logical_props.dir);
                            },
                            ActionOpcode::R => {
                                map[mov].logical_props.dir.rotate_r();
                                log::debug!("Bot rotate R. {} -> {}", map[mov].logical_props.prev_dir , map[mov].logical_props.dir);
                            },
                        }
                    }
                    Some(Instr::Parent(..)) => {
                        panic!("Unrecognised instr {:?}", props.prog.curr_op());
                    },
                }

                // Conclude pane successfully if hero finds with goal.
                if map.any_has_effect(map[mov].pos(), Effect::Win) {
                    return WidgetContinuation::Break(WidgetConclusion::Win)
                }

                // Continue pane without concluding.
                return WidgetContinuation::Continue(());
            },
            ProgpuzzAI::Stay => {
                log::trace!("ProgpuzzGameLogic::move_mov: Stay\n");
                // Do nothing
            },
            }
        return WidgetContinuation::Continue(());
    }
}
