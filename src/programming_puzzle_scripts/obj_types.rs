
// STUB: Properties that can be applied to Ent.
// Makes more sense if there's a mod with Ent class and non-public classes only used inside it.
// Can these become a generic properties defined in load, rather than being specified by engine?

// Passable. Whether other movs can move through an ent or not.
#[derive(Clone, PartialEq, Debug)]
pub enum Pass {
    Empty, // No impediment to movement, e.g. floor.
    Solid, // Block movement, e.g. wall.
    Mov, // Something which can move itself, e.g. hero, enemy
    // INSERT: Obj, // Something which can be moved or maybe coexisted with, e.g. furniture
}

// Types of movement-control logic ents can use
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum AI {
    Prog, // Move according to instruction sequence: F, L, etc.
}
