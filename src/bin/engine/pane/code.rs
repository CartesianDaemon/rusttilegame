use super::{PaneContinuation, PaneConclusion};
use crate::engine::input::Input;
use super::PaneBase;

#[derive(Clone, Debug)]
pub struct Code {
    // Palette of available instructions, array of assembled instructions, etc.
}

impl PaneBase for Code
{
    fn advance(&mut self, _input: &mut Input) -> PaneContinuation {
        // TODO

        return PaneContinuation::Continue(());
    }
}
