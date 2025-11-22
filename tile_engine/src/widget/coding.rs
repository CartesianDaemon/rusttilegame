use super::*;
use crate::map_coords::MoveCmd;

// NB: Can we move the specifics ops to ProgPuzz?
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Op {
    F,
    L,
    R,
    group,
}

impl Op {
    pub fn _d_connector(self) -> bool {
        match self {
            Self::F => true,
            Self::L => true,
            Self::R => true,
            Self::group => true,
        }
    }

    pub fn _r_connector(self) -> usize {
        match self {
            Self::F => 0,
            Self::L => 0,
            Self::R => 0,
            Self::group => 1,
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
            "{}" => Op::group,
            // "x2" => Op::x2,
            _ => panic!("Unrecognised txt for instr: {}", txt)
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub op: Op,
    pub subnodes: Option<NodeParent>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NodeParent {
    // Index of previous instruction executed. Used for display and knowing when we enter subnodes
    pub prev_ip: usize,
    // Index of next instruction to execute.
    pub next_ip: usize,
    // Internal counter, used to implement loops and other stateful instructions.
    pub count: usize,
    // Vector of one or more instrs to execute. Some parent ops have a specific number of nested instrs.
    pub instrs: Vec<Node>
}

impl From<Vec<Op>> for NodeParent {
    fn from(ops: Vec<Op>) -> Self {
        Self {
            instrs: ops.iter().map(|op| Node{op:*op, subnodes:None }).collect(),
            ..Self::default()
        }
    }
}

impl NodeParent {
    pub fn prev_node(&mut self) -> Option<&mut Node> {
        self.instrs.get_mut(self.prev_ip)
    }

    // pub fn op(&self, idx: usize) -> Option<Op> {
    //     self.instrs.get(idx).map(|node| node.op)
    // }

    // Returns None if we run off end of program.
    // Else advances control flow state and returns next basic external op, eg move, rotate.
    pub fn advance_next_instr(&mut self) -> Option<Op> {
        self.prev_ip = self.next_ip;
        use Op::*;
        match self.prev_node()?.op {
            // Control flow instrs
            group => {
                let op = self.prev_node()?.subnodes.as_mut().unwrap().advance_next_instr();
                match op {
                    Some(_) => op,
                    None => {
                        self.next_ip += 1;
                        self.advance_next_instr()
                    }
                }
            }
            // Regular instrs
            _ => {
                self.next_ip += 1;
                Some(self.prev_node()?.op)
            },
        }
        // (self.prev_ip, self.next_ip) = (self.next_ip, self.next_ip + 1);

        // self.instrs.get(self.prev_ip).map(|node| node.op)
    }
}

pub use NodeParent as Prog;

#[derive(Clone, Debug)]
pub struct Coding {
    pub supply: Vec<Bin>,
    pub prog: NodeParent,
}

impl Coding {
    pub fn from_vec(supplies: &[(Op, u16)]) -> Coding {
        Coding {
            supply: supplies.iter().map(|(op,count)|
            Bin::new(*op, *count)
            ).collect(),
            prog: NodeParent::default(),
        }
    }
}

impl BaseWidget for Coding
{
    fn advance(&mut self, _cmd: MoveCmd) -> WidgetContinuation {
        // TODO

        return WidgetContinuation::Continue(());
    }

    fn tick_based(&self) -> crate::ui::TickStyle {
        crate::ui::TickStyle::Continuous
    }
}
