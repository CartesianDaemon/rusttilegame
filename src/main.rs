use macroquad::prelude::*;

use std::collections::LinkedList;

type Point = (i16, i16);

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

// "Map": Grid of locations. Most of the current state of game.
// Might have more than one in future.
#[allow(dead_code)]
struct Map {
    w: u16,
    h: u16,
    // Stored as a collection of columns
    locs: Vec<Vec<Loc>>,
}

impl Map {
    fn new(sz: u16) -> Map {
        Map {
            w: sz,
            h: sz,
            locs: vec!(vec!(Loc::new(); sz.into()); sz.into()),
        }
    }

    // Consider implementing index [idx] for map returning map.locs[idx]
    // Consider "for x,y in map.coords()" to iterate over x and y at the same time.
}

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    dir: Point,
}

// Gameplay state: current level, map, etc.
#[allow(dead_code)]
struct Play {
    // Number of squares on each side of map.
    // Should be removed in favour of map size
    squares: u16,
    // map
    map: Map,
    // Coordinates of fruit (soon to be character).
    fruit: Point,
}

impl Play {
    fn new() -> Play {
        Play {
            squares: 0,
            map: Map::new(16),
            fruit: (0, 0),
        }
    }
}

// Render state: screen size, etc.
#[allow(dead_code)]
struct Render {
    // Size of grid squares. Set proportional to window size at start of current frame.
    // Should not be kept around outside a frame?
    sq_size: f32,
}

impl Render {
    fn new() -> Render {
        Render {
            sq_size: 0.,
        }
    }

    // Draw a tile's texture given the object's window coordinates.
    fn draw_sq(
        self: &Render,
        tex: &Texture2D,
        x: f32,
        y: f32,
    ) {
        draw_texture_ex(
            &tex,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.sq_size, self.sq_size)),
                ..Default::default()
            },
        );
    }
}


// Overall game state.
#[allow(dead_code)]
struct Game {
    p: Play,
    r: Render,

    // TO BE REMOVED:
    squares: u16,
}

// Functions requiring only gameplay state
impl Game {
    // update state? needing some form of keypress input etc
}

// Functions requiring all of game state
impl Game {
    // draw frame?
}

