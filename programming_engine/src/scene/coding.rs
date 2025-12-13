#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ActionData {
    pub blocked: bool,
}

impl ActionData {
    pub const fn default() -> Self {
        ActionData {blocked: false}
    }
    pub const fn blocked() -> Self {
        ActionData {blocked: true}
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ActionOpcode {
    F,
    L,
    R,
    No, // Only used during testing. Test treats it as always failing.
}

impl ActionOpcode {
    // Used for representing a single instruction graphically
    pub fn as_text(&self) -> String {
        use ActionOpcode::*;
        match self {
            F => "F",
            L => "L",
            R => "R",
            No => "No",
        }.to_string()
    }
}

impl std::fmt::Display for ActionOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_text())
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParentOpcode {
    group,
    x2,
    LOOP,
    loop5,
    Else,
}

impl std::fmt::Display for ParentOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_text())
    }
}

impl ParentOpcode {
    pub fn r_connect_max(&self) -> usize {
        use ParentOpcode::*;
        match self {
            group |
            LOOP => 999,
            x2 => 1,
            loop5 => 5,
            Else => 2,
        }
    }

    // Used for representing a single instruction graphically
    pub fn as_text(&self) -> String {
        use ParentOpcode::*;
        match self {
            group => "{...}",
            LOOP => "LOOP",
            x2 => "x2",
            loop5 => "loop5",
            Else => "Else",
        }.to_string()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opcode {
    Action(ActionOpcode),
    Parent(ParentOpcode),
}

// Conversion to string for "Friendly" representation of programs.
// Aim for allowing reconstruction of original program but not current state.
impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Opcode::*;
        use ActionOpcode::*;
        use ParentOpcode::*;
        match self {
            Action(F) => std::fmt::Display::fmt("F", f),
            Action(L) => std::fmt::Display::fmt("L", f),
            Action(R) => std::fmt::Display::fmt("R", f),
            Action(No) => std::fmt::Display::fmt("No", f),
            Parent(group) => std::fmt::Display::fmt("group", f),
            Parent(LOOP) => std::fmt::Display::fmt("loop", f),
            Parent(x2) => std::fmt::Display::fmt("x2", f),
            Parent(loop5) => std::fmt::Display::fmt("loop5", f),
            Parent(Else) => std::fmt::Display::fmt("Else", f),
        }
    }
}

impl From<&str> for Opcode {
    fn from(txt: &str) -> Self {
        match txt {
            "F" => Opcode::Action(ActionOpcode::F),
            "L" => Opcode::Action(ActionOpcode::L),
            "R" => Opcode::Action(ActionOpcode::R),
            "No" => Opcode::Action(ActionOpcode::No),
            "group" => Opcode::Parent(ParentOpcode::group),
            "loop" => Opcode::Parent(ParentOpcode::LOOP),
            "x2" => Opcode::Parent(ParentOpcode::x2),
            "loop5" => Opcode::Parent(ParentOpcode::loop5),
            "Else" => Opcode::Parent(ParentOpcode::Else),
            _ => panic!("Unrecognised txt for instr: {}", txt)
        }
    }
}

