use macroquad::prelude::*;

use crate::gamedata::BaseGameLogic;

use crate::widget::*;

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
    instr: Node,
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
            y: self.y - self.h * (scale - 1.)/2.,
            w: self.w * scale,
            h: self.h * scale,
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
            border_col: GRAY,
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
    active_idx: Option<usize>,

    fr_pos: FrameCoords,

    dragging: Option<DragOrigin>,
}

impl UiCodingArena
{
    pub fn new() -> Self {
        Self {
            is_coding: false,
            active_idx: None,
            fr_pos: FrameCoords::default(),
            dragging: None,
        }

    }

    pub fn background_col(&self) -> Color {
        LIGHTGRAY
    }

    pub fn border_cols(&self) -> Color {
        if self.is_coding {DARKGRAY} else {SKYBLUE}
    }

    pub fn connector_col(&self) -> Color {
        if self.is_coding {DARKGRAY} else {BLUE}
    }

    pub fn font_col(&self) -> Color {
        DARKGRAY
    }

    fn initialise_frame_coords(&mut self, coding: bool) {
        // Arena
        let arena = PRect {
            x: 0.,
            y: 0.,
            w: screen_height().min(screen_width() * if coding {0.8} else {0.9} ) ,
            h: screen_height(),
        };
        let arena_w = arena.w;

        // Supply
        let supply_x = arena_w;
        let supply_y = 0.;
        let supply_w = screen_width() - arena_w;
        let supply_h = if coding {screen_height() * 0.3} else { 0. };

        // Prog
        let spacing_pc = 0.5;
        let prog_x = arena_w;
        let prog_y = supply_h;
        let prog_w = screen_width() - arena_w;
        let prog_h = screen_height() - supply_h;

        // Prog instrs
        let prog_n = 6.;
        let prog_instr_h = (prog_w * 0.8).min(prog_h / (spacing_pc + prog_n*(1.+spacing_pc)));
        let prog_instr_w = prog_instr_h;
        let prog_instr_spacing =  prog_instr_w * spacing_pc;

        // Supply op
        let flow_n = 2.;
        let supply_op_w_max = (supply_h * 0.8).min(supply_w / (spacing_pc + flow_n*(1.+spacing_pc)));
        let supply_op_w = supply_op_w_max.min(prog_instr_w);
        let supply_op_h = supply_op_w;
        let supply_op_font_sz = supply_op_h * 1.35;
        let supply_op_spacing = supply_op_w * spacing_pc;

        self.is_coding = coding;
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

    pub async fn render<GameLogic: BaseGameLogic>(
            &mut self,
            coding_arena: &mut CodingArena<GameLogic>,
            texture_cache: &mut TextureCache,
            anim: AnimState,
        ) {
        self.active_idx = GameLogic::get_active_idx(coding_arena);
        self.initialise_frame_coords(coding_arena.is_coding());

        self.draw_background(coding_arena);

        if self.is_coding {
            UiArena::render(&coding_arena.init_arena, texture_cache, self.fr_pos.arena, anim).await;
        } else {
            UiArena::render(coding_arena.curr_arena.as_mut().unwrap(), texture_cache, self.fr_pos.arena, anim).await;
        }

        self.draw_prog(&coding_arena.coding);
        if self.is_coding {
            self.draw_supply(&mut coding_arena.coding);
            self.draw_dragging();
        }

        self.interact_subprog(0, 0, &mut coding_arena.coding.prog, true);
        if self.is_coding {
            self.interact_supply(&mut coding_arena.coding);
            self.interact_dragging(&mut coding_arena.coding);
        }
    }

    fn draw_background<GameLogic: BaseGameLogic>(&self, _coding_arena: &mut CodingArena<GameLogic>) {
        // Clear background if necessary.
        crate::ui::clear_background_for_current_platform(self.background_col());

        // Draw lev info. TODO: Move to sep fn
        draw_text(format!("Level: 1", ).as_str(), 10., 20., 20., self.font_col());
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
    fn draw_v_connector(&self, prev: OpCoords, next: OpCoords) {
        let (top_x, top_y) = (prev.x + prev.w/2., prev.y + prev.h);
        let (bottom_x, bottom_y) = (next.x + next.w/2., next.y);
        draw_line(top_x, top_y,  bottom_x, bottom_y,  2., self.connector_col());
    }

    /// Draw open connector from bottom edge of given rect.
    fn draw_v_placeholder(&self, c: OpCoords) {
        let r = c.rect_spacing/6.;
        let (top_x, top_y) = (c.x + c.w/2., c.y + c.h);
        let (centre_x, centre_y) = (top_x, top_y + c.rect_spacing/2.);
        let (join_x, join_y) = (top_x, centre_y - r);
        draw_line(top_x, top_y,  join_x, join_y,  2., self.connector_col());
        draw_circle_lines(centre_x, centre_y, r, 2., self.connector_col());
    }

    /// Draw connector from right edge of rect to left edge of next rect
    fn draw_r_connector(&self, c: OpCoords) {
        let (x,y) = (c.x + c.w, c.y + c.h/2.);
        draw_line(x, y,  x + c.rect_spacing, y,  2., self.connector_col());
    }

    /// Draw supply area and all supply bins
    fn draw_supply(&self, coding: &mut Coding) {
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
                self.drop_to_supply(coding);
            }
        }
    }

