
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
    Stay, // No self movement. Not added to Roster's list of movs.
    Hero, // Controlled by keys. Assume only one hero, added to Roster's hero entry.
    // Everything else may spontaneously move or need to be enumerated, ie needs to be added to roster.
    Snake, // Move in direction, move orthogonally towards hero. Maybe: bounce off walls.
    Bounce, // Move in direction, reverse direction at walls.
    Drift, // Move in direction, reverse direction at walls, move diagonally towards hero at reversal.
    Scuttle, // Move in direction, when hit wall change to move orthogonally towards hero.
}

// Effect when intersect with hero (as mov or stay)
#[derive(Clone, PartialEq, Debug)]
pub enum Effect {
    Nothing,
    Kill,
    Win,
    // STUB: Can add effects like when ent dies
    // STUB: Could convert Win, Kill, to Progress(Win),... with enum Progress {Win, Lose}
}
