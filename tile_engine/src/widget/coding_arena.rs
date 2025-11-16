use super::*;
use crate::map_coords::MoveCmd;
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
    fn advance(&mut self, cmd: Option<MoveCmd>) -> PaneContinuation {
        match self.phase {
            SplitPhase::Coding => {
                if let Some(move_cmd) = cmd {
                    log::debug!("CodingArena coding advance(): {:?}", move_cmd);

                    log::debug!("Start program running.");

                    // Run game-specific logic for sync'ing different panes at start of run.
                    GameLogic::harmonise(self);

                    self.phase = SplitPhase::Running;
                }
            },
            SplitPhase::Running => {
                if let Some(move_cmd) = cmd {
                    log::debug!("CodingArena arena advance(): {:?}", move_cmd);

                    // log::debug!("Advance arena...");
                    // For now ignore input and execute program.
                    // Once run off end will always return ConclusionDie.
                    let _conclusion = self.arena.advance(cmd);

                    // TODO: If input space, start. Else advance arena.
                    // Will ignore input except for space = "stop"?
                    // Win => return Win. Die => Stop running.
                }
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
