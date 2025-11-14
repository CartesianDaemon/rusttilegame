use std::collections::HashMap;

use macroquad::prelude::*;
use assrt::rsst;

use crate::gamedata::BaseMovementLogic;

use crate::pane::*;
use crate::pane_arena::MapObj;
use crate::map_coords::CoordDelta;

use super::UiSplit;

type TextureCache = HashMap<String, Texture2D>;

/// Seems like clear_background is mandatory for wasm, broken for windows.
/// Don't know about linux or android.
/// Don't know if fixed in more recent macroquad.
pub fn clear_background_for_current_platform(color: Color) {
    match std::env::consts::OS {
        "windows" => (),
        _ => clear_background(color),
    }
}

/// NB: Split Render up into a struct or trait implemented for each
/// Pane. Or Screen of multiple Panes.
/// NB: Or have ProgpuzzView (elsewhere called screen), which might
/// delegate drawing to one preimplemented View, or multuple tiled.
/// Which combine Render code in a View struct or trait specialised
/// for the relevant Widget/Gamedata.
/// And Gamedata contains one or more Widgets.
/// And Views may display data from one Widget, or from more than one.
pub struct UiBase {
    /// Loaded textures
    texture_cache: TextureCache,
    render_split: UiSplit,
}

impl UiBase {
    pub fn new() -> UiBase {
        UiBase {
            texture_cache: HashMap::new(),
            render_split: UiSplit::new(),
        }
    }

    /// Draw current gameplay to screen.
    /// TODO: Avoid passing slide and anim through so many layers? Add to struct?
    pub async fn draw_frame<MovementLogic: BaseMovementLogic>(
        &mut self, state: &mut Pane<MovementLogic>, slide_pc: f32, anim_pc: f32
    ) {
        match state {
            Pane::Arena(state) => {
                UiArena::render(state, &mut self.texture_cache, slide_pc, anim_pc, state.map_w(), state.map_h()).await;
            }
            Pane::Splash(state) => {
                let _r = UiSplash::render(state);
            }
            Pane::Split(state) => {
                self.render_split.render(state);
            }
        }
    }
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

/// Render state for one frame of level
// NB: Good to move Input relating to map movement in here.
//#[derive(Clone)]
pub struct UiArena<'a> {
    // COORDS FOR CURRENT FRAME. In gl units which are pixels.
    // Distance from edge of drawing surface to arena area
    offset_x: f32,
    // Distance from edge of drawing surface to arena area
    offset_y: f32,
    // Size of each tile
    sq_w: f32,
    sq_h: f32,
    texture_cache: &'a mut TextureCache,
    slide_pc: f32,
    anim_pc: f32,
}

impl<'a> UiArena<'a> {
    pub async fn render<MovementLogic: BaseMovementLogic>(
        state: &Arena<MovementLogic>,
        texture_cache: &mut TextureCache,
        slide_pc: f32, anim_pc: f32,
        w: u16, h: u16,
    ) {
        assert_eq!(w, h);
        let game_size = screen_width().min(screen_height());
        let offset_y = (screen_height() - game_size) / 2. + 10.;

        let mut render_lev = UiArena {
            // FIXME: Why does this work with landscape orientation?
            offset_x: (screen_width() - game_size) / 2. + 10.,
            offset_y: (screen_height() - game_size) / 2. + 10.,
            sq_w: (screen_height() - offset_y * 2.) / w as f32,
            sq_h: (screen_height() - offset_y * 2.) / w as f32,
            texture_cache,
            slide_pc,
            anim_pc,
        };

        render_lev.draw_backdrop();

        render_lev.draw_map(state).await;
    }

    pub async fn draw_map<MovementLogic: BaseMovementLogic>(
        self: &mut Self, state: &Arena<MovementLogic>,
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
        clear_background_for_current_platform(LIGHTGRAY);

        draw_text(format!("Level: 1", ).as_str(), 10., 20., 20., DARKGRAY);
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

        let base_px = self.offset_x + self.sq_w * vx as f32;
        let base_py = self.offset_y + self.sq_h * vy as f32;

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

            let rotation = match logical_props.dir {
                CoordDelta{dx:0, dy:-1} => std::f32::consts::PI / 2.,
                CoordDelta{dx:1, dy: 0} => std::f32::consts::PI,
                CoordDelta{dx:0, dy: 1} => std::f32::consts::PI * 1.5,
                _ => 0.
            };
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

// Render state for one frame of "Show text, press enter to continue"
// Currently not needing any global graphics state
// NB: Good to move Input relating to "continue" in here.
pub struct UiSplash {
}

impl UiSplash
{
    pub fn render(splash: &Splash) {
        clear_background(WHITE);

        let text = &splash.splash_text;
        let font_size = 30.;
        let text_size = measure_text(text, None, font_size as _, 1.0);

        draw_text(
            text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. + text_size.height / 2.,
            font_size,
            DARKGRAY,
        );

        let avatar_x = screen_width() * 0.25;
        let avatar_w = f32::min(screen_height(), screen_width()) / 10.;
        let avatar_h = avatar_w * 0.75;
        let text_x = avatar_x + avatar_w + 20.;
        let mut next_y = 40.;
        let entry_spacing = 20.;
        for (idx, dialogue_line) in (&splash.dialogue.entries).iter().enumerate() {
            let font_size = 25.;
            let _tex_path = &dialogue_line.tex_path;
            let text = &dialogue_line.text;

            let avatar_y = next_y;

            draw_rectangle_lines(avatar_x, avatar_y, avatar_w, avatar_h, 2., if idx%2>0 {GREEN} else {BLUE} );

            let text_size = measure_text(text, None, font_size as _, 1.0);

            // Bottom of text level with given y coordinate
            let text_y = avatar_y + 5. + text_size.height;

            draw_text(
                text,
                text_x,
                text_y,
                font_size,
                DARKGRAY,
            );

            next_y += f32::max(avatar_h, text_size.height) + entry_spacing;
        }
    }
}
