// Types used by common implementations of CustomProps.

use crate::engine::for_gamedata::BaseCustomProps;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SimpleCustomProps {
    pub ai: SimpleAI,
}

impl BaseCustomProps for SimpleCustomProps {
    fn default() -> Self {
        Self {
            ai: SimpleAI::Stay,
        }
    }

    fn is_hero(props: Self) -> bool {
        props.ai == SimpleAI::Hero
    }
    fn is_any_mov(props: Self) -> bool {
        props.ai != SimpleAI::Stay
    }
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
// TODO: Make a copy for each game specialisatoin with different types.
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum SimpleAI {
    Stay, // No self movement. Not added to Roster's list of movs.
    Hero, // Controlled by keys. Assume only one hero, added to Roster's hero entry.
    // Everything else may spontaneously move or need to be enumerated, ie needs to be added to roster.
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
