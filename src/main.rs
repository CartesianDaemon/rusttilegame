use macroquad::prelude::*;

#[allow(unused_imports)]
use std::collections::LinkedList;

// Tile coords (but without specifying height)
type Pos = (i16, i16);

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
                }
            }
        }

        draw_rectangle(
            offset_x + g.p.map.snake.head.0 as f32 * sq_size,
            offset_y + g.p.map.snake.head.1 as f32 * sq_size,
            sq_size,
            sq_size,
            DARKGREEN,
        );

        draw_rectangle(
            offset_x + g.p.map.fruit.0 as f32 * sq_size,
            offset_y + g.p.map.fruit.1 as f32 * sq_size,
            sq_size,
            sq_size,
            GOLD,
        );

        g.r.draw_sq(
            &g.p.map.locs[0][0].ents[0].tex.as_ref().unwrap(), // Should only be temporary. If not can we use ?.
            offset_x + g.p.map.fruit.0 as f32 * sq_size,
            offset_y + g.p.map.fruit.1 as f32 * sq_size,
            sq_size as f32,
            sq_size as f32,
        );

        draw_text(format!("SCORE: {}", 42).as_str(), 10., 20., 20., DARKGRAY);
    }
}

// Gameplay state: current level, map, etc.
// May split out values relevant to current mode (level, menu, etc).
struct Play {
    game_over: bool,

    // Layout of current level.
    map: Map,
}

impl Play {
    fn new_empty_level() -> Play {
        Play {
            game_over: false,
            map: Map::new_example_level(16),
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

        // Initialise fruit
        {
            play.map.fruit = (3, 8);
            // PUSH ONTO MAP

            // Crab texture
            play.map.locs[0][0].ents[0].tex = Some(load_texture("imgs/ferris.png").await.unwrap()); // Some() necessary?
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

        // if snake on same row xor column as fruit, change dir to face fruit
        if (self.map.snake.head.0 == self.map.fruit.0) != (self.map.snake.head.1 == self.map.fruit.1) {
            self.map.snake.dir = ((self.map.fruit.0 - self.map.snake.head.0).signum(),(self.map.fruit.1 - self.map.snake.head.1).signum())
        }

        // move head to new location
        self.map.snake.head = (self.map.snake.head.0 + self.map.snake.dir.0, self.map.snake.head.1 + self.map.snake.dir.1);
        if self.map.snake.head == self.map.fruit {
            // If new head is on fruit, eat it. Body is already the right length.
            self.map.fruit = (3, 8); // TODO: Removed the random here as not wanted long term.
        }

        // die if head out of bounds
        if self.map.snake.head.0 < 0
            || self.map.snake.head.1 < 0
            || self.map.snake.head.0 as i32 >= self.map.w() as i32 // TODO: Better comparisons
            || self.map.snake.head.1 as i32>= self.map.h() as i32
        {
            self.game_over = true;
        }

        // Move character

        if let Some(key) = last_key_pressed {
            match key {
                KeyCode::Left  => self.map.fruit.0 -= 1,
                KeyCode::Right => self.map.fruit.0 += 1,
                KeyCode::Up    => self.map.fruit.1 -= 1,
                KeyCode::Down  => self.map.fruit.1 += 1,
                _ => (),
            }
        }
    }

    fn advance_game_over(&mut self, key: Option<KeyCode>) {
        if Some(KeyCode::Enter) == key {
            self.map.snake = Snake {
                head: (0, 0),
                dir: (1, 0),
            };
            self.map.fruit = (3, 8);
            self.game_over = false;
        }
    }
}

// "Map": Grid of locations. Most of the current state of game.
struct Map {
    // Stored as a collection of columns.
    // Must always be square.
    locs: Vec<Vec<Loc>>,

    // Coordinates of fruit (soon to be character).
    // Will have vec of enemies etc too.
    // Those all need to have "pointers" into map
    // And act like a cache?
    fruit: Pos,

    snake: Snake,
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
            fruit: (0, 0),
            snake: Snake {
                head: (0, 0),
                dir: (1, 0),
            }
        }
    }

    fn w(&self) -> u16 {
        // TODO: Could return usize, most callers need to cast anyway.
        self.locs.len() as u16
    }

    fn h(&self) -> u16 {
        self.locs[0].len() as u16
    }

    // INSERT: move_to(&self, Pos, To) fn

    // Consider implementing index [idx] for map returning map.locs[idx]
    // Consider "for x,y in map.coords()" to iterate over x and y at the same time.
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
    h: i16,
    border: Option<Color>,
    fill: Option<Color>,
    tex: Option<Texture2D>,
}

impl Ent {
    fn invalid() -> Ent {
        Ent {
            x: -1,
            y: -1,
            h: -1,
            border: None,
            fill: None,
            tex: None,
        }
    }

    fn new_tex(x: i16, y:i16, tex: Texture2D) -> Ent {
        Ent {
            x: x,
            y: y,
            h: 1, // TODO
            tex: Some(tex),
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
    head: Pos,
    dir: Pos,
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
