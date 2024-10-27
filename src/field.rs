// Map, location, and entity types.
//
// But movement logic etc are in Play.
// These are also used by level data files, even though
// they don't need any of the indexing.

use std::mem;
use std::ops::Index;
use std::ops::IndexMut;

use culpa::try_fn;

use crate::*;

use map_coords::*;

use obj::Obj;

// "Map": Grid of locations. Most of the current state of game.
#[derive(Clone)]
pub struct Map {
    // Stored as a collection of columns, e.g. map.locs[x][y]
    // Must always be rectangular.
    locs: Vec<Vec<Loc>>,
}

/// Map together with Ros. Separate so they can more easily be borrowed separately.
#[derive(Clone, Debug)]
pub struct Field {
    pub map: Map,
    // Moveable objects in the current map.
    pub ros: Ros,
    // Key used to represent map as ascii for init and debugging. Not completely comprehensive.
    pub map_key: std::collections::HashMap<char, Vec<Obj>>,
}

impl Field {
    pub fn empty(sz: u16) -> Field {
        Field {
            map: Map::new(sz),
            ros: Ros::new(),
            map_key: std::collections::HashMap::new(),
        }
    }

    /// Create an object in the map and in the roster.
    ///
    /// All new objs go through this to keep map and roster in sync.
    pub fn place_obj_at(&mut self, x: i16, y:i16, orig_obj: Obj)
    {
        let hdl = self.map.place_obj_at(x, y, orig_obj);
        let placed_obj = &self.map[hdl];

        if placed_obj.is_hero() {
            self.ros.hero = hdl;
        } else if placed_obj.is_other_mov() {
            self.ros.push_mov(hdl);
        }
    }

    pub fn as_ascii(&self) -> Vec<String> {
        (&self.map.locs).into_iter().map(|row|
            (&row).into_iter().map(|loc| {
                self.map_key.iter().find_map(|(ch,objs)|
                    if loc.0 == *objs {Some(ch.to_string())} else {None}
                ).unwrap_or("?".to_string())
            }).collect::<Vec<_>>().join("")
        ).collect()
    }
}

impl Index<MapHandle> for Map {
    type Output = Obj;

    fn index(&self, pos: MapHandle) -> &Self::Output {
        &self.locs[pos.x as usize][pos.y as usize].0[pos.h as usize]
    }
}

impl IndexMut<MapHandle> for Map {
    fn index_mut(&mut self, pos: MapHandle) -> &mut Self::Output {
        &mut self.locs[pos.x as usize][pos.y as usize].0[pos.h as usize]
    }
}

impl std::fmt::Debug for Map {
    #[try_fn]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Map[")?;
        for (x, y, loc) in self.locs() {
            loc.map_fmt(f)?;
            if x ==0 && y > 0 {
                write!(f, "|")?;
            }
        }
        write!(f, "]")?;
    }
}

impl Map {
    pub fn new(sz: u16) -> Map {
        Map {
            locs: vec!(vec!(Loc::new(); sz.into()); sz.into()),
        }
    }

    pub fn w(&self) -> u16 {
        self.locs.len() as u16
    }

    pub fn h(&self) -> u16 {
        self.locs[0].len() as u16
    }

    #[allow(dead_code)]
    pub fn is_edge(&self, x: i16, y: i16) -> bool {
        x == 0 || x == self.w() as i16 -1 || y == 0 || y == self.h() as i16 -1
    }

    /// Place a (copy of an) object into the map. Return a handle to its position.
    ///
    /// Only used by Field::place_obj_at which keeps Roster in sync.
    fn place_obj_at(&mut self, x: i16, y:i16, orig_obj: Obj) -> MapHandle {
        let new_pos = MapHandle::from_xyh(x, y, self.at_xy(x, y).len() as u16);
        let prev_pos = if orig_obj.cached_pos.x >=0 { orig_obj.cached_pos } else {new_pos};
        self.at_xym(x, y).push(
            Obj {
                cached_pos: new_pos,
                prev_pos,
                ..orig_obj
            }
        );
        new_pos
    }

