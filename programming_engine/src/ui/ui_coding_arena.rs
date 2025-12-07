use macroquad::prelude::*;

use crate::gamedata::{BaseGameLogic, BaseGamedata};

use crate::ui::ui_helpers::{was_any_input, was_key_pressed};
use crate::scene::*;

use super::{TextureCache, PRect, AnimState, ui_arena::UiArena};

#[derive(Copy, Clone, PartialEq)]
enum InstrRef {
    Supply {
        idx: usize
    },
    Prog{
        idx: usize,
    },
}

#[derive(Clone)]
struct DragOrigin {
    instr: Instr,
    op_ref: InstrRef,
    orig_offset_x: f32,
    orig_offset_y: f32,
}

// NB: This approaches implementing a UI with nested controls inheriting from a control trait.
#[derive(Default)]
struct FrameCoords {
    arena: PRect,

    supply_x: f32,
    supply_y: f32,
    supply_w: f32,
    supply_h: f32,

    supply_op_w: f32,
    supply_op_h: f32,
    supply_op_font_sz: f32,
    supply_op_spacing: f32,

    // TODO: Rename prog -> prog
    prog_x: f32,
    prog_y: f32,
    prog_w: f32,
    prog_h: f32,

    prog_instr_w: f32,
    prog_instr_h: f32,
    prog_instr_spacing: f32,
}

#[derive(Copy, Clone, Default)]
pub struct OpCoords {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub rect_spacing: f32,
}

impl OpCoords {
    /// Scale down near top of square
    pub fn scaled_down_to(self, scale: f32) -> OpCoords {
        let shrink_in_by = (1. - scale)/2.;
        OpCoords {
            x: self.x + self.w * shrink_in_by,
            y: self.y - self.h * shrink_in_by,
            w: self.w * scale,
            h: self.h * scale,
            rect_spacing: self.rect_spacing,
        }
    }

    /// Expand square to proportion
    pub fn expand_to(self, scale: f32) -> OpCoords {
        OpCoords {
            x: self.x - self.w * (scale - 1.)/2.,
            y: self.y - self.h * (scale - 1.)*0.75,
            w: self.w * scale,
            h: self.h * scale + self.h * (scale -1.)*0.25,
            rect_spacing: self.rect_spacing,
        }
    }

    pub fn expand_up_and_sides(self, scale: f32) -> OpCoords {
        OpCoords {
            x: self.x - self.w * (scale - 1.)/2.,
            y: self.y - self.h * (scale - 1.)*0.75,
            w: self.w + self.w * (scale - 1.),
            h: self.h + self.h * (scale - 1.)*0.75,
            rect_spacing: self.rect_spacing,
        }
    }

    pub fn expand_down_and_sides(self, scale: f32) -> OpCoords {
        OpCoords {
            x: self.x - self.w * (scale - 1.)/2.,
            y: self.y,
            w: self.w + self.w * (scale - 1.),
            h: self.h + self.h * (scale - 1.)/2.,
            rect_spacing: self.rect_spacing,
        }
    }

    fn middle(self) -> (f32, f32) {
        (self.x + self.w/2., self.y + self.h/2.)
    }

    fn contains(self, pt: (f32, f32)) -> bool {
        (self.x..self.x+self.w).contains(&pt.0) && (self.y..self.y+self.h).contains(&pt.1)
    }
}

struct LineStyle {
    border_width: f32,
    border_col: Color,
}

struct OpStyle {
    border_width: f32,
    border_col: Color,
    fill_col: Color,
    scale: f32,
}

impl OpStyle {
    pub fn coding() -> Self {
        Self {
            border_width: 2.,
            border_col: DARKGRAY,
            fill_col: WHITE,
            scale: 1.0,
        }
    }

    pub fn dragging() -> Self {
        Self {
            border_width: 2.,
            border_col: DARKGRAY,
            // Covers over background when dragging
            fill_col: Color {r: 1.0, g: 1.0, b: 1.0, a:0.5 },
            scale: 1.0,
        }
    }

    pub fn highlighted(orig_style: Self) -> Self {
        Self {
            border_width: 4.,
            border_col: YELLOW,
            ..orig_style
        }
    }

