// Map, location, and entity types.
//
// But movement logic etc are in Play.
// Most of this can be used by level data files, even though
// they usually don't need indexing, only creation.

use macroquad::prelude::*;

use std::collections::LinkedList;
use std::mem;
use futures::executor::block_on;
use std::ops::Index;
use std::ops::IndexMut;

// TODO: Put these into a "Coords" module.
use super::Pos;
use super::Point;
use super::Delta;

// Utils
fn load_texture_blocking_unwrap(path: &str) -> Texture2D {
    block_on(load_texture(path)).unwrap()
}

// "Map": Grid of locations. Most of the current state of game.
pub struct Map {
    // Stored as a collection of columns, e.g. map.locs[x][y]
    // Must always be rectangular.
    locs: Vec<Vec<Loc>>,
}

impl Index<Pos> for Map {
    type Output = Ent;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.locs[pos.0 as usize][pos.1 as usize].ents[pos.2 as usize]
    }
}

impl IndexMut<Pos> for Map {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.locs[pos.0 as usize][pos.1 as usize].ents[pos.2 as usize]
    }
}

impl Map {
    /*
    fn new(sz: u16) -> Map {
        panic!("New default Map unimplemented.");
    }*/

    fn new(sz: u16) -> Map {
        // Some of this may move back up to Play, or from there to here.
        Map {
            locs: vec!(vec!(Loc::new(); sz.into()); sz.into()),
        }
    }

    fn w(&self) -> u16 {
        // TODO: Could return usize, most callers need to cast anyway.
        self.locs.len() as u16
    }

    fn h(&self) -> u16 {
        self.locs[0].len() as u16
    }

    fn is_edge(&self, x: i16, y: i16) -> bool {
        x == 0 || x == self.w() as i16 -1 || y == 0 || y == self.h() as i16 -1
    }

    // All map-altering fns go through a fn like this to keep Map/Ros coords in sync.
    // Nothing happens if target is off map. Higher layer should prevent that.
    fn move_to(&mut self, pos: &mut Pos, to: Point) {
        let ent = if pos.2 as usize == self.at(*pos).len() {
            self.atm(*pos).pop().unwrap()
        } else {
            mem::replace(&mut self[*pos], Ent::placeholder())
        };

        // TODO: Could be moved into "if pop" branch above
        while !self.at(*pos).is_empty() &&
            self.at(*pos).last().unwrap().is_placeholder() {
            self.atm(*pos).pop();
        }

        *pos = (to.0, to.1, 0);

        self.put_at(pos, ent);
    }

    fn can_move(&self, pos: &Pos, delta: Delta) -> bool {
        self.loc_at( (pos.0 + delta.0, pos.1 + delta.1, 0) ).passable()
    }

    // Nothing happens if target is off map. Higher layer should prevent that.
    fn move_delta(&mut self, pos: &mut Pos, delta: Delta) {
        self.move_to(pos, (pos.0 + delta.0, pos.1 + delta.1));
    }

    fn loc_at(&self, pos: Pos) -> &Loc {
        &self.locs[pos.0 as usize][pos.1 as usize]
    }

    // Access loc.ents stacked at given coords (not using height field in Pos)
    // Used to add and remove from map, mostly internally
    fn at(&self, pos: Pos) -> &Vec<Ent> {
        &self.loc_at(pos).ents
    }

    // As "at" but mutably
    fn atm(&mut self, pos: Pos) -> &mut Vec<Ent> {
        &mut self.locs[pos.0 as usize][pos.1 as usize].ents
    }

    // Add an ent at x,y, not tied to any roster.
    fn set_at(&mut self, x: i16, y: i16, val: Ent) {
        let mut ent = val;
        ent.x = x;
        ent.y = y;
        ent.h = self.at((x, y, 0)).len() as u16;

        self.atm( (x, y, 0) ).push(ent);
    }

    // Add an ent at pos.x, pos.y and update pos.z to match.
    fn put_at(&mut self, pos: &mut Pos, val: Ent) {
        let mut ent = val;
        ent.x = pos.0;
        ent.y = pos.1;
        ent.h = self.at(*pos).len() as u16;
        pos.2 = ent.h;

        self.atm(*pos).push(ent);
    }

    // e.g. `for ( x, y ) in map.coords()`
    fn coords(&self) -> CoordIterator {
        CoordIterator {
            w: self.w(),
            h: self.h(),
            x: 0,
            y: -1,
        }
    }

