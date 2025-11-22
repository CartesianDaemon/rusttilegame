use super::*;
use crate::map_coords::MoveCmd;

// Can we move the specifics ops to ProgPuzz?
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Op {
    // Could create a separate Enum for the different "types" of instr.
    // Action instrs
    F,
    L,
    R,

    // Control flow instrs
    group,
    x2,
}

impl Op {
    pub fn _d_connector(self) -> bool {
        match self {
            Self::F => true,
            Self::L => true,
            Self::R => true,
            Self::group => true,
            Self::x2 => true,
        }
    }

    pub fn is_action_instr(self) -> bool {
        self.r_connector() == 0
    }

    pub fn is_parent_instr(self) -> bool {
        !self.is_action_instr()
    }

    pub fn r_connector(self) -> usize {
        match self {
            Self::F |
            Self::L |
            Self::R => 0,
            Self::group => 999,
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
    // When used for iteration, counts down. During last iteration it will have value 1.
    pub repeat: usize,
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
    pub fn has_reached_end(&self) -> bool {
        self.next_ip >= self.instrs.len()
    }

    // Currently executing node. Either action instr, or parent instr.
    // Panics if program has stopped.
    fn curr_node(&mut self) -> &mut Node {
        self.instrs.get_mut(self.prev_ip).unwrap()
    }

    // Currently executing op. Action instr from list, or from a parent instr.
    // Panics if program has stopped.
    pub fn curr_op(&self) -> Op {
        let node = self.instrs.get(self.prev_ip).unwrap();
        if node.op.is_action_instr() {
            node.op
        } else {
            assert!(node.op.is_parent_instr());
            node.subnodes.as_ref().unwrap().curr_op()
        }
    }

    // Next action op to execute. (Or panic.)
    pub fn next_op(&self) -> Op {
        let node = self.instrs.get(self.next_ip).unwrap();
        if node.op.is_action_instr() {
            node.op
        } else {
            assert!(node.op.is_parent_instr());
            node.subnodes.as_ref().unwrap().next_op()
        }
    }

    pub fn initialise(&mut self, control_flow_op: Op) {
        self.prev_ip = 0;
        self.next_ip = 0;
        self.repeat = match control_flow_op {
            Op::group => 1,
            Op::x2 => 2,
            _ => panic!(),
        }
    }

    fn advance_own_ip(&mut self) {
        self.next_ip += 1;
        if self.has_reached_end() && self.repeat > 1 {
            self.repeat -= 1;
            self.next_ip = 0;
        }
    }

    fn advance_current_subprog(&mut self, beginning_current_instr: bool) {
        // Example sequence of prev and next ip executing through a group instr.
        // [_*R ,  R,  [_*F,  F  ],  R  ] // do op at *, then advance to next line
        // [ _R , *R,  [_*F,  F  ],  R  ]
        // [  R , _R, *[_*F,  F  ],  R  ]
        // [  R ,  R,_*[_*F,  F  ],  R  ]
        // [  R ,  R,_*[ _F, *F  ],  R  ]
        // [  R ,  R, _[  F, _F *], *R  ]
        // [  R ,  R,  [  F, _F *], _R *]

        let op = self.curr_node().op;
        let subprog = self.curr_node().subnodes.as_mut().unwrap();

        if beginning_current_instr {
            subprog.initialise(op);
        }

        subprog.advance_next_instr();

        if subprog.has_reached_end() {
            self.advance_own_ip();
        }
    }

    // Advances control flow state. Use curr_op() to return basic external op, eg move, rotate.
    // Will panic if we have reached the end of the program.
    pub fn advance_next_instr(&mut self) {
        let beginning_current_instr = self.next_ip != self.prev_ip;
        self.prev_ip = self.next_ip;

        let op = self.curr_node().op;
        if op.is_action_instr() {
            assert!(op.is_action_instr());
            self.advance_own_ip();
        } else if op.is_parent_instr() {
            self.advance_current_subprog(beginning_current_instr);
        } else {
            panic!("Unrecognised category of instr: {}", op);
        }
        assert!(self.curr_op().is_action_instr());
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
