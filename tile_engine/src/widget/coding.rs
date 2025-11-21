use super::*;
use crate::map_coords::MoveCmd;

// NB: Can we move the specifics ops to ProgPuzz?
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Op {
    F,
    L,
    R,
    x2,
}

impl Op {
    pub fn _d_connector(self) -> bool {
        match self {
            Self::F => true,
            Self::L => true,
            Self::R => true,
            Self::x2 => true,
        }
    }

    pub fn _r_connector(self) -> usize {
        match self {
            Self::F => 0,
            Self::L => 0,
            Self::R => 0,
            Self::x2 => 1,
        }
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl From<&str> for Op {
    fn from(txt: &str) -> Self {
        match txt {
            "F" => Op::F,
            "L" => Op::L,
            "R" => Op::R,
            "x2" => Op::x2,
            _ => panic!("Unrecognised txt for instr: {}", txt)
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instr {
    op: Op,
    subnodes: Vec<Instr>,
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
    pub ops: Vec<Op>,
    pub instrs: Vec<Instr>,
}

impl From<Vec<Op>> for Prog {
    fn from(vec: Vec<Op>) -> Prog {
        Prog {
            ops: vec,
            instrs: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Coding {
    pub supply: Vec<Bin>,
    pub prog: Prog,
}

impl Coding {
    pub fn from_vec(supplies: &[(Op, u16)]) -> Coding {
        Coding {
            supply: supplies.iter().map(|(op,count)|
            Bin::new(*op, *count)
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
