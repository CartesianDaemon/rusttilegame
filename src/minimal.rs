// Minimalist example of map types to try out different ways of handling ownership.
//
// Cases to deal with:
//
// Iterating through map (e.g. render)
// Iterating through bots in roster
//  And updating map in the way...

// ENH: Macro for s!{Hello} rather than "Hello".as_string()

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
