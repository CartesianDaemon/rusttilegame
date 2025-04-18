use std::ops::Add;
use derive_more::*;

// Coord types (in theory)
//
// FIXME: Move to a coord type module.
// FIXME: Decide whether implementing types would help.
//
// Dimension: Width/height of map. Unsigned. Vars w,h.
// MapCoord: Coords on map. Signed to allow looping past edge.
//           May need cast to index vector? Vars x,y.
// ViewCoord: As MapCoord but relative to visible part of map (currently all).
//            Vars vx, vy.
// Delta: Offset of map coord. Signed. Vars dx, dy.
// PixCoord: Coords on screen. f32. Vars px, py.
// Pos: Coords including height.
//
// Ideally allowing arithmetic between dimension, map, delta with least casting.
// And multiplication of p coords by map coords.
//
// TODO: Would it be worth defining my own float type which can be multiplied by int?

/// Identify loc in map.
#[derive(Copy, Clone, PartialEq, Debug, Add, Mul)]
pub struct MapCoord {
    pub x: i16,
    pub y: i16,
}

impl MapCoord {
    pub fn from_xy(x: i16, y: i16) -> MapCoord {
        MapCoord {x, y}
    }
}

impl Add<CoordDelta> for MapCoord {
    type Output = MapCoord;
    fn add(self, delta: CoordDelta) -> MapCoord {
        MapCoord { x: self.x + delta.dx, y: self.y + delta.dy }
    }
}

#[derive(Add, Copy, Clone, PartialEq, Debug, Neg)] // , Add, Mul
pub struct CoordDelta {
    pub dx: i16,
    pub dy: i16,
}

impl CoordDelta {
    pub fn from_xy(dx: i16, dy: i16) -> CoordDelta {
        CoordDelta {dx, dy}
    }
}

// Different file?
pub enum Cmd {
    Stay,
    Left,
    Right,
    Up,
    Down,
}

impl Cmd {
    pub fn as_dir(self: Self) -> CoordDelta {
        match self {
            Self::Stay  => CoordDelta::from_xy(0, 0),
            Self::Left  => CoordDelta::from_xy(-1, 0),
            Self::Right => CoordDelta::from_xy(1, 0),
            Self::Up    => CoordDelta::from_xy(0, -1),
            Self::Down  => CoordDelta::from_xy(0, 1),
        }
    }

    pub fn default_cmd() -> Self {
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
