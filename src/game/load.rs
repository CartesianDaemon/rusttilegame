// Code for loading or instatiating each level.

use crate::game::Ent;
use crate::game::Play;

pub fn new_default_level() -> Play {
    // Some of this may move to Map, or to a new intermediate struct.

    let mut play = Play::new_empty_level();

    // Initialise Floor
    {
        // TODO: Can use "x, y, ents" if I implement Loc::put()?
        // Or will that fall afoul of borrow checker?
        for (x, y) in play.map.coords() {
            play.map.set_at(x as i16, y as i16, Ent::new_floor());
            if play.map.is_edge(x, y) {
                play.map.set_at(x, y, Ent::new_wall());
            }
        }
    }

    // Initialise hero
    play.spawn_hero(3, 8, Ent::new_hero_crab());

    // Initialise snake
    play.spawn_mov(1, 1, Ent::new_snake((1,0)));
    play.spawn_mov(9, 9, Ent::new_snake((-1,0)));

    play
}
