
// Types of movement-control logic ents can use
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum AI {
    Hero, // Move according to instruction sequence: F, L, etc. TODO: Rename to Prog, fixing errors in engine.
    Stay, // No movement. E.g. the goal.
    // TODO: Used in tests. Need to move those tests to pushing puzzle dir
    Bounce,
    Drift,
}
