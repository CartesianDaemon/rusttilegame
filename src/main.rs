use macroquad::prelude::*;

#[allow(unused_imports)]
use std::collections::LinkedList;
use std::mem;
use std::ops::Index;
use std::ops::IndexMut;
use futures::executor::block_on;

// Might like types:
//
// Dimension: Width/height of map. Unsigned. Vars w,h.
// MapCoord: Coords on map. Signed to allow looping past edge.
//           May need cast to index vector? Vars x,y.
// ViewCoord: As MapCoord but relative to visible part of map (currently all).
//            Vars vx, vy.
// Delta: Offset of map coord. Signed. Vars dx, dy.
// PixCoord: Coords on screen. f32. Vars px, py.
// Pos: Coords including height.
//
// Ideally allowing arithmetic between dimension, map, delta with least casting.
// And multiplication of p coords by map coords.

type Pos = (i16, i16, u16);
type Point = (i16, i16);
type Delta = (i16, i16);

// Utils
fn load_texture_blocking_unwrap(path: &str) -> Texture2D {
    block_on(load_texture(path)).unwrap()
}

// Overall game state.
struct Game {
    p: Play,
    i: Input,
}

impl Game {
    fn new_default() -> Game {
        Game {
            p: Play::new_default_level(),
            i: Input::new_default(),
        }
    }

    fn do_frame(&mut self) {
        // Can I make a single function for this and ready_for_tick()?
        self.i.read_input();

        // Would it be easier to read with a layout like:
        //
        // while (not ready for tick) {
        //     accumulate_input();
        //     draw_frame();
        // }
        //
        // advance();
        //
        // draw_frame();
        //
        // ?
        //
        // But probably needs yield which we don't actually have?

        // Wait for tick if needed.
        // Need to know at this level to treat input differently on a tick
        // But maybe ready_for_tick can take a "tick wanted" parameter from Play mode.
        if self.p.continuous() || self.i.ready_for_tick() {
            // Advance game state according to current mode (level, menu, game over, etc)
            if !self.p.game_over {
                self.p.advance_level(self.i.consume_keypresses());
            } else {
                self.p.advance_game_over(self.i.consume_keypresses());

                // Reset "most recent tick" when leaving menu.
                // Need to move into input code. Maybe "when starting level"?
                // As part of some standard mode transition code?
                self.i.last_update = get_time();
            }
        }

        self.draw_frame();
    }

    fn draw_frame(&self) {
        if !self.p.game_over {
        let r = RenderLevel::begin(self.p.map.w(), self.p.map.h());
            // Coords of first visible tile. Currently always 0,0.
            let (ox, oy) = (0, 0);
            for (x, y, loc) in self.p.map.locs() {
                for ent in &loc.ents {
                    r.draw_ent(x - ox, y - oy, ent);
                }
            }
        } else {
            let _r = RenderGameOver::begin();
        }
    }
}

// Gameplay state: current level, map, etc.
// May split out values relevant to current mode (level, menu, etc).
struct Play {
    game_over: bool,

    // Layout of current level.
    map: Map,
    ros: Ros,
}

impl Play {
    fn new_empty_level() -> Play {
        Play {
            game_over: false,
            map: Map::new(16),
            ros: Ros::new(16), // These two should be generated together
        }
    }

    fn new_default_level() -> Play {
        // Some of this may move to Map, or to a new intermediate struct.

        let mut play = Self::new_empty_level();

        // Initialise Floor
        {
            // TODO: Can use "x, y, ents" if I implement Loc::put()?
            // Or will that fall afoul of borrow checker?
            for (x, y) in play.map.coords() {
                play.map.set_at(x as i16, y as i16, Ent::new_floor(x, y));
            }
        }

        // Initialise hero
        play.spawn_hero(3, 8, Ent::new_hero_crab(3, 8));

        // Initialise snake
        play.spawn_mov(1, 1, Ent::new_snake(1, 1, (1,0)));

        play
    }

    fn spawn_hero(&mut self, x: i16, y: i16, ent: Ent) {
        self.ros.hero = (x, y, 0);
        self.map.put_at(&mut self.ros.hero, ent);
    }

    fn spawn_mov(&mut self, x: i16, y: i16, ent: Ent) {
        let mut new_pos = (x, y, 0);
        self.map.put_at(&mut new_pos, ent);
        self.ros.push_mov(new_pos);
    }

    // Does current mode need UI to wait for tick before updating state?
    // Currently yes in level, no in game over.
    fn continuous(&self) -> bool {
        self.game_over
    }

