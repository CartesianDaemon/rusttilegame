// Map types.
//
// Map is a 2d array of Loc. A Loc is a stack of Objs.
// Field is a Map along with a Roster of moveable objects.
//
// But movement logic etc are in Play.
// These are also used by level data files, even though
// they don't need any of the indexing.

use std::collections::HashMap;
use std::ops::Index;
use std::ops::IndexMut;

use culpa::try_fn;

use crate::scripts::*;

use super::scene::SceneContinuation;

use super::map_coords::*;

use super::obj::ObjProperties;

#[derive(Copy, Clone, PartialEq, Debug)] // , Add, Mul
pub struct RosterIndex {
    pub ros_idx: u16,
}

/// Map together with Ros. Those are two separate classes so they can more easily be borrowed separately.
#[derive(Clone, Debug)]
pub struct Field {
    map: Map,
    roster: Roster,
    // Used to represent map as ascii for init and debugging. Not comprehensive.
    map_key: std::collections::HashMap<char, Vec<ObjProperties>>,
}

impl Field {
    /////////////////
    /// Initialisers
    pub fn empty(w: u16, h: u16) -> Field {
        Field {
            map: Into::into(Map::new(w, h)),
            roster: Roster::new(),
            map_key: std::collections::HashMap::new(),
        }
    }