    fn interact_supply_op(&mut self, coding: &mut Coding, idx: usize)
    {
        let coords = self.supply_op_coords(idx);

        if self.is_coding {
            if is_mouse_button_pressed(MouseButton::Left) && self.mouse_in(coords) {
                self.drag_supply_op(coding, idx, mouse_position().0 - coords.x, mouse_position().1 - coords.y);
            } else if self.is_dragging_over(coords) && is_mouse_button_released(MouseButton::Left) {
                self.drop_to_supply_bin(coding, idx);
            }
        }
    }

    fn draw_prog(&self, coding: &Coding) {
        draw_rectangle_lines(self.fr_pos.prog_x, self.fr_pos.prog_y, self.fr_pos.prog_w, self.fr_pos.prog_h, 2., self.border_cols());

        if coding.prog.instrs.len() == 0 {
            self.draw_start(0, 0);
        } else {
            self.draw_subprog(0, 0, &coding.prog, true);
        }
    }

    fn draw_start(&self, xidx: usize, yidx: usize)
    {
        let coords = self.prog_instr_coords(xidx, yidx);
        let txt = "...".to_string();
        self.draw_op_rect(coords, self.calculate_op_style(coords, false, false, InstrRef::Prog {idx: yidx}, self.is_droppable_on_prog_instr(0)), &txt);
    }

    /// Draw subprog, either top-level prog, or inside a parent instr. At specified instr coords.
    ///
    /// Recurses between draw_subprog and draw_prog_instr, with the same recursion as interact_subprog.
    fn draw_subprog(&self, subprog_xidx: usize, subprog_yidx: usize, prog: &Prog, v_placeholder: bool) {
        let mut prev_instr_yidx = None;
        let mut instr_yidx = subprog_yidx;
        for node in &prog.instrs {
            self.draw_prog_instr(subprog_xidx, prev_instr_yidx, instr_yidx, node);
            prev_instr_yidx = Some(instr_yidx);
            instr_yidx += node.v_len();
        }
        if v_placeholder && let Some(placeholder_yidx) = prev_instr_yidx {
            let coords = self.prog_instr_coords(subprog_xidx, placeholder_yidx);
            self.draw_v_placeholder(coords);
        }
    }

    /// Draw instr node in program, recursing into subprog if a parent instr.
    fn draw_prog_instr(&self, xidx: usize, prev_yidx: Option<usize>, yidx: usize, node: &Node)
    {
        let coords = self.prog_instr_coords(xidx, yidx);
        let active = Some(yidx) == self.active_idx;

        // TODO: Use idx to calculate droppable more consistently.
        let idx = yidx;
        self.draw_op_rect(coords, self.calculate_op_style(coords, active, true, InstrRef::Prog {idx: yidx}, self.is_droppable_on_prog_instr(idx)), &node.op.to_string());
        if let Some(prev_yidx) = prev_yidx {
            self.draw_v_connector(self.prog_instr_coords(xidx, prev_yidx), coords);
        }

        if node.op.is_parent_instr() {
            self.draw_r_connector(coords);

            let subprog = &node.subnodes.as_ref().unwrap();
            self.draw_subprog(xidx + 1, yidx, subprog, subprog.instrs.len() < node.op.r_connect_max());
        }
    }

