// Minimalist example of map types to try out different ways of handling ownership.

#![allow(dead_code)]

// ----------- MAP -----------

pub struct Map {
    loc_objs: Vec<Vec<Obj>>, // Horizontal array of piles of objects at each location
}

impl Map
{
    pub fn default() -> Map {
        Map {loc_objs: vec![loc_dancer_on_floor(), loc_empty_floor(), loc_empty_floor(), loc_empty_floor()]}
    }

    pub fn move_to(&mut self, handle: (usize, usize), new_pos: usize) {
        let obj = self.loc_objs[handle.0].remove(handle.1);
        self.loc_objs[new_pos+1].push(obj);
    }

    pub fn loc_coords(&self) -> impl Iterator<Item=usize> {
        0..self.loc_objs.len()
    }

    pub fn movable_objs(&self) -> impl Iterator<Item=usize> + use<'_> {
        let loc_coords = self.loc_coords();
        std::iter::from_coroutine(#[coroutine] || {
            for idx in loc_coords {
                yield idx
            }
        })
    }

    pub fn iterate(&mut self) {
        for idx in self.loc_coords() {
            let loc = &self.loc_objs[idx];
            let obj = match loc.get(1) {
                None => continue,
                Some(obj) => obj
            };
            if obj.auto_move_right && idx +1 < self.loc_objs.len() {
                self.move_to( (idx,1), idx+1 );
            }
        }
    }
}

// ----------- OBJECT -----------

#[derive(Clone)]
struct Obj {
    name: String,
    auto_move_right: bool,
}

fn obj_floor() -> Obj {
    Obj {name: "Floor".to_string(), auto_move_right: false}
}

fn obj_dancer() -> Obj {
    Obj {name: "Dancer".to_string(), auto_move_right: true}
}

fn loc_empty_floor() -> Vec<Obj> {
    vec![obj_floor()]
}

fn loc_dancer_on_floor() -> Vec<Obj> {
    vec![obj_floor(), obj_dancer()]
}
