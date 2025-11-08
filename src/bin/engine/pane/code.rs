use super::{PaneContinuation, PaneConclusion};
use crate::engine::input::Input;
use super::BasePane;

use std::collections::HashMap;

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
struct Supply {
    orig_count: u16,
    curr_count: u16,
}

#[derive(Clone, Debug)]
struct Flowchart<T> {
    elems: Vec<T>,
}

#[derive(Clone, Debug)]
pub struct Code {
    // TODO: Need IndexMap or Vec to maintain order.
    supplies: HashMap<Instr, Supply>,
    prog: Flowchart<Instr>,
}

impl Code {
    // `fn from_ascii(txt: String) -> Code {
    // `    Code {
    // `        resources: vec![],
    // `        prog: Flowchart { elems: vec![] },
    // `    }
    // `}
}

impl BasePane for Code
{
    fn advance(&mut self, _input: &mut Input) -> PaneContinuation {
        // TODO

        return PaneContinuation::Continue(());
    }

    fn need_sync_to_ticks(&self) -> bool {
        false
    }
}