    /// Interact program, or subprog inside a parent instr, at specified instr coords.
    ///
    /// Recurses between interact_subprog and interact_prog_instr, with the same recursion as interact_subprog.
    fn interact_subprog(&mut self, subprog_xidx: usize, subprog_yidx: usize, prog: &mut Prog, v_placeholder: bool) {
        let mut instr_yidx = subprog_yidx;
        for idx in 0..prog.instrs.len() {
            self.interact_prog_instr(subprog_xidx, instr_yidx, prog, idx);
            instr_yidx += prog.instrs[idx].v_len();
        }
        if v_placeholder {
            self.interact_prog_instr(subprog_xidx, instr_yidx, prog, prog.instrs.len());
        }
    }

    /// Interact dragging/dropping with an instr in program. Including subprog.
    fn interact_prog_instr(&mut self, xidx: usize, yidx: usize, prog: &mut Prog, idx: usize)
    {
        if self.is_coding {
            let coords = self.prog_instr_coords(xidx, yidx);

            if idx < prog.instrs.len() {
                if is_mouse_button_pressed(MouseButton::Left) && self.mouse_in(coords) {
                    self.drag_prog_instr(prog, idx, mouse_position().0 - coords.x, mouse_position().1 - coords.y);
                } else if self.is_dragging_over(coords) && is_mouse_button_released(MouseButton::Left) {
                    self.drop_to_prog(prog, idx);
                } else {
                    let node: &mut Node  = prog.instrs.get_mut(idx).unwrap();
                    if node.op.is_parent_instr() {
                        let subprog: &mut Prog = node.subnodes.as_mut().unwrap();
                        self.interact_subprog(xidx + 1, yidx, subprog, subprog.instrs.len() < node.op.r_connect_max());
                    }
                }
            } else {
                // self.draw_op_rect(coords, self.calculate_style(coords, active, false, InstrRef::Prog {idx: yidx}, instr), &txt);
                if self.is_dragging_over(coords) && is_mouse_button_released(MouseButton::Left) {
                    self.drop_to_prog(prog, idx);
                }
            }
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
            self.draw_op_rect(coords, OpStyle::dragging(), &instr.op.to_string());
        }
    }

    fn is_droppable_on_supply_bin(&self, idx: usize, op_type: Op) -> bool {
        let coords = self.supply_op_coords(idx);
        match &self.dragging {
            Some(DragOrigin { instr, ..}) => self.is_dragging_over(coords) && instr.op == op_type,
            _ => false,
        }
    }

    fn is_droppable_on_prog_instr(&self, idx: usize) -> bool {
        self.is_dragging_over(self.prog_instr_coords(idx, 0))
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

            if matches!(self.dragging, None) && has_op && self.mouse_in(coords) {
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

    // If dragged op is intersecting a specific op. Including padding.
    fn is_dragging_over(&self, coords: OpCoords) -> bool {
        if let Some(dragging_coords) = self.dragging_op_coords() {
            coords.expand_to(1.5).contains(dragging_coords.middle())
        } else {
            false
        }
    }

    fn mouse_in(&self, coords: OpCoords) -> bool {
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
                instr: Node {op: bin.op, subnodes: if bin.op.is_parent_instr() {Some(Subprog::default())} else {None} },
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
        let node = prog.instrs.remove(idx);
        log::debug!("INFO: Dragging {:?} from prog", node);
        self.dragging = Some(DragOrigin {
            instr: node,
            op_ref: InstrRef::Prog { idx },
            orig_offset_x,
            orig_offset_y
        })
    }

    fn drop_to_supply_bin(&mut self, coding: &mut Coding, idx: usize) {
        if let Some(DragOrigin {instr, ..}) = &self.dragging {
            log::debug!("INFO: Dropping {:?} to supply bin", instr);
            let bin = &mut coding.supply.get_mut(idx).unwrap();
            if bin.op == instr.op {
                bin.curr_count += 1;
                self.dragging = None;
            }
        }
    }

    fn drop_to_supply(&mut self, coding: &mut Coding) {
        // TODO: Handle node with subnodes
        if let Some(DragOrigin {instr: Node{op, ..}, ..}) = self.dragging.clone() {
            log::debug!("INFO: Dropping {:?} to supply", op);
            for bin in &mut coding.supply {
                if bin.op == op {
                    bin.curr_count += 1;
                    break;
                }
            }
            self.dragging = None;
        }
    }

    fn drop_to_prog(&mut self, prog: &mut Prog, idx: usize) {
        if let Some(DragOrigin { instr: node, .. }) = &self.dragging {
            log::debug!("INFO: Dropping {:?} to prog", node);
            prog.instrs.insert(idx, node.clone());
            self.dragging = None;
        }
    }
}
