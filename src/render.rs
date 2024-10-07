use std::collections::HashMap;

use macroquad::prelude::*;

use crate::*;

use play::Play;
use ent::Ent;
use play::Mode;

pub fn draw_frame(p: &Play) {
    // STUB: Avoid passing in whole Play object.
    match p.mode {
        Mode::LevPlay => {
            let mut r = RenderLev::begin(p.map.w(), p.map.h());
            // Coords of first visible tile. Currently always 0,0.
            let (ox, oy) = (0, 0);
            for (x, y, loc) in p.map.locs() {
                for ent in &loc.ents {
                    r.draw_ent(x - ox, y - oy, ent);
                }
            }
        }
        Mode::Splash => {
            let _r = RenderSplash::begin(&p.splash_text);
        }
    }
}

// Render state for one frame of level
// Created each frame, but now has tex_cache should be maintained between frames.
pub struct RenderLev {
    // COORDS FOR CURRENT FRAME. In gl units which are pixels.
    // Distance from edge of drawing surface to play area
    offset_x: f32,
    // Distance from edge of drawing surface to play area
    offset_y: f32,
    // Size of each tile
    sq_w: f32,
    sq_h: f32,
    ///
    tex_cache: HashMap<String, Texture2D>,
}

/// Sync load macroquad texture. Panic on failure.
pub fn load_texture_blocking_unwrap(path: &str) -> Texture2D {
    futures::executor::block_on(load_texture(path)).unwrap()
}

impl RenderLev {
    pub fn begin(w: u16, h: u16) -> RenderLev {
        assert_eq!(w, h);
        let game_size = screen_width().min(screen_height());
        let offset_y = (screen_height() - game_size) / 2. + 10.;

        let r = RenderLev {
            // FIXME: Why does this work with landscape orientation?
            offset_x: (screen_width() - game_size) / 2. + 10.,
            offset_y: (screen_height() - game_size) / 2. + 10.,
            sq_w: (screen_height() - offset_y * 2.) / w as f32,
            sq_h: (screen_height() - offset_y * 2.) / w as f32,
            tex_cache: HashMap::new(),
        };

        r.draw_backdrop();

        r
    }

    fn draw_backdrop(&self)
    {
        clear_background(LIGHTGRAY);

        draw_text(format!("Level: 1", ).as_str(), 10., 20., 20., DARKGRAY);
    }

    // Draw ent's texture/colour to the screen at specified tile coords.
    // Works out pixel coords given pixel size of play area in RenderLev.
    pub fn draw_ent(
        self: &mut RenderLev,
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

        if let Some(tex_path) = ent.tex_path.clone() {
            // Can reduce number of clones? Can you HashMap<&String> instead of String?
            let tex_data = self.tex_cache.entry(tex_path.clone()).or_insert_with(||load_texture_blocking_unwrap(&tex_path));
            draw_texture_ex(
                &tex_data,
                px,
                py,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.sq_w, self.sq_h)),
                    ..Default::default()
                },
            );
        }

        if let Some(text) = ent.text.clone() {
            let text_col = ent.text_col.unwrap_or(DARKGRAY);
            draw_text(&text, px + self.sq_w*0.1, py + self.sq_h*0.6, self.sq_h*0.4, text_col);
        }
    }
}

// Render state for one frame of "Show text, press enter to continue"
// Currently not needing any global graphics state
pub struct RenderSplash {
}

impl RenderSplash
{
    pub fn begin(text: &str) -> RenderSplash {
        clear_background(WHITE);
        let font_size = 30.;
        let text_size = measure_text(text, None, font_size as _, 1.0);

        // FIXME: Multi-line text. Ideally with dialog pics etc.
        draw_text(
            text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. + text_size.height / 2.,
            font_size,
            DARKGRAY,
        );

        RenderSplash {}
    }
}
