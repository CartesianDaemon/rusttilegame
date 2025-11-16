use super::*;
use crate::map_coords::Cmd;
use crate::for_gamedata;

#[derive(Clone, Debug)]
pub enum SplitPhase {
    Coding,
    Running,
}

// NB: Move into Prog Puzz. Or make into a general multi-widget widget.
#[derive(Clone, Debug)]
pub struct CodingArena<GameLogic : for_gamedata::BaseGameLogic> {
    pub arena: Arena<GameLogic>,
    pub coding: Coding,
    phase: SplitPhase,
}

impl<GameLogic : for_gamedata::BaseGameLogic> BaseWidget for CodingArena<GameLogic>
{
    fn advance(&mut self, cmd: Option<Cmd>) -> PaneContinuation {
        match self.phase {
            SplitPhase::Coding => {
                log::debug!("Start program running.");
                // NB: Need to have some permanent debug logging.
                // That is often more useful for "how it went wrong" than more detailed tests.
                // For now ignore input and treat anything as "start running".
                self.phase = SplitPhase::Running;

                // Run game-specific logic for sync'ing different panes at start of run.
                GameLogic::harmonise(self);

                // TODO: Edit program according to input. Or start running.
                let _ = &self.coding.supply;
                let _ = &self.coding.prog;
            },
            SplitPhase::Running => {
                // log::debug!("Advance arena...");
                // For now ignore input and execute program.
                // Once run off end will always return ConclusionDie.
                let _conclusion = self.arena.advance(cmd);

                // TODO: If input space, start. Else advance arena.
                // Will ignore input except for space = "stop"?
                // Win => return Win. Die => Stop running.
            },
        }

        return PaneContinuation::Continue(());
    }

    fn tick_based(&self) -> bool {
        // TODO: Need "stop" to happen at any time. But could trigger bot movement on key, or on tick?
        false
    }
}

impl<GameLogic : for_gamedata::BaseGameLogic> CodingArena<GameLogic>
{
    pub fn new<const HEIGHT: usize>(
        arena: Arena<GameLogic>,
        code: Coding,
    ) -> Self {
        Self {
            arena,
            coding: code,
            phase: SplitPhase::Coding,
        }
    }
}
