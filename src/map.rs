// Map, location, and entity types.
//
// But movement logic etc are in Play.
// These are also used by level data files, even though
// they don't need any of the indexing.

use std::mem;
use std::ops::Index;
use std::ops::IndexMut;

use crate::*;

use types::MapHandle;
use types::MapCoord;
use types::CoordDelta;

use ent::Ent;

// "Map": Grid of locations. Most of the current state of game.
#[derive(Clone)]
pub struct Map {
    // Stored as a collection of columns, e.g. map.locs[x][y]
    // Must always be rectangular.
    locs: Vec<Vec<Loc>>,
}

impl Index<MapHandle> for Map {
    type Output = Ent;

    fn index(&self, pos: MapHandle) -> &Self::Output {
        &self.locs[pos.0 as usize][pos.1 as usize].ents[pos.2 as usize]
    }
}

impl IndexMut<MapHandle> for Map {
    fn index_mut(&mut self, pos: MapHandle) -> &mut Self::Output {
        &mut self.locs[pos.0 as usize][pos.1 as usize].ents[pos.2 as usize]
    }
}

// ENH: Fehler
impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Map[")?;
        for (x, y, loc) in self.locs() {
            loc.map_fmt(f)?;
            if x ==0 && y > 0 {
                write!(f, "|")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl Map {
    /*
    pub fn new(sz: u16) -> Map {
        panic!("New default Map unimplemented.");
    }*/

    pub fn new(sz: u16) -> Map {
        // Some of this may move back up to Play, or from there to here.
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

    // All map-altering fns go through a fn like this to keep Map/Ros coords in sync.
    // Nothing happens if target is off map, that's a gameplay error but not an
    // engine error.
    pub fn move_to(&mut self, hdl: &mut Handle, to: MapCoord) {
        let on_top = hdl.2 as usize == self.at(*hdl).len();

        let ent = if on_top {
            // Pop ent from top of stack.
            self.atm(*hdl).pop().unwrap()
        } else {
            // Replace ent with a placeholder type ignored by render and gameplay.
            // This keeps height coords of other ents valid.
            mem::replace(&mut self[*hdl], Ent::placeholder())
        };

        // Remove any placeholders now at the top of the stack. Should only happen
        // if we popped ent from on top of them.
        while !self.at(*hdl).is_empty() &&
            self.at(*hdl).last().unwrap().is_placeholder() {
            self.atm(*hdl).pop();
        }

        // Update caller's handle to new coords. Height will be set by put_at().
        *hdl = to.to_pos();

        // Add Ent to top of stack at new map coords. Updates hdl to match new height.
        self.put_at(hdl, ent);
    }

    pub fn can_move(&self, pos: &MapHandle, delta: CoordDelta) -> bool {
        self.loc_at( (pos.0 + delta.0, pos.1 + delta.1, 0) ).passable()
    }

    // Nothing happens if target is off map. Higher layer should prevent that.
    pub fn move_delta(&mut self, pos: &mut MapHandle, delta: CoordDelta) {
        self.move_to(pos, MapCoord::from_pos(*pos) + delta);
    }

    pub fn loc_at(&self, pos: MapHandle) -> &Loc {
        &self.locs[pos.0 as usize][pos.1 as usize]
    }

    // Access loc.ents stacked at given coords (not using height field in Pos)
    // Used to add and remove from map, mostly internally
    pub fn at(&self, pos: MapHandle) -> &Vec<Ent> {
        &self.loc_at(pos).ents
    }

    // As "at" but mutably
    pub fn atm(&mut self, pos: MapHandle) -> &mut Vec<Ent> {
        &mut self.locs[pos.0 as usize][pos.1 as usize].ents
    }

    // Add an ent at x,y, not tied to any roster.
    // FIXME: Maybe replace with place_at
    /*
    pub fn set_at(&mut self, x: i16, y: i16, val: Ent) {
        self.place_at(x, y, None, val);
    }
    */

    // Add an ent at pos.x, pos.y and update pos.z to match.
    // FIXME: Maybe replace with place_at
    pub fn put_at(&mut self, pos: &mut MapHandle, val: Ent) {
        self.place_at(pos.0, pos.1, Some(pos), val);
    }

    // Add an ent at x,y. Set out_pos to coords if present.
    // TODO: Could take vec as parameter instead?
    // TODO: Caller could specify _ instead of maybe?
    pub fn place_at(&mut self, x: i16, y:i16, out_pos: Option<&mut MapHandle>, val: Ent) {
        let mut ent = val;
        ent.x = x;
        ent.y = y;
        ent.h = self.at((x,y,0)).len() as u16;

        if let Some(pos) = out_pos {
            *pos = (ent.x, ent.y, ent.h);
        }

        self.atm( (x, y, 0) ).push(ent);
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

// Roster of character, enemies, etc. Indexes into map.
#[derive(Clone, Debug)]
pub struct Ros {
    // Hero
    // FIXME: Better name for protagonist than "hero".
    pub hero: Handle,

    // Anything which updates each tick, especially enemies.
    //
    // Might be replaced by a set of lists of "everything that has this property" etc
    // like a Component system.
    pub movs: Vec<Handle>,
}

impl Ros {
    pub fn new() -> Ros {
        Ros {
            hero: (0, 0, 1),
            movs: vec![],
        }
    }

    pub fn push_mov(&mut self, hdl: Handle) {
        self.movs.push(hdl);
    }
}


type Handle = MapHandle;

// "Location": Everything at a single coordinate in the current room.
// #[derive(Clone)] // implemented below
#[derive(Debug, Clone)]
pub struct Loc {
    pub ents: Vec<Ent>,
}

impl Loc {
    pub fn new() -> Loc {
        Loc { ents: vec![] }
    }

    pub fn passable(&self) -> bool {
        !self.impassable()
    }

    pub fn impassable(&self) -> bool {
        // Can this fn work without knowledge of specific properties?
        use ent::Pass;
        self.ents.iter().any(|x| x.pass == Pass::Solid)
    }

    fn map_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for ent in &self.ents {
            write!(f, "{},", ent.name)?;
        }
        write!(f, ";")
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