impl Opcode {
    pub fn as_text(&self) -> String {
        use Opcode::*;
        match self {
            Action(op) => op.as_text(),
            Parent(op) => op.as_text(),
        }

    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Instr {
    Action(ActionOpcode, ActionData),
    Parent(ParentOpcode, Subprog),
}

impl Instr {
    pub fn from_opcode(op: Opcode) -> Self {
        match op {
            Opcode::Action(action_op) => Self::Action(action_op, ActionData::default()),
            Opcode::Parent(parent_op) => Self::Parent(parent_op, Subprog::default()),
        }
    }

    pub fn as_text(&self) -> String {
        match self {
            Instr::Action(op, _) => op.as_text(),
            Instr::Parent(op, _) => op.as_text(),
        }
    }

    pub fn has_opcode(&self, op: Opcode) -> bool {
        match self {
            Instr::Action(opcode_a, _) => matches!(&op, Opcode::Action(opcode_b) if opcode_a == opcode_b ),
            Instr::Parent(opcode_a, _) => matches!(&op, Opcode::Parent(opcode_b) if opcode_a == opcode_b ),
        }
    }

    pub fn as_action_op(&self) -> ActionOpcode {
        match self {
            Self::Action(op, _) => *op,
            _ => panic!("Not an action instr"),
        }
    }

    pub fn as_action_data(&self) -> &ActionData {
        match self {
            Self::Action(_, data) => data,
            _ => panic!("Not an action instr"),
        }
    }

    pub fn as_action_data_mut(&mut self) -> &mut ActionData {
        match self {
            Self::Action(_, data) => data,
            _ => panic!("Not an action instr"),
        }
    }

    pub fn as_parent_subprog(&self) -> &Subprog {
        match self {
            Self::Parent(_, subprog) => subprog,
            _ => panic!("Not a parent instr"),
        }
    }

    pub fn as_parent_subprog_mut(&mut self) -> &mut Subprog {
        match self {
            Self::Parent(_, subprog) => subprog,
            _ => panic!("Not a parent instr"),
        }
    }

    // More naturally part of opcode?
    pub fn v_connector(&self) -> bool {
        use Instr::*;
        use ParentOpcode::*;
        match self {
            Action(_, _) => true,
            Parent(LOOP, _) => false,
            Parent(_, _) => true,
        }
    }

    // More naturally part of opcode.
    pub fn r_connect_max(&self) -> usize {
        use Instr::*;
        match self {
            Action(_, _) => 0,
            Parent(parent_opcode, _) => parent_opcode.r_connect_max(),
        }
    }

    // TODO: Move to fn of ControlFlowOp not Op.
    // More naturally part of opcode.
    pub fn repeat_count(&self, subprog: &Subprog) -> usize {
        // assert!(std::ptr::eq(self, *subprog.instrs.get(idx).as_ref().unwrap()));
        use Instr::*;
        use ParentOpcode::*;
        match self {
            Action(_, _) => panic!("Repeat count not specified for non-parent instr"),
            Parent(group, _) => 1,
            Parent(x2, _) => 2,
            Parent(LOOP, _) => 999,
            Parent(loop5, _) => 5,
            Parent(Else, _) => if subprog.instrs.get(subprog.prev_ip).map_or(false, Instr::blocked) {
                log::debug!("Calculating Else repeat count as 1 in {subprog}");
                1
            } else {
                log::debug!("Calculating Else repeat count as 0 in {subprog}");
                0
            },
        }
    }

    pub fn blocked(&self) -> bool {
        match self {
            Instr::Action(_, data) => data.blocked,
            Instr::Parent(_, data) => data.instrs.last().map_or(false, Instr::blocked),
        }
    }

    pub fn v_len(&self) -> usize {
        match &self {
            Instr::Parent(_, subprog) => subprog.v_len(),
            _ => 1,
        }
    }
}

// Conversion to string for "Friendly" representation.
// Aim for allowing reconstruction of original program but not current state.
impl std::fmt::Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Instr::*;
        match self {
            Action(op, _) => std::fmt::Display::fmt(op, f),
            Parent(op, _) => std::fmt::Display::fmt(op, f),
        }
    }
}

// Conversion to string for detailed representation.
// Aim for allowing reconstruction of current state.
impl std::fmt::Debug for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Instr::*;
        match self {
            Action(op, _) => write!(f, "{op:?}{}", if self.blocked() {"b"} else {""} ),
            Parent(op, _) => std::fmt::Debug::fmt(op, f),
        }
    }
}

