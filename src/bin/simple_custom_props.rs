// Properties of objects related to scripting.
// NB: Would be nice to subsume into one CustomProps struct. Defined in
// specialised game data, a member of LogicalProps.

use crate::engine::for_gamedata::{BaseCustomProps, BaseAI};

#[derive(Clone, PartialEq, Debug)]
pub struct SimpleCustomProps {
}

// TODO: Have separate "properties". Move "AI" into MovementLogic entirely??
impl BaseCustomProps for SimpleCustomProps {
    type AI = AI;

}

// Passable. Whether other movs can move through an ent or not.
#[derive(Clone, PartialEq, Debug)]
pub enum Pass {
    Empty, // No impediment to movement, e.g. floor.
    Solid, // Block movement, e.g. wall.
    Mov, // Something which can move itself, e.g. hero, enemy
    // INSERT: Obj, // Something which can be moved or maybe coexisted with, e.g. furniture
}

// Types of movement-control logic ents can use
// ObjProperties struct data depends on this, but no engine functions do.
// TODO: Want to move the types back to game-specific scripting obj_properties.rs.
// Should be easy. But need to template ObjProperties on that. And what special cases for
// is_hero, is_mob, etc defined in engine.
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum AI {
    Stay, // No self movement. Not added to Roster's list of movs.
    Hero, // Controlled by keys. Assume only one hero, added to Roster's hero entry.
    // Everything else may spontaneously move or need to be enumerated, ie needs to be added to roster.
    Bounce, // Move in direction, reverse direction at walls.
    Drift, // Move in direction, reverse direction at walls, move diagonally towards hero at reversal.
    Scuttle, // Move in direction, when hit wall change to move orthogonally towards hero.
}

impl BaseAI for AI {
    fn default() -> AI {
        Self::Stay
    }
    fn is_hero(ai: Self) -> bool {
        ai == Self::Hero
    }
    fn is_any_mov(ai: Self) -> bool {
        ai != Self::Stay
    }
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
