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
    loop5,
}

impl Op {
    pub fn _d_connector(self) -> bool {
        match self {
            Self::F => true,
            Self::L => true,
            Self::R => true,
            Self::group => true,
            Self::x2 => true,
            Self::loop5 => true,
        }
    }

    pub fn is_action_instr(self) -> bool {
        !self.is_parent_instr()
    }

    pub fn is_parent_instr(self) -> bool {
        self.r_connect_max() > 0
    }

    pub fn r_connect_max(self) -> usize {
        match self {
            Self::F |
            Self::L |
            Self::R => 0,
            Self::group => 999,
            Self::x2 => 1,
            Self::loop5 => 5,
        }
    }

    pub fn repeat_count(self) -> usize {
        match self {
            Op::group => 1,
            Op::x2 => 2,
            Op::loop5 => 5,
            _ => panic!("Repeat count not specified for non-parent instr"),
        }
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[cfg(any())]
impl From<&str> for Op {
    fn from(txt: &str) -> Self {
        match txt {
            "F" => Op::F,
            "L" => Op::L,
            "R" => Op::R,
            "{}" => Op::group,
            "x2" => Op::x2,
            "loop" => Op::loop5,
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

/// An instruction as it exists in a specific program, including subprog and current state.
///
/// Could go back to calling this "Instr" not "Node".
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub op: Op,
    pub subnodes: Option<Subprog>,
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

impl Node {
    pub fn v_len(&self) -> usize {
        match &self.subnodes {
            None => 1,
            Some(subprog) => subprog.v_len(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Subprog {
    // Index of instruction currently executing. 0 when program has not started.
    pub curr_ip: usize,
    // Internal counter, used to implement loops and other stateful instructions.
    // When used for iteration, counts number of times current execution of parent instr has executed this subprog.
    pub counter: usize,
    // Vector of one or more instrs to execute. Some parent ops have a specific number of nested instrs.
    pub instrs: Vec<Node>
}

impl From<Vec<Op>> for Subprog {
    fn from(ops: Vec<Op>) -> Self {
        Self {
            instrs: ops.iter().map(|op| Node{op:*op, subnodes:None }).collect(),
            ..Self::default()
        }
    }
}

impl std::fmt::Display for Subprog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.<1$}[", "", self.counter)?;
        for (idx, node) in self.instrs.iter().enumerate() {
            if idx >0 {write!(f, ",")?}
            if idx == self.curr_ip {write!(f, "*")?}
            write!(f, "{}", node.op)?;
            if node.op.is_parent_instr() {
                write!(f, "{}", node.subnodes.as_ref().unwrap())?;
            }
        }
        write!(f, "]")
    }
}

impl std::ops::Index<usize> for Subprog {
    type Output = Node;

    fn index(&self, idx: usize) -> &Self::Output {
        self.instrs.get(idx).unwrap()
    }
}

impl std::ops::IndexMut<usize> for Subprog {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.instrs.get_mut(idx).unwrap()
    }
}

impl Subprog {
    // Number of instructions within if laid out vertically. Used for drawing.
    // Always at least 1.
    pub fn v_len(&self) -> usize {
        std::cmp::max(1, self.instrs.iter().map(|node| node.v_len()).sum())
    }

    // Currently executing op. Action instr from list, or from a parent instr.
    // None for an empty program, or when program reaches an empty parent instr.
    pub fn curr_op(&self) -> Option<Op> {
        if self.instrs.len() == 0 {
            return None;
        } else {
            assert!(self.curr_ip < self.instrs.len());
        }
        let node = self.instrs.get(self.curr_ip).unwrap();
        if node.op.is_action_instr() {
            Some(node.op)
        } else {
            assert!(node.op.is_parent_instr());
            node.subnodes.as_ref().unwrap().curr_op()
        }
    }

    pub fn unwrap_curr_op(&self) -> Op {
        self.curr_op().unwrap()
    }

    // Advances curr_ip. Returns Some, unless program wraps round.
    fn advance_ip(&mut self) -> Option<()> {
        if self.curr_ip + 1 < self.instrs.len() {
            self.curr_ip += 1;
            Some(())
        } else {
            self.curr_ip = 0;
            None
        }
    }

    fn advance_current_subprog(&mut self, parent_op: Op) -> Option<()> {
        let subprog = self.instrs.get_mut(self.curr_ip).unwrap().subnodes.as_mut().unwrap();
        match subprog.advance_next_instr() {
            Some(()) => Some(()),
            None => {
                if subprog.counter + 1 < parent_op.repeat_count() {
                    subprog.counter += 1;
                    Some(())
                } else {
                    subprog.counter = 0;
                    self.advance_ip()
                }
            }
        }
    }

    // Advances control flow state.
    //
    // Advances into parent instructions. Except stops at empty parent instructions, when
    // curr_op() will return None.
    //
    // Returns Some(), or None if program wrapped round.
    pub fn advance_next_instr(&mut self) -> Option<()> {
        if self.instrs.len() == 0 {
            return None;
        }

        let op = self.instrs.get_mut(self.curr_ip).unwrap().op;
        if op.is_action_instr() {
            self.advance_ip()?;
        } else if op.is_parent_instr() {
            self.advance_current_subprog(op)?;
        } else {
            panic!("Unrecognised category of instr: {}", op);
        }
        assert!(self.curr_op().is_none() || self.curr_op().unwrap().is_action_instr());
        log::debug!("Advanced prog to {}.", self); // to #{}. Next: #{}.", self, self.prev_ip, self.next_ip);
        Some(())
    }
}

pub use Subprog as Prog;

#[derive(Clone, Debug)]
pub struct Coding {
    pub supply: Vec<Bin>,
    pub prog: Subprog,
}

impl Coding {
    pub fn from_vec(supplies: &[(Op, u16)]) -> Coding {
        Coding {
            supply: supplies.iter().map(|(op,count)|
            Bin::new(*op, *count)
            ).collect(),
            prog: Subprog::default(),
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
    use crate::infra::initialise_logging_for_tests;
    use super::*;
    use Op::*;

    fn run_prog_and_test(mut prog: Prog, expected_ops: &[Op]) {
        for (idx, expected_op) in expected_ops.iter().enumerate() {
            assert_eq!(prog.curr_op().unwrap(), *expected_op, "At idx {} of {}", idx, prog);
            let cont = prog.advance_next_instr();
            if idx < expected_ops.len() - 1 {
                assert!(cont == Some(()));
            } else {
                assert!(cont == None);
            }
        }
    }

    #[test]
    fn test_linear_prog() {
        initialise_logging_for_tests();
        run_prog_and_test(Prog::from(vec![F,F,R,F]), &[F, F, R, F]);
    }

    #[test]
    fn test_simple_repeat() {
        initialise_logging_for_tests();
        let mut prog = Prog::from(vec![L, x2, L]);
        prog[1].subnodes = Some(Prog::from(vec![F, R]));
        run_prog_and_test(prog, &[L,F,R,F,R,L]);
    }

    #[test]
    fn test_bare_repeat() {
        initialise_logging_for_tests();
        let mut prog = Prog::from(vec![x2]);
        prog[0].subnodes = Some(Prog::from(vec![F]));
        run_prog_and_test(prog, &[F, F]);
    }

    #[test]
    fn test_bare_nested_repeat() {
        initialise_logging_for_tests();
        let mut prog = Prog::from(vec![x2]);
        prog[0].subnodes = Some(Prog::from(vec![x2]));
        prog[0][0].subnodes = Some(Prog::from(vec![F]));
        run_prog_and_test(prog, &[F, F, F, F]);
    }

    #[test]
    fn test_twice_nested_repeat() {
        initialise_logging_for_tests();
        let mut prog = Prog::from(vec![x2]);
        prog[0].subnodes = Some(Prog::from(vec![x2, x2]));
        prog[0][0].subnodes = Some(Prog::from(vec![F]));
        prog[0][1].subnodes = Some(Prog::from(vec![R]));
        run_prog_and_test(prog, &[F, F, R, R, F, F, R, R]);
    }

    #[test]
    fn test_nested_repeat_two_instr() {
        initialise_logging_for_tests();
        let mut prog = Prog::from(vec![x2]);
        prog[0].subnodes = Some(Prog::from(vec![x2]));
        prog[0][0].subnodes = Some(Prog::from(vec![L, R]));
        run_prog_and_test(prog, &[L, R, L, R, L, R, L, R, ]);
    }

    #[test]
    fn test_repeat_nested_group() { // x2(group(x2(F), R))
        initialise_logging_for_tests();
        let mut prog = Prog::from(vec![x2]);
        prog[0].subnodes = Some(Prog::from(vec![group]));
        prog[0][0].subnodes = Some(Prog::from(vec![x2, R]));
        prog[0][0][0].subnodes = Some(Prog::from(vec![F]));
        run_prog_and_test(prog, &[F, F, R, F, F, R]);
    }

    #[test]
    fn test_f_then_nested_repeat_two_instr() {
        initialise_logging_for_tests();
        let mut prog = Prog::from(vec![F, x2]);
        prog[1].subnodes = Some(Prog::from(vec![x2]));
        prog[1][0].subnodes = Some(Prog::from(vec![L, R]));
        run_prog_and_test(prog, &[F, L, R, L, R, L, R, L, R, ]);
    }
}