    /// Move an obj identified by hdl from one loc to another. Update hdl to match.
    ///
    /// Also update prev_pos.
    ///
    /// All moves should go through this function. Anything using the play class
    /// should call it with a handle from the roster so the roster is updated.
    ///
    /// TODO: Reduce need for code outside map.rs to know how to use roster handles.
    pub fn move_to(&mut self, hdl: &mut MapHandle, to: MapCoord) {
        let on_top = hdl.h as usize == self.at_hdl(*hdl).len();

        let orig_obj = if on_top {
            // Pop ent from top of stack.
            self.at_hdlm(*hdl).pop().unwrap()
        } else {
            // Replace ent with a placeholder type ignored by render and gameplay.
            // This keeps height coords of other ents valid.
            // ENH: Can we update the other objects here and do away with placeholder?
            // Would need to update Roster in sync.
            mem::replace(&mut self[*hdl], Obj::placeholder())
        };

        let obj = Obj {prev_pos: *hdl, ..orig_obj};

        // Remove any placeholders now at the top of the stack. Should only happen
        // if we popped ent from on top of them.
        while !self.at_hdl(*hdl).is_empty() &&
            self.at_hdl(*hdl).last().unwrap().is_placeholder() {
            self.at_hdlm(*hdl).pop();
        }

        // Update caller's handle to new coords. Height will be set by put_at().
        *hdl = to.to_hdl();

        // Add Ent to top of stack at new map coords. Updates hdl to match new height.
        *hdl = self.place_obj_at(to.x, to.y, obj);
    }

    // As move_to, but move relative not abs.
    pub fn move_delta(&mut self, pos: &mut MapHandle, delta: CoordDelta) {
        self.move_to(pos, MapCoord::from_hdl(*pos) + delta);
    }

    pub fn can_move(&self, pos: MapHandle, delta: CoordDelta) -> bool {
        self.loc_at( MapCoord::from_hdl(pos) + delta ).passable()
    }

    // Loc at given coords.
    pub fn loc_at_xy(&self, x: i16, y: i16) -> &Loc {
        &self.locs[x as usize][y as usize]
    }

    // Loc at given MapCoord.
    pub fn loc_at(&self, pos: MapCoord) -> &Loc {
        self.loc_at_xy(pos.x, pos.y)
    }

    // Ents at given MapCoord.
    pub fn at_xy(&self, x: i16, y:i16) -> &Vec<Obj> {
        &self.loc_at_xy(x, y).0
    }

    // Ents at given MapCoord.
    // Used to add and remove from map, mostly internally. And in Play?
    pub fn at(&self, pos: MapCoord) -> &Vec<Obj> {
        &self.loc_at(pos).0
    }

    pub fn at_hdl(&self, pos: MapHandle) -> &Vec<Obj> {
        &self.loc_at(MapCoord::from_hdl(pos)).0
    }

    // As "at" but mutably
    pub fn at_hdlm(&mut self, pos: MapHandle) -> &mut Vec<Obj> {
        &mut self.locs[pos.x as usize][pos.y as usize].0
    }

    // As "at" but mutably
    pub fn at_xym(&mut self, x: i16, y: i16) -> &mut Vec<Obj> {
        &mut self.locs[x as usize][y as usize].0
    }

    // e.g. `for ( x, y ) in map.coords()`
    #[allow(dead_code)]
    pub fn coords(&self) -> CoordIterator {
        CoordIterator {
            w: self.w(),
            h: self.h(),
            x: 0,
            y: -1,
        }
    }

    pub fn locs(&self) -> LocIterator {
        LocIterator {
            w: self.w(),
            h: self.h(),
            x: 0,
            y: -1,
            map: &self,
        }
    }

    /*
    fn locs_mut(&mut self) -> LocIteratorMut {
        LocIteratorMut {
            w: self.w(),
            h: self.h(),
            x: 0,
            y: -1,
            map: self,
        }
    }
    */
}

pub struct CoordIterator {
    // Original dimensions to iterate up to
    w: u16,
    h: u16,
    // Previously returned coords, or (0, -1) initially.
    x: i16,
    y: i16,
}

pub struct LocIterator<'a> {
    // Original dimensions to iterate up to
    w: u16,
    h: u16,
    // Previously returned coords, or (0, -1) initially.
    x: i16,
    y: i16,
    // Pointer back to original collection
    map: &'a Map,
}

/*
struct LocIteratorMut<'a> {
    // Original dimensions to iterate up to
    w: u16,
    h: u16,
    // Previously returned coords, or (0, -1) initially.
    x: i16,
    y: i16,
    // Pointer back to original collection
    map: &'a mut Map,
}
*/