    pub fn drag_origin(orig_style: Self) -> Self {
        Self {
            border_width: 6.,
            border_col: BEIGE,
            ..orig_style
        }
    }

    pub fn coding_placeholder(background_col: Color) -> Self {
        Self {
            border_width: 1.,
            border_col: DARKGRAY,
            // Covers over excess connecting line
            fill_col: background_col,
            scale: 1.0,
        }
    }

    pub fn running() -> Self {
        Self {
            border_width: 8.,
            border_col: SKYBLUE,
            fill_col: WHITE,
            scale: 1.0,
        }
    }

    pub fn running_active(orig_style: Self) -> Self {
        Self {
            border_col: RED,
            ..orig_style
        }
    }

    pub fn running_placeholder() -> Self {
        Self {
            ..Self::running()
        }
    }
}

// NB Original intention was to split this into a parent struct and UiCoding struct.
pub struct UiCodingArena {
    is_coding: bool,
    is_won: bool,
    is_dead: bool,
    active_idx: Option<usize>,

    fr_pos: FrameCoords,

    dragging: Option<DragOrigin>,
}

impl UiCodingArena
{
    pub fn new() -> Self {
        macroquad::rand::srand(12345);
        Self {
            is_coding: false,
            is_won: false,
            is_dead: false,

            active_idx: None,

            fr_pos: FrameCoords::default(),

            dragging: None,
        }
    }

    fn background_col(&self) -> Color {
        if self.is_won {
            YELLOW
        } else if self.is_dead {
            BLACK
        } else {
            LIGHTGRAY
        }
    }

    fn border_cols(&self) -> Color {
        if self.is_coding {DARKGRAY} else {SKYBLUE}
    }

    fn connector_col(&self, _highlight: bool) -> LineStyle {
        if self.is_coding {
            LineStyle {
                border_col: DARKGRAY,
                border_width: 2.,
            }
        } else {
            LineStyle {
                border_col: BLUE,
                border_width: 2.,
            }
        }
    }

    fn placeholder_col(&self, highlight: bool) -> LineStyle {
        if self.is_coding {
            if highlight {
                LineStyle {
                    border_col: YELLOW,
                    border_width: 2.,
                }
            } else {
                LineStyle {
                    border_col: DARKGRAY,
                    border_width: 2.,
                }
            }
        } else {
            LineStyle {
                border_col: BLUE,
                border_width: 2.,
            }
        }
    }

    fn font_col(&self) -> Color {
        DARKGRAY
    }

    fn prog_instr_sz(&self, prog_w: f32, prog_h: f32, spacing_pc: f32, prog_n: f32) -> f32 {
        // Space for 6 instructions, 7 gaps, and half a 7th instruction (for placeholder)
        (prog_w * 0.8).min(prog_h / (spacing_pc + prog_n*(1.+spacing_pc) + 0.5))
    }

    fn initialise_frame_coords(&mut self, coding: CodingRunningPhase, prog_n: usize) {
        self.is_coding = coding == CodingRunningPhase::Coding;
        self.is_won = coding == CodingRunningPhase::Won;
        self.is_dead = coding == CodingRunningPhase::Died;

        // Arena
        let arena = PRect {
            x: 0.,
            y: 0.,
            w: screen_height().min(screen_width() * if self.is_coding {0.8} else {0.9} ) ,
            h: screen_height(),
        };
        let arena_w = arena.w;

        // Supply
        let supply_x = arena_w;
        let supply_y = 0.;
        let supply_w = screen_width() - arena_w;
        let supply_h = if self.is_coding {screen_height() * 0.3} else { 0. };

        // Prog
        let spacing_pc = 0.5;
        let prog_x = arena_w;
        let prog_y = supply_h;
        let prog_w = screen_width() - arena_w;
        let prog_h = screen_height() - supply_h;

        // Prog instrs
        let prog_n = prog_n.max(6) as f32;
        let prog_instr_h = self.prog_instr_sz(prog_w, prog_h, spacing_pc, prog_n);
        let prog_instr_w = prog_instr_h;
        let prog_instr_spacing =  prog_instr_w * spacing_pc;

        // Supply op
        let flow_n = 2.;
        let supply_op_w_max = (supply_h * 0.8).min(supply_w / (spacing_pc + flow_n*(1.+spacing_pc)));
        let supply_op_w = supply_op_w_max.min(self.prog_instr_sz(prog_w, prog_h, spacing_pc, 6.));
        let supply_op_h = supply_op_w;
        let supply_op_font_sz = supply_op_h * 1.35;
        let supply_op_spacing = supply_op_w * spacing_pc;

        self.fr_pos = FrameCoords {
            arena,

            supply_x,
            supply_y,
            supply_w,
            supply_h,

            supply_op_w,
            supply_op_h,
            supply_op_font_sz,
            supply_op_spacing,

            prog_x,
            prog_y,
            prog_w,
            prog_h,

            prog_instr_w,
            prog_instr_h,
            prog_instr_spacing,
        }

    }