    fn advance_level(&mut self, last_key_pressed: Option<KeyCode>) {
        // Move movs

        // TODO: Change `snake` to `mov`
        for snake in &mut self.ros.movs {
            match self.map[*snake].ai {
                AI::Stay => {
                    // Do nothing
                },
                AI::Hero => {
                    // TODO
                },
                AI::Snake => {
                    // if snake on same row xor column as hero, change dir to face hero
                    if (snake.0 == self.ros.hero.0) != (snake.1 == self.ros.hero.1) {
                        let new_dir: Delta = ((self.ros.hero.0 - snake.0).signum(),(self.ros.hero.1 - snake.1).signum());
                        self.map[*snake].dir = new_dir;
                    }

                    // die if snake would go out of bounds
                    // TODO: Instead: Game over if snake eats char; Respawn snake if dies.
                    if !(0..self.map.w() as i16).contains(&(snake.0 + self.map[*snake].dir.0)) ||
                        !(0..self.map.h() as i16).contains(&(snake.1 + self.map[*snake].dir.1))
                    {
                        self.game_over = true;
                    }
                    else
                    {
                        // move snake to new location
                        let dir = self.map[*snake].dir;
                        self.map.move_delta(snake, dir);
                    }

                    // eat hero?
                    if snake.0 == self.ros.hero.0 && snake.1 == self.ros.hero.1 {
                        self.map.move_to(&mut self.ros.hero, (3, 8));
                    }
                }
            }
        }

        // Move character

        if let Some(key) = last_key_pressed {
            match key {
                KeyCode::Left  => self.map.move_delta(&mut self.ros.hero, (-1, 0)),
                KeyCode::Right => self.map.move_delta(&mut self.ros.hero, (1, 0)),
                KeyCode::Up    => self.map.move_delta(&mut self.ros.hero, (0, -1)),
                KeyCode::Down  => self.map.move_delta(&mut self.ros.hero, (0, 1)),
                _ => (),
            }
        }
    }

    fn advance_game_over(&mut self, key: Option<KeyCode>) {
        if Some(KeyCode::Enter) == key {
            *self = Play::new_default_level();
        }
    }
}

// "Map": Grid of locations. Most of the current state of game.
struct Map {
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

    // INSERT: create_at ...

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

    // Nothing happens if target is off map. Higher layer should prevent that.
    fn move_delta(&mut self, pos: &mut Pos, delta: Delta) {
        self.move_to(pos, (pos.0 + delta.0, pos.1 + delta.1));
    }

