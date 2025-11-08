use super::{PaneContinuation, PaneConclusion};
use crate::engine::input::Input;
use super::BasePane;

// Todo: Could introduce AttemptAction with things like "move 1" and
// "Rotate L/R", returned by a fn on Instr variants and similarly by
// keys in pushpuzz. And interpreted further by an attempt_action fn
// in simple_custom_props which examines passability etc.
#[derive(Clone, Debug)]
enum Instr {
    F,
    L,
    R,
    Loop(Vec<Instr>),
}

#[derive(Clone, Debug)]
struct Bin<T> {
    elem: T,
    orig_count: u16,
    curr_count: u16,
}

#[derive(Clone, Debug)]
struct Flowchart<T> {
    elems: Vec<T>,
}

#[derive(Clone, Debug)]
pub struct Code {
    // Palette of available instructions, array of assembled instructions, etc.
    supply: Vec<Bin<Instr>>,
    prog: Flowchart<Instr>,
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
