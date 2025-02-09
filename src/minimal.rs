// Minimalist example of map types to try out different ways of handling ownership.
//
// Things to think about:
//
// Convenience of setting loc blah to thing foo.
// How to have a "handle" to a location? To an object? From within the map?
//  2dPos
//  3dpos? Probably not??
//  handle to object? When we need that?
//  how to have map "available" in all functions?? How that works with iteration etc?
// More specifically, what specific accesses did I end up using in the game?
//  Iterate through map
//  And move something??
//  Maybe: object with a "pointer" to another object?
//  Maybe: "roster" of movables?
// How would I do this in python or C? What's the rust equivalent?
//  Just always have a pointer to the map and deal with it directly?
//   What cases do I need to think about?
//   How to actually do that? An almost-global runtime-lock access? The lock would be fine, are pointers to it inconvenient?
//  ..
// And Q
//  Avoiding boilerplate. Easy to have a "fat" pointer to map and position? To have more than one?? Or to always have map ptr param?
//  Just global?
//
// Cases to deal with:
//
// Iterating through map (e.g. render)
// Iterating through bots in roster
//  And updating map in the way...

#![allow(dead_code)]

/* ----------- MAP ----------- */

pub struct Map {
    locs: Vec<Obj>,
}

/* ----------- OBJECT ----------- */

#[derive(Clone)]
struct Obj {
    name: String,
    automove: bool,
}

fn obj_floor() -> Obj {
    Obj {name: "Floor".to_string(), automove: false}
}

fn obj_dancer() -> Obj {
    Obj {name: "Dancer".to_string(), automove: true}
}

pub fn mymap() -> Map {
    Map {locs: vec![obj_dancer(), obj_floor(), obj_floor(), obj_floor()]}
}

/* ----------- GENERAL FUNCTIONS ----------- */

pub fn iterate(map: &mut Map) {
    for idx in 0..map.locs.len() {
        let loc = &map.locs[idx];
        if loc.automove && idx +1 < map.locs.len() {
            // TODO: Use swap, or map.move() fn instead of clone.
            map.locs[idx+1] = map.locs[idx].clone();
            map.locs[idx] = obj_floor();
        }
    }
}
