use crate::pane::*;
use crate::input::Input;

#[derive(Clone, Debug)]
pub enum SplitPhase {
    Coding,
    Running,
}

#[derive(Clone, Debug)]
pub struct Split<MovementLogic : super::for_gamedata::BaseMovementLogic> {
    pub arena: Arena<MovementLogic>,
    pub code: Code,
    phase: SplitPhase,
}

// NB: Should this be calling input at all or not?
// NB: Have separate Cmd for menu, movement, programming, etc. Pane chooses which?
impl<MovementLogic : super::for_gamedata::BaseMovementLogic> BasePane for Split<MovementLogic>
{
    fn advance(&mut self, input: &mut Input) -> PaneContinuation {
        match self.phase {
            SplitPhase::Coding => {
                // For now ignore input and treat anything as "start running"?
                self.phase = SplitPhase::Running;
                // TODO: Edit program according to input. Or start running.
                let _ = &self.code.supplies;
                let _ = &self.code.prog;
            },
            SplitPhase::Running => {
                // For now ignore input and execute program.
                // Once run off end will always return ConclusionDie.
                let _conclusion = self.arena.advance(input);

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

impl<MovementLogic : super::for_gamedata::BaseMovementLogic> Split<MovementLogic>
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