    pub fn advance<GameData: BaseGamedata>(&self, coding_arena: &mut CodingArena<GameData::GameLogic>) {
        use crate::ui::KeyType::*;
        // TODO: Want to combine KeyType into InputCmd. Have one function to turn
        // keyboard into that. And have UI generate those based on mouse. Then interpret
        // which of Ok, Cancel etc means what in the scene.
        // That is more like what I had before but makes sense with those values...
        match coding_arena.phase {
            CodingRunningPhase::Coding => {
                if matches!(was_key_pressed(), Some(Ok | Normal)) ||
                    is_mouse_button_pressed(MouseButton::Left) && self.mouse_in_rect(self.fr_pos.arena) {
                    _ = coding_arena.advance(InputCmd::NextPhase);
                    }
            },
            CodingRunningPhase::Died => {
                if was_any_input() {
                    _ = coding_arena.advance(InputCmd::NextPhase);
                }
            },
            CodingRunningPhase::Won => {
                if was_any_input() {
                    // TODO: This one needs to be propagated
                    _ = coding_arena.advance(InputCmd::NextPhase);
                }
            },
            CodingRunningPhase::Running => {
                // While executing
                if matches!(was_key_pressed(), Some(Escape)) ||
                    is_mouse_button_pressed(MouseButton::Left) && !self.mouse_in_rect(self.fr_pos.arena) {
                        // Cancel
                        // TODO: Maybe pause
                    _ = coding_arena.advance(InputCmd::NextPhase);
                } else if matches!(was_key_pressed(), Some(Ok | Normal)) {
                    // Need to reset tick counter here?
                    _ = coding_arena.advance(InputCmd::Tick);
                }
            }
        }
    }

    pub async fn do_frame<GameData: BaseGamedata>(
            &mut self,
            coding_arena: &mut CodingArena<GameData::GameLogic>,
            texture_cache: &mut TextureCache,
            anim: AnimState,
            game_state: &GameData,
        ) {
        self.advance::<GameData>(coding_arena);

        // TODO: Get prog from arena or from coding pane as appropriate?
        self.active_idx = GameData::GameLogic::get_active_idx(coding_arena);
        self.initialise_frame_coords(coding_arena.phase, coding_arena.coding.prog.v_len());

        crate::ui::clear_background_for_current_platform(self.background_col());

        if self.is_coding {
            UiArena::render(&coding_arena.init_arena, texture_cache, self.fr_pos.arena, anim).await;
        } else {
            UiArena::render(coding_arena.curr_arena.as_mut().unwrap(), texture_cache, self.fr_pos.arena, anim).await;
        }

        self.draw_prog(&coding_arena.coding);
        if self.is_coding {
            self.draw_supply(&mut coding_arena.coding, game_state);
            self.draw_dragging();
        }

        self.interact_prog(&mut coding_arena.coding);
        if self.is_coding {
            self.interact_supply(&mut coding_arena.coding);
            self.interact_dragging(&mut coding_arena.coding);
        }
    }

    fn supply_rect(&self) -> PRect {
        PRect {
            x: self.fr_pos.supply_x,
            y: self.fr_pos.supply_y,
            w: self.fr_pos.supply_w,
            h: self.fr_pos.supply_h,
        }
    }

