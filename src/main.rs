use macroquad::prelude::*;

use std::collections::LinkedList;

type Point = (i16, i16);

// Overall game state.
#[allow(dead_code)]
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

    fn draw_frame(&self) {
        if !self.p.game_over {
            self.draw_level();
        } else {
            self.r.draw_game_over();
        }
    }

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
            offset_x + g.p.snake.head.0 as f32 * sq_size,
            offset_y + g.p.snake.head.1 as f32 * sq_size,
            sq_size,
            sq_size,
            DARKGREEN,
        );

        for (x, y) in &g.p.snake.body {
            draw_rectangle(
                offset_x + *x as f32 * sq_size,
                offset_y + *y as f32 * sq_size,
                sq_size,
                sq_size,
                LIME,
            );
        }

        draw_rectangle(
            offset_x + g.p.fruit.0 as f32 * sq_size,
            offset_y + g.p.fruit.1 as f32 * sq_size,
            sq_size,
            sq_size,
            GOLD,
        );

        g.r.draw_sq(
            &g.p.map.locs[0][0].ents[0].tex.as_ref().unwrap(), // Should only be temporary. If not can we use ?.
            offset_x + g.p.fruit.0 as f32 * sq_size,
            offset_y + g.p.fruit.1 as f32 * sq_size,
            sq_size as f32,
            sq_size as f32,
        );

        draw_text(format!("SCORE: {}", g.p.score).as_str(), 10., 20., 20., DARKGRAY);
    }
}

// Gameplay state: current level, map, etc.
#[allow(dead_code)]
struct Play {
    score: i32,
    game_over: bool,

    // Layout of current level.
    map: Map,

    // Coordinates of fruit (soon to be character).
    // Will have vec of enemies etc too.
    // Those all need to have "pointers" into map
    // And act like a cache?
    fruit: Point,

    snake: Snake,
}

impl Play {
    fn new_empty_level() -> Play {
        Play {
            score: 0,
            game_over: false,
            map: Map::new(16),
            fruit: (0, 0),
            snake: Snake {
                head: (0, 0),
                dir: (1, 0),
                body: LinkedList::new(),
            }
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
            play.fruit = (3, 8);
            // PUSH ONTO MAP

            // Crab texture
            play.map.locs[0][0].ents[0].tex = Some(load_texture("imgs/ferris.png").await.unwrap()); // Some() necessary?
        }

        play
    }

    fn advance(&mut self, last_key_pressed: Option<KeyCode>) {
        // Move snake

        // add old head to top of body
        self.snake.body.push_front(self.snake.head);

        // if snake on same row xor column as fruit, change dir to face fruit
        if (self.snake.head.0 == self.fruit.0) != (self.snake.head.1 == self.fruit.1) {
            self.snake.dir = ((self.fruit.0 - self.snake.head.0).signum(),(self.fruit.1 - self.snake.head.1).signum())
        }

        // move head to new location
        self.snake.head = (self.snake.head.0 + self.snake.dir.0, self.snake.head.1 + self.snake.dir.1);
        if self.snake.head == self.fruit {
            // If new head is on fruit, eat it. Body is already the right length.
            self.fruit = (3, 8); // TODO: Removed the random here as not wanted long term.
            self.score += 100;
        } else {
            // If snake didn't eat anything, remove tip of tail.
            self.snake.body.pop_back();
        }
        // die if head out of bounds
        if self.snake.head.0 < 0
            || self.snake.head.1 < 0
            || self.snake.head.0 as i32 >= self.map.w() as i32 // TODO: Better comparisons
            || self.snake.head.1 as i32>= self.map.h() as i32
        {
            self.game_over = true;
        }
        // die if head intersects body
        for (x, y) in &self.snake.body {
            if *x == self.snake.head.0 && *y == self.snake.head.1 {
                self.game_over = true;
            }
        }

        // Move character

        if let Some(key) = last_key_pressed {
            match key {
                KeyCode::Left  => self.fruit.0 -= 1,
                KeyCode::Right => self.fruit.0 += 1,
                KeyCode::Up    => self.fruit.1 -= 1,
                KeyCode::Down  => self.fruit.1 += 1,
                _ => (),
            }
        }
    }
}

// "Map": Grid of locations. Most of the current state of game.
#[allow(dead_code)]
struct Map {
    // Stored as a collection of columns.
    // Must always be square.
    locs: Vec<Vec<Loc>>,
}

impl Map {
    fn new(sz: u16) -> Map {
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
    head: Point,
    body: LinkedList<Point>,
    dir: Point,
}

struct Input {
    speed: f64,
    last_update: f64,
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
}

// Render state: screen size, etc.
#[allow(dead_code)]
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
        // Read input each frame
        // TODO: Maybe glob all keys in queue.
        if let Some(key) = get_last_key_pressed() {
            g.i.last_key_pressed = Some(key);
        }

        // Update game state each tick
        if !g.p.game_over && get_time() - g.i.last_update > g.i.speed {
            // Remember when we advanced game state, to know when next time is due.
            g.i.last_update = get_time();

            g.p.advance(g.i.last_key_pressed);

            g.i.last_key_pressed = None;
        }

        if g.p.game_over {
            if is_key_down(KeyCode::Enter) {
                g.p.snake = Snake {
                    head: (0, 0),
                    dir: (1, 0),
                    body: LinkedList::new(),
                };
                g.p.fruit = (3, 8);
                g.p.score = 0;
                g.i.speed = 0.3;
                g.i.last_update = get_time();
                g.p.game_over = false;
            }
        }

        g.draw_frame();

        next_frame().await;
    }
}