impl Iterator for CoordIterator {
    type Item = (i16, i16);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < (self.h-1) as i16 {
            // Continue to next coord down current column
            self.y += 1;
            Some((self.x, self.y))
        } else if self.x < (self.w-1) as i16 {
            // Continue to top of next column
            self.x += 1;
            self.y = 0;
            Some((self.x, self.y))
        } else {
            // Previous coord was w-1, h-1, the last coord.
            None
        }
    }
}

impl<'a> Iterator for LocIterator<'a> {
    type Item = (i16, i16, &'a Loc);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < (self.h-1) as i16 {
            // Continue to next coord down current column
            self.y += 1;
            Some((self.x, self.y, &self.map.locs[self.x as usize][self.y as usize]))
        } else if self.x < (self.w-1) as i16 {
            // Continue to top of next column
            self.x += 1;
            self.y = 0;
            Some((self.x, self.y, &self.map.locs[self.x as usize][self.y as usize]))
        } else {
            // Previous coord was w-1, h-1, the last coord.
            None
        }
    }
}

/*
// STUB: Fix or remove
impl<'a> Iterator for LocIteratorMut<'a> {
    type Item = (i16, i16, &'a mut Loc);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < (self.h-1) as i16 {
            // Continue to next coord down current column
            self.y += 1;
            Some((self.x, self.y, &mut self.map.locs[self.x as usize][self.y as usize]))
        } else if self.x < (self.w-1) as i16 {
            // Continue to top of next column
            self.x += 1;
            self.y = 0;
            Some((self.x, self.y, &mut self.map.locs[self.x as usize][self.y as usize]))
        } else {
            // Previous coord was w-1, h-1, the last coord.
            None
        }
    }
}
*/

/// Roster of character, enemies, etc. Indexes into map.
///
/// TODO: Rename Roster
/// TODO: Do we benefit much from caching moveables rather than iterating whole map?
/// Mainly only for hero for things which need to know the coords of the hero?
/// Does it help to make Roster a non-mutable cache of hero location?
#[derive(Clone, Debug)]
pub struct Ros {
    // Hero
    // FIXME: Better name for protagonist than "hero".
    pub hero: MapHandle,

    // Anything which updates each tick, especially enemies.
    //
    // Might be replaced by a set of lists of "everything that has this property" etc
    // like a Component system.
    pub movs: Vec<MapHandle>,
}

impl Ros {
    pub fn new() -> Ros {
        Ros {
            hero: MapHandle::from_xyh(0, 0, 1),
            movs: vec![],
        }
    }

    pub fn push_mov(&mut self, hdl: MapHandle) {
        self.movs.push(hdl);
    }
}

// "Location": Everything at a single coordinate in the current room.
// #[derive(Clone)] // implemented below
#[derive(Debug, Clone)]
pub struct Loc(Vec<Obj>);

/// Square in map. Almost equivalent to Vec<Obj>
///
/// Should make it Vec<Objs> newtype with push etc and these impl fns
///
/// TODO: Remove remaining places in this file using .0
/// TODO: Check places using .at and see if they do need a list of objs or not.
impl Loc {
    pub fn new() -> Loc {
        Loc { 0: vec![] }
    }

    pub fn passable(&self) -> bool {
        !self.impassable()
    }

    pub fn impassable(&self) -> bool {
        // Can this fn work without knowledge of specific properties?
        use obj::Pass;
        self.iter().any(|x| x.pass == Pass::Solid)
    }

    fn map_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for ent in self {
            write!(f, "{},", ent.name)?;
        }
        write!(f, ";")
    }

    pub fn iter(&self) -> std::slice::Iter<'_,Obj> {
        self.0.iter()
    }

    /// Or implement SliceIndex?
    pub fn get(&self, idx: usize) -> Option<&Obj> {
        self.0.get(idx)
    }
}

impl IntoIterator for Loc {
    type Item = <Vec<Obj> as IntoIterator>::Item;
    type IntoIter = <Vec<Obj> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Loc {
    type Item = <&'a Vec<Obj> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<Obj> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Previously we only cloned empty locs, now we clone maps.
/* impl Clone for Loc {
    fn clone(&self) -> Loc {
        assert!(self.ents.is_empty());
        Loc::new()
    }

    // Consider implementing index [idx] for Loc returning loc.ents[idx]
} */