#[macroquad::main("Snake")]
async fn main() {
    let tex_crab: Texture2D = load_texture("imgs/ferris.png").await.unwrap();

    let mut snake = Snake {
        head: (0, 0),
        dir: (1, 0),
        body: LinkedList::new(),
    };
    let mut g = Game { squares: 16, p: Play::new(), r: Render::new() };

    let mut score = 0;
    let mut speed = 0.3;
    let mut last_update = get_time();
    let mut game_over = false;

    let mut last_key_pressed : Option<KeyCode> = None;

    let mut map = Map::new(16);

    // Initialise Floor
    {
        for x in 0..map.w {
            for y in 0..map.h {
                map.locs[x as usize][y as usize].ents.push(Ent::new_floor(x, y))
            }
        }
    }

    // Initialise fruit
    {
        g.p.fruit = (3, 8);
        // PUSH ONTO MAP
    }

    loop {
        // Read input each frame
        // TODO: Maybe glob all keys in queue.
        if let Some(key) = get_last_key_pressed() {
            last_key_pressed = Some(key);
        }

        // Update game state each tick
        if !game_over && get_time() - last_update > speed {
            // Remember time we drew current frame, to know when to draw next frame.
            last_update = get_time();

            // Move snake

            // add old head to top of body
            snake.body.push_front(snake.head);

            // if snake on same row xor column as fruit, change dir to face fruit
            if (snake.head.0 == g.p.fruit.0) != (snake.head.1 == g.p.fruit.1) {
                snake.dir = ((g.p.fruit.0 - snake.head.0).signum(),(g.p.fruit.1 - snake.head.1).signum())
            }

            // move head to new location
            snake.head = (snake.head.0 + snake.dir.0, snake.head.1 + snake.dir.1);
            if snake.head == g.p.fruit {
                // If new head is on fruit, eat it. Body is already the right length.
                g.p.fruit = (3, 8); // TODO: Removed the random here as not wanted long term.
                score += 100;
                speed *= 0.9;
            } else {
                // If snake didn't eat anything, remove tip of tail.
                snake.body.pop_back();
            }
            // die if head out of bounds
            if snake.head.0 < 0
                || snake.head.1 < 0
                || snake.head.0 as i32 >= g.squares as i32 // TODO: Better comparisons
                || snake.head.1 as i32>= g.squares as i32
            {
                game_over = true;
            }
            // die if head intersects body
            for (x, y) in &snake.body {
                if *x == snake.head.0 && *y == snake.head.1 {
                    game_over = true;
                }
            }

            // Move character
            if let Some(key) = last_key_pressed {
                match key {
                    KeyCode::Left  => g.p.fruit.0 -= 1,
                    KeyCode::Right => g.p.fruit.0 += 1,
                    KeyCode::Up    => g.p.fruit.1 -= 1,
                    KeyCode::Down  => g.p.fruit.1 += 1,
                    _ => (),
                }
            }
            last_key_pressed = None;
        }
        if !game_over {
            // TODO: MOVE ALL THIS BRANCH INTO GAME, CALLING RENDER.

            clear_background(LIGHTGRAY);

            let game_size = screen_width().min(screen_height());
            let offset_x = (screen_width() - game_size) / 2. + 10.;
            let offset_y = (screen_height() - game_size) / 2. + 10.;
            g.r.sq_size = (screen_height() - offset_y * 2.) / g.squares as f32;

            for x in 0..map.w {
                for y in 0..map.h {
                    for ent in &map.locs[x as usize][y as usize].ents {
                        if let Some(col) = ent.fill {
                            draw_rectangle(
                                offset_x + g.r.sq_size * x as f32,
                                offset_y + g.r.sq_size * y as f32,
                                g.r.sq_size as f32,
                                g.r.sq_size as f32,
                                col,
                            );
                        }
                        if let Some(col) = ent.border {
                            draw_rectangle_lines(
                                offset_x + g.r.sq_size * x as f32,
                                offset_y + g.r.sq_size * y as f32,
                                g.r.sq_size as f32,
                                g.r.sq_size as f32,
                                2.,
                                col,
                            );
                        }
                    }
                }
            }

            draw_rectangle(
                offset_x + snake.head.0 as f32 * g.r.sq_size,
                offset_y + snake.head.1 as f32 * g.r.sq_size,
                g.r.sq_size,
                g.r.sq_size,
                DARKGREEN,
            );

            for (x, y) in &snake.body {
                draw_rectangle(
                    offset_x + *x as f32 * g.r.sq_size,
                    offset_y + *y as f32 * g.r.sq_size,
                    g.r.sq_size,
                    g.r.sq_size,
                    LIME,
                );
            }

            draw_rectangle(
                offset_x + g.p.fruit.0 as f32 * g.r.sq_size,
                offset_y + g.p.fruit.1 as f32 * g.r.sq_size,
                g.r.sq_size,
                g.r.sq_size,
                GOLD,
            );

            g.r.draw_sq(
                &tex_crab,
                offset_x + g.p.fruit.0 as f32 * g.r.sq_size,
                offset_y + g.p.fruit.1 as f32 * g.r.sq_size,
            );

            draw_text(format!("SCORE: {score}").as_str(), 10., 20., 20., DARKGRAY);
        } else {
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

            if is_key_down(KeyCode::Enter) {
                snake = Snake {
                    head: (0, 0),
                    dir: (1, 0),
                    body: LinkedList::new(),
                };
                g.p.fruit = (3, 8);
                score = 0;
                speed = 0.3;
                last_update = get_time();
                game_over = false;
            }
        }
        next_frame().await;
    }
}
