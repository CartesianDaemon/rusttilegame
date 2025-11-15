use super::*;
use crate::map_coords::Cmd;

use std::collections::HashMap;

// NB: Nice to move to progpuzz if we can.
// NB: Could be combined with putative AttemptedAction defined for Cmd.
// Breadcrumb: Could implement to_txt and txt_to in terms of common trait.
// NB: Need enum to be Instr including subsiduary values, and then define
// Op in terms of that, in terms of pure index.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Op {
    F,
    L,
    R,
    // NB: We're going to need to box this before we instantiate it anywhere, right?
    // Loop(Vec<Instr>),
}

fn txt_to_op(txt: &str) -> Op {
    match txt {
        "F" => Op::F,
        "L" => Op::L,
        "R" => Op::R,
        // "Loop" => Op::Loop(vec![]),
        _ => panic!("Unrecognised txt for instr")
    }
}

pub fn op_to_txt(instr: &Op) -> String {
    match instr {
        Op::F => "F",
        Op::L => "L",
        Op::R => "R",
        // Op::Loop(_) => "Loop",
    }.to_string()
}

#[derive(Clone, Debug)]
pub struct Bin {
    pub op: Op,
    pub orig_count: u16,
    pub curr_count: u16,
}

impl Bin {
    fn new(op: Op, orig_count: u16) -> Self {
        Self {
            op,
            orig_count,
            curr_count: orig_count,
        }
    }

    pub fn put(&mut self) -> Result<(), ()> {
        if self.curr_count < self.orig_count {
            self.curr_count +=1;
            Result::Ok(())
        } else {
            Result::Err(())
        }
    }
}

// Breadcrumb: Derive for implementing default value?
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Prog {
    pub instrs: Vec<Op>,
}

impl Prog {
    // E.g. from("F,F,R,Loop")
    pub fn from(txt: &str) -> Prog {
        Prog {
            // NB: Try using my chain crate
            instrs: txt.split_terminator(',').map(|x| txt_to_op(x)).collect()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Coding {
    // TODO: Need IndexMap or Vec to maintain order.
    pub supply: Vec<Bin>,
    pub prog: Prog,
}

impl Coding {
    pub fn from_ascii(supplies: HashMap<&str, u16>) -> Coding {
        Coding {
            supply: supplies.iter().map(|(txt,count)|
                Bin::new(txt_to_op(&txt), *count)
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
