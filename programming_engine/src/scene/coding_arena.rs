use super::*;
use crate::for_gamedata;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CodingRunningPhase {
    Coding,
    Running,
    Died,
    Won,
}

#[derive(Clone, Debug)]
pub struct CodingArena<MovementLogic : for_gamedata::BaseMovementLogic> {
    pub init_arena: Arena<MovementLogic>,
    // Maybe move into Running state, not core data?
    pub curr_arena: Option<Arena<MovementLogic>>,
    pub coding: Coding,
    pub phase: CodingRunningPhase,
    ready_for_next_level: Option<SceneConclusion>,
}

impl<MovementLogic : for_gamedata::BaseMovementLogic> BaseScene for CodingArena<MovementLogic>
{
    fn advance(&mut self, cmd: InputCmd) {
        use InputCmd::*;
        match self.phase {
            CodingRunningPhase::Coding => {
                if cmd == Continue {
                    self.start_execution();
                }
            },
            CodingRunningPhase::Running => {
                match cmd {
                    Tick => {
                        log::debug!("Advance bot program.");

                        self.curr_arena.as_mut().unwrap().advance(cmd);
                        let conclusion = self.curr_arena.as_ref().unwrap().ready_for_next_level();
                        if conclusion == Some(for_gamedata::SceneConclusion::Fail) {
                            log::debug!("Ran off end of program.");
                            self.died();
                        } else if conclusion == Some(for_gamedata::SceneConclusion::Succeed) {
                            log::debug!("Bot found target!");
                            self.won();
                        } else if conclusion == None {
                            log::trace!("Bot advanced normally. Continue executing program.");
                        }
                    },
                    Continue | Cancel => {
                        self.cancel_execution();
                    },
                }
            },
            CodingRunningPhase::Won => {
                match cmd {
                    Continue => self.continue_to_next_level(),
                    Cancel => self.continue_coding(),
                    Tick => (), // unreachable!(),
                }
            },
            CodingRunningPhase::Died => {
                match cmd {
                    Continue | Cancel => self.continue_coding(),
                    Tick => (), // unreachable!(),
                }
            },
        }
    }

    fn ready_for_next_level(&self) -> Option<SceneConclusion> {
        self.ready_for_next_level
    }
}

impl<MovementLogic: for_gamedata::BaseMovementLogic> CodingArena<MovementLogic>
{
    pub fn new<const HEIGHT: usize>(
        arena: Arena<MovementLogic>,
        code: Coding,
    ) -> Self {
        Self {
            init_arena: arena,
            curr_arena: None,
            coding: code,
            phase: CodingRunningPhase::Coding,
            ready_for_next_level: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.phase == CodingRunningPhase::Running
    }

    pub fn is_coding(&self) -> bool {
        self.phase == CodingRunningPhase::Coding
    }

    pub fn start_execution(&mut self) {
        self.transition(CodingRunningPhase::Running);
    }

    pub fn won(&mut self) {
        self.transition(CodingRunningPhase::Won);
    }

    pub fn died(&mut self) {
        self.transition(CodingRunningPhase::Died);
    }

    pub fn continue_to_next_level(&mut self) {
        self.ready_for_next_level = Some(SceneConclusion::Succeed);
    }

    pub fn cancel_execution(&mut self) {
        self.continue_coding();
    }

    pub fn continue_coding(&mut self) {
        self.transition(CodingRunningPhase::Coding);
    }

    fn transition(&mut self, new_phase: CodingRunningPhase) {
        use CodingRunningPhase::*;
        let old_phase = self.phase;
        log::debug!("Coding Arena transition: {old_phase:?} -> {new_phase:?}");
        self.phase = new_phase;
        match (old_phase, new_phase) {
            (Running|Won|Died, Coding) => {
                // Stopping running. De-init interactive arena
                self.curr_arena = None;
            }
            (Coding, Running) => {
                // Starting running. Init interactive arena
                self.curr_arena = Some(self.init_arena.clone());
                // Starting running. Run game-specific logic to copy prog into bot.
                MovementLogic::harmonise(self);
            },
            (Running,Won|Died) => (),
            _ => panic!("Coding Arena: Unexpected transition!"),
        }
    }

}