    // Access loc.ents stacked at given coords (not using height field in Pos)
    // Used to add and remove from map, mostly internally
    fn at(&self, pos: Pos) -> &Vec<Ent> {
        &self.locs[pos.0 as usize][pos.1 as usize].ents
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
struct Ros {
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
struct Loc {
    ents: Vec<Ent>,
}

impl Loc {
    fn new() -> Loc {
        Loc { ents: vec![] }
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
struct Ent {
    // Cache of coords ent is at on map. These might be useful for movement logic, but probably
    // aren't required.
    x: i16,
    y: i16,
    h: u16,

    // Visual display.
    border: Option<Color>,
    fill: Option<Color>,
    tex: Option<Texture2D>,

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
    fn empty(x: i16, y:i16) -> Ent {
        Ent {
            x: x,
            y: y,
            ..Ent::invalid()
        }
    }

    fn is_placeholder(&self) -> bool {
        self.x == -1
    }

    #[allow(dead_code)]
    fn new_tex(x: i16, y:i16, tex: Texture2D) -> Ent {
        Ent {
            x: x,
            y: y,
            h: 1, // TODO
            tex: Some(tex),
            ..Ent::invalid()
        }
    }

    fn new_tex_col(x: i16, y:i16, tex: Texture2D, fill: Color) -> Ent {
        // TODO: Shouldn't need coords as put_at should take care of that.
        Ent {
            x: x,
            y: y,
            h: 1, // TODO
            tex: Some(tex),
            fill: Some(fill),
            ..Ent::invalid()
        }
    }

    fn new_col(x: i16, y:i16, fill: Color) -> Ent {
        Ent {
            x: x,
            y: y,
            h: 1, // TODO
            fill: Some(fill),
            ..Ent::invalid()
        }
    }

    fn new_col_outline(x: i16, y:i16, fill: Color, outline: Color) -> Ent {
        Ent {
            x: x,
            y: y,
            h: 1, // TODO
            fill: Some(fill),
            border: Some(outline),
            ..Ent::invalid()
        }
    }

    // Specific ent types
    fn new_hero_crab(x: i16, y:i16) -> Ent {
        Ent {
            ai: AI::Hero,
            ..Ent::new_tex_col(x, y, load_texture_blocking_unwrap("imgs/ferris.png"), GOLD)
        }
    }

    fn new_snake(x: i16, y:i16, dir: Delta) -> Ent {
        Ent {
            dir: dir,
            ai: AI::Snake,
            ..Ent::new_col(x, y, DARKGREEN)
        }
    }

    fn new_floor(x: i16, y: i16) -> Ent {
        Ent {
            ..Ent::new_col_outline(x, y, WHITE, LIGHTGRAY)
        }
    }
}

#[derive(Clone)]
enum AI {
    Stay, // No self movement.
    Hero, // Controlled by keys.
    Snake, // Move in direction, bounce of walls, move orthogonally towards hero.
}

struct Input {
    speed: f64,
    last_update: f64,
    // Should change to list.
    // Ideally contain Move(1,0) action not KeyRight.
    last_key_pressed: Option<KeyCode>,
}

impl Input {
    fn new_default() -> Input {
        Input {
            speed: 0.3,
            last_update: get_time(),
            last_key_pressed: None,
        }
    }

    fn read_input(&mut self) {
        if let Some(key) = get_last_key_pressed() {
            self.last_key_pressed = Some(key);
        }
    }

    fn ready_for_tick(&mut self) -> bool {
        if get_time() - self.last_update > self.speed {
            self.last_update = get_time();
            true
        } else {
            false
        }
    }

    fn consume_keypresses(&mut self) -> Option<KeyCode> {
        self.last_key_pressed.take()
    }
}

// Render state for one frame of level
// Currently not needing any global graphics state
struct RenderLevel {
    // COORDS FOR CURRENT FRAME. In gl units which are pixels.
    // Distance from edge of drawing surface to play area
    offset_x: f32,
    // Distance from edge of drawing surface to play area
    offset_y: f32,
    // Size of each tile
    sq_w: f32,
    sq_h: f32,
}

impl RenderLevel {
    fn begin(w: u16, h: u16) -> RenderLevel {
        assert_eq!(w, h);
        let game_size = screen_width().min(screen_height());
        let offset_y = (screen_height() - game_size) / 2. + 10.;

        let r = RenderLevel {
            // TODO: Why does this work with landscape orientation?
            offset_x: (screen_width() - game_size) / 2. + 10.,
            offset_y: (screen_height() - game_size) / 2. + 10.,
            sq_w: (screen_height() - offset_y * 2.) / w as f32,
            sq_h: (screen_height() - offset_y * 2.) / w as f32,
        };

        r._draw_backdrop();

        r
    }

    fn _draw_backdrop(&self)
    {
        clear_background(LIGHTGRAY);

        draw_text(format!("Level: 1", ).as_str(), 10., 20., 20., DARKGRAY);
    }

    // Draw ent's texture/colour to the screen at specified tile coords.
    // Works out pixel coords given pixel size of play area in RenderLevel.
    fn draw_ent(
        self: &RenderLevel,
        // View coords in map. Relative to first visible tile (currently always the same).
        vx: i16,
        vy: i16,
        // Ent to draw
        ent: &Ent,
    ) {
       let px = self.offset_x + self.sq_w * vx as f32;
       let py = self.offset_y + self.sq_h * vy as f32;

        if let Some(col) = ent.fill {
            draw_rectangle(px, py, self.sq_w, self.sq_h, col);
        }

        if let Some(col) = ent.border {
            draw_rectangle_lines(px, py, self.sq_w, self.sq_h, 2., col);
        }

        if let Some(tex) = &ent.tex {
            draw_texture_ex(
                &tex,
                px,
                py,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.sq_w, self.sq_h)),
                    ..Default::default()
                },
            );
        }
    }
}

// Render state for one frame of game over
// Currently not needing any global graphics state
struct RenderGameOver {
}

impl RenderGameOver
{
    fn begin() -> RenderGameOver {
        clear_background(WHITE);
        let text = "Game Over. Press [enter] to play again.";
        let font_size = 30.;
        let text_size = measure_text(text, None, font_size as _, 1.0);

        draw_text(
            text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. + text_size.height / 2.,
            font_size,
            DARKGRAY,
        );

        RenderGameOver {}
    }
}

#[macroquad::main("Tile Game")]
async fn main() {
    let mut g = Game::new_default();

    loop {
        g.do_frame();

        next_frame().await;
    }
}
