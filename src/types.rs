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

/// Handle identifying an Ent in the map.
///
/// Implemented as the MapCoord and index into Ents at that Loc.
pub type MapHandle = (i16, i16, u16);

pub type CoordDelta = (i16, i16);

impl MapCoord {
    pub fn to_pos(&self) -> MapHandle {
        (self.x, self.y, 0 )
    }

    pub fn from_pos(pos: MapHandle) -> MapCoord {
        MapCoord { x: pos.0, y: pos.1}
    }
}

impl Add<CoordDelta> for MapCoord {
    type Output = MapCoord;
    fn add(self, rhs: CoordDelta) -> MapCoord {
        MapCoord { x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

/* // Can't do this when type is actually a tuple. When it's reimplemented then yes.
impl Add<Delta> for Pos {
    type Output = Pos;
    fn add(self, rhs: Delta) -> Pos {
        (self.0 + rhs.0, self.1 + rhs.1, self.2)
    }
} */
