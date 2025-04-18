// Map types.
//
// Map is a 2d array of Loc. A Loc is a stack of Objs.
// Field is a Map along with a Roster of moveable objects.
//
// But movement logic etc are in Play.
// These are also used by level data files, even though
// they don't need any of the indexing.

use std::cell::RefCell;
use std::mem;
use std::ops::Add;
use std::ops::Index;
use std::ops::IndexMut;
use macroquad::input::KeyCode;

use culpa::try_fn;

use crate::scripts::*;

use super::scene::SceneEnding;

use super::map_coords::*;

use super::obj::Obj;

// "Map": Grid of locations. Represents state of current level.
#[derive(Clone)]
pub struct InternalMap {
    // Stored as a collection of columns, e.g. map.locs[x][y]
    // Must always be rectangular.
    locs: Vec<Vec<Loc>>,
}

/// A handle identifying an Ent in the map.
///
/// Implemented as the MapCoord and index into Ents at that Loc.
///
/// TODO: Include RosIdx in MapHandle too??
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

    pub fn invalid() -> MapHandle {
        MapHandle {
            x: -1, // For now "-1" flags "this element is a placeholder in height vector"
            y: -1,
            h: 0,
        }
    }

    // TODO: Indicates places which shouldn't take a handle to start with..
    pub fn from_coord(coord: MapCoord) -> MapHandle {
        MapHandle{x: coord.x, y: coord.y, h:0 }
    }

    pub fn to_coord(pos: MapHandle) -> MapCoord {
        MapCoord { x: pos.x, y: pos.y}
    }
}

// TODO: Can we remove this? And "use Add".
impl Add<CoordDelta> for MapHandle {
    type Output = MapCoord;
    fn add(self, delta: CoordDelta) -> MapCoord {
        MapCoord { x: self.x + delta.dx, y: self.y + delta.dy }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)] // , Add, Mul
pub struct RichMapHandle {
    // TODO: Think of as "Mov handle"? Think of ros_idx as value and x, y, h as cached coords?
    pub ros_idx: usize,
    pub x: i16,
    pub y: i16,
    pub h: u16,
}

impl RichMapHandle {
    fn pos(self: RichMapHandle) -> MapCoord {
        MapCoord{x: self.x, y: self.y}
    }
}

/// Map together with Ros. Those are two separate classes so they can more easily be borrowed separately.
#[derive(Clone, Debug)]
pub struct Field {
    pub map: RefCell<InternalMap>,
    // Moveable objects in the current map.
    pub roster: Roster,
    // Key used to represent things in map as ascii for init and debugging. Not comprehensive.
    pub map_key: std::collections::HashMap<char, Vec<Obj>>,
}

impl Field {
    pub fn empty(w: u16, h: u16) -> Field {
        Field {
            map: Into::into(InternalMap::new(w, h)),
            roster: Roster::new(),
            map_key: std::collections::HashMap::new(),
        }
    }

    pub fn advance(&mut self, cmd: Option<Cmd>) -> SceneEnding  {
        // FIXME: Decide order of char, enemy. Before or after not quite right. Or need
        // to handle char moving onto enemy.
        // STUB: Maybe display char moving out of sync with enemy.

        // Before movement, reset "prev". Will be overwritten if movement happens.
        // Should be moved into obj_move*() fn.
        let tmp = self.map.borrow()[self.roster.hero].cached_pos;
        self.map.borrow_mut()[self.roster.hero].prev_pos = tmp;

        let rich_hero = RichMapHandle { ros_idx: 100, x: self.roster.hero.x, y: self.roster.hero.y, h: self.roster.hero.h };
        move_character_refactored(rich_hero, self, cmd)?;

        // Move all movs
        for mov in &mut self.roster.movs {
            // Before movement, reset "prev". Will be overwritten if movement happens.
            let tmp = self.map.borrow()[*mov].cached_pos;
            self.map.borrow_mut()[*mov].prev_pos = tmp;

            move_mov(&mut self.map.borrow_mut(), &self.roster.hero, mov)?;
        }
        SceneEnding::ContinuePlaying
    }

    /// Create an object in the map and in the roster.
    ///
    /// All new objs go through this to keep map and roster in sync.
    pub fn place_obj_at(&mut self, x: i16, y:i16, orig_obj: Obj)
    {
        let hdl = self.map.borrow_mut().place_obj_at(x, y, orig_obj);
        let placed_obj = &self.map.borrow()[hdl];

        if placed_obj.is_hero() {
            self.roster.hero = hdl;
        } else if placed_obj.is_mob() {
            self.roster.push_mov(hdl);
        }
    }

    pub fn obj_can_move_refactored(self: &mut Field, rich_hdl: RichMapHandle, dir: CoordDelta) -> bool {
        self.map.borrow().obj_can_move_refactored(rich_hdl, dir)
    }

    pub fn obj_move_delta_refactored(&mut self, rich_hdl: RichMapHandle, delta: CoordDelta) {
        let mov_roster_hdl = &mut self.roster[rich_hdl.ros_idx];
        self.map.borrow_mut().obj_move_to(mov_roster_hdl, *mov_roster_hdl + delta);
    }