    /// Draw a prog or supply instr outline and content, at given coords in given style.
    fn draw_op_rect(&self, coords: OpCoords, style: OpStyle, txt: &str) {
        let c = coords.scaled_down_to(style.scale);

        draw_rectangle(c.x, c.y, c.w, c.h, style.fill_col);
        draw_rectangle_lines(c.x, c.y, c.w, c.h, style.border_width, style.border_col);
        let font_sz = c.w * if txt.len() <= 1 { 1.35 } else { 0.5 };
        draw_text(txt, c.x + 0.2 * c.w, c.y + 0.85 * c.h, font_sz, DARKGRAY);
    }

    /// Draw connector from bottom edge of given rect to top edge of next rect
    fn draw_v_connector(&self, prev: OpCoords, next: OpCoords, highlight_above: bool) {
        let (top_x, top_y) = (prev.x + prev.w/2., prev.y + prev.h);
        let (bottom_x, bottom_y) = (next.x + next.w/2., next.y);
        let line_style = self.connector_col(highlight_above);
        draw_line(top_x, top_y,  bottom_x, bottom_y,  line_style.border_width, line_style.border_col);
    }

    /// Draw open connector from bottom edge of given rect.
    fn draw_v_placeholder_below(&self, c: OpCoords, highlight: bool) {
        let r = c.rect_spacing/6.;
        let (top_x, top_y) = (c.x + c.w/2., c.y + c.h);
        let (centre_x, centre_y) = (top_x, top_y + c.rect_spacing/2.);
        let (join_x, join_y) = (top_x, centre_y - r);
        let line_style = self.placeholder_col(highlight);
        draw_line(top_x, top_y,  join_x, join_y,  line_style.border_width, line_style.border_col);
        draw_circle_lines(centre_x, centre_y, r, line_style.border_width, line_style.border_col);
    }

    /// Draw open connector from bottom edge of given rect.
    fn draw_r_placeholder_right(&self, xidx: usize, yidx: usize) {
        let parent_coords = self.prog_instr_coords(xidx, yidx);
        let highlight = false;
        self.draw_r_connector(parent_coords, highlight);

        self.draw_placeholder_rect(xidx + 1, yidx);
    }

    /// Draw connector from right edge of rect to left edge of next rect
    fn draw_r_connector(&self, c: OpCoords, highlight: bool) {
        let (x,y) = (c.x + c.w, c.y + c.h/2.);
        let line_style = self.connector_col(highlight);
        draw_line(x, y,  x + c.rect_spacing, y, line_style.border_width, line_style.border_col);
    }

    /// Draw supply area and all supply bins
    fn draw_supply<GameData: BaseGamedata>(&self, coding: &mut Coding, game_state: &GameData,) {
        draw_text(game_state.get_level_str().as_str(), self.fr_pos.supply_x + 10., 20., 20., self.font_col());

        for (idx, bin) in coding.supply.iter().enumerate() {
            self.draw_supply_op(idx, bin);
        }

        draw_rectangle_lines(self.fr_pos.supply_x, self.fr_pos.supply_y, self.fr_pos.supply_w, self.fr_pos.supply_h+1., 2., self.border_cols());
    }

    fn draw_supply_op(&self, idx: usize, bin: &Bin)
    {
        let coords = self.supply_op_coords(idx);
        let active = false;
        let has_op = bin.curr_count > 0;
        self.draw_op_rect(coords, self.calculate_op_style(coords, active, has_op, InstrRef::Supply {idx}, self.is_droppable_on_supply_bin(idx, bin.op)), &bin.op.to_string());

        // Draw count
        let count_txt = format!("{}/{}", bin.curr_count, bin.orig_count);
        draw_text(&count_txt, coords.x + 0.5*self.fr_pos.supply_op_w, coords.y+1.25*self.fr_pos.supply_op_h, self.fr_pos.supply_op_font_sz * 0.25, self.font_col());
    }

    /// Interact supply area and all supply bins
    fn interact_supply(&mut self, coding: &mut Coding) {
        for idx in 0..coding.supply.len() {
            self.interact_supply_op(coding, idx);
        }

        if self.mouse_in_rect(self.supply_rect()) {
            if is_mouse_button_released(MouseButton::Left) {
                self.drop_drag_to_supply(coding);
            }
        }
    }