impl From<&str> for Instr {
    fn from(txt: &str) -> Self {
        match txt.into() {
            Opcode::Action(op) => Instr::Action(op, ActionData::default()),
            Opcode::Parent(op) => Instr::Parent(op, Subprog::default()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bin {
    pub op: Opcode,
    pub orig_count: u16,
    pub curr_count: u16,
}

impl Bin {
    fn new(op: Opcode, orig_count: u16) -> Self {
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

#[cfg(any())]
impl std::fmt::Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.instr)?;
        if let Instr::Parent(_,subprog) = &self.instr {
            write!(f, "[{}]", subprog)?;
        }
        Ok(())
    }
}

impl std::ops::Index<i16> for Instr {
    type Output = Instr;

    fn index(&self, idx: i16) -> &Self::Output {
        &self.as_parent_subprog()[idx]
    }
}

impl std::ops::IndexMut<i16> for Instr {
    fn index_mut(&mut self, idx: i16) -> &mut Self::Output {
        &mut self.as_parent_subprog_mut()[idx]
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Subprog {
    // Index of instruction currently executing. 0 when program has not started.
    pub curr_ip: usize,
    // Previously executed instr.
    prev_ip: usize,
    // Internal counter, used to implement loops and other stateful instructions.
    // When used for iteration, counts number of times current execution of parent instr has executed this subprog.
    pub counter: usize,
    // Vector of one or more instrs to execute. Some parent ops have a specific number of nested instrs.
    pub instrs: Vec<Instr>
}

impl From<Vec<Instr>> for Subprog {
    fn from(instrs: Vec<Instr>) -> Self {
        Self {
            instrs,
            ..Self::default()
        }
    }
}

impl From<&[Instr]> for Subprog {
    fn from(instrs: &[Instr]) -> Self {
        Self {
            instrs: instrs.to_vec(),
            ..Self::default()
        }
    }
}

#[cfg(any())]
impl<T: Iterator<Item=Instr>> From<T> for Subprog {
    fn from(ops: T) -> Self {
        Self {
            instrs: ops.map(|op| Instr{op:*op, subnodes:None }).collect(),
            ..Self::default()
        }
    }
}

#[cfg(test)]
fn find_matching_closing_bracket(txt: &str) -> Option<usize> {
    assert_eq!(txt.as_bytes()[0], b'[', "Expected '{txt}' to start with '['");
    find_unmatched_closing_bracket(&txt[1..]).map(|idx| idx+1)
}

fn find_unmatched_closing_bracket(txt: &str) -> Option<usize> {
    let mut nesting_count = 0;
    for (idx, c) in txt.chars().enumerate() {
        match c {
            '[' => nesting_count += 1,
            ']' if nesting_count == 0 => return Some(idx),
            ']' if nesting_count > 0 => nesting_count-=1,
            _ => (),
        }
    }
    None
}

impl Subprog {
    pub fn from_text(txt: &str) -> Self {
        let mut ret = Self::default();
        let mut remaining = txt;
        while remaining.len() > 0 {
            let (segment, sep, trailing) = match remaining.find([',','[',']']) {
                Some(idx) => (
                    &remaining[0..idx],
                    &remaining[idx..idx+1],
                    &remaining[idx+1..],
                ),
                None => (
                    remaining,
                    ",",
                    "",
                )
            };

            ret.instrs.push(segment.trim().into());

            match ret.instrs.last_mut().unwrap() {
                instr @ Instr::Action(_, _) => {
                    assert_eq!(sep, ",", "Expected ',' after {instr}");
                    remaining = trailing;
                },
                ref mut instr @ Instr::Parent(..) => {
                    assert_eq!(sep, "[", "Expected '[' after {instr}");
                    let idy = find_unmatched_closing_bracket(trailing).unwrap();
                    let (subprog_txt, rest_of_string) = (&trailing[0..idy], &trailing[idy+1..]);
                    *instr.as_parent_subprog_mut() = Self::from_text(subprog_txt);

                    let rest_of_string = rest_of_string.trim();
                    if rest_of_string.is_empty() {
                        remaining = rest_of_string;
                    } else {
                        assert!(rest_of_string.starts_with(','), "Expected ',' after ']' in {trailing} at {idy}");
                        remaining = &rest_of_string[1..];

                    }
                } ,
            }
        }
        ret
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
        write!(f, "[")?;
        for (idx, instr) in self.instrs.iter().enumerate() {
            if idx >0 {write!(f, ",")?}
            write!(f, "{}", instr.as_text())?;
            if let Instr::Parent(_, subprog) = &instr {
                write!(f, "{}", subprog)?;
            }
        }
        write!(f, "]")
    }
}

impl std::fmt::Debug for Subprog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_{}[", self.counter)?;
        for (idx, instr) in self.instrs.iter().enumerate() {
            if idx >0 {write!(f, ",")?}
            if idx == self.curr_ip {
                write!(f, "*")?;
            }
            write!(f, "{}", instr.to_string())?;
            if let Instr::Parent(_, subprog) = &instr {
                write!(f, "{}", subprog)?;
            }
        }
        if self.curr_ip >= self.instrs.len() {
            write!(f, ",*")?;
        }
        write!(f, "]")
    }
}

impl std::ops::Index<i16> for Subprog {
    type Output = Instr;

    fn index(&self, idx: i16) -> &Self::Output {
        if idx >= 0 {
            self.instrs.get(idx as usize).unwrap()
        } else {
            for instr in &self.instrs {
                if let Instr::Parent(_, subprog) = &instr && subprog.instrs.len() == 0 {
                    return instr;
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
            for instr in &mut self.instrs {
                if let Instr::Parent(_, subprog) = &instr && subprog.instrs.len() == 0 {
                    return instr;
                }
            }
            panic!();
        }
    }
}

impl Subprog {
    pub const fn default() -> Self {
        Subprog {counter: 0, curr_ip: 0, prev_ip: 0, instrs: vec![]}
    }

    // Number of instructions within if laid out vertically. Used for drawing.
    // Always at least 1.
    pub fn v_len(&self) -> usize {
        std::cmp::max(1, self.instrs.iter().map(|node| node.v_len()).sum())
    }

    pub fn finished(&self) -> bool {
        self.curr_ip >= self.instrs.len()
    }

    // Current instr in this prog (either action instr or parent instr)
    fn curr_instr(&self) -> Option<&Instr> {
        self.instrs.get(self.curr_ip)
    }

    fn curr_instr_mut(&mut self) -> Option<&mut Instr> {
        self.instrs.get_mut(self.curr_ip)
    }

    // Currently executing op. Action instr from list, or nested inside parent instrs.
    // None when past end of program, or when program reaches an empty parent instr.
    pub fn curr_action_instr(&self) -> Option<&Instr> {
        match &self.curr_instr()? {
            instr @ Instr::Action(..) => Some(instr),
            Instr::Parent(_, subprog) => subprog.curr_action_instr(),
        }
    }

    pub fn curr_op_mut(&mut self) -> Option<&mut Instr> {
        match self.curr_instr_mut()? {
            instr @ Instr::Action(..) => Some(instr),
            Instr::Parent(_, subprog) => subprog.curr_op_mut(),
        }
    }

    pub fn unwrap_curr_op(&self) -> &Instr {
        self.curr_action_instr().unwrap()
    }

    fn advance_ip(&mut self) {
        self.curr_ip += 1;
        // Skip over any repeat-0 instr.
        if matches!(self.curr_instr(), Some(instr @ Instr::Parent(..)) if instr.repeat_count(self) == 0) {
            self.advance_ip();
        }
    }

    fn reset(&mut self) {
        self.curr_ip = 0;
        self.counter = 0;
    }

    fn iterate(&mut self) {
        self.curr_ip = 0;
        self.counter += 1;
    }

    fn advance_current_subprog(&mut self, parent_op: &Instr) {
        let repeat_count = parent_op.repeat_count(self);
        let subprog = self.instrs.get_mut(self.curr_ip).unwrap().as_parent_subprog_mut();
        subprog.advance_next_instr();
        if subprog.finished() {
            if subprog.counter + 1 < repeat_count {
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
        log::debug!("------");
        self.prev_ip = self.curr_ip;
        if self.finished() {
            self.reset();
            return;
        }

        let op = &self.instrs.get_mut(self.curr_ip).unwrap().clone();
        match op {
            Instr::Action(..) => self.advance_ip(),
            Instr::Parent(..) => self.advance_current_subprog(op),
        }
        assert!(self.curr_action_instr().is_none() || matches!(self.curr_action_instr(), Some(Instr::Action(..))));
        log::debug!("Advanced prog to {:?}.", self); // to #{}. Next: #{}.", self, self.prev_ip, self.next_ip);
    }
}

pub use Subprog as Prog;

#[derive(Clone, Debug)]
pub struct Coding {
    pub supply: Vec<Bin>,
    pub prog: Subprog,
}

impl Coding {
    pub fn from_vec(supplies: &[(Opcode, u16)]) -> Coding {
        Coding {
            supply: supplies.iter().map(|(op,count)|
            Bin::new(*op, *count)
            ).collect(),
            prog: Subprog::default(),
        }
    }
}

pub mod action_ops {
    #![allow(non_upper_case_globals)]
    use super::*;

    pub const F: ActionOpcode = ActionOpcode::F;
    pub const L: ActionOpcode = ActionOpcode::L;
    pub const R: ActionOpcode = ActionOpcode::R;
    pub const No: ActionOpcode = ActionOpcode::No;
}

pub mod supply_ops {
    #![allow(non_upper_case_globals)]
    use super::*;

    pub const F: Opcode = Opcode::Action(ActionOpcode::F);
    pub const L: Opcode = Opcode::Action(ActionOpcode::L);
    pub const R: Opcode = Opcode::Action(ActionOpcode::R);
    pub const No: Opcode = Opcode::Action(ActionOpcode::No);

    pub const x2: Opcode = Opcode::Parent(ParentOpcode::x2);
    pub const group: Opcode = Opcode::Parent(ParentOpcode::group);
    pub const loop5: Opcode = Opcode::Parent(ParentOpcode::loop5);
    pub const LOOP: Opcode = Opcode::Parent(ParentOpcode::LOOP);
    pub const Else: Opcode = Opcode::Parent(ParentOpcode::Else);
}

pub mod prog_ops {
    #![allow(non_upper_case_globals)]

    use super::*;

    pub const F: Instr = Instr::Action(ActionOpcode::F, ActionData::default());
    pub const L: Instr = Instr::Action(ActionOpcode::L, ActionData::default());
    pub const R: Instr = Instr::Action(ActionOpcode::R, ActionData::default());
    pub const No: Instr = Instr::Action(ActionOpcode::No, ActionData::blocked());

    // TODO: Introduce fn if we first subsume Subprog into ParentOp
    // pub fn x2(ops: Vec<Op>) -> Op = Op::Parent(ParentOp::x2);

    // TODO: make Subprog::default a const function to avoid duplication.
    pub const default_subprog: Subprog = Subprog {counter: 0, curr_ip: 0, prev_ip: 0, instrs: vec![]};
    pub const x2: Instr = Instr::Parent(ParentOpcode::x2, default_subprog);
    pub const group: Instr = Instr::Parent(ParentOpcode::group, default_subprog);
    pub const loop5: Instr = Instr::Parent(ParentOpcode::loop5, default_subprog);
    pub const Else: Instr = Instr::Parent(ParentOpcode::Else, default_subprog);
}

pub mod prog_fn_ops {
    #![allow(non_snake_case, non_upper_case_globals)]

    use super::*;

    pub const F: Instr = Instr::Action(ActionOpcode::F, ActionData::default());
    pub const L: Instr = Instr::Action(ActionOpcode::L, ActionData::default());
    pub const R: Instr = Instr::Action(ActionOpcode::R, ActionData::default());
    pub const No: Instr = Instr::Action(ActionOpcode::No, ActionData::blocked());

    pub fn group(ops: &[Instr]) -> Instr { Instr::Parent(ParentOpcode::group, Subprog::from(ops)) }
    pub fn x2(ops: &[Instr]) -> Instr { Instr::Parent(ParentOpcode::x2, Subprog::from(ops)) }
    pub fn LOOP(ops: &[Instr]) -> Instr { Instr::Parent(ParentOpcode::LOOP, Subprog::from(ops)) }
    pub fn loop5(ops: &[Instr]) -> Instr { Instr::Parent(ParentOpcode::loop5, Subprog::from(ops)) }
    pub fn Else(ops: &[Instr]) -> Instr { Instr::Parent(ParentOpcode::Else, Subprog::from(ops)) }
}

#[cfg(test)]
mod tests {
    use crate::infra::initialise_logging_for_tests;
    use super::*;

    #[test]
    fn parse_linear_prog() {
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

    #[test]
    fn test_matching_nested_brackets() {
        assert_eq!(find_matching_closing_bracket("[]"), Some(1));
        assert_eq!(find_matching_closing_bracket("[   ]"), Some(4));
        assert_eq!(find_matching_closing_bracket("[[ ]]"), Some(4));
        assert_eq!(find_matching_closing_bracket("["), None);
        assert_eq!(find_matching_closing_bracket("[ []"), None);
        assert_eq!(find_matching_closing_bracket("[ [] [] ]"), Some(8));
        assert_eq!(find_matching_closing_bracket("[ [ [ ] ] ]"), Some(10));
        assert_eq!(find_matching_closing_bracket("[[][]]"), Some(5));
        assert_eq!(find_matching_closing_bracket("[[[]]]"), Some(5));
    }

    #[test]
    fn parse_nested_prog() {
        use prog_ops::*;
        if true {
            assert_eq!(Prog::from_text("F"), Prog::from(vec![F]));
            assert_eq!(Prog::from_text("F "), Prog::from(vec![F]));
            assert_eq!(Prog::from_text(" F "), Prog::from(vec![F]));
            assert_eq!(Prog::from_text("F,R"), Prog::from(vec![F, R]));
            assert_eq!(Prog::from_text("F, R"), Prog::from(vec![F, R]));
            assert_eq!(Prog::from_text("F ,R"), Prog::from(vec![F, R]));
            assert_eq!(Prog::from_text("F ,R "), Prog::from(vec![F, R]));
            assert_eq!(Prog::from_text(" F ,R    , L"), Prog::from(vec![F, R, L]));
            assert_eq!(Prog::from_text("F,R,L,x2[],group[],loop5[],"), Prog::from(vec![F, R, L, x2, group, loop5]));
            assert_eq!(Prog::from_text("F,R,L,x2[],group[],loop5[]"), Prog::from(vec![F, R, L, x2, group, loop5]));
        }
        if true {
            use prog_fn_ops::*;
            assert_eq!(Prog::from_text("group [ F ] "), Prog::from(vec![group(&[F])]));
            assert_eq!(Prog::from_text("group [ F, R ] "), Prog::from(vec![group(&[F, R])]));
            assert_eq!(Prog::from_text("group [ F ,R ] "), Prog::from(vec![group(&[F, R])]));
            assert_eq!(Prog::from_text("group [ F, R,]"), Prog::from(vec![group(&[F, R])]));
            assert_eq!(Prog::from_text("group [ x2 [ F, R] ] "), Prog::from(vec![group(&[x2(&[F, R])])]));
            assert_eq!(Prog::from_text("group [ x2 [ F, R], loop [ L, L] ] "), Prog::from(vec![group(&[x2(&[F, R]), LOOP(&[L, L])])]));
        } else {
            // ...
        }
    }

    fn run_prog_and_test(mut prog: Prog, expected_ops: &[ActionOpcode]) {
        for (idx, expected_op) in expected_ops.iter().enumerate() {
            assert!(!prog.finished());
            assert!(
                matches!(prog.curr_action_instr(), Some(Instr::Action(op, _)) if op==expected_op),
                "At idx {idx} of {prog} expected {:?} to match {expected_op:?}", prog.curr_action_instr(),
            );
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
        prog[-1] = Instr::Parent(ParentOpcode::x2, Prog::from("F,R"));
        run_prog_and_test(prog, &[L, F, R, F, R, L]);
    }

    #[test]
    fn test_bare_repeat() {
        use prog_fn_ops::*;
        initialise_logging_for_tests();
        let prog = Prog::from(vec![x2(&[F])]);

        {
            use action_ops::*;
            run_prog_and_test(prog, &[F, F]);
        }
    }

    #[test]
    fn test_bare_nested_repeat() {
        initialise_logging_for_tests();
        let mut prog = Prog::from("x2");
        prog[0] = Instr::Parent(ParentOpcode::x2, Prog::from("x2"));
        prog[0][-1] = Instr::Parent(ParentOpcode::x2, Prog::from("F"));
        run_prog_and_test(prog, &[F, F, F, F]);
    }

    #[test]
    fn test_twice_nested_repeat() {
        initialise_logging_for_tests();
        let mut prog = Prog::from("x2");
        prog[0] = Instr::Parent(ParentOpcode::x2, Prog::from("x2, x2"));
        prog[0][-1] = Instr::Parent(ParentOpcode::x2, Prog::from("F"));
        prog[0][-1] = Instr::Parent(ParentOpcode::x2, Prog::from("R"));
        run_prog_and_test(prog, &[F, F, R, R, F, F, R, R]);
    }

    #[test]
    fn test_nested_repeat_two_instr() {
        initialise_logging_for_tests();
        let mut prog = Prog::from("x2");
        prog[0] = Instr::Parent(ParentOpcode::x2, Prog::from("x2"));
        prog[0][-1] = Instr::Parent(ParentOpcode::x2, Prog::from("L, R"));
        run_prog_and_test(prog, &[L, R, L, R, L, R, L, R]);
    }

    #[test]
    fn test_repeat_nested_group() { // x2(group(x2(F), R))
        initialise_logging_for_tests();
        let mut prog = Prog::from("x2");
        prog[0] = Instr::Parent(ParentOpcode::x2, Prog::from("group"));
        prog[0][0] = Instr::Parent(ParentOpcode::group, Prog::from("x2, R"));
        prog[0][0][0] = Instr::Parent(ParentOpcode::x2, Prog::from("F"));
        run_prog_and_test(prog, &[F, F, R, F, F, R]);
    }

    #[test]
    fn test_f_then_nested_repeat_two_instr() {
        initialise_logging_for_tests();
        let mut prog = Prog::from("F, x2");
        prog[1] = Instr::Parent(ParentOpcode::x2, Prog::from("x2"));
        prog[1][0] = Instr::Parent(ParentOpcode::x2, Prog::from("L, R"));
        run_prog_and_test(prog, &[F, L, R, L, R, L, R, L, R]);
    }

    #[test]
    fn test_else() {
        initialise_logging_for_tests();
        use prog_fn_ops::*;
        initialise_logging_for_tests();
        let prog = Prog::from(vec![F, Else(&[L]), No, Else(&[L])]);

        {
            use action_ops::*;
            run_prog_and_test(prog, &[F, No, L]);
        }
    }
}
