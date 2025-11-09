// Map types.
//
// Map contains a grid of locations, and a roster of moveable objects.
// Refactoring keeps pulling those into separate member classes so they
// can be borrowed separately, then recombining them to be more concise.
//
// Movement logic etc are in Arena.
// The Grid/Map are also exported to level data files.

use std::collections::HashMap;
use std::ops::Index;
use std::ops::IndexMut;

use culpa::try_fn;

use super::pane::{BasePane, PaneContinuation};
use super::simple_custom_props;
use super::for_gamedata::BaseMovementLogic;
use crate::map_coords::Cmd;

use super::map_coords::*;

use super::obj::{FreeObj, LogicalProps, VisualProps};

#[derive(Copy, Clone, PartialEq, Debug)] // , Add, Mul
pub struct RosterIndex {
    ros_idx: u16,
}

/// Grid together with Ros. Those are two separate classes so they can more easily be borrowed separately.
#[derive(Clone, Debug)]
pub struct Arena<MovementLogic: super::for_gamedata::BaseMovementLogic> {
    map: Grid<MovementLogic>,
    roster: Roster,
    // Used to represent map as ascii for init and debugging. Not comprehensive.
    map_key: std::collections::HashMap<char, Vec<FreeObj<MovementLogic::CustomProps>>>,
}

impl<MovementLogic : super::for_gamedata::BaseMovementLogic> BasePane for Arena<MovementLogic>
{
    fn advance(&mut self, cmd: Option<Cmd>) -> PaneContinuation  {
        // TODO: Decide order of char, enemy. Before or after not quite right. Or need
        // to handle char moving onto enemy.
        // TODO: Consider: Maybe display char moving out of sync with enemy.
        let hero = Roster::hero();

        let cmd = cmd.unwrap_or(Cmd::default());

        // Before movement, reset "prev". Will be overwritten if movement happens.
        // Should be moved into obj_move*() fn.
        self[hero].refs.prev_pos = self[hero].refs.pos;

        MovementLogic::move_mov(self, hero, cmd)?;

        for mov in self.roster.all_movs() {
            // Before movement, reset "prev". Will be overwritten if movement happens.
            // Going through tmp is necessary to avoid two dynamic borrows at the same time..
            // NOTE: If map is RefCell needs to be done in two steps else runtime panic.
            // NOTE: And obj_at() is also incompatible with RefCell.
            self[mov].refs.prev_pos = self[mov].refs.pos;

            MovementLogic::move_mov(self, mov, cmd)?;
        }
        PaneContinuation::Continue(())
    }

    fn tick_based(&self) -> bool {
        true
    }
}

impl<MovementLogic: BaseMovementLogic> Arena<MovementLogic> {
    // TODO: Remove again, redundant.
    pub fn from_ascii<const HEIGHT: usize>(
        ascii_map: &[&str; HEIGHT],
        map_key: HashMap<char, Vec<FreeObj<MovementLogic::CustomProps>>>,
    ) -> Self {
        Self::from_map_and_key(ascii_map, map_key)
    }

    /////////////////
    /// Initialisers
    pub fn empty(w: u16, h: u16) -> Self {
        Self {
            map: Into::into(Grid::new(w, h)),
            roster: Roster::new(),
            map_key: std::collections::HashMap::new(),
        }
    }

