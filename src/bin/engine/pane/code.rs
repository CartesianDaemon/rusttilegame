use super::{PaneContinuation, PaneConclusion};
use crate::engine::input::Input;
use super::BasePane;

use std::collections::HashMap;

fn txt_to_instr(txt: &str) -> Instr {
    match txt {
        "F" => Instr::F,
        "L" => Instr::L,
        "R" => Instr::R,
        "Loop" => Instr::Loop(vec![]),
        _ => panic!("Unrecognised txt for instr")
    }
}

fn instr_to_txt(instr: &Instr) -> String {
    match instr {
        Instr::F => "F",
        Instr::L => "L",
        Instr::R => "R",
        Instr::Loop(_) => "Loop",
    }.to_string()
}

// Todo: Could introduce AttemptAction with things like "move 1" and
// "Rotate L/R", returned by a fn on Instr variants and similarly by
// keys in pushpuzz. And interpreted further by an attempt_action fn
// in simple_custom_props which examines passability etc.
// Breadcrumb: Could implement to_txt and txt_to in terms of common trait.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

impl Supply {
    fn new(orig_count: u16) -> Self {
        Self {
            orig_count: orig_count,
            curr_count: orig_count,
        }
    }
}

// Breadcrumb: Derive for implementing default value?
#[derive(Clone, Debug, Default)]
pub struct Prog {
    elems: Vec<Instr>,
}

#[derive(Clone, Debug)]
pub struct Code {
    // TODO: Need IndexMap or Vec to maintain order.
    supplies: HashMap<Instr, Supply>,
    prog: Prog,
}

impl Code {
    fn from_ascii(supplies: HashMap<String, u16>) -> Code {
        Code {
            supplies: supplies.iter().map(|(txt,count)|
                (txt_to_instr(&txt),Supply::new(*count))
            ).collect(),
            prog: Prog::default(),
        }
    }
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
