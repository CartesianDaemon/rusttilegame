// Map types.
//
// Map is a 2d array of Loc. A Loc is a stack of Objs.
// Field is a Map along with a Roster of moveable objects.
//
// But movement logic etc are in Play.
// These are also used by level data files, even though
// they don't need any of the indexing.

use std::mem;
use std::collections::HashMap;
use std::ops::Index;
use std::ops::IndexMut;

use culpa::try_fn;

use crate::scripts::*;

use super::scene::SceneEnding;

use super::map_coords::*;

use super::obj::Obj;

#[derive(Copy, Clone, PartialEq, Debug)] // , Add, Mul
pub struct RosterHandle {
    // TODO: Think of as "Mov handle"? Think of ros_idx as value and x, y, h as cached coords?
    pub ros_idx: RosIndex,
}

impl RosterHandle {
    pub fn invalid() -> RosterHandle {
        RosterHandle {ros_idx: 99}
    }
}

/// Map together with Ros. Those are two separate classes so they can more easily be borrowed separately.
#[derive(Clone, Debug)]
pub struct Field {
    map: InternalMap,
    roster: Roster,
    // Used to represent map as ascii for init and debugging. Not comprehensive.
    map_key: std::collections::HashMap<char, Vec<Obj>>,
}

impl Field {
    pub fn empty(w: u16, h: u16) -> Field {
        Field {
            map: Into::into(InternalMap::new(w, h)),
            roster: Roster::new(),
            map_key: std::collections::HashMap::new(),
        }
    }