    fn interact_supply_op(&mut self, coding: &mut Coding, idx: usize)
    {
        let coords = self.supply_op_coords(idx);

        if self.is_coding {
            if is_mouse_button_pressed(MouseButton::Left) && self.mouse_in_coords(coords) {
                self.drag_supply_op(coding, idx, mouse_position().0 - coords.x, mouse_position().1 - coords.y);
            } else if self.is_droppable_on_coords(coords.expand_to(1.5)) && is_mouse_button_released(MouseButton::Left) {
                self.drop_to_supply_bin(coding, idx);
            }
        }
    }

    fn draw_prog(&self, coding: &Coding) {
        draw_rectangle_lines(self.fr_pos.prog_x, self.fr_pos.prog_y, self.fr_pos.prog_w, self.fr_pos.prog_h, 2., self.border_cols());

        if coding.prog.instrs.len() == 0 {
            // Draw "Start" instr.
            self.draw_placeholder_rect(0, 0);
        } else {
            self.draw_subprog(0, 0, &coding.prog, true);
        }
    }

    fn draw_placeholder_rect(&self, xidx: usize, yidx: usize) {
        let coords = self.prog_instr_coords(xidx, yidx);
        let txt = "...".to_string();
        self.draw_op_rect(coords, self.calculate_op_style(coords, false, false, InstrRef::Prog {idx: 0}, self.is_droppable_onto_prog_instr(xidx, yidx)), &txt);
    }

    /// Draw subprog, either top-level prog, or inside a parent instr. At specified instr coords.
    ///
    /// Recurses between draw_subprog and draw_prog_instr, with the same recursion as interact_subprog.
    fn draw_subprog(&self, subprog_xidx: usize, subprog_yidx: usize, prog: &Prog, room_for_more: bool) {
        let mut prev_instr_yidx = None;
        let mut instr_yidx = subprog_yidx;

        for instr in &prog.instrs {
            self.draw_prog_instr(subprog_xidx, prev_instr_yidx, instr_yidx, instr, room_for_more);
            prev_instr_yidx = Some(instr_yidx);
            instr_yidx += instr.v_len();
        }

        if room_for_more && let Some(placeholder_yidx) = prev_instr_yidx {
            let coords = self.prog_instr_coords(subprog_xidx, placeholder_yidx);
            let highlight = self.is_pickable_from_placeholder_below(subprog_xidx, placeholder_yidx) || self.is_droppable_on_placeholder_below(subprog_xidx, placeholder_yidx);
            self.draw_v_placeholder_below(coords, highlight);
        }
    }

    /// Draw instr node in program, recursing into subprog if a parent instr.
    fn draw_prog_instr(&self, xidx: usize, prev_yidx: Option<usize>, yidx: usize, instr: &Instr, room_for_more: bool)
    {
        let coords = self.prog_instr_coords(xidx, yidx);
        let active = Some(yidx) == self.active_idx;
        let highlight_above = room_for_more && self.is_droppable_before_prog_instr(xidx, yidx);

        self.draw_op_rect(coords, self.calculate_op_style(coords, active, true, InstrRef::Prog {idx: yidx}, highlight_above), &instr.to_string());

        if let Some(connector_yidx) = prev_yidx {
            self.draw_v_connector(self.prog_instr_coords(xidx, connector_yidx), coords, highlight_above);
        }

        if let Instr::Parent(_, subprog) = &instr {
            let highlight = false;

            if subprog.instrs.len() > 0 {
                self.draw_r_connector(coords, highlight);
                self.draw_subprog(xidx + 1, yidx, subprog, subprog.instrs.len() < instr.r_connect_max());
            } else {
                self.draw_r_placeholder_right(xidx, yidx);
            }
        }
    }

    fn interact_prog(&mut self, coding: &mut Coding)
    {
        if self.is_coding
        {
            // Specially treat START or first instr as accepting a drop anywhere?
            self.interact_prog_instr(0, 0, &mut coding.prog, 0, true);

            // Deal with all subsequent instr normally. Ie. Dropped onto top or bottom of instr for before or after.
            self.interact_subprog(0, 0, &mut coding.prog, true);
        }
    }

