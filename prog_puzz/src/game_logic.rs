use tile_engine::for_gamedata::*;

// NB: Custom props could be enum we need different data for different object types.
// Would need to figure out which types can have which AIs.
#[derive(Clone, Debug)]
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
            Some(arena[arena.hero()].logical_props.custom_props.prog.prev_ip)
        } else {
            None
        }
        // coding_arena.curr_arena.as_ref().map(|curr_arena| curr_arena[curr_arena.hero()].logical_props.custom_props.ip)
    }

    fn move_mov(map: &mut Arena<Self>, mov: RosterIndex, _cmd: MoveCmd) -> WidgetContinuation {
        let props = &map[mov].logical_props.custom_props;
        match props.ai {
            ProgpuzzAI::Prog => {
                let prog = &mut map[mov].logical_props.custom_props.prog;

                // Op to actually affect robot, as opposed to control flow
                let external_op = prog.instrs.get(prog.next_ip).cloned();

                // Advance to next instr for next time.
                (prog.prev_ip, prog.next_ip) = (prog.next_ip, prog.next_ip + 1);

                match external_op {
                    // Conclude pane with failure if we reach the end of the program.
                    None => {
                        log::debug!("Bot reached end of program.");
                        return WidgetContinuation::Break(WidgetConclusion::Die);
                    }
                    Some(Node{op, ..}) => match op {
                        // Move forward
                        Op::F => {
                            let target_pos = map[mov].pos() + map[mov].logical_props.dir;
                            if map.passable(target_pos) {
                                log::debug!("Bot move F. {} -> {}", map[mov].pos(), target_pos);
                                map.move_obj_to(mov, target_pos);
                            } else {
                                log::debug!("Bot blocked F. {} -/-> {}", map[mov].pos(), target_pos);
                            }
                        },
                        Op::L => {
                            map[mov].logical_props.dir.rotate_l();
                            log::debug!("Bot rotate L. {} -> {}", map[mov].logical_props.prev_dir , map[mov].logical_props.dir);
                        },
                        Op::R => {
                            map[mov].logical_props.dir.rotate_r();
                            log::debug!("Bot rotate R. {} -> {}", map[mov].logical_props.prev_dir , map[mov].logical_props.dir);
                        },
                        Op::x2 => {
                            unimplemented!();
                        },
                    }
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