    pub fn from_map_and_key<const HEIGHT: usize>(
        ascii_map: &[&str; HEIGHT],
        map_key: HashMap<char, Vec<Obj>>,
    ) -> Field {
        let mut field = Field {
            map_key: map_key.clone(),
            ..Field::empty(ascii_map[0].len() as u16, HEIGHT as u16)
        };

        for (y, line) in ascii_map.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                for ent in map_key.get(&ch).unwrap() {
                    field.spawn_obj_at(x as i16, y as i16, ent.clone());
                }
            }
        }

        field
    }

    pub fn advance(&mut self, cmd: Cmd) -> SceneEnding  {
        // TODO: Decide order of char, enemy. Before or after not quite right. Or need
        // to handle char moving onto enemy.
        // TODO: Consider: Maybe display char moving out of sync with enemy.

        // Before movement, reset "prev". Will be overwritten if movement happens.
        // Should be moved into obj_move*() fn.
        self.obj_props_m(self.roster.hero_hdl()).prev_pos = self.obj_props(self.roster.hero_hdl()).curr_pos;

        move_mov(self, self.rich_hero(), cmd)?;

        // Transitioning to this version of "Move all movs"
        for rich_mov in self.roster.all_movs() {
            // Before movement, reset "prev". Will be overwritten if movement happens.
            // Going through tmp is necessary to avoid two dynamic borrows at the same time..
            // NOTE: If map is RefCell needs to be done in two steps else runtime panic.
            // NOTE: And obj_at() is also incompatible with RefCell.
            self.obj_props_m(rich_mov).prev_pos = self.obj_pos(rich_mov);

            move_mov(self, rich_mov, cmd)?;
        }
        SceneEnding::ContinuePlaying
    }

    pub fn map_w(&self) -> u16 {
        self.map.w()
    }

    pub fn map_h(&self) -> u16 {
        self.map.h()
    }

    // TODO: Any better way to expose this for iterating?
    pub fn map_locs(&self) -> LocIterator {
        self.map.locs()
    }

    /// Spawn new object.
    ///
    /// Internally adds it to the map, and to the roster if its animate.
    pub fn spawn_obj_at(&mut self, x: i16, y:i16, orig_obj: Obj)
    {
        let objmapref = self.put_obj_in_map_and_return_updated_objmapref(x, y, orig_obj);
        self.map[objmapref].curr_roster_handle = self.roster.add_to_roster_if_mov(objmapref, &self.map[objmapref])
    }

    /// Move obj to a new location.
    ///
    /// Handles placeholder objects to keep height of objects consistent with
    /// handles. Hope to get rid of that if handle's location is stored only
    /// in roster.
    ///
    /// Update roster (actually not needed?), obj.curr_pos and obj.prev_pos.
    ///
    /// TODO: Second half of function is a bit old, could be updated.
    pub fn move_obj_to(&mut self, roster_hdl: RosterHandle, pos: MapCoord) {
        let objmapref = &mut self.roster[roster_hdl.ros_idx];

        let on_top = objmapref.h as usize == self.map.ents_at_objmapref(*objmapref).len();

        let orig_obj = if on_top {
            // Pop ent from top of stack.
            self.map.at_objmaoref_m(*objmapref).pop().unwrap()
        } else {
            // Replace ent with a placeholder type ignored by render and gameplay.
            // This keeps height coords of other ents valid.
            // ENH: Can we update the other objects here and do away with placeholder?
            // Would need to update Roster in sync.
            mem::replace(&mut self.map[*objmapref], Obj::placeholder())
        };

        let obj = Obj {prev_pos: objmapref.pos(), ..orig_obj};

        // Remove any placeholders now at the top of the stack. Should only happen
        // if we popped ent from on top of them.
        while !self.map.ents_at_objmapref(*objmapref).is_empty() &&
            self.map.ents_at_objmapref(*objmapref).last().unwrap().is_placeholder() {
            self.map.at_objmaoref_m(*objmapref).pop();
        }

        // Add Ent to top of stack at new map coords. Updates roster hdl to match new height.
        self.roster[roster_hdl.ros_idx] = self.put_obj_in_map_and_return_updated_objmapref(pos.x, pos.y, obj);
    }

    /// Place an object in the map.
    ///
    /// Update curr_roster_handle, curr_pos, prev_pos. Return new obj ref.
    ///
    /// All obj placement and movement goes through spawn_at or move_obj_to, then this fn.
    fn put_obj_in_map_and_return_updated_objmapref(&mut self, x: i16, y:i16, orig_obj: Obj) -> ObjMapRef {
        let new_curr_pos = MapCoord::from_xy(x, y);
        let obj_ref = ObjMapRef { x, y, h: self.map.ents_at_xy(x, y).len() as u16 };
        let prev_pos = if orig_obj.curr_pos.x >=0 { orig_obj.curr_pos } else {new_curr_pos};
        self.map.at_xym(x, y).push(
            Obj {
                curr_pos: new_curr_pos,
                prev_pos,
                ..orig_obj
            }
        );
        obj_ref
    }

    pub fn rich_hero(&self) -> RosterHandle {
        RosterHandle { ros_idx: 100 }
    }

    pub fn obj_props(&self, roster_hdl: RosterHandle) -> &Obj {
        &self.map[self.roster[roster_hdl.ros_idx]]
    }

    pub fn obj_props_m(&mut self, roster_hdl: RosterHandle) -> &mut Obj {
        &mut self.map[self.roster[roster_hdl.ros_idx]]
    }

    pub fn obj_pos(&self, roster_hdl: RosterHandle) -> MapCoord {
        self.roster[roster_hdl.ros_idx].pos()
    }

    // TODO: Only valid if "dir" represents actual direction of movement, not just facing.
    pub fn obj_target_pos(&self, roster_hdl: RosterHandle) -> MapCoord {
        self.obj_pos(roster_hdl) + self.obj_props(roster_hdl).dir
    }

    pub fn any_effect(&self, pos: MapCoord, sought_effect: Effect) -> bool {
        self.map.loc_at(pos).any_effect(sought_effect)
    }

    pub fn all_pass(&self, pos: MapCoord, sought_pass: Pass) -> bool {
        self.map.loc_at(pos).all_pass(sought_pass)
    }

    /// Ascii representation of map. Test functions check it's as expected.
    pub fn as_ascii_cols(&self) -> Vec<String> {
        (&self.map.locs).into_iter().map(|row|
            (&row).into_iter().map(|loc| {
                self.map_key.iter().find_map(|(ch,objs)|
                    if loc.0 == *objs {Some(ch.to_string())} else {None}
                ).unwrap_or("?".to_string())
            }).collect::<Vec<_>>().join("")
        ).collect()
    }

    /// Ascii representation of map. Test functions check it's as expected.
    pub fn as_ascii_rows(&self) -> Vec<String> {
        (0..self.map.h() as i16).map(|y|
            (0..self.map.w() as i16).map(|x| {
                self.map_key.iter().find_map(|(ch,objs)|
                    if self.map.ents_at_xy(x,y) == objs {Some(ch.to_string())} else {None}
                ).unwrap_or("?".to_string())
            }).collect::<Vec<_>>().join("")
        ).collect()
    }
}

// "Map": Grid of locations. Represents state of current level.
// NOTE: Could currently be moved back into Field. Not borrowed separately.
#[derive(Clone)]
struct InternalMap {
    // Stored as a collection of columns, e.g. map.locs[x][y]
    // Must always be rectangular.
    locs: Vec<Vec<Loc>>,
}

