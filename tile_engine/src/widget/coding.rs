use super::*;
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

pub fn instr_to_txt(instr: &Instr) -> String {
    match instr {
        Instr::F => "F",
        Instr::L => "L",
        Instr::R => "R",
        Instr::Loop(_) => "Loop",
    }.to_string()
}

#[derive(Clone, Debug)]
pub struct Bin {
    pub orig_count: u16,
    pub curr_count: u16,
}

impl Bin {
    fn new(_orig_count: u16) -> Self {
        Self {
            orig_count: _orig_count,
            curr_count: _orig_count,
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
pub struct Coding {
    // TODO: Need IndexMap or Vec to maintain order.
    pub supplies: HashMap<Instr, Bin>,
    pub prog: Prog,
}

impl Coding {
    pub fn from_ascii(supplies: HashMap<&str, u16>) -> Coding {
        Coding {
            supplies: supplies.iter().map(|(txt,count)|
                (txt_to_instr(&txt),Bin::new(*count))
            ).collect(),
            prog: Prog::default(),
        }
    }
}

impl BaseWidget for Coding
{
    fn advance(&mut self, _cmd: Option<Cmd>) -> PaneContinuation {
        // TODO

        return PaneContinuation::Continue(());
    }

    fn tick_based(&self) -> bool {
        false
    }
}
