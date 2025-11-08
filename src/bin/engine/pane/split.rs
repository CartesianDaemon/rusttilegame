use super::{Arena, Code};
use super::{PaneContinuation, PaneConclusion};
use crate::engine::input::Input;
use crate::engine::obj::FreeObj;
use super::BasePane;

use std::collections::HashMap;

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
        // TODO: If input space, start/stop. Else:
        // TODO: If coding, advance code by input.
        // TODO: If running, advance arena deterministically.

        return PaneContinuation::Continue(());
    }

    fn is_continuous(&self) -> bool {
        // TODO: Depend on "running" or "coding" state.
        true
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
