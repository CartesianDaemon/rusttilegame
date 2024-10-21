use std::collections::HashMap;

use macroquad::prelude::*;
use assrt::rsst;

use crate::*;

use play::Play;
use obj::Obj;
use map_coords::CoordDelta;

type TextureCache = HashMap<String, Texture2D>;

pub struct Render {
    /// Loaded textures
    texture_cache: TextureCache,
}

impl Render {
    pub fn new() -> Render {
        Render {
            texture_cache: HashMap::new(),
        }
    }

    /// Draw current gameplay to screen.
    pub async fn draw_frame(&mut self, play_state: &Play, slide_real_pc: f32, anim_real_pc: f32, ghost_state: &play::LevPlay, ghost_opacity: f32, anim_ghost_pc: f32) {
        // ENH: Avoid passing in whole Play object.
        match play_state {
            Play::LevPlay(play_state) => {
                let mut render_lev = RenderLev::begin(&mut self.texture_cache, play_state.field.map.w(), play_state.field.map.h());
                // Coords of first visible tile. Currently always 0,0.
                let (ox, oy) = (0, 0);
                let max_h = 5;
                for h in 0..max_h {
                    for (x, y, loc) in play_state.field.map.locs() {
                        if let Some(ent) = loc.get(h) {
                            render_lev.draw_ent(x - ox, y - oy, ent, anim_real_pc, slide_real_pc).await;
                        }
                    }
                }
                let draw_ghosts = false;
                if draw_ghosts
                {
                    let mut r = RenderLev::begin_ghost_overlay(render_lev, 1.0 - ghost_opacity);
                    let (ox, oy) = (0, 0); // TODO: Dedup to RenderLev::function
                    for (x, y, loc) in ghost_state.field.map.locs() {
                        for ent in loc {
                            r.draw_ent(x - ox, y - oy, ent, anim_ghost_pc, anim_ghost_pc).await;
                        }
                    }
                }
            }
            Play::Splash(play_state) => {
                let _r = RenderSplash::begin(&play_state.splash_text);
            }
        }
    }
}

/// Render state for one frame of level
/// TODO: Does this still want to be a separate class? Or more like a struct?
//#[derive(Clone)]
pub struct RenderLev<'a> {
    // COORDS FOR CURRENT FRAME. In gl units which are pixels.
    // Distance from edge of drawing surface to play area
    offset_x: f32,
    // Distance from edge of drawing surface to play area
    offset_y: f32,
    // Size of each tile
    sq_w: f32,
    sq_h: f32,
    as_ghost: bool,
    /// Transparency for rendering ghost movement
    ghost_alpha: f32,
    texture_cache: &'a mut TextureCache,
}

/// Sync load macroquad texture. Panic on failure.
pub async fn load_texture_unwrap(path: &str) -> Texture2D {
    // futures::executor::block_on(load_texture(path))

    // TODO: Remove this fallback again. But have some way of outputting errors in wasm?
    match load_texture(path).await {
        Result::Ok(tex_data) => tex_data,
        Result::Err(_err) => {
            // display error somewhere?
            Texture2D::empty()
        }
    }
}

impl<'a> RenderLev<'a> {
    pub fn begin(texture_cache: &mut TextureCache, w: u16, h: u16) -> RenderLev {
        assert_eq!(w, h);
        let game_size = screen_width().min(screen_height());
        let offset_y = (screen_height() - game_size) / 2. + 10.;

        let r = RenderLev {
            // FIXME: Why does this work with landscape orientation?
            offset_x: (screen_width() - game_size) / 2. + 10.,
            offset_y: (screen_height() - game_size) / 2. + 10.,
            sq_w: (screen_height() - offset_y * 2.) / w as f32,
            sq_h: (screen_height() - offset_y * 2.) / w as f32,
            as_ghost: false,
            ghost_alpha: 0.5, // Should be unused
            texture_cache,
        };

        r.draw_backdrop();

        r
    }

    // TODO: Is this still needed?
    pub fn begin_ghost_overlay(orig_renderlev: RenderLev, ghost_alpha: f32) -> RenderLev {
        RenderLev {
            as_ghost: true,
            ghost_alpha,
            ..orig_renderlev
        }
    }

    fn draw_backdrop(&self)
    {
        clear_background(LIGHTGRAY);

        draw_text(format!("Level: 1", ).as_str(), 10., 20., 20., DARKGRAY);
    }

