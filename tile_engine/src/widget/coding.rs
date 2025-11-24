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
        self.r_connect_max() == 0
    }

    pub fn is_parent_instr(self) -> bool {
        !self.is_action_instr()
    }

    pub fn r_connect_max(self) -> usize {
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

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.op)?;
        if let Some(subprog) = &self.subnodes {
            write!(f, "[{}]", subprog)?;
        }
        Ok(())
    }
}

impl std::ops::Index<usize> for Node {
    type Output = Node;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.subnodes.as_ref().unwrap()[idx]
    }
}

impl std::ops::IndexMut<usize> for Node {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.subnodes.as_mut().unwrap()[idx]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeParent {
    // Index of previous instruction executed. Used for display and knowing when we enter subnodes
    // Or 9999 for "not yet executed"
    pub prev_ip: usize,
    // Index of next instruction to execute.
    pub next_ip: usize,
    // Internal counter, used to implement loops and other stateful instructions.
    // When used for iteration, counts down. During last iteration it will have value 1.
    pub repeat: usize,
    // Vector of one or more instrs to execute. Some parent ops have a specific number of nested instrs.
    pub instrs: Vec<Node>
}

impl Default for NodeParent {
    fn default() -> Self {
        Self {
            prev_ip: 9999,
            next_ip: 0,
            repeat: 0,
            instrs: vec![],
        }
    }
}

impl From<Vec<Op>> for NodeParent {
    fn from(ops: Vec<Op>) -> Self {
        Self {
            instrs: ops.iter().map(|op| Node{op:*op, subnodes:None }).collect(),
            ..Self::default()
        }
    }
}

impl std::fmt::Display for NodeParent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.<1$}[", "", self.repeat)?;
        for (idx, node) in self.instrs.iter().enumerate() {
            if idx >0 {write!(f, ",")?}
            if idx == self.prev_ip {write!(f, "_")?}
            if idx == self.next_ip {write!(f, "*")?}
            write!(f, "{}", node.op)?;
            if node.op.is_parent_instr() {
                write!(f, "{}", node.subnodes.as_ref().unwrap())?;
            }
        }
        if self.next_ip >= self.instrs.len() {write!(f, ",*")?}
        write!(f, "]")
    }
}

impl std::ops::Index<usize> for NodeParent {
    type Output = Node;

    fn index(&self, idx: usize) -> &Self::Output {
        self.instrs.get(idx).unwrap()
    }
}

impl std::ops::IndexMut<usize> for NodeParent {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.instrs.get_mut(idx).unwrap()
    }
}

impl NodeParent {
    pub fn not_begun(&self) -> bool {
        self.prev_ip == 9999
    }

    pub fn has_reached_end(&self) -> bool {
        self.next_ip >= self.instrs.len()
    }

    // Currently executing node. Either action instr, or parent instr.
    // Panics if program has stopped.
    fn curr_node(&mut self) -> &mut Node {
        self.instrs.get_mut(self.prev_ip).unwrap()
    }

    // Next executing node. Either action instr, or parent instr.
    // Panics if program has stopped.
    fn next_node(&mut self) -> &mut Node {
        self.instrs.get_mut(self.next_ip).unwrap()
    }

    // Currently executing op. Action instr from list, or from a parent instr.
    // Panics if program hasn't started yet, or has stopped.
    pub fn curr_op(&self) -> Op {
        let node = if self.not_begun() {
            self.instrs.get(0).unwrap()
        } else {
            self.instrs.get(self.prev_ip).unwrap()
        };
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
        self.prev_ip = 9999;
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

        if !self.has_reached_end() {
            let op = self.next_node().op;
            if op.is_parent_instr() {
                self.next_node().subnodes.as_mut().unwrap().initialise(op);
            }
        }
    }

    fn advance_current_subprog(&mut self) {
        // Example sequence of prev and next ip executing through a group instr.
        // [_*R ,  R,  [_*F,  F  ],  R  ] // do op at *, then advance to next line
        // [ _R , *R,  [_*F,  F  ],  R  ]
        // [  R , _R, *[_*F,  F  ],  R  ]
        // [  R ,  R,_*[_*F,  F  ],  R  ]
        // [  R ,  R,_*[ _F, *F  ],  R  ]
        // [  R ,  R, _[  F, _F *], *R  ]
        // [  R ,  R,  [  F, _F *], _R *]

        let subprog = self.curr_node().subnodes.as_mut().unwrap();

        subprog.advance_next_instr();

        if subprog.has_reached_end() {
            self.advance_own_ip();
        }
    }

    // Advances control flow state. Use curr_op() to return basic external op, eg move, rotate.
    // Will panic if we have reached the end of the program.
    pub fn advance_next_instr(&mut self) {
        if self.prev_ip == 9999 {
            if let Some(node) = self.instrs.get_mut(0) {
                if node.op.is_parent_instr() {
                    node.subnodes.as_mut().unwrap().initialise(node.op);
                }
            }
        }
        self.prev_ip = self.next_ip;

        let op = self.curr_node().op;
        if op.is_action_instr() {
            assert!(op.is_action_instr());
            self.advance_own_ip();
        } else if op.is_parent_instr() {
            self.advance_current_subprog();
        } else {
            panic!("Unrecognised category of instr: {}", op);
        }
        assert!(self.curr_op().is_action_instr());
        log::debug!("Advanced prog to {}.", self); // to #{}. Next: #{}.", self, self.prev_ip, self.next_ip);
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

#[cfg(test)]
mod tests {
    use super::*;
    use Op::*;

    fn run_prog_and_test(mut prog: Prog, expected_ops: &[Op]) {
        for (idx, expected_op) in expected_ops.iter().enumerate() {
            prog.advance_next_instr();
            assert_eq!(prog.curr_op(), *expected_op, "At idx {} of {}", idx, prog);
        }
        assert!(prog.has_reached_end());
    }

    #[test]
    fn test_linear_prog() {
        run_prog_and_test(Prog::from(vec![F,F,R,F]), &[F, F, R, F]);
    }

    #[test]
    fn test_simple_repeat() {
        let mut prog = Prog::from(vec![L, x2, L]);
        prog.instrs[1].subnodes = Some(Prog::from(vec![F, R]));
        run_prog_and_test(prog, &[L,F,R,F,R,L]);
    }

    #[test]
    fn test_bare_repeat() {
        let mut prog = Prog::from(vec![x2]);
        prog.instrs[0].subnodes = Some(Prog::from(vec![F]));
        run_prog_and_test(prog, &[F, F]);
    }

    #[test]
    fn test_bare_nested_repeat() {
        let mut prog = Prog::from(vec![x2]);
        prog.instrs[0].subnodes = Some(Prog::from(vec![x2]));
        prog.instrs[0].subnodes.as_mut().unwrap().instrs[0].subnodes = Some(Prog::from(vec![F]));
        run_prog_and_test(prog, &[F, F, F, F]);
    }

    #[test]
    fn test_twice_nested_repeat() {
        let mut prog = Prog::from(vec![x2]);
        prog.instrs[0].subnodes = Some(Prog::from(vec![x2, x2]));
        prog.instrs[0].subnodes.as_mut().unwrap().instrs[0].subnodes = Some(Prog::from(vec![F]));
        prog.instrs[0].subnodes.as_mut().unwrap().instrs[1].subnodes = Some(Prog::from(vec![R]));
        run_prog_and_test(prog, &[F, F, R, R, F, F, R, R]);
    }
}