    fn locs(&self) -> LocIterator {
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

struct CoordIterator {
    // Original dimensions to iterate up to
    w: u16,
    h: u16,
    // Previously returned coords, or (0, -1) initially.
    x: i16,
    y: i16,
}

struct LocIterator<'a> {
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
pub struct Ros {
    // Hero
    hero: Handle, // TODO: Better name for protagonist than "hero".

    // Anything which updates each tick, especially enemies.
    //
    // Might be replaced by a set of lists of "everything that has this property" etc
    // like a Component system.
    movs: Vec<Handle>,
}

impl Ros {
    fn new(sz: u16) -> Ros {
        assert_eq!(sz, 16);
        Ros {
            hero: (0, 0, 1),
            movs: vec![],
        }
    }

    fn push_mov(&mut self, hdl: Handle) {
        self.movs.push(hdl);
    }
}


type Handle = Pos;

// "Location": Everything at a single coordinate in the current room.
// #[derive(Clone)]
pub struct Loc {
    ents: Vec<Ent>,
}

impl Loc {
    fn new() -> Loc {
        Loc { ents: vec![] }
    }

    fn passable(&self) -> bool {
        !self.impassable()
    }

    fn impassable(&self) -> bool {
        self.ents.iter().any(|x| x.pass == Pass::Solid)
    }
}

impl Clone for Loc {
    fn clone(&self) -> Loc {
        assert!(self.ents.is_empty());
        Loc::new()
    }

    // Consider implementing index [idx] for Loc returning loc.ents[idx]
}

// "Entity": Anything tile-sized and drawable including floor, wall, object, being.
#[derive(Clone)]
#[allow(dead_code)]
pub struct Ent {
    // Cache of coords ent is at on map. These might be useful for movement logic, but probably
    // aren't required.
    x: i16,
    y: i16,
    h: u16,

    // Visual display.
    border: Option<Color>,
    fill: Option<Color>,
    tex: Option<Texture2D>,

    // Solidity, e.g. wall, floor
    pass: Pass,

    // Movement control logic for enemies
    ai: AI,

    // Internal status for specific ent types.
    dir: Delta,
}

impl Ent {
    // An unitialised ent
    fn invalid() -> Ent {
        Ent {
            x: -1, // For now "-1" flags "this element is a placeholder in height vector"
            y: -1,
            h: 0,

            border: None,
            fill: None,
            tex: None,

            pass: Pass::Empty,

            ai: AI::Stay, // Could use this as a better placeholder flag

            dir: (0, 0),
        }
    }

    // An ent which is ignored when it exists in the map.
    fn placeholder() -> Ent {
        Ent::invalid()
    }

    // Default values for fields not used in a particular ent type.
    #[allow(dead_code)]
    fn empty() -> Ent {
        Ent {
            ..Ent::invalid()
        }
    }

    fn is_placeholder(&self) -> bool {
        self.x == -1
    }

    #[allow(dead_code)]
    fn new_tex(tex: Texture2D) -> Ent {
        Ent {
            h: 1, // TODO
            tex: Some(tex),
            ..Ent::invalid()
        }
    }

    fn new_tex_col(tex: Texture2D, fill: Color) -> Ent {
        // TODO: Shouldn't need coords as put_at should take care of that.
        Ent {
            tex: Some(tex),
            fill: Some(fill),
            ..Ent::invalid()
        }
    }

    fn new_col(fill: Color) -> Ent {
        Ent {
            fill: Some(fill),
            ..Ent::invalid()
        }
    }

    fn new_col_outline(fill: Color, outline: Color) -> Ent {
        Ent {
            fill: Some(fill),
            border: Some(outline),
            ..Ent::invalid()
        }
    }

    // Specific ent types
    fn new_hero_crab() -> Ent {
        Ent {
            pass: Pass::Mov,
            ai: AI::Hero,
            ..Ent::new_tex_col(load_texture_blocking_unwrap("imgs/ferris.png"), GOLD)
        }
    }
    fn new_snake(dir: Delta) -> Ent {
        Ent {
            pass: Pass::Mov,
            ai: AI::Snake,
            dir: dir,
            ..Ent::new_col(DARKGREEN)
        }
    }

    fn new_floor() -> Ent {
        Ent {
            ..Ent::new_col_outline(WHITE, LIGHTGRAY)
        }
    }

    fn new_wall() -> Ent {
        Ent {
            pass: Pass::Solid,
            ..Ent::new_col(DARKGRAY)
        }
    }
}

// Passable. Whether other movs can move through an ent or not.
#[derive(Clone, PartialEq)]
enum Pass {
    Empty, // No impediment to movement, e.g. floor.
    Solid, // Block movement, e.g. wall.
    Mov, // Something which can move itself, e.g. hero, enemy
    // INSERT: Obj, // Something which can be moved or maybe coexisted with, e.g. furniture
}

// Types of movement-control logic ents can use
#[derive(Clone)]
enum AI {
    Stay, // No self movement.
    Hero, // Controlled by keys.
    Snake, // Move in direction, move orthogonally towards hero. Maybe: bounce off walls.
}


