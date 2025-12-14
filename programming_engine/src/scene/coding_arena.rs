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
    outcome_to_store: Option<OutcomeToStore>,
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
                            // TODO: Call died() instead when bot dies for other reasons.
                            log::debug!("Ran off end of program.");
                            self.finish();
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
            outcome_to_store: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.phase == CodingRunningPhase::Running
    }

    pub fn is_coding(&self) -> bool {
        self.phase == CodingRunningPhase::Coding
    }

    fn start_execution(&mut self) {
        assert!(self.phase == CodingRunningPhase::Coding);
        self.transition(CodingRunningPhase::Running);

        // Init interactive arena
        self.curr_arena = Some(self.init_arena.clone());
        // Run game-specific logic to copy prog into bot.
        MovementLogic::harmonise(self);
    }

    fn won(&mut self) {
        assert!(self.phase == CodingRunningPhase::Running);
        self.record_outcome("Won");
        self.transition(CodingRunningPhase::Won);
    }

    fn finish(&mut self) {
        assert!(self.phase == CodingRunningPhase::Running);
        self.record_outcome("Fin");
        self.transition(CodingRunningPhase::Died);
    }

    fn _died(&mut self) {
        assert!(self.phase == CodingRunningPhase::Running);
        self.record_outcome("Die");
        self.transition(CodingRunningPhase::Died);
    }

    fn continue_to_next_level(&mut self) {
        assert!(self.phase == CodingRunningPhase::Won);
        self.ready_for_next_level = Some(SceneConclusion::Succeed);
    }

    fn cancel_execution(&mut self) {
        assert!(self.phase == CodingRunningPhase::Running);
        self.record_outcome("Esc");
        self.continue_coding();
    }

    fn continue_coding(&mut self) {
        assert!(self.phase != CodingRunningPhase::Coding);
        self.transition(CodingRunningPhase::Coding);

        // De-init interactive arena
        self.curr_arena = None;
    }

    fn transition(&mut self, new_phase: CodingRunningPhase) {
        use CodingRunningPhase::*;
        let old_phase = self.phase;
        log::debug!("Coding Arena transition: {old_phase:?} -> {new_phase:?}");
        assert_ne!(old_phase, new_phase);
        self.phase = new_phase;
        match (old_phase, new_phase) {
            // From start_execution()
            (Coding, Running) => (),
            // From continue_coding()
            (Running|Won|Died, Coding) => (),
            // From won() or died()
            (Running,Won|Died) => (),
            // An unexpected transition could fail to init or deinit arena.
            _ => panic!("Coding Arena: Unexpected transition!"),
        }
    }

    fn record_outcome(&mut self, outcome: &str) {
        // Record result of execution
        assert!(self.outcome_to_store.is_none());
        self.outcome_to_store = Some(OutcomeToStore::new(outcome.to_string(), self.coding.prog.to_string()));
    }

    pub fn consume_outcome_to_store(&mut self) -> Option<OutcomeToStore> {
        self.outcome_to_store.take()
    }
}
