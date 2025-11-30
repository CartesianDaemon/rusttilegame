use super::*;
use crate::map_coords::MoveCmd;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ActionData {
    pub successful: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ActionOp {
    F,
    L,
    R,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParentOp {
    group,
    x2,
    loop5,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Op {
    Action(ActionOp),
    // TODO: Want to merge Subprog into here. Remove Node as separate type.
    // Although, could have Op (with no data) and Instr (with data)..
    Parent(ParentOp),
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Op::*;
        match self {
            Action(op) => std::fmt::Debug::fmt(op, f),
            Parent(op) => std::fmt::Debug::fmt(op, f),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Instr {
    Action(ActionOp, ActionData),
    // TODO: Want to merge Subprog into here. Remove Node as separate type.
    // Although, could have Op (with no data) and Instr (with data)..
    Parent(ParentOp),
}

impl Instr {
    pub fn from_op(op: Op) -> Self {
        match op {
            Op::Action(action_op) => Self::Action(action_op, ActionData::default()),
            Op::Parent(parent_op) => Self::Parent(parent_op),
        }
    }

    pub fn is_op(&self, op: Op) -> bool {
        match self {
            Instr::Action(opcode_a, _) => matches!(&op, Op::Action(opcode_b) if opcode_a == opcode_b ),
            Instr::Parent(opcode_a) => matches!(&op, Op::Parent(opcode_b) if opcode_a == opcode_b ),
        }
    }

    pub fn _d_connector(self) -> bool {
        use Instr::*;
        match self {
            Action(_, _) => true,
            Parent(_) => true,
        }
    }

    // TODO: Replace with match?
    pub fn is_action_instr(self) -> bool {
        !self.is_parent_instr()
    }

    // TODO: Replace with match?
    pub fn is_parent_instr(self) -> bool {
        self.r_connect_max() > 0
    }

    pub fn r_connect_max(self) -> usize {
        use Instr::*;
        use ParentOp::*;
        match self {
            Action(_, _) => 0,
            Parent(group) => 999,
            Parent(x2) => 1,
            Parent(loop5) => 5,
        }
    }

    // TODO: Move to fn of ControlFlowOp not Op.
    pub fn repeat_count(self) -> usize {
        use Instr::*;
        use ParentOp::*;
        match self {
            Action(_, _) => panic!("Repeat count not specified for non-parent instr"),
            Parent(group) => 1,
            Parent(x2) => 2,
            Parent(loop5) => 5,
        }
    }
}

impl std::fmt::Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Instr::*;
        match self {
            Action(op, _) => std::fmt::Debug::fmt(op, f),
            Parent(op) => std::fmt::Debug::fmt(op, f),
        }
    }
}

impl From<&str> for Instr {
    fn from(txt: &str) -> Self {
        match txt {
            "F" => Instr::Action(ActionOp::F, ActionData::default()),
            "L" => Instr::Action(ActionOp::L, ActionData::default()),
            "R" => Instr::Action(ActionOp::R, ActionData::default()),
            "group" => Instr::Parent(ParentOp::group),
            "x2" => Instr::Parent(ParentOp::x2),
            "loop5" => Instr::Parent(ParentOp::loop5),
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
    pub instr: Instr,
    pub subnodes: Option<Subprog>,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.instr)?;
        if let Some(subprog) = &self.subnodes {
            write!(f, "[{}]", subprog)?;
        }
        Ok(())
    }
}

impl std::ops::Index<i16> for Node {
    type Output = Node;

    fn index(&self, idx: i16) -> &Self::Output {
        &self.subnodes.as_ref().unwrap()[idx]
    }
}

impl std::ops::IndexMut<i16> for Node {
    fn index_mut(&mut self, idx: i16) -> &mut Self::Output {
        &mut self.subnodes.as_mut().unwrap()[idx]
    }
}

impl Node {
    pub fn from_op(op: Op) -> Self {
        Self {
            instr: Instr::from_op(op),
            subnodes: match op {Op::Action(_) => None, Op::Parent(_) => Some(Subprog::default())},
        }
    }

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

impl From<Vec<Instr>> for Subprog {
    fn from(ops: Vec<Instr>) -> Self {
        Self {
            instrs: ops.iter().map(|op| Node{instr:*op, subnodes:None }).collect(),
            ..Self::default()
        }
    }
}

#[cfg(any())]
impl<T: Iterator<Item=Instr>> From<T> for Subprog {
    fn from(ops: T) -> Self {
        Self {
            instrs: ops.map(|op| Node{op:*op, subnodes:None }).collect(),
            ..Self::default()
        }
    }
}

impl From<&str> for Subprog {
    fn from(txt: &str) -> Self {
        let ops : Vec<Instr> = txt.split(",").map(|s| s.trim().into()).collect();
        Self::from(ops)
    }
}

impl std::fmt::Display for Subprog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.<1$}[", "", self.counter)?;
        for (idx, node) in self.instrs.iter().enumerate() {
            if idx >0 {write!(f, ",")?}
            if idx == self.curr_ip {write!(f, "*")?}
            write!(f, "{}", node.instr)?;
            if node.instr.is_parent_instr() {
                write!(f, "{}", node.subnodes.as_ref().unwrap())?;
            }
        }
        write!(f, "]")
    }
}

impl std::ops::Index<i16> for Subprog {
    type Output = Node;

    fn index(&self, idx: i16) -> &Self::Output {
        if idx >= 0 {
            self.instrs.get(idx as usize).unwrap()
        } else {
            for node in &self.instrs {
                if let Some(subnodes) = &node.subnodes && subnodes.instrs.len() == 0 {
                    return node;
                }
            }
            panic!();
        }
    }
}

// Returns n'th node in subprog, if any subprog exists.
// If -1, returns first empty parent node, else panics
impl std::ops::IndexMut<i16> for Subprog {
    fn index_mut(&mut self, idx: i16) -> &mut Self::Output {
        if idx >= 0 {
            self.instrs.get_mut(idx as usize).unwrap()
        } else {
            for node in &mut self.instrs {
                if node.instr.is_parent_instr() && (
                        node.subnodes.is_none() || node.subnodes.as_ref().unwrap().instrs.len() == 0
                    ) {
                    return node;
                }
            }
            panic!();
        }
    }
}

impl Subprog {
    // Number of instructions within if laid out vertically. Used for drawing.
    // Always at least 1.
    pub fn v_len(&self) -> usize {
        std::cmp::max(1, self.instrs.iter().map(|node| node.v_len()).sum())
    }

    pub fn finished(&self) -> bool {
        self.curr_ip >= self.instrs.len()
    }

    // Currently executing op. Action instr from list, or from a parent instr.
    // None when past end of program, or when program reaches an empty parent instr.
    pub fn curr_op(&self) -> Option<Instr> {
        if self.curr_ip >= self.instrs.len() {
            None
        } else {
            let node = self.instrs.get(self.curr_ip).unwrap();
            if node.instr.is_action_instr() {
                Some(node.instr)
            } else {
                assert!(node.instr.is_parent_instr());
                node.subnodes.as_ref().unwrap().curr_op()
            }
        }
    }

    pub fn curr_op_mut(&mut self) -> Option<Instr> {
        if self.curr_ip >= self.instrs.len() {
            None
        } else {
            let node = self.instrs.get_mut(self.curr_ip).unwrap();
            if node.instr.is_action_instr() {
                Some(node.instr)
            } else {
                assert!(node.instr.is_parent_instr());
                node.subnodes.as_mut().unwrap().curr_op()
            }
        }
    }

    pub fn unwrap_curr_op(&self) -> Instr {
        self.curr_op().unwrap()
    }

    fn advance_ip(&mut self) {
        self.curr_ip += 1;
    }

    fn reset(&mut self) {
        self.curr_ip = 0;
        self.counter = 0;
    }

    fn iterate(&mut self) {
        self.curr_ip = 0;
        self.counter += 1;
    }

    fn advance_current_subprog(&mut self, parent_op: Instr) {
        let subprog = self.instrs.get_mut(self.curr_ip).unwrap().subnodes.as_mut().unwrap();
        subprog.advance_next_instr();
        if subprog.finished() {
            if subprog.counter + 1 < parent_op.repeat_count() {
                subprog.iterate();
            } else {
                subprog.reset();
                self.advance_ip();
            }
        }
    }

    // Advances control flow state.
    //
    // Advances into parent instructions. Except stops at empty parent instructions, when
    // curr_op() will return None.
    //
    // Returns Some(), or None if program wrapped round.
    pub fn advance_next_instr(&mut self) {
        if self.finished() {
            self.reset();
            return;
        }

        let op = self.instrs.get_mut(self.curr_ip).unwrap().instr;
        if op.is_action_instr() {
            self.advance_ip();
        } else if op.is_parent_instr() {
            self.advance_current_subprog(op);
        } else {
            panic!("Unrecognised category of instr: {}", op);
        }
        assert!(self.curr_op().is_none() || self.curr_op().unwrap().is_action_instr());
        log::debug!("Advanced prog to {}.", self); // to #{}. Next: #{}.", self, self.prev_ip, self.next_ip);
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

pub mod action_ops {
    #![allow(non_upper_case_globals)]
    use super::*;

    pub const F: ActionOp = ActionOp::F;
    pub const L: ActionOp = ActionOp::L;
    pub const R: ActionOp = ActionOp::R;
}

pub mod supply_ops {
    #![allow(non_upper_case_globals)]
    use super::*;

    pub const F: Op = Op::Action(ActionOp::F);
    pub const L: Op = Op::Action(ActionOp::L);
    pub const R: Op = Op::Action(ActionOp::R);

    pub const x2: Op = Op::Parent(ParentOp::x2);
    pub const group: Op = Op::Parent(ParentOp::group);
    pub const loop5: Op = Op::Parent(ParentOp::loop5);
}

pub mod prog_ops {
    #![allow(non_upper_case_globals)]
    use super::*;

    pub const a: ActionData = ActionData { successful: false};
    pub const F: Instr = Instr::Action(ActionOp::F, a);
    pub const L: Instr = Instr::Action(ActionOp::L, a);
    pub const R: Instr = Instr::Action(ActionOp::R, a);

    // TODO: Introduce fn if we first subsume Subprog into ParentOp
    // pub fn x2(ops: Vec<Op>) -> Op = Op::Parent(ParentOp::x2);

    pub const x2: Instr = Instr::Parent(ParentOp::x2);
    pub const group: Instr = Instr::Parent(ParentOp::group);
    pub const loop5: Instr = Instr::Parent(ParentOp::loop5);
}

#[cfg(test)]
mod tests {
    use crate::infra::initialise_logging_for_tests;
    use super::*;

    #[test]
    fn parse() {
        use prog_ops::*;
        assert_eq!(Prog::from("F"), Prog::from(vec![F]));
        assert_eq!(Prog::from("F "), Prog::from(vec![F]));
        assert_eq!(Prog::from(" F "), Prog::from(vec![F]));
        assert_eq!(Prog::from("F,R"), Prog::from(vec![F, R]));
        assert_eq!(Prog::from("F, R"), Prog::from(vec![F, R]));
        assert_eq!(Prog::from("F ,R"), Prog::from(vec![F, R]));
        assert_eq!(Prog::from("F ,R "), Prog::from(vec![F, R]));
        assert_eq!(Prog::from(" F ,R    , L"), Prog::from(vec![F, R, L]));
        assert_eq!(Prog::from("F,R,L,x2,group,loop5"), Prog::from(vec![F, R, L, x2, group, loop5]));
    }

    fn run_prog_and_test(mut prog: Prog, expected_ops: &[ActionOp]) {
        for (idx, expected_op) in expected_ops.iter().enumerate() {
            assert!(!prog.finished());
            assert!(matches!(prog.curr_op(), Some(Instr::Action(op, _)) if op==*expected_op) , "At idx {} of {}", idx, prog);
            prog.advance_next_instr();
        }
        assert!(prog.finished());
    }

    use action_ops::*;

    #[test]
    fn test_linear_prog() {
        initialise_logging_for_tests();
        run_prog_and_test(Prog::from("F,F,R,F"), &[F, F, R, F]);
    }

    #[test]
    fn test_simple_repeat() {
        initialise_logging_for_tests();
        let mut prog = Prog::from("L,x2,L");
        prog[-1].subnodes = Some(Prog::from("F,R"));
        run_prog_and_test(prog, &[L, F, R, F, R, L]);
    }

    #[test]
    fn test_bare_repeat() {
        initialise_logging_for_tests();
        let mut prog = Prog::from("x2");
        prog[-1].subnodes = Some(Prog::from("F"));
        run_prog_and_test(prog, &[F, F]);
    }

    #[test]
    fn test_bare_nested_repeat() {
        initialise_logging_for_tests();
        let mut prog = Prog::from("x2");
        prog[0].subnodes = Some(Prog::from("x2"));
        prog[0][-1].subnodes = Some(Prog::from("F"));
        run_prog_and_test(prog, &[F, F, F, F]);
    }

    #[test]
    fn test_twice_nested_repeat() {
        initialise_logging_for_tests();
        let mut prog = Prog::from("x2");
        prog[0].subnodes = Some(Prog::from("x2,x2"));
        prog[0][-1].subnodes = Some(Prog::from("F"));
        prog[0][-1].subnodes = Some(Prog::from("R"));
        run_prog_and_test(prog, &[F, F, R, R, F, F, R, R]);
    }

    #[test]
    fn test_nested_repeat_two_instr() {
        initialise_logging_for_tests();
        let mut prog = Prog::from("x2");
        prog[0].subnodes = Some(Prog::from("x2"));
        prog[0][-1].subnodes = Some(Prog::from("L, R"));
        run_prog_and_test(prog, &[L, R, L, R, L, R, L, R]);
    }

    #[test]
    fn test_repeat_nested_group() { // x2(group(x2(F), R))
        initialise_logging_for_tests();
        let mut prog = Prog::from("x2");
        prog[0].subnodes = Some(Prog::from("group"));
        prog[0][0].subnodes = Some(Prog::from("x2, R"));
        prog[0][0][0].subnodes = Some(Prog::from("F"));
        run_prog_and_test(prog, &[F, F, R, F, F, R]);
    }

    #[test]
    fn test_f_then_nested_repeat_two_instr() {
        initialise_logging_for_tests();
        let mut prog = Prog::from("F, x2");
        prog[1].subnodes = Some(Prog::from("x2"));
        prog[1][0].subnodes = Some(Prog::from("L, R"));
        run_prog_and_test(prog, &[F, L, R, L, R, L, R, L, R]);
    }
}