    /// Interact program, or subprog inside a parent instr, at specified instr coords.
    ///
    /// Recurses between interact_subprog and interact_prog_instr, with the same recursion as interact_subprog.
    ///
    /// If idx is equal to prog len, treats an instr-rect sized placeholder at that index. Currently only used
    /// when both are 0.
    fn interact_subprog(&mut self, subprog_xidx: usize, subprog_yidx: usize, prog: &mut Prog, room_for_more: bool) {
        let mut prev_instr_yidx = None;
        let mut instr_yidx = subprog_yidx;
        for idx in 0..prog.instrs.len() {
            self.interact_prog_instr(subprog_xidx, instr_yidx, prog, idx, room_for_more);
            if idx >= prog.instrs.len() {
                // TODO: More explicltly deal with prog changing while recursing.
                // Either use calculations based on original. Or bail out when finding first pick-up.
                break;
            }
            prev_instr_yidx = Some(instr_yidx);
            instr_yidx += prog.instrs[idx].v_len();
        }
        if room_for_more && let Some(placeholder_yidx) = prev_instr_yidx {
            self.interact_placeholder_below(subprog_xidx, placeholder_yidx, prog, prog.instrs.len());
        }
    }

    /// Interact dragging/dropping with an instr in program. Including subprog.
    fn interact_prog_instr(&mut self, xidx: usize, yidx: usize, prog: &mut Prog, idx: usize, room_for_more: bool)
    {
        // TODO: Better guards for altered program.
        let coords = self.prog_instr_coords(xidx, yidx);
        if self.is_pickable_from_prog_instr(xidx, yidx) && is_mouse_button_pressed(MouseButton::Left) {
            if idx < prog.instrs.len() {
                self.drag_prog_instr(prog, idx, mouse_position().0 - coords.x, mouse_position().1 - coords.y);
            }
        } else if room_for_more && self.is_droppable_before_prog_instr(xidx, yidx) && is_mouse_button_released(MouseButton::Left) {
            if idx <= prog.instrs.len() {
                self.drop_to_prog(prog, idx);
            }
        } else {
            // Recurse to detect interaction in subprog
            if idx < prog.instrs.len() {
                let instr: &mut Instr  = prog.instrs.get_mut(idx).unwrap();
                if let Instr::Parent(instr, subprog) = instr {
                    let subprog_room_for_more = subprog.instrs.len() < instr.r_connect_max();
                    if subprog.instrs.len() > 0 {
                        self.interact_subprog(xidx + 1, yidx, subprog, subprog_room_for_more);
                    } else {
                        self.interact_prog_instr(xidx + 1, yidx, subprog, 0, subprog_room_for_more);
                    }
                }
            }
        }
    }

    fn interact_placeholder_below(&mut self, xidx: usize, yidx: usize, prog: &mut Prog, idx: usize)
    {
        if self.is_droppable_on_placeholder_below(xidx, yidx) && is_mouse_button_released(MouseButton::Left) {
            self.drop_to_prog(prog, idx);
        }
    }

    fn interact_dragging(&mut self, coding: &mut Coding) {
        // If mouse is released anywhere else, cancel drag, return dragged op to its origin.
        // Use "!is_mouse_button_down" not "is_mouse_buttom_released" to ensure dragging is stopped.
        if !is_mouse_button_down(MouseButton::Left) {
            match &self.dragging {
                Some(DragOrigin { instr:_instr, op_ref: InstrRef::Supply { idx }, ..}) => {
                    log::debug!("INFO: Cancelling drag. Returning {:?} to supply idx {:?}", _instr, idx);
                    self.drop_to_supply_bin(coding, *idx);
                },
                Some(DragOrigin { instr: _instr, op_ref: InstrRef::Prog { idx }, ..}) => {
                    log::debug!("INFO: Cancelling drag. Returning {:?} to supply idx {:?}", _instr, idx);
                    // TODO: !!
                    self.drop_to_prog(&mut coding.prog, *idx);
                },
                None => (),
            }
        }
    }

    fn draw_dragging(&self)
    {
        if let Some(DragOrigin{instr, ..}) = &self.dragging {
            let coords = self.dragging_op_coords().unwrap();
            self.draw_op_rect(coords, OpStyle::dragging(), &instr.to_string());
        }
    }