// TODO: at_ and loc_at_ etc fns only used in this file. Simplify what they should be?
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

    // Loc at given MapCoord.
    // TODO: Instead make loc indexable, and have at() or [] return loc?
    pub fn loc_at(&self, pos: MapCoord) -> &Loc {
        &self.locs[pos.x as usize][pos.y as usize]
    }

    // Ents at given coords.
    pub fn ents_at_xy(&self, x: i16, y:i16) -> &Vec<Obj> {
        &self.loc_at(MapCoord::from_xy(x, y)).0
    }

    pub fn ents_at_objmapref(&self, pos: ObjMapRef) -> &Vec<Obj> {
        &self.loc_at(pos.pos()).0
    }

    // As "at" but mutably
    pub fn at_objmaoref_m(&mut self, pos: ObjMapRef) -> &mut Vec<Obj> {
        &mut self.locs[pos.x as usize][pos.y as usize].0
    }

    // As "at" but mutably
    pub fn at_xym(&mut self, x: i16, y: i16) -> &mut Vec<Obj> {
        &mut self.locs[x as usize][y as usize].0
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
}

impl Index<ObjMapRef> for InternalMap {
    type Output = Obj;

    fn index(&self, pos: ObjMapRef) -> &Self::Output {
        &self.locs[pos.x as usize][pos.y as usize].0[pos.h as usize]
    }
}

impl IndexMut<ObjMapRef> for InternalMap {
    fn index_mut(&mut self, pos: ObjMapRef) -> &mut Self::Output {
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

/// Coords and height of Ent in map.
/// Used in Roster to cache index to object to access it.
/// We should keep this ONLY in roster, so it can be updated when objs move.
#[derive(Copy, Clone, PartialEq, Debug)] // , Add, Mul
struct ObjMapRef {
    pub x: i16,
    pub y: i16,
    pub h: u16,
}

impl ObjMapRef
{
    pub fn invalid() -> ObjMapRef {
        ObjMapRef {x: 0, y: 0, h: 1}
    }

    pub fn pos(self: ObjMapRef) -> MapCoord {
        MapCoord { x: self.x, y: self.y}
    }
}

type RosIndex = usize;

/// Roster of objects which move autonomously.
///
/// Objects are stored as MapHandles.
///
/// It would be simpler to iterate through the Map looking for any moveable objects, but
/// it's theoretically correct to have a roster. Especially for hero location.
///
/// Would still like to simplify how ownership of map objects works.
// NOTE: Could currently be moved back into Field. Not borrowed separately.
#[derive(Clone, Debug)]
struct Roster {
    // Hero
    // FIXME: Better name for protagonist than "hero".
    pub hero: ObjMapRef,

    // Anything which updates each tick, especially enemies.
    //
    // Might be replaced by a set of lists of "everything that has this property" etc
    // like a Component system.
    movs: Vec<ObjMapRef>,
}

impl Roster {
    pub fn new() -> Roster {
        Roster {
            hero: ObjMapRef::invalid(),
            movs: vec![],
        }
    }

    pub fn hero_hdl(&self) -> RosterHandle {
        RosterHandle { ros_idx: 100 }
    }

    pub fn non_mov_handle(&self) -> RosterHandle {
        RosterHandle { ros_idx: 98 }
    }

    pub fn all_movs(&self) -> RosterMovsIterator {
        // TODO: Better as return range.into_iter().map()? Or as generator?
        RosterMovsIterator {ros_idx: 0, max_idx: self.movs.len()}
    }

    fn add_to_roster_if_mov(&mut self, objmapref: ObjMapRef, placed_obj: &Obj) -> RosterHandle {
        if placed_obj.is_hero() {
            self.hero = objmapref;
            self.hero_hdl()
        } else if placed_obj.is_mob() {
            self.movs.push(objmapref);
            RosterHandle { ros_idx: self.movs.len()-1 }
        } else {
            self.non_mov_handle()
        }
    }
}

pub struct RosterMovsIterator {
    ros_idx: usize,
    max_idx: usize,
}

impl Iterator for RosterMovsIterator {
    type Item = RosterHandle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ros_idx < self.max_idx {
            self.ros_idx += 1;
            Some(RosterHandle{ ros_idx: self.ros_idx-1})
        } else {
            None
        }
    }
}

impl Index<RosIndex> for Roster {
    type Output = ObjMapRef;

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
            0..98 => &mut self.movs[idx],
            98 => panic!("Used non-mov obj 98 index into roster"),
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

    pub fn any_effect(&self, sought_effect: Effect) -> bool {
        self.iter().any(|x| x.effect == sought_effect)
    }

    pub fn any_pass(&self, sought_pass: Pass) -> bool {
        self.iter().any(|x| x.pass == sought_pass)
    }

    pub fn all_pass(&self, sought_pass: Pass) -> bool {
        self.iter().all(|x| x.pass == sought_pass)
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