    pub fn from_map_and_key<const HEIGHT: usize>(
        ascii_map: &[&str; HEIGHT],
        map_key: HashMap<char, Vec<ObjProperties>>,
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

    //////////////////////////////////////////////
    /// Exposed upward to front end of game engine

    pub fn advance(&mut self, cmd: Cmd) -> SceneContinuation  {
        // TODO: Decide order of char, enemy. Before or after not quite right. Or need
        // to handle char moving onto enemy.
        // TODO: Consider: Maybe display char moving out of sync with enemy.

        // Before movement, reset "prev". Will be overwritten if movement happens.
        // Should be moved into obj_move*() fn.
        self.hero_backpos_m().prev_pos = self.hero_backpos().curr_pos;

        move_mov(self, Roster::hero_handle(), cmd)?;

        for rich_mov in self.roster.all_movs() {
            // Before movement, reset "prev". Will be overwritten if movement happens.
            // Going through tmp is necessary to avoid two dynamic borrows at the same time..
            // NOTE: If map is RefCell needs to be done in two steps else runtime panic.
            // NOTE: And obj_at() is also incompatible with RefCell.
            self.backpos_m(rich_mov).prev_pos = self.obj_pos(rich_mov);

            move_mov(self, rich_mov, cmd)?;
        }
        SceneContinuation::Continue(())
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

    //////////////////////////////////////////////////////////////////////////////////
    /// Obj spawn, move and update fns.
    ///
    /// Objects are only spawned or moved in map by place_obj_at and move_obj_to. Those
    /// functions update coords in roster, roster_idx, prev_pos, curr_pos to maintain
    /// consistency.
    ///
    /// Objects can be changed but not moved by other obj fns. Mainly (only?) .obj().

    /// Spawn new object.
    pub fn spawn_obj_at(&mut self, x: i16, y:i16, props: ObjProperties)
    {
        let pos = MapCoord::from_xy(x, y);
        let h = self.map[pos].objs().len() as u16;
        let new_roster_idx = self.roster.add_to_roster_if_mov( MapRef{x, y, h}, &props );
        let backpos = Backpos {
            curr_roster_idx: new_roster_idx,
            curr_pos: pos,
            prev_pos: pos,
        };
        self.map[pos].objs_m().push( Obj{backpos, props} );
    }

    /// Move obj to a new location.
    ///
    /// Update roster and backpos.curr_pos and backpos.prev_pos. Still untested for multiple movs.
    pub fn move_obj_to(&mut self, roster_idx: RosterIndex, target_pos: MapCoord) {
        let orig_pos = self.roster[roster_idx].pos();
        let orig_h = self.roster[roster_idx].h;

        // Remove object from previous map location.
        let obj = self.map[orig_pos].objs_m().remove(orig_h as usize);

        // For each other object in location, update its mapref in roster with changed height.
        for h in orig_h+1..self.map[orig_pos].len() as u16 {
            let other_roster_idx = self.backpos_at_ref(MapRef {x: orig_pos.x, y: orig_pos.y, h}).curr_roster_idx;
            self.roster[other_roster_idx].h = h;
        }

        // TODO: Put in assert that put_obj_in_map_and_return_updated_mapref updates prev_pos as expected.
        // let obj = Obj {prev_pos: mapref.pos(), ..obj};

        // Add object to top of stack at new map location.
        self.map[target_pos].objs_m().push(
            Obj {
                backpos: Backpos {
                    curr_roster_idx: obj.backpos.curr_roster_idx,
                    curr_pos: target_pos,
                    prev_pos: obj.backpos.curr_pos,
                },
               props: obj.props,
            }
        );

        // Update roster hdl to match new position and height.
        self.roster[roster_idx].x = target_pos.x;
        self.roster[roster_idx].y = target_pos.y;
        self.roster[roster_idx].h = self.map[target_pos].len() as u16 -1;
    }

    pub fn obj(&self, roster_idx: RosterIndex) -> &ObjProperties {
        self.props_at_ref(self.roster[roster_idx])
    }

    pub fn objm(&mut self, roster_idx: RosterIndex) -> &mut ObjProperties {
        self.props_at_ref_m(self.roster[roster_idx])
    }

    pub fn backpos(&self, roster_idx: RosterIndex) -> &Backpos {
        self.backpos_at_ref(self.roster[roster_idx])
    }

    pub fn obj_pos(&self, roster_idx: RosterIndex) -> MapCoord {
        self.roster[roster_idx].pos()
    }

    #[allow(dead_code)]
    pub fn hero_props(&mut self) -> &mut ObjProperties {
        self.objm(Roster::hero_handle())
    }

    pub fn hero_backpos(&self) -> &Backpos {
        self.backpos(Roster::hero_handle())
    }

    pub fn hero_pos(&self) -> MapCoord {
        self.obj_pos(Roster::hero_handle())
    }

    // TODO: Only valid if "dir" represents actual direction of movement, not just facing.
    pub fn obj_target_pos(&self, roster_idx: RosterIndex) -> MapCoord {
        self.obj_pos(roster_idx) + self.obj(roster_idx).dir
    }

    pub fn any_effect(&self, pos: MapCoord, sought_effect: Effect) -> bool {
        self.map[pos].any_effect(sought_effect)
    }

    pub fn all_pass(&self, pos: MapCoord, sought_pass: Pass) -> bool {
        self.map[pos].all_pass(sought_pass)
    }

    //////////////////////////////////////////////
    /// More obj functions, non-pub helpers
    ///
    /// ..?

    // Only used one place. Might be better to inline?
    pub fn backpos_m(&mut self, roster_idx: RosterIndex) -> &mut Backpos {
        self.backpos_at_ref_m(self.roster[roster_idx])
    }

    pub fn hero_backpos_m(&mut self) -> &mut Backpos {
        self.backpos_m(Roster::hero_handle())
    }

    // TODO: Could have a dummy intermediate class self.ref[mapref]
    fn props_at_ref(&self, mapref: MapRef) -> &ObjProperties {
        &self.map.locs[mapref.x as usize][mapref.y as usize][mapref.h as usize].props
    }

    fn props_at_ref_m(&mut self, mapref: MapRef) -> &mut ObjProperties {
        &mut self.map.locs[mapref.x as usize][mapref.y as usize][mapref.h as usize].props
    }

    fn backpos_at_ref(&self, mapref: MapRef) -> &Backpos {
        &self.map.locs[mapref.x as usize][mapref.y as usize][mapref.h as usize].backpos
    }

    fn backpos_at_ref_m(&mut self, mapref: MapRef) -> &mut Backpos {
        &mut self.map.locs[mapref.x as usize][mapref.y as usize][mapref.h as usize].backpos
    }

    //////////////////////////////////////////////////////
    /// Representations of map. Used in logging and debug.

    /// Ascii representation of map. Test functions check it's as expected.
    #[allow(dead_code)]
    pub fn as_ascii_cols(&self) -> Vec<String> {
        (&self.map.locs).into_iter().map(|row|
            (&row).into_iter().map(|loc| {
                self.map_key.iter().find_map(|(ch,objs)|
                    if loc.obj_props() == *objs {Some(ch.to_string())} else {None}
                ).unwrap_or("?".to_string())
            }).collect::<Vec<_>>().join("")
        ).collect()
    }

    /// Ascii representation of map. Test functions check it's as expected.
    #[cfg(test)]
    pub fn as_ascii_rows(&self) -> Vec<String> {
        (0..self.map.h() as i16).map(|y|
            (0..self.map.w() as i16).map(|x| {
                self.map_key.iter().find_map(|(ch,objs)|
                    if self.map[MapCoord::from_xy(x, y)].obj_props() == *objs {Some(ch.to_string())} else {None}
                ).unwrap_or("?".to_string())
            }).collect::<Vec<_>>().join("")
        ).collect()
    }
}

// TODO: Better name. "Cachedpos"? ..?
#[derive(Clone, Debug)]
pub struct Backpos {
    pub curr_roster_idx: RosterIndex,
    pub curr_pos: MapCoord,
    pub prev_pos: MapCoord,
}

#[derive(Clone, Debug)]
pub struct Obj { // TODO: Rename MapObj?
    pub backpos: Backpos,
    pub props: ObjProperties,
}

// "Map": Grid of locations. Represents state of current level.
// NOTE: Could currently be moved back into Field. Not borrowed separately.
#[derive(Clone)]
struct Map {
    // Stored as a collection of columns, e.g. map.locs[x][y]
    // Must always be rectangular.
    locs: Vec<Vec<Loc>>,
}

impl Map {
    pub fn new(w: u16, h: u16) -> Map {
        Map {
            locs: vec!(vec!(Loc::new(); h.into()); w.into()),
        }
    }

    pub fn w(&self) -> u16 {
        self.locs.len() as u16
    }

    pub fn h(&self) -> u16 {
        self.locs[0].len() as u16
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

impl Index<MapCoord> for Map {
    type Output = Loc;

    fn index(&self, pos: MapCoord) -> &Self::Output {
        &self.locs[pos.x as usize][pos.y as usize]
    }
}

impl IndexMut<MapCoord> for Map {
    fn index_mut(&mut self, pos: MapCoord) -> &mut Self::Output {
        &mut self.locs[pos.x as usize][pos.y as usize]
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
struct MapRef {
    pub x: i16,
    pub y: i16,
    pub h: u16,
}

impl MapRef
{
    pub fn pos(self: MapRef) -> MapCoord {
        MapCoord { x: self.x, y: self.y}
    }
}

/// Roster of objects which move autonomously.
///
/// Objects are stored as MapRef.
///
/// It would be simpler to iterate through the Map looking for any moveable objects, but
/// it's theoretically correct to have a roster. Especially for hero location.
///
/// Would still like to simplify how ownership of map objects works.
///
/// Could in theory extend to a component-like system storing overlapping lists of
/// indexhandles for "objects with this property".
//
// NOTE: Could currently be moved back into Field. Not borrowed separately.
#[derive(Clone, Debug)]
struct Roster {
    pub hero: MapRef,

    movs: Vec<MapRef>,
}

impl Roster {
    pub fn new() -> Roster {
        Roster {
            hero: MapRef{x:0, y:0, h:1}, // Overwritten immediate, but can we avoid placeholder?
            movs: vec![],
        }
    }

    pub fn hero_handle() -> RosterIndex {
        RosterIndex { ros_idx: 100 }
    }

    pub fn non_mov_handle() -> RosterIndex {
        RosterIndex { ros_idx: 98 }
    }

    pub fn all_movs(&self) -> Vec<RosterIndex> {
        // TODO: Possible to return iter() instead of collection, without borrow problems?
        (0..self.movs.len() as u16).into_iter().map(|ros_idx| RosterIndex { ros_idx } ).collect()
    }

    fn add_to_roster_if_mov(&mut self, mapref: MapRef, props: &ObjProperties) -> RosterIndex {
        if ObjProperties::is_hero(props.ai) {
        self.hero = mapref;
            Self::hero_handle()
        } else if ObjProperties::is_mob(props.ai) {
            self.movs.push(mapref);
            RosterIndex { ros_idx: self.movs.len() as u16 - 1 }
        } else {
            Self::non_mov_handle()
        }
    }
}

impl Index<RosterIndex> for Roster {
    type Output = MapRef;

    fn index(&self, hdl: RosterIndex) -> &Self::Output {
        let idx = hdl.ros_idx as usize;
        match idx {
            0..99 => &self.movs[idx],
            99 => panic!("Used invalid 99 index into roster"),
            100 => &self.hero,
            _ => panic!("Unknown index into roster: {}", idx),
        }
    }
}

impl IndexMut<RosterIndex> for Roster {
    fn index_mut(&mut self, hdl: RosterIndex) -> &mut Self::Output {
        let idx = hdl.ros_idx as usize;
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
/// TODO: Check places using .at and see if they do need a list of objs or not.
impl Loc {
    pub fn new() -> Loc {
        Loc { 0: vec![] }
    }

    pub fn any_effect(&self, sought_effect: Effect) -> bool {
        self.0.iter().any(|x| x.props.effect == sought_effect)
    }

    pub fn any_pass(&self, sought_pass: Pass) -> bool {
        self.0.iter().any(|x| x.props.pass == sought_pass)
    }

    pub fn all_pass(&self, sought_pass: Pass) -> bool {
        self.0.iter().all(|x| x.props.pass == sought_pass)
    }

    fn map_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for ent in self {
            write!(f, "{},", ent.props.name)?;
        }
        write!(f, ";")
    }

    /// Only used by render() when unsure about height?
    pub fn get(&self, idx: usize) -> Option<&Obj> {
        self.0.get(idx)
    }

    // Reimplementations of list operations. Any better way of avoiding without lots of ".0"?
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn objs(&self) -> &Vec<Obj> {
        &self.0
    }

    pub fn objs_m(&mut self) -> &mut Vec<Obj> {
        &mut self.0
    }

    pub fn obj_props(&self) -> Vec<ObjProperties> {
        // TODO: Avoid clone
        self.0.iter().map(|o| o.props.clone()).collect()
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
        self.0.iter()
    }
}

impl Index<usize> for Loc {
    type Output = Obj;

    fn index(&self, h: usize) -> &Self::Output {
        &self.0[h]
    }
}

impl IndexMut<usize> for Loc {
    fn index_mut(&mut self, h: usize) -> &mut Self::Output {
        &mut self.0[h]
    }
}