    fn is_droppable_on_supply_bin(&self, idx: usize, op_type: Opcode) -> bool {
        let coords = self.supply_op_coords(idx);
        match &self.dragging {
            Some(DragOrigin { instr, ..}) => self.is_droppable_on_coords(coords.expand_to(1.5)) && instr.has_opcode(op_type),
            _ => false,
        }
    }

    fn calculate_op_style(&self, coords: OpCoords, active: bool, has_op: bool, instr_ref: InstrRef, droppable: bool) -> OpStyle
    {
        let drag_origin = matches!(self.dragging, Some(DragOrigin{op_ref: orig_op_ref, ..}) if orig_op_ref == instr_ref);

        let mut style;
        if self.is_coding {
            style = if has_op {
                OpStyle::coding()
            } else {
                OpStyle::coding_placeholder(self.background_col())
            };

            if matches!(self.dragging, None) && has_op && self.mouse_in_coords(coords) {
                // Available to pick up
                style = OpStyle::highlighted(style);
            }

            if droppable {
                // Available to drop onto
                style = OpStyle::highlighted(style);
            } else if drag_origin {
                // Where drop will snap back to
                style = OpStyle::drag_origin(style);
            }
        } else {
            style = if has_op {
                OpStyle::running()
            } else {
                OpStyle::running_placeholder()
            };

            if active {
                style = OpStyle::running_active(style);
            }
        };

        style
    }

    fn supply_op_coords(&self, idx: usize) -> OpCoords {
        let fdx = idx as f32;
        OpCoords {
            x: self.fr_pos.supply_x + self.fr_pos.supply_op_spacing + fdx * (self.fr_pos.supply_op_w + self.fr_pos.supply_op_spacing),
            y: self.fr_pos.supply_y + self.fr_pos.supply_h/2. - self.fr_pos.supply_op_h*0.6,
            w: self.fr_pos.supply_op_h,
            h: self.fr_pos.supply_op_h,
            rect_spacing: 0.,
        }
    }

    fn prog_instr_coords(&self, xidx: usize, yidx: usize) -> OpCoords {
        let (xfdx, yfdx) = (xidx as f32, yidx as f32);
        let x = self.fr_pos.prog_x + self.fr_pos.prog_instr_spacing + xfdx * (self.fr_pos.prog_instr_h + self.fr_pos.prog_instr_spacing);
        let y = self.fr_pos.prog_y + self.fr_pos.prog_instr_spacing + yfdx * (self.fr_pos.prog_instr_h + self.fr_pos.prog_instr_spacing);

        OpCoords {x, y, w: self.fr_pos.prog_instr_w, h: self.fr_pos.prog_instr_h, rect_spacing: self.fr_pos.prog_instr_spacing}
    }

    fn dragging_op_coords(&self) -> Option<OpCoords> {
        match &self.dragging {
            Some(DragOrigin{orig_offset_x, orig_offset_y,..}) => {
                let (mx, my) = mouse_position();
                let (x,y) = (mx - orig_offset_x, my - orig_offset_y);
                Some(OpCoords {x, y, w:self.fr_pos.prog_instr_w, h:self.fr_pos.prog_instr_h, rect_spacing: 0.})
            },
            _ => None,
        }
    }

    // Including spacing between instr and half the space for an instr below.
    fn placeholder_below_coords(&self, xidx: usize, yidx: usize) -> OpCoords {
        let instr_coords = self.prog_instr_coords(xidx, yidx);
        OpCoords {
            x: instr_coords.x,
            y: instr_coords.y + instr_coords.h,
            w: instr_coords.w,
            h: instr_coords.rect_spacing + instr_coords.h*0.2,
            rect_spacing: instr_coords.rect_spacing,
        }
    }

    fn is_pickable_from_prog_instr(&self, xidx: usize, yidx: usize) -> bool {
        self.is_pickable_from_coords(self.prog_instr_coords(xidx, yidx))
    }

    fn is_pickable_from_placeholder_below(&self, xidx: usize, yidx: usize) -> bool {
        self.is_pickable_from_coords(self.placeholder_below_coords(xidx, yidx))
    }

    fn is_pickable_from_coords(&self, coords: OpCoords) -> bool {
        self.dragging.is_none() && self.mouse_in_coords(coords)
    }

