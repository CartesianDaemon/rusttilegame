use crate::pane::*;
use crate::map_coords::Cmd;

use std::collections::HashMap;

// NB: Nice to move to progpuzz if we can.
// NB: Could be combined with putative AttemptedAction defined for Cmd.
// Breadcrumb: Could implement to_txt and txt_to in terms of common trait.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Instr {
    F,
    L,
    R,
    // NB: We're going to need to box this before we instantiate it anywhere, right?
    Loop(Vec<Instr>),
}

fn txt_to_instr(txt: &str) -> Instr {
    match txt {
        "F" => Instr::F,
        "L" => Instr::L,
        "R" => Instr::R,
        "Loop" => Instr::Loop(vec![]),
        _ => panic!("Unrecognised txt for instr")
    }
}

fn _instr_to_txt(instr: &Instr) -> String {
    match instr {
        Instr::F => "F",
        Instr::L => "L",
        Instr::R => "R",
        Instr::Loop(_) => "Loop",
    }.to_string()
}

#[derive(Clone, Debug)]
pub struct Supply {
    _orig_count: u16,
    _curr_count: u16,
}

impl Supply {
    fn new(_orig_count: u16) -> Self {
        Self {
            _orig_count: _orig_count,
            _curr_count: _orig_count,
        }
    }
}

// Breadcrumb: Derive for implementing default value?
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Prog {
    pub instrs: Vec<Instr>,
}

impl Prog {
    // E.g. from("F,F,R,Loop")
    pub fn from(txt: &str) -> Prog {
        Prog {
            // NB: Try using my chain crate
            instrs: txt.split_terminator(',').map(|x| txt_to_instr(x)).collect()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Code {
    // TODO: Need IndexMap or Vec to maintain order.
    pub supplies: HashMap<Instr, Supply>,
    pub prog: Prog,
}

impl Code {
    pub fn from_ascii(supplies: HashMap<&str, u16>) -> Code {
        Code {
            supplies: supplies.iter().map(|(txt,count)|
                (txt_to_instr(&txt),Supply::new(*count))
            ).collect(),
            prog: Prog::from("F,F,R,F"),
        }
    }
}

impl BasePane for Code
{
    fn advance(&mut self, _cmd: Option<Cmd>) -> PaneContinuation {
        // TODO

        return PaneContinuation::Continue(());
    }

    fn tick_based(&self) -> bool {
        false
    }
}