    pub fn from_map_and_key<const HEIGHT: usize>(
        ascii_map: &[&str; HEIGHT],
        map_key: HashMap<char, Vec<FreeObj<MovementLogic::CustomProps>>>,
    ) -> Self {
        let mut map = Self {
            map_key: map_key.clone(),
            ..Self::empty(ascii_map[0].len() as u16, HEIGHT as u16)
        };

        for (y, line) in ascii_map.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                for ent in map_key.get(&ch).unwrap() {
                    map.spawn_obj_at(x as i16, y as i16, ent.clone());
                }
            }
        }

        map
    }

    //////////////////////////////////////////////
    /// Exposed upward to front end of game engine

    pub fn map_w(&self) -> u16 {
        self.map.w()
    }

    pub fn map_h(&self) -> u16 {
        self.map.h()
    }

    // TODO: Any better way to expose this for iterating?
    pub fn map_locs(&self) -> LocIterator<MovementLogic> {
        self.map.locs()
    }

    //////////////////////////////////////////////////////////////////////////////////
    /// Obj spawn and move fns.
    ///
    /// Objects are only spawned or moved in map by place_obj_at and move_obj_to. Those
    /// functions update coords in roster, roster_idx, prev_pos, curr_pos to maintain
    /// consistency.
    ///
    /// Objects can be changed but not moved by map[] references.
    /// TODO: Actually, add some interface there to avoid &mut Backref

    /// Spawn new object.
    pub fn spawn_obj_at(&mut self, x: i16, y:i16, template_obj: FreeObj<MovementLogic::CustomProps>)
    {
        let pos = MapCoord::from_xy(x, y);
        let h = self.map[pos].objs.len() as u16;
        let new_roster_idx = self.roster.add_to_roster_if_mov( MapRef{x, y, h}, &template_obj );
        let mappos = Refs {
            curr_roster_idx: new_roster_idx,
            pos,
            prev_pos: pos,
        };
        let obj = MapObj::<MovementLogic::CustomProps>{
            refs: mappos,
            logical_props: template_obj.logical_props,
            visual_props: template_obj.visual_props,
        };
        self.map[pos].objs.push(obj);
    }

    /// Move obj to a new location.
    ///
    /// Update roster and backpos.curr_pos and backpos.prev_pos. Still untested for multiple movs.
    pub fn move_obj_to(&mut self, roster_idx: RosterIndex, target_pos: MapCoord) {
        let orig_pos = self.roster[roster_idx].pos();
        let orig_h = self.roster[roster_idx].h;

        // Remove object from previous map location.
        let obj = self.map[orig_pos].objs.remove(orig_h as usize);

        // For each other object in location, update its mapref in roster with changed height.
        for h in orig_h+1..self.map[orig_pos].len() as u16 {
            let other_roster_idx = self.map[orig_pos][h].refs.curr_roster_idx;
            self.roster[other_roster_idx].h = h;
        }

        // TODO: Put in assert that put_obj_in_map_and_return_updated_mapref updates prev_pos as expected.

        // Add object to top of stack at new map location.
        self.map[target_pos].objs.push(
            MapObj {
                refs: Refs {
                    curr_roster_idx: obj.refs.curr_roster_idx,
                    pos: target_pos,
                    prev_pos: obj.refs.pos,
                },
                ..obj
            }
        );

        // Update roster hdl to match new position and height.
        self.roster[roster_idx].x = target_pos.x;
        self.roster[roster_idx].y = target_pos.y;
        self.roster[roster_idx].h = self.map[target_pos].len() as u16 -1;
    }

    ///////////////////////////////////////////////////
    /// External helper functions for accessing objects.
    ///
    /// TODO: Would move into fns of Obj if we do that.

    pub fn hero(&self) -> RosterIndex {
        Roster::hero()
    }

    /// Where object would move to based on current direction.
    /// Candidate for AttemptAction like struct.
    /// TODO: Only valid if "dir" represents actual direction of movement, not just facing.
    pub fn obj_target_pos(&self, roster_idx: RosterIndex) -> MapCoord {
        self[roster_idx].refs.pos + self[roster_idx].logical_props.dir
    }

    // NB: Should remove engine dependency on simple_custom_props
    pub fn any_has_effect(&self, pos: MapCoord, sought_effect: simple_custom_props::Effect) -> bool {
        self.map[pos].any_effect(sought_effect)
    }

    pub fn passable(&self, pos: MapCoord) -> bool {
        self.map[pos].passable()
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

    /// Ascii representation of map. Checked by test functions. Used for debug logging.
    /// NB: Store an appropriate char for representing object in the object, or something,
    /// instead of comparing to original and misusing PartialEq to ignore some fields.
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

impl<MovementLogic: BaseMovementLogic> Index<RosterIndex> for Arena<MovementLogic> {
    type Output = MapObj<MovementLogic::CustomProps>;

    fn index(&self, roster_idx: RosterIndex) -> &Self::Output {
        let mapref = self.roster[roster_idx];
        &self.map.locs[mapref.x as usize][mapref.y as usize][mapref.h]
    }
}

impl<MovementLogic: BaseMovementLogic> IndexMut<RosterIndex> for Arena<MovementLogic> {
    fn index_mut(&mut self, roster_idx: RosterIndex) -> &mut Self::Output {
        let mapref = self.roster[roster_idx];
        &mut self.map.locs[mapref.x as usize][mapref.y as usize][mapref.h]
    }
}

/// Ref back from object to current coords in map
#[derive(Clone, Debug)]
pub struct Refs {
    curr_roster_idx: RosterIndex,
    pub pos: MapCoord,
    pub prev_pos: MapCoord,
}

/// "Grid": Grid of locations. Represents state of current level.
/// NOTE: Could currently be moved back into Arena. Not borrowed separately.
#[derive(Clone)]
struct Grid<MovementLogic: BaseMovementLogic> {
    // Stored as a collection of columns, e.g. map.locs[x][y]
    // Must always be rectangular.
    locs: Vec<Vec<Loc<MovementLogic::CustomProps>>>,
}

impl<MovementLogic: BaseMovementLogic> Grid<MovementLogic> {
    pub fn new(w: u16, h: u16) -> Self {
        Self {
            locs: vec!(vec!(Loc::new(); h.into()); w.into()),
        }
    }

    pub fn w(&self) -> u16 {
        self.locs.len() as u16
    }

    pub fn h(&self) -> u16 {
        self.locs[0].len() as u16
    }

    pub fn locs(&self) -> LocIterator<MovementLogic> {
        LocIterator {
            w: self.w(),
            h: self.h(),
            x: 0,
            y: -1,
            map: &self,
        }
    }
}

impl<MovementLogic: BaseMovementLogic> Index<MapCoord> for Grid<MovementLogic> {
    type Output = Loc<MovementLogic::CustomProps>;

    fn index(&self, pos: MapCoord) -> &Self::Output {
        &self.locs[pos.x as usize][pos.y as usize]
    }
}

impl<MovementLogic: BaseMovementLogic> IndexMut<MapCoord> for Grid<MovementLogic> {
    fn index_mut(&mut self, pos: MapCoord) -> &mut Self::Output {
        &mut self.locs[pos.x as usize][pos.y as usize]
    }
}

impl<MovementLogic: BaseMovementLogic> std::fmt::Debug for Grid<MovementLogic> {
    #[try_fn]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Arena[")?;
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

pub struct LocIterator<'a, MovementLogic: BaseMovementLogic> {
    // Original dimensions to iterate up to
    w: u16,
    h: u16,
    // Previously returned coords, or (0, -1) initially.
    x: i16,
    y: i16,
    // Pointer back to original collection
    map: &'a Grid<MovementLogic>,
}

impl Iterator for CoordIterator {
    // TODO: Could convert to usize on input. Would need another way of representing "Before map" coord.
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

impl<'a, MovementLogic: BaseMovementLogic> Iterator for LocIterator<'a, MovementLogic> {
    type Item = (i16, i16, &'a Loc<MovementLogic::CustomProps>);

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
// NOTE: Could currently be moved back into Arena. Not borrowed separately.
#[derive(Clone, Debug)]
struct Roster {
    pub hero: MapRef,

    movs: Vec<MapRef>,
}

impl Roster {
    pub fn new() -> Self {
        Self {
            hero: MapRef{x:0, y:0, h:1}, // Overwritten immediate, but can we avoid placeholder?
            movs: vec![],
        }
    }

    pub fn hero() -> RosterIndex {
        RosterIndex { ros_idx: 100 }
    }

    pub fn non_mov_handle() -> RosterIndex {
        RosterIndex { ros_idx: 98 }
    }

    pub fn all_movs(&self) -> Vec<RosterIndex> {
        // TODO: Possible to return iter() instead of collection, without borrow problems?
        (0..self.movs.len() as u16).into_iter().map(|ros_idx| RosterIndex { ros_idx } ).collect()
    }

    fn add_to_roster_if_mov<CustomProps: super::for_gamedata::BaseCustomProps>(&mut self, mapref: MapRef, props: &FreeObj<CustomProps>) -> RosterIndex {
        if props.logical_props.custom_props.is_hero() {
            self.hero = mapref;
            Self::hero()
        } else if props.logical_props.custom_props.is_any_mov() {
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
#[derive(Debug, Clone)]
pub struct Loc<CustomProps: super::for_gamedata::BaseCustomProps> {
    objs: Vec<MapObj<CustomProps>>
}

/// One square in map. Defined by the stack of objects in that square.
impl<CustomProps: super::for_gamedata::BaseCustomProps> Loc<CustomProps> {
    pub fn new() -> Self {
        Loc { objs: vec![] }
    }

    pub fn any_effect(&self, sought_effect: simple_custom_props::Effect) -> bool {
        self.objs.iter().any(|x| x.logical_props.effect == sought_effect)
    }

    pub fn passable(&self) -> bool {
        self.objs.iter().all(|x| x.logical_props.pass == crate::simple_custom_props::Pass::Empty)
    }

    fn map_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for ent in self {
            write!(f, "{},", ent.logical_props.name)?;
        }
        write!(f, ";")
    }

    /// Only used by render() when unsure about height?
    pub fn get(&self, idx: usize) -> Option<&MapObj<CustomProps>> {
        self.objs.get(idx)
    }

    pub fn len(&self) -> usize {
        self.objs.len()
    }

    pub fn obj_props(&self) -> Vec<FreeObj<CustomProps>> {
        // TODO: Avoid clone
        self.objs.iter().map(|obj|
            FreeObj{logical_props:obj.logical_props.clone(), visual_props:obj.visual_props.clone()}
        ).collect()
    }
}

impl<CustomProps: super::for_gamedata::BaseCustomProps> IntoIterator for Loc<CustomProps>  {
    type Item = <Vec<MapObj<CustomProps>> as IntoIterator>::Item;
    type IntoIter = <Vec<MapObj<CustomProps>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.objs.into_iter()
    }
}

impl<'a, CustomProps: super::for_gamedata::BaseCustomProps> IntoIterator for &'a Loc<CustomProps> {
    type Item = <&'a Vec<MapObj<CustomProps>> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<MapObj<CustomProps>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.objs.iter()
    }
}

impl<CustomProps: super::for_gamedata::BaseCustomProps> Index<u16> for Loc<CustomProps> {
    type Output = MapObj<CustomProps>;

    fn index(&self, h: u16) -> &Self::Output {
        &self.objs[h as usize]
    }
}

impl<CustomProps: super::for_gamedata::BaseCustomProps> IndexMut<u16> for Loc<CustomProps> {
    fn index_mut(&mut self, h: u16) -> &mut Self::Output {
        &mut self.objs[h as usize]
    }
}

/// Specific object in map (Including current coords as well as LogicalProps::<simple_custom_props::DefaultCustomProps>, VisualProps)
#[derive(Clone, Debug)]
pub struct MapObj<CustomProps: super::for_gamedata::BaseCustomProps> {
    refs: Refs,
    pub logical_props: LogicalProps::<CustomProps>,
    pub visual_props: VisualProps,
}

impl<CustomProps: super::for_gamedata::BaseCustomProps> MapObj<CustomProps> {
    pub fn pos(&self) -> MapCoord {
        self.refs.pos
    }

    pub fn prev_pos(&self) -> MapCoord {
        self.refs.prev_pos
    }
}
