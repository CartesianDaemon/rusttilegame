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

/// Identify loc in map.
#[derive(Copy, Clone, PartialEq, Debug, Add, Mul)]
pub struct MapCoord {
    x: i16,
    y: i16,
}

impl MapCoord {
    // TODO: Indicates places which shouldn't take a handle to start with..
    pub fn to_hdl(self) -> MapHandle {
        MapHandle{x: self.x, y: self.y, h:0 }
    }

    pub fn from_hdl(pos: MapHandle) -> MapCoord {
        MapCoord { x: pos.x, y: pos.y}
    }
}

impl Add<CoordDelta> for MapCoord {
    type Output = MapCoord;
    fn add(self, delta: CoordDelta) -> MapCoord {
        MapCoord { x: self.x + delta.dx, y: self.y + delta.dy }
    }
}

/// A handle identifying an Ent in the map.
///
/// Implemented as the MapCoord and index into Ents at that Loc.
#[derive(Copy, Clone, PartialEq, Debug)] // , Add, Mul
pub struct MapHandle {
    pub x: i16,
    pub y: i16,
    pub h: u16,
}

impl MapHandle
{
    pub fn from_xyh(x: i16, y: i16, h: u16) -> MapHandle {
        MapHandle {x, y, h}
    }
}

#[derive(Copy, Clone, PartialEq, Debug)] // , Add, Mul
pub struct CoordDelta {
    pub dx: i16,
    pub dy: i16,
}

impl CoordDelta {
    pub fn from_xy(dx: i16, dy: i16) -> CoordDelta {
        CoordDelta {dx, dy}
    }
}

/* // Can't do this when type is actually a tuple. When it's reimplemented then yes.
impl Add<Delta> for Pos {
    type Output = Pos;
    fn add(self, rhs: Delta) -> Pos {
        (self.0 + rhs.0, self.1 + rhs.1, self.2)
    }
} */