    fn is_droppable_before_prog_instr(&self, xidx: usize, yidx: usize) -> bool {
        self.is_droppable_on_coords(self.prog_instr_coords(xidx, yidx).expand_up_and_sides(1.5))
    }

    fn is_droppable_onto_prog_instr(&self, xidx: usize, yidx: usize) -> bool {
        self.is_droppable_on_coords(self.prog_instr_coords(xidx, yidx).expand_up_and_sides(1.5))
    }

    fn is_droppable_on_placeholder_below(&self, xidx: usize, yidx: usize) -> bool {
        self.is_droppable_on_coords(self.placeholder_below_coords(xidx, yidx).expand_down_and_sides(1.5))
    }

    // If dragged op is intersecting a specific op. Including padding.
    fn is_droppable_on_coords(&self, op_coords: OpCoords) -> bool {
        if let Some(dragging_coords) = self.dragging_op_coords() {
            op_coords.contains(dragging_coords.middle())
        } else {
            false
        }
    }

    fn mouse_in_coords(&self, coords: OpCoords) -> bool {
        coords.contains(mouse_position())
    }

    fn mouse_in_rect(&self, rect: PRect) -> bool {
        OpCoords {
            x: rect.x,
            y: rect.y,
            w: rect.w,
            h: rect.h,
            rect_spacing: 0.,
        }.contains(mouse_position())
    }

    fn drag_supply_op(&mut self, coding: &mut Coding, idx: usize, orig_offset_x: f32, orig_offset_y: f32) {
        // TODO: Test not already dragging?
        let bin = &mut coding.supply.get_mut(idx).unwrap();
        log::debug!("INFO: Dragging {:?} from supply", bin.op);
        self.dragging = if bin.curr_count > 0 {
            bin.curr_count -= 1;
            Some(DragOrigin {
                instr: Instr::from_opcode(bin.op),
                op_ref: InstrRef::Supply { idx },
                orig_offset_x,
                orig_offset_y
            })
        } else {
            None
        }
    }

    fn drag_prog_instr(&mut self, prog: &mut Prog, idx: usize, orig_offset_x: f32, orig_offset_y: f32) {
        // TODO: Test not already dragging?
        let instr = prog.instrs.remove(idx);
        log::debug!("INFO: Dragging {:?} from prog", instr);
        self.dragging = Some(DragOrigin {
            instr,
            op_ref: InstrRef::Prog { idx },
            orig_offset_x,
            orig_offset_y
        })
    }

    fn drop_to_supply_bin(&mut self, coding: &mut Coding, idx: usize) {
        if let Some(DragOrigin {instr, ..}) = &self.dragging {
            log::debug!("INFO: Dropping {:?} to supply bin", instr);
            let bin = &mut coding.supply.get_mut(idx).unwrap();
            if instr.has_opcode(bin.op) {
                bin.curr_count += 1;
                if let Instr::Parent(_, subprog) = &instr {
                    for subnode in &mut subprog.instrs.clone() {
                        let supply = &mut coding.supply;
                        self.drop_node_to_supply(supply, subnode.clone());
                    }
                }
                self.dragging = None;
            }
        }
    }

    fn drop_node_to_supply(&mut self, supply: &mut Vec<Bin>, instr: Instr) {
        log::debug!("INFO: Dropping {:?} to supply", instr);
        for bin in &mut *supply {
            if instr.has_opcode(bin.op) {
                bin.curr_count += 1;
                break;
            }
        }
        if let Instr::Parent(_, subprog) = &instr {
            for instr in subprog.instrs.clone() {
                self.drop_node_to_supply(supply, instr);
            }
        }
    }

    fn drop_drag_to_supply(&mut self, coding: &mut Coding) {
        if let Some(DragOrigin {instr, ..}) = self.dragging.clone() {
            self.drop_node_to_supply(&mut coding.supply, instr);
            self.dragging = None;
        }
    }

    fn drop_to_prog(&mut self, prog: &mut Prog, idx: usize) {
        if let Some(DragOrigin { instr, .. }) = &self.dragging {
            log::debug!("INFO: Dropping {:?} to prog", instr);
            prog.instrs.insert(idx, instr.clone());
            self.dragging = None;
        }
    }
}
