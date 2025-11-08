use super::{Arena, Code};
use super::PaneContinuation;
use crate::input::Input;
use super::BasePane;

#[derive(Clone, Debug)]
pub enum SplitPhase {
    Coding,
    Running,
}

#[derive(Clone, Debug)]
pub struct Split<MovementLogic : super::super::for_gamedata::BaseMovementLogic> {
    pub arena: Arena<MovementLogic>,
    pub code: Code,
    phase: SplitPhase,
}

impl<MovementLogic : super::super::for_gamedata::BaseMovementLogic> BasePane for Split<MovementLogic>
{
    fn advance(&mut self, _input: &mut Input) -> PaneContinuation {
        match self.phase {
            SplitPhase::Coding => {
                // TODO: Advance code by input. Check input, or check return, to know when to start running.
                unimplemented!();
            },
            SplitPhase::Running => {
                // TODO: If input space, start. Else advance arena.
                // Will ignore input except for space = "stop"?
                // Win => return Win. Die => Stop running.
            },
        }

        return PaneContinuation::Continue(());
    }

    fn need_sync_to_ticks(&self) -> bool {
        // TODO: Need "stop" to happen at any time. But could trigger bot movement on key, or on tick?
        false
    }
}

impl<MovementLogic : super::super::for_gamedata::BaseMovementLogic> Split<MovementLogic>
{
    pub fn new<const HEIGHT: usize>(
        arena: Arena<MovementLogic>,
        code: Code,
    ) -> Self {
        Self {
            arena,
            code,
            phase: SplitPhase::Coding,
        }
    }
}
