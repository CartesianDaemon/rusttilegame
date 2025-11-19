use super::*;
use crate::map_coords::MoveCmd;

use std::collections::HashMap;

// NB: Can we move the specifics ops to ProgPuzz?
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Op {
    F,
    L,
    R,
    x2,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instr {
    F,
    L,
    R,
    x2(Box<[Instr;2]>),
}

impl ToString for Op {
    fn to_string(&self) -> String {
        match self {
            Op::F => "F",
            Op::L => "L",
            Op::R => "R",
            Op::x2 => "x2",
        }.to_string()
    }
}

impl From<&str> for Op {
    fn from(txt: &str) -> Self {
        match txt {
            "F" => Op::F,
            "L" => Op::L,
            "R" => Op::R,
            "x2" => Op::x2,
            _ => panic!("Unrecognised txt for instr")
        }
    }
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
    pub fn from(prog_txt: &str) -> Prog {
        Prog {
            // NB: Try using my chain crate
            instrs: prog_txt.split_terminator(',').map(|op_txt| Op::from(op_txt)).collect()
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
                Bin::new(Op::from(*txt), *count)
            ).collect(),
            prog: Prog::default(),
        }
    }
}

impl BaseWidget for Coding
{
    fn advance(&mut self, _cmd: MoveCmd) -> PaneContinuation {
        // TODO

        return PaneContinuation::Continue(());
    }

    fn tick_based(&self) -> crate::ui::TickStyle {
        crate::ui::TickStyle::Continuous
    }
}