    /// Ascii representation of map. Test functions check it's as expected.
    pub fn as_ascii_cols(&self) -> Vec<String> {
        (&self.map.borrow().locs).into_iter().map(|row|
            (&row).into_iter().map(|loc| {
                self.map_key.iter().find_map(|(ch,objs)|
                    if loc.0 == *objs {Some(ch.to_string())} else {None}
                ).unwrap_or("?".to_string())
            }).collect::<Vec<_>>().join("")
        ).collect()
    }

    /// Ascii representation of map. Test functions check it's as expected.
    pub fn as_ascii_rows(&self) -> Vec<String> {
        (0..self.map.borrow().h() as i16).map(|y|
            (0..self.map.borrow().w() as i16).map(|x| {
                self.map_key.iter().find_map(|(ch,objs)|
                    if self.map.borrow().at_xy(x,y) == objs {Some(ch.to_string())} else {None}
                ).unwrap_or("?".to_string())
            }).collect::<Vec<_>>().join("")
        ).collect()
    }
}

impl Index<MapHandle> for InternalMap {
    type Output = Obj;

    fn index(&self, pos: MapHandle) -> &Self::Output {
        &self.locs[pos.x as usize][pos.y as usize].0[pos.h as usize]
    }
}

impl IndexMut<MapHandle> for InternalMap {
    fn index_mut(&mut self, pos: MapHandle) -> &mut Self::Output {
        &mut self.locs[pos.x as usize][pos.y as usize].0[pos.h as usize]
    }
}

impl std::fmt::Debug for InternalMap {
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

impl InternalMap {
    pub fn new(w: u16, h: u16) -> InternalMap {
        InternalMap {
            locs: vec!(vec!(Loc::new(); h.into()); w.into()),
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
    /// Only used externally by Field::place_obj_at which keeps Roster in sync.
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
    pub fn obj_move_to(&mut self, hdl: &mut MapHandle, to: MapCoord) {
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
        *hdl = MapHandle::from_coord(to);

        // Add Ent to top of stack at new map coords. Updates hdl to match new height.
        *hdl = self.place_obj_at(to.x, to.y, obj);
    }

    // As move_to, but move relative not abs.
    pub fn obj_move_delta(&mut self, pos: &mut MapHandle, delta: CoordDelta) {
        self.obj_move_to(pos, *pos + delta);
    }

    pub fn obj_can_move_refactored(&self, hdl: RichMapHandle, delta: CoordDelta) -> bool {
        self.loc_at( hdl.pos() + delta ).passable()
    }

    pub fn obj_can_move(&self, pos: MapHandle, delta: CoordDelta) -> bool {
        self.loc_at( pos + delta ).passable()
    }

    // Loc at given coords.
    pub fn loc_at_xy(&self, x: i16, y: i16) -> &Loc {
        &self.locs[x as usize][y as usize]
    }

    // Loc at given MapCoord.
    // TODO: Instead make loc indexable, and have at() or [] return loc?
    pub fn loc_at(&self, pos: MapCoord) -> &Loc {
        self.loc_at_xy(pos.x, pos.y)
    }

    // Ents at given MapCoord.
    pub fn at_xy(&self, x: i16, y:i16) -> &Vec<Obj> {
        &self.loc_at_xy(x, y).0
    }

    // Ents at given MapCoord.
    // Used to add and remove from map, mostly internally. And in Play?
    #[allow(dead_code)]
    pub fn at(&self, pos: MapCoord) -> &Vec<Obj> {
        &self.loc_at(pos).0
    }

    pub fn at_hdl(&self, pos: MapHandle) -> &Vec<Obj> {
        &self.loc_at(MapHandle::to_coord(pos)).0
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
    map: &'a InternalMap,
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

type RosIndex = usize;

/// Roster of objects which move autonomously.
///
/// Objects are stored as MapHandles.
///
/// It would be simpler to iterate through the Map looking for any moveable objects, but
/// it's theoretically correct to have a roster. Especially for hero location.
///
/// Would still like to simplify how ownership of map objects works.
#[derive(Clone, Debug)]
pub struct Roster {
    // Hero
    // FIXME: Better name for protagonist than "hero".
    pub hero: MapHandle,

    // Anything which updates each tick, especially enemies.
    //
    // Might be replaced by a set of lists of "everything that has this property" etc
    // like a Component system.
    pub movs: Vec<MapHandle>,
}

impl Roster {
    pub fn new() -> Roster {
        Roster {
            hero: MapHandle::from_xyh(0, 0, 1),
            movs: vec![],
        }
    }

    pub fn push_mov(&mut self, hdl: MapHandle) {
        self.movs.push(hdl);
    }
}

impl Index<RosIndex> for Roster {
    type Output = MapHandle;

    fn index(&self, idx: RosIndex) -> &Self::Output {
        match idx {
            0..99 => &self.movs[idx],
            99 => panic!("Used invalid 99 index into roster"),
            100 => &self.hero,
            _ => panic!("Unknown index into roster: {}", idx),
        }
    }
}

impl IndexMut<RosIndex> for Roster {
    fn index_mut(&mut self, idx: RosIndex) -> &mut Self::Output {
        match idx {
            0..99 => &mut self.movs[idx],
            99 => panic!("Used invalid 99 index into roster"),
            100 => &mut self.hero,
            _ => panic!("Unknown index into roster: {}", idx),
        }
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
