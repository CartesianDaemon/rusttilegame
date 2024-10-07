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

// Coord types defined approximate theoretical types:
pub type Pos = (i16, i16, u16);
// pub type Point = (i16, i16);
pub type Delta = (i16, i16);

#[derive(Copy, Clone, PartialEq, Debug, Add, Mul)]
pub struct Point {
    x: i16,
    y: i16,
}

impl Point {
    pub fn to_pos(&self) -> Pos {
        (self.x, self.y, 0 )
    }

    pub fn from_pos(pos: Pos) -> Point {
        Point { x: pos.0, y: pos.1}
    }
}

// Can this be derived? Not yet?
impl Add<Delta> for Point {
    type Output = Point;
    fn add(self, rhs: Delta) -> Point {
        Point { x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

/* // Can't do this when type is actually a tuple. When it's reimplemented then yes.
impl Add<Delta> for Pos {
    type Output = Pos;
    fn add(self, rhs: Delta) -> Pos {
        (self.0 + rhs.0, self.1 + rhs.1, self.2)
    }
} */
