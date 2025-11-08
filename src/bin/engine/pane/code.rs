use super::{PaneContinuation, PaneConclusion};
use crate::engine::input::Input;
use super::BasePane;

#[derive(Clone, Debug)]
pub struct Code {
    // Palette of available instructions, array of assembled instructions, etc.
}

impl BasePane for Code
{
    fn advance(&mut self, _input: &mut Input) -> PaneContinuation {
        // TODO

        return PaneContinuation::Continue(());
    }

    fn is_continuous(&self) -> bool {
        true
    }
}
