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
        match self.phase {
            CodingRunningPhase::Coding => {
                if cmd == InputCmd::Continue {
                    self.start_execution();
                }
            },
            CodingRunningPhase::Running => {
                if cmd == InputCmd::Tick {
                    log::debug!("Advance bot program.");

                    self.curr_arena.as_mut().unwrap().advance(cmd);
                    let conclusion = self.curr_arena.as_ref().unwrap().ready_for_next_level();
                    if conclusion == Some(for_gamedata::SceneConclusion::Fail) {
                        log::debug!("Ran off end of program. Stopped.");
                        self.phase = CodingRunningPhase::Died;
                    } else if conclusion == Some(for_gamedata::SceneConclusion::Succeed) {
                        log::debug!("Bot found target!");
                        self.phase = CodingRunningPhase::Won;
                    } else if conclusion == None {
                        log::trace!("Bot advanced normally. Continue executing program.");
                    }
                } else {
                    log::debug!("Cancelling execution.");
                    self.cancel_execution();
                }
            },
            CodingRunningPhase::Won => {
                self.ready_for_next_level = Some(SceneConclusion::Succeed);
            },
            CodingRunningPhase::Died => {
                self.cancel_execution();
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
        log::debug!("Start program running.");

        // Init interactive arena
        self.curr_arena = Some(self.init_arena.clone());
        // Run game-specific logic for sync'ing different scenes at start of run.
        MovementLogic::harmonise(self);

        self.phase = CodingRunningPhase::Running;
    }

    pub fn cancel_execution(&mut self) {
        log::debug!("Returning to coding screen");
        self.phase = CodingRunningPhase::Coding;
        self.curr_arena = None;
    }

}
