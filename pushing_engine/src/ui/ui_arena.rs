use macroquad::prelude::*;

use assrt::rsst;
use crate::scene::arena::MapObj;
use crate::scene::Arena;
use crate::gamedata::BaseGameLogic;

use super::*;

/// Render state for one frame of level
// NB: Good to move Input relating to map movement in here.
//#[derive(Clone)]
pub struct UiArena<'a> {
    // COORDS FOR CURRENT FRAME. In gl units which are pixels.
    // Distance from edge of drawing surface to arena area
    // Or to centre??
    game_x: f32,
    // Distance from edge of drawing surface to arena area
    game_y: f32,
    // Size of each tile
    sq_w: f32,
    sq_h: f32,
    texture_cache: &'a mut TextureCache,
    slide_pc: f32,
    anim_pc: f32,
}

impl<'a> UiArena<'a> {
    pub async fn render<GameLogic: BaseGameLogic>(
        state: &Arena<GameLogic>,
        texture_cache: &mut TextureCache,
        // Whole screen, or smaller area, in which to fit a square map.
        draw_area: PRect,
        anim: AnimState,
    ) {
        let game_sz = draw_area.w.min(draw_area.h);
        let game_x = draw_area.x + (draw_area.w - game_sz) / 2. + 10.;
        let game_y = draw_area.y + (draw_area.h - game_sz) / 2. + 10.;

        // Area of map to display. Currently all of it.
        let w = state.map_w();
        let h = state.map_h();
        assert_eq!(w, h);

        let sq_sz = (draw_area.h - game_y * 2.) / w as f32;

        let mut render_lev = UiArena {
            // FIXME: Why does this work with landscape orientation?
            game_x,
            game_y,
            sq_w: sq_sz,
            sq_h: sq_sz,
            texture_cache,
            slide_pc: anim.slide_pc,
            anim_pc: anim.anim_pc,
        };

        render_lev.draw_backdrop();

        render_lev.draw_map(state).await;
    }

    pub async fn draw_map<GameLogic: BaseGameLogic>(
        self: &mut Self, state: &Arena<GameLogic>,
    ) {
        // Coords of first visible tile. Currently always 0,0.
        let (ox, oy) = (0, 0);
        let max_h = 5;
        for h in 0..max_h {
            for (x, y, loc) in state.map_locs() {
                if let Some(ent) = loc.get(h) {
                    self.draw_ent(x - ox, y - oy, ent).await;
                }
            }
        }
    }

    fn draw_backdrop(&self)
    {
        // clear_background_for_current_platform(LIGHTGRAY);

        // draw_text(format!("Level: 1", ).as_str(), 10., 20., 20., DARKGRAY);
    }

    fn alpha_col(col: Color, alpha: f32) -> Color
    {
        Color {a: col.a * alpha, ..col}
    }

    // Draw ent's texture/colour to the screen at specified tile coords.
    // Works out pixel coords given pixel size of arena area in RenderLev.
    pub async fn draw_ent<CustomProps: crate::for_gamedata::BaseCustomProps>(
        self: &mut UiArena<'a>,
        // View coords in map. Relative to first visible tile (currently always the same).
        vx: i16,
        vy: i16,
        // Ent to draw
        obj: &MapObj<CustomProps>,
    ) {
        let visual_props = &obj.visual_props;
        let logical_props = &obj.logical_props;
        let pos = obj.pos();
        let prev_pos = obj.prev_pos();

        let base_px = self.game_x + self.sq_w * vx as f32;
        let base_py = self.game_y + self.sq_h * vy as f32;

        // Used to draw tile smaller than real size. Not used at the moment.
        let pc_size = 1.;

        let dx = pos.x - prev_pos.x;
        let dy = pos.y - prev_pos.y;

        // Switch to using fixed frame throughout from here?
        let slide_in_frame_units = Some(3);
        let slide_fr_pc = if let Some(fixed_frames) = slide_in_frame_units {
            (self.slide_pc * fixed_frames as f32).floor() / fixed_frames as f32
        } else {
            self.slide_pc
        };

        let px = base_px + self.sq_w * (1.-pc_size) / 2. - (dx as f32 * (1.-slide_fr_pc) * self.sq_w);
        let py = base_py + self.sq_h * (1.-pc_size) / 2. - (dy as f32 * (1.-slide_fr_pc) * self.sq_h);
        let w = self.sq_w * pc_size;
        let h = self.sq_h * pc_size;

        if !logical_props.custom_props.is_any_mov() {rsst!(prev_pos == pos)}

        let alpha = 1.;

        if let Some(col) = visual_props.fill {
            draw_rectangle(px, py, w, h, Self::alpha_col(col, alpha));
        }

        if let Some(col) = visual_props.border {
            draw_rectangle_lines(px, py, w, h, 2., Self::alpha_col(col, alpha));
        }

        if visual_props.tex_paths.len() > 0 {
            // TODO: Simplify calc? Prevent anim_pc being 100? Or being 0?
            let tex_frame_idx = (visual_props.tex_paths.len()-1).min((self.anim_pc * visual_props.tex_paths.len() as f32) as usize);
            let tex_path = &visual_props.tex_paths[tex_frame_idx];

            let tex_data: &Texture2D = if let Some(tex_data) = self.texture_cache.get(tex_path) {
                tex_data
            } else {
                self.texture_cache.insert(tex_path.clone(), load_texture_unwrap(tex_path).await);
                self.texture_cache.get(tex_path).unwrap()
            };

            // log::debug!("{:?}, {:?}", logical_props.prev_dir, logical_props.dir);
            let prev_rotation = logical_props.prev_dir.as_angle();
            let curr_rotation = logical_props.dir.as_angle();
            let rotation = prev_rotation + (curr_rotation-prev_rotation)*self.slide_pc;
            draw_texture_ex(
                &tex_data,
                px - w * (visual_props.tex_scale-1.0) / 2.,
                py - h * (visual_props.tex_scale-1.0) / 2.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(w * visual_props.tex_scale, h * visual_props.tex_scale)),
                    rotation,
                    ..Default::default()
                    // TODO: alpha
                },
            );
        }

        if let Some(text) = visual_props.text.clone() {
            let text_col = Self::alpha_col(visual_props.text_col.unwrap_or(DARKGRAY), alpha);
            draw_text(&text, (px + w*0.1).floor(), (py + h*0.6).floor(), 15., text_col);
        }
    }
}
