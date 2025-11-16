use std::ops::Add;
use derive_more::{Add, Mul, Neg};

// Coord types (in theory)
//
// Dimension: Width/height of map. Unsigned. Vars w,h.
// MapCoord: Coords on map. Signed to allow looping past edge.
//           May need cast to index vector? Vars x,y.
// ViewCoord: As MapCoord but relative to visible part of map (currently all).
//            Vars vx, vy.
// Delta: Offset of map coord. Signed. Vars dx, dy.
// PixCoord: Coords on screen. f32. Vars px, py. Used in ui_arena particularly.
// MapRef: Coords including height, only used internally to identify objects.

// Types of index (the int type most easily converted to MapCoord::x, ros_idx, etc)
//
// Index into map. i16.
// Map diff. i16.
// Index into roster. u16. Although could be an enum?
//
// TODO: Is it useful to have a type for them? May just add clutter.

#[derive(Copy, Clone, PartialEq, Debug, Add, Mul)]
pub struct MapCoord {
    pub x: i16,
    pub y: i16,
}

impl MapCoord {
    pub fn from_xy(x: i16, y: i16) -> MapCoord {
        MapCoord {x, y}
    }

    pub fn delta_to(self, target: MapCoord) -> CoordDelta {
        CoordDelta { dx: target.x - self.x, dy: target.y - self.y }
    }

    pub fn dir_to(self, target: MapCoord) -> CoordDelta {
        CoordDelta { dx: (target.x - self.x).signum(), dy: (target.y - self.y).signum() }
    }
}

impl Add<CoordDelta> for MapCoord {
    type Output = MapCoord;
    fn add(self, delta: CoordDelta) -> MapCoord {
        MapCoord { x: self.x + delta.dx, y: self.y + delta.dy }
    }
}

// NB: Need separate "Facing" type which can easily be converted to a CoordDelta.
#[derive(Add, Copy, Clone, PartialEq, Debug, Neg)] // , Add, Mul
pub struct CoordDelta {
    pub dx: i16,
    pub dy: i16,
}

impl CoordDelta {
    pub fn from_xy(dx: i16, dy: i16) -> Self {
        CoordDelta {dx, dy}
    }

    pub fn reverse(&mut self) {
        *self = self.reversed()
    }

    fn reversed(self) -> Self {
        CoordDelta { dx: self.dx * -1, dy: self.dy * -1 }
    }

    // Cycles forward through:
    // Facing N 0,-1
    // Facing E 1, 0
    // Facing S 0, -1
    // Facing W -1, 0
    fn rotated_r(&self) -> CoordDelta {
        CoordDelta {
            dx: -self.dy,
            dy: self.dx,
        }
    }

    // Reverse of rotated_r
    fn rotated_l(&self) -> CoordDelta {
        CoordDelta {
            dx: self.dy,
            dy: -self.dx,
        }
    }

    pub fn rotate_r(&mut self) {
        *self = self.rotated_r()
    }

    pub fn rotate_l(&mut self) {
        *self = self.rotated_l()
    }
}

// Translation of Key or Mouse into attempted movement of hero.
// NB: Should love to interface exposed by input.
// NB: Have separate Cmd for menu, movement, programming, etc. Pane chooses which?
// Not quite right currently as Progpuzz Arena bot should accept something
// like Cmd from executing program. Only pushpuzz hero gets it from user?
// NB: Or could turn into AttemptAction struct in simple_logic, which is
// used by most game movement logic but doesn't have to be? Along with an
// attempt_action fn which handles passability etc.
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MoveCmd {
    Stay,
    Left,
    Right,
    Up,
    Down,
}

impl MoveCmd {
    pub fn as_dir(self: Self) -> CoordDelta {
        match self {
            // NB: Could be mapped in terms of rotatable Facing type.
            Self::Stay  => CoordDelta::from_xy(0, 0),
            Self::Left  => CoordDelta::from_xy(-1, 0),
            Self::Right => CoordDelta::from_xy(1, 0),
            Self::Up    => CoordDelta::from_xy(0, -1),
            Self::Down  => CoordDelta::from_xy(0, 1),
        }
    }

    pub fn default() -> Self {
        Self::Stay
    }
}

/* // Can't do this when type is actually a tuple. When it's reimplemented then yes.
impl Add<Delta> for Pos {
    type Output = Pos;
    fn add(self, rhs: Delta) -> Pos {
        (self.0 + rhs.0, self.1 + rhs.1, self.2)
    }
} */
