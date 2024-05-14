use macroquad::prelude::*;

#[allow(unused_imports)]
use std::collections::LinkedList;
use std::mem;
use std::ops::Index;
use std::ops::IndexMut;

// Tile coords (but without specifying height)
type Pos = (i16, i16, u16);
type Point = (i16, i16);
type Delta = (i16, i16);

// Overall game state.
struct Game {
    p: Play,
    r: Render,
    i: Input,
}

impl Game {
    async fn new_default() -> Game {
        Game {
            p: Play::new_default_level().await,
            r: Render::new_default(),
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
            self.draw_level();
        } else {
            self.r.draw_game_over();
        }
    }

    // Some of this needs to move into Render. Some may be in Map.
    fn draw_level(&self) {
        let g = self;

        clear_background(LIGHTGRAY);

        // TODO: Make sure sizing works with non-square maps.
        // It SHOULD work with windows portrait not landscape but not tested.
        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / g.p.map.w() as f32;

        for x in 0..g.p.map.w() {
            for y in 0..g.p.map.h() {
                for ent in &g.p.map.locs[x as usize][y as usize].ents {
                    if let Some(col) = ent.fill {
                        draw_rectangle(
                            offset_x + sq_size * x as f32,
                            offset_y + sq_size * y as f32,
                            sq_size as f32,
                            sq_size as f32,
                            col,
                        );
                    }
                    if let Some(col) = ent.border {
                        draw_rectangle_lines(
                            offset_x + sq_size * x as f32,
                            offset_y + sq_size * y as f32,
                            sq_size as f32,
                            sq_size as f32,
                            2.,
                            col,
                        );
                    }
                    if let Some(tex) = &ent.tex {
                        g.r.draw_sq(
                            &tex,
                            offset_x + sq_size * x as f32,
                            offset_y + sq_size * y as f32,
                            sq_size as f32,
                            sq_size as f32,
                        );
                    }
                }
            }
        }

        /*
        draw_rectangle(
            offset_x + g.p.ros.snake.pos.0 as f32 * sq_size,
            offset_y + g.p.ros.snake.pos.1 as f32 * sq_size,
            sq_size,
            sq_size,
            DARKGREEN,
        );
        */

        draw_text(format!("SCORE: {}", 42).as_str(), 10., 20., 20., DARKGRAY);
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
            map: Map::new_example_level(16),
            ros: Ros::new_example_level(16), // These two should be generated together
        }
    }

    async fn new_default_level() -> Play {
        // Some of this may move to Map, or to a new intermediate struct.

        let mut play = Self::new_empty_level();

        // Initialise Floor
        {
            for x in 0..play.map.w() {
                for y in 0..play.map.h() {
                    play.map.locs[x as usize][y as usize].ents.push(Ent::new_floor(x, y))
                }
            }
        }

        // Initialise hero
        {
            play.ros.hero = (3, 8, 1);
            play.map.locs[3][8].ents.push(Ent::new_tex_col(3, 8, load_texture("imgs/ferris.png").await.unwrap(), GOLD));
        }

        // Initialise snake
        {
            // TODO: create_at
            play.ros.snake.pos = (0, 0, 1);
            play.ros.snake.dir = (1, 0);
            play.map.locs[0][0].ents.push(Ent::new_col(3, 8, DARKGREEN));
        }


        play
    }

    // Does current mode need UI to wait for tick before updating state?
    // Currently yes in level, no in game over.
    fn continuous(&self) -> bool {
        self.game_over
    }

    fn advance_level(&mut self, last_key_pressed: Option<KeyCode>) {
        // Move snake

        // if snake on same row xor column as hero, change dir to face hero
        if (self.ros.snake.pos.0 == self.ros.hero.0) != (self.ros.snake.pos.1 == self.ros.hero.1) {
            self.ros.snake.dir = ((self.ros.hero.0 - self.ros.snake.pos.0).signum(),(self.ros.hero.1 - self.ros.snake.pos.1).signum())
        }

        // die if snake would go out of bounds
        // TODO: Instead: Game over if snake eats char; Respawn snake if dies.
        if !(0..self.map.w() as i16).contains(&(self.ros.snake.pos.0 + self.ros.snake.dir.0)) ||
            !(0..self.map.h() as i16).contains(&(self.ros.snake.pos.1 + self.ros.snake.dir.1))
        {
            self.game_over = true;
        }
        else
        {
            // move snake to new location
            self.map.move_delta(&mut self.ros.snake.pos, self.ros.snake.dir);
        }

        // eat hero?
        if self.ros.snake.pos.0 == self.ros.hero.0 && self.ros.snake.pos.1 == self.ros.hero.1 {
            self.map.move_to(&mut self.ros.hero, (3, 8));
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
            self.map.move_to(&mut self.ros.hero, (8, 3));
            self.map.move_to(&mut self.ros.snake.pos, (1,1));
            self.ros.snake.dir = (1, 0);
            self.game_over = false;
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

    fn new_example_level(sz: u16) -> Map {
        // Some of this may move back up to Play, or from there to here.
        Map {
            locs: vec!(vec!(Loc::new(); sz.into()); sz.into()),
            // cf coords in Ros::new_example_level()
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
        let ent = if pos.2 as usize == self.locs[pos.0 as usize][pos.1 as usize].ents.len() {
            self.locs[pos.0 as usize][pos.1 as usize].ents.pop().unwrap()
        } else {
            mem::replace(&mut self[*pos], Ent::placeholder())
        };

        // pop any new or old placeholders from vector
        while !self.locs[pos.0 as usize][pos.1 as usize].ents.is_empty() &&
            self.locs[pos.0 as usize][pos.1 as usize].ents.last().unwrap().is_placeholder() {
            self.locs[pos.0 as usize][pos.1 as usize].ents.pop();
        }

        self.locs[to.0 as usize][to.1 as usize].ents.push(ent);
        *pos = (to.0, to.1, (self.locs[to.0 as usize][to.1 as usize].ents.len()-1) as u16);
        self.locs[pos.0 as usize][pos.1 as usize].ents.last_mut().unwrap().x = pos.0;
        self.locs[pos.0 as usize][pos.1 as usize].ents.last_mut().unwrap().y = pos.1;
        self.locs[pos.0 as usize][pos.1 as usize].ents.last_mut().unwrap().h = pos.2;
    }

    // Nothing happens if target is off map. Higher layer should prevent that.
    fn move_delta(&mut self, pos: &mut Pos, delta: Delta) {
        self.move_to(pos, (pos.0 + delta.0, pos.1 + delta.1));
    }

    fn at(&mut self, pos: Pos) -> &mut Vec<Ent> {
        &mut self.locs[pos.0 as usize][pos.1 as usize].ents
    }

    // Consider "for x,y in map.coords()" to iterate over x and y at the same time.
}

// Roster of character, enemies, etc. Indexes into map.
struct Ros {
    // Coordinates of hero (soon to be character).
    // Will have vec of enemies etc too.
    // Those all need to have "pointers" into map
    // And act like a cache?
    hero: Pos, // TODO: Better name for protagonist than "hero".
    snake: Snake,
}

impl Ros {
    fn new_example_level(sz: u16) -> Ros {
        assert_eq!(sz, 16);
        Ros {
            hero: (0, 0, 1), // TODO: Put in invalid coords to start?
            snake: Snake {
                pos: (0, 0, 1), // TODO: Shouldn't need to match coords elsewhere but may
                dir: (1, 0),
            }
        }
    }
}

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
    x: i16,
    y: i16,
    h: u16,
    border: Option<Color>,
    fill: Option<Color>,
    tex: Option<Texture2D>,
}

impl Ent {
    fn invalid() -> Ent {
        Ent {
            x: -1, // For now "-1" flags "this element is a placeholder in height vector"
            y: -1,
            h: 0,
            border: None,
            fill: None,
            tex: None,
        }
    }

    fn placeholder() -> Ent {
        Ent::invalid()
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
            tex: None,
            fill: Some(fill),
            ..Ent::invalid()
        }
    }

    // TODO: Want to combine into a "make_at" function which initialises locations ok.
    // TODO: Should be global or part of map, or part of Ent?
    fn new_floor(x: u16, y: u16) -> Ent {
        Ent {
            x: x as i16,
            y: y as i16,
            h: 0,
            border: Some(LIGHTGRAY),
            fill: Some(WHITE),
            tex: None,
        }
    }
}

struct Snake {
    pos: Pos,
    dir: Delta,
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

// Render state: screen size, etc.
struct Render {
}

impl Render {
    fn new_default() -> Render {
        Render {
        }
    }

    // Draw a tile's texture given the object's window coordinates.
    fn draw_sq(
        self: &Render,
        tex: &Texture2D,
        // All in gl coords (approximately pixels).
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) {
        draw_texture_ex(
            &tex,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                ..Default::default()
            },
        );
    }

    fn draw_game_over(&self) {
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
    }
}

#[macroquad::main("Snake")]
async fn main() {
    let mut g = Game::new_default().await;

    loop {
        g.do_frame();

        next_frame().await;
    }
}
