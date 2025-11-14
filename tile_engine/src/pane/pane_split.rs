use super::*;
use crate::map_coords::Cmd;

#[derive(Clone, Debug)]
pub enum SplitPhase {
    Coding,
    Running,
}

#[derive(Clone, Debug)]
pub struct Split<GameLogic : crate::for_gamedata::BaseGameLogic> {
    pub arena: Arena<GameLogic>,
    pub code: Code,
    phase: SplitPhase,
}

// NB: Should this be calling input at all or not?
// NB: Have separate Cmd for menu, movement, programming, etc. Pane chooses which?
impl<GameLogic : crate::for_gamedata::BaseGameLogic> BasePane for Split<GameLogic>
{
    fn advance(&mut self, cmd: Option<Cmd>) -> PaneContinuation {
        match self.phase {
            SplitPhase::Coding => {
                // NB: Need to have some permanent debug logging.
                // That is often more useful for "how it went wrong" than more detailed tests.
                // For now ignore input and treat anything as "start running".
                self.phase = SplitPhase::Running;

                // Run game-specific logic for sync'ing different panes at start of run.
                GameLogic::harmonise(self);

                // TODO: Edit program according to input. Or start running.
                let _ = &self.code.supplies;
                let _ = &self.code.prog;
            },
            SplitPhase::Running => {
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

impl<GameLogic : crate::for_gamedata::BaseGameLogic> Split<GameLogic>
{
    pub fn new<const HEIGHT: usize>(
        arena: Arena<GameLogic>,
        code: Code,
    ) -> Self {
        Self {
            arena,
            code,
            phase: SplitPhase::Coding,
        }
    }
}
