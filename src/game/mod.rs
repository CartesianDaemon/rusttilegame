use macroquad::prelude::*;

mod input;
use input::*;
mod map;
use map::*;
mod load;

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

// Overall game state.
pub struct Game {
    p: Play,
    i: Input,
}

impl Game {
    pub fn new_default() -> Game {
        Game {
            p: load::load_level(1),
            i: Input::new_default(),
        }
    }

    pub fn do_frame(&mut self) {
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


//
// PLAY
//
// TODO: Move Play to play submod?
//

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
            let mut dir = (0, 0);
            match key {
                KeyCode::Left  => dir = (-1, 0),
                KeyCode::Right => dir = (1, 0),
                KeyCode::Up    => dir = (0, -1),
                KeyCode::Down  => dir = (0, 1),
                _ => (),
            }
            if dir != (0, 0) {
                if self.map.can_move(&self.ros.hero, dir) {
                    self.map.move_delta(&mut self.ros.hero, dir);
                }
            }
        }
    }

    fn advance_game_over(&mut self, key: Option<KeyCode>) {
        if Some(KeyCode::Enter) == key {
            *self = load::load_level(1);
        }
    }
}


//
// RENDER
//
// TODO: Move Render structs to render submod?
//

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