    fn alpha_col(col: Color, alpha: f32) -> Color
    {
        Color {a: col.a * alpha, ..col}
    }

    // Draw ent's texture/colour to the screen at specified tile coords.
    // Works out pixel coords given pixel size of play area in RenderLev.
    pub async fn draw_ent(
        self: &mut RenderLev<'a>,
        // View coords in map. Relative to first visible tile (currently always the same).
        vx: i16,
        vy: i16,
        // Ent to draw
        obj: &Obj,
        // Proportion of animation from previous state to current (frame)
        anim_pc: f32,
        // Proportion of animation from previous state to current (position)
        slide_pc: f32,
        // TODO: Move as_ghost to parameter?
    ) {
        // TODO: move to calling function?
        if self.as_ghost && obj.pass != obj::Pass::Mov {
            return;
        }

        let base_px = self.offset_x + self.sq_w * vx as f32;
        let base_py = self.offset_y + self.sq_h * vy as f32;

        let pc_size = if self.as_ghost {0.9} else {1.};
        //let pc_size = if self.as_ghost {0.5 + 0.5*self.ghost_alpha} else {1.};

        // FYI "let px = base_px + self.sq_w * (1.-pc_size) / 2. + self.sq_w * anim_pc;" makes me really seasick.

        let dx = obj.cached_pos.x - obj.prev_pos.x;
        let dy = obj.cached_pos.y - obj.prev_pos.y;

        // Switch to using fixed frame throughout from here?
        let slide_in_frame_units = Some(3);
        let slide_fr_pc = if let Some(fixed_frames) = slide_in_frame_units {
            (slide_pc * fixed_frames as f32).floor() / fixed_frames as f32
        } else {
            slide_pc
        };

        let px = base_px + self.sq_w * (1.-pc_size) / 2. - (dx as f32 * (1.-slide_fr_pc) * self.sq_w);
        let py = base_py + self.sq_h * (1.-pc_size) / 2. - (dy as f32 * (1.-slide_fr_pc) * self.sq_h);
        let w = self.sq_w * pc_size;
        let h = self.sq_h * pc_size;

        if !obj.is_any_mov() {rsst!(obj.prev_pos == obj.cached_pos)}

        let alpha = if self.as_ghost {self.ghost_alpha} else {1.};

        if let Some(col) = obj.fill {
            draw_rectangle(px, py, w, h, Self::alpha_col(col, alpha));
        }

        if let Some(col) = obj.border {
            draw_rectangle_lines(px, py, w, h, 2., Self::alpha_col(col, alpha));
        }

        // TODO: For tex, disable ghosts or make transparency work.
        // TODO: Or better, add idle animation in place.
        if obj.tex_paths.len() > 0 {
            // TODO: Simplify calc? Prevent anim_pc being 100? Or being 0?
            let tex_frame_idx = (obj.tex_paths.len()-1).min((anim_pc * obj.tex_paths.len() as f32) as usize);
            let tex_path = &obj.tex_paths[tex_frame_idx];

            let tex_data: &Texture2D = if let Some(tex_data) = self.texture_cache.get(tex_path) {
                tex_data
            } else {
                self.texture_cache.insert(tex_path.clone(), load_texture_unwrap(tex_path).await);
                self.texture_cache.get(tex_path).unwrap()
            };

            let rotation = match obj.dir {
                CoordDelta{dx:0, dy:-1} => std::f32::consts::PI / 2.,
                CoordDelta{dx:1, dy: 0} => std::f32::consts::PI,
                CoordDelta{dx:0, dy: 1} => std::f32::consts::PI * 1.5,
                _ => 0.
            };
            draw_texture_ex(
                &tex_data,
                px - w * (obj.tex_scale-1.0) / 2.,
                py - h * (obj.tex_scale-1.0) / 2.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(w * obj.tex_scale, h * obj.tex_scale)),
                    rotation,
                    ..Default::default()
                    // TODO: alpha
                },
            );
        }

        if let Some(text) = obj.text.clone() {
            let text_col = Self::alpha_col(obj.text_col.unwrap_or(DARKGRAY), alpha);
            draw_text(&text, (px + w*0.1).floor(), (py + h*0.6).floor(), 15., text_col);
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
