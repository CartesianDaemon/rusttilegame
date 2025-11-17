use macroquad::prelude::*;

use crate::gamedata::BaseGameLogic;

use crate::widget::*;

use super::{TextureCache, PRect, AnimState, ui_arena::UiArena};

#[derive(Copy, Clone)]
enum InstrRef {
    Supply {
        idx: usize
    },
    Prog{
        idx: usize,
    },
}

// TODO: CodingArena.dragging to Option(DragInfo).
enum Dragging {
    No,
    Yes{
        op: Op,
        op_ref: InstrRef,
        orig_offset_x: f32,
        orig_offset_y: f32,
    },
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
    pub v_spacing: f32,
}

impl OpCoords {
    pub fn scaled_down_to(self, scale: f32) -> OpCoords {
        let shrink_in_by = (1. - scale)/2.;
        OpCoords {
            x: self.x + self.w * shrink_in_by,
            y: self.y - self.h * shrink_in_by,
            w: self.w * scale,
            h: self.h * scale,
            v_spacing: self.v_spacing,
        }
    }
}

struct OpStyle {
    border_width: f32,
    border_col: Color,
    fill_col: Color,
    scale: f32,
    v_connector: bool,
}

impl OpStyle {
    pub fn coding() -> Self {
        Self {
            border_width: 2.,
            border_col: WHITE,
            fill_col: Color {r: 0., g:0., b:0., a:0. },
            scale: 1.0,
            v_connector: true,
        }
    }

    pub fn dragging() -> Self {
        Self {
            border_width: 2.,
            border_col: WHITE,
            // Covers over background when dragging
            fill_col: Color {r: 0., g:0., b:0., a:0.5 },
            scale: 1.0,
            v_connector: false,
        }
    }

    pub fn highlighted(orig_style: Self) -> Self {
        Self {
            border_width: 4.,
            border_col: YELLOW,
            ..orig_style
        }
    }

    pub fn running() -> Self {
        Self {
            border_width: 2.,
            border_col: SKYBLUE,
            fill_col: Color {r: 0., g:0., b:0., a:0. },
            scale: 1.0,
            v_connector: true,
        }
    }

    pub fn placeholder() -> Self {
        Self {
            border_width: 1.,
            border_col: GRAY,
            // Covers over excess connecting line
            fill_col: BLACK,
            scale: 1.0,
            v_connector: false,
        }
    }

    fn draw_at(self, coords: OpCoords, txt: &str) {
        let c = coords.scaled_down_to(self.scale);

        draw_rectangle(c.x, c.y, c.w, c.h, self.fill_col);
        draw_rectangle_lines(c.x, c.y, c.w, c.h, self.border_width, self.border_col);
        let font_sz = c.w * 1.35;
        draw_text(txt, c.x + 0.2*c.w, c.y+0.85*c.h, font_sz, WHITE);

        if self.v_connector {
            draw_line(
                c.x + c.w/2.,
                c.y + c.h,
                c.x + c.w/2.,
                c.y + c.h + c.v_spacing,
                2.,
                LIGHTGRAY
            );
        }
    }
}


// NB Original intention was to split this into a parent struct and UiCoding struct.
pub struct UiCodingArena {
    is_coding: bool,

    fr_pos: FrameCoords,

    dragging: Dragging,
}

impl UiCodingArena
{
    pub fn new() -> Self {
        Self {
            is_coding: false,
            fr_pos: FrameCoords::default(),
            dragging: Dragging::No,
        }

    }

    pub async fn render<GameLogic: BaseGameLogic>(
            &mut self,
            coding_arena: &mut CodingArena<GameLogic>,
            texture_cache: &mut TextureCache,
            anim: AnimState,
        ) {
        let _arena = &mut coding_arena.curr_arena;

        self.initialise_frame_coords(coding_arena.is_coding());

        if self.is_coding {
            UiArena::render(&coding_arena.init_arena, texture_cache, self.fr_pos.arena, anim).await;
        } else {
            UiArena::render(coding_arena.curr_arena.as_mut().unwrap(), texture_cache, self.fr_pos.arena, anim).await;
        }

        self.draw_background(coding_arena);
        self.draw_prog(&mut coding_arena.coding);
        if self.is_coding {
            self.draw_supply(&mut coding_arena.coding);
        }

        self.interact_prog(&mut coding_arena.coding);
        if self.is_coding {
            self.interact_supply(&mut coding_arena.coding);
            self.draw_dragging(&mut coding_arena.coding);
        }
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

        // Supply op
        let spacing_pc = 0.5;
        let flow_n = 2.;
        let supply_op_w = (supply_h * 0.8).min(supply_w / (spacing_pc + flow_n*(1.+spacing_pc)));
        let supply_op_h = supply_op_w;
        let supply_op_font_sz = supply_op_h * 1.35;
        let supply_op_spacing = supply_op_w * spacing_pc;

        // Prog
        let prog_x = arena_w;
        let prog_y = supply_h;
        let prog_w = screen_width() - arena_w;
        let prog_h = screen_height() - supply_h;

        // Prog instrs
        let prog_n = 6.;
        let prog_instr_h = (prog_w * 0.8).min(prog_h / (spacing_pc + prog_n*(1.+spacing_pc)));
        let prog_instr_w = prog_instr_h;
        let prog_instr_spacing =  prog_instr_w * spacing_pc;

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

    fn draw_background<GameLogic: BaseGameLogic>(&mut self, _coding_arena: &mut CodingArena<GameLogic>) {
        // Clear background if necessary.
        crate::ui::clear_background_for_current_platform(LIGHTGRAY);

        // Draw lev info. TODO: Move to sep fn
        draw_text(format!("Level: 1", ).as_str(), 10., 20., 20., DARKGRAY);
    }

    fn draw_supply(&self, coding: &mut Coding) {
        draw_rectangle_lines(self.fr_pos.supply_x, self.fr_pos.supply_y, self.fr_pos.supply_w, self.fr_pos.supply_h+1., 2., WHITE);

        for (idx, bin) in coding.supply.clone().iter().enumerate() {
            self.draw_supply_op(idx, bin);
        }
    }

    fn interact_supply(&mut self, coding: &mut Coding) {
        for idx in 0..coding.supply.len() {
            self.interact_supply_op(coding, idx);
        }

        if self.mouse_in(self.fr_pos.supply_x, self.fr_pos.supply_y, self.fr_pos.supply_w, self.fr_pos.supply_h) {
            if is_mouse_button_released(MouseButton::Left) {
                self.drop_to_supply(coding);
            }
        }

    }

    fn draw_prog(&self, coding: &Coding) {
        let border_col = if self.is_coding {WHITE} else {SKYBLUE};
        draw_rectangle_lines(self.fr_pos.prog_x, self.fr_pos.prog_y, self.fr_pos.prog_w, self.fr_pos.prog_h, 2., border_col);

        // NB: Clone means that we draw the original instrs, even if one is dragged out.
        for (idx, instr) in coding.prog.instrs.clone().iter().enumerate() {
            self.draw_prog_instr(idx, Some(instr));
        }
        self.draw_prog_instr(coding.prog.instrs.len(), None);
    }

    fn interact_prog(&mut self, coding: &mut Coding) {
        for (idx, instr) in coding.prog.instrs.clone().iter().enumerate() {
            self.interact_prog_instr(coding, idx, Some(instr));
        }
        self.interact_prog_instr(coding, coding.prog.instrs.len(), None);
    }

    fn draw_dragging(&mut self, coding: &mut Coding) {
        //// Draw dragging

        // If mouse is released anywhere non-actionable, cancel any dragging.
        // Use "!is_mouse_button_down" not "is_mouse_buttom_released" to ensure dragging is stopped.
        if !is_mouse_button_down(MouseButton::Left) && let Dragging::Yes {..} = self.dragging  {
            self.drop_cancel(coding);
        }

        if let Dragging::Yes{op, orig_offset_x, orig_offset_y, op_ref,..} = &self.dragging {
            let (mx, my) = mouse_position();
            let (x,y) = (mx - orig_offset_x, my - orig_offset_y);
            // TODO: get txt from original op via InstrRef
            match op_ref {
                InstrRef::Supply{..} => {
                    let coords = OpCoords {x, y, w:self.fr_pos.prog_instr_w, h:self.fr_pos.prog_instr_h, v_spacing: 0.};
                    let style = OpStyle::dragging();
                    style.draw_at(coords, &op.to_string());
                },
                InstrRef::Prog{..} => {
                    let coords = OpCoords {x, y, w:self.fr_pos.prog_instr_w, h:self.fr_pos.prog_instr_h, v_spacing: 0.};
                    let style = OpStyle::dragging();
                    style.draw_at(coords, &op.to_string());
                },
            }
        }
    }

    fn interact_supply_op(&mut self, coding: &mut Coding, idx: usize)
    {
        let coords = self.supply_op_coords(idx);

        if self.is_coding && self.mouse_in(coords.x, coords.y, self.fr_pos.supply_op_w, self.fr_pos.supply_op_h) {
            if is_mouse_button_pressed(MouseButton::Left) {
                self.drag_supply_op(coding, idx, mouse_position().0 - coords.x, mouse_position().1 - coords.y);
            } else if is_mouse_button_released(MouseButton::Left) {
                self.drop_to_supply_bin(coding, idx);
            }
        }
    }

    fn draw_supply_op(&self, idx: usize, bin: &Bin)
    {
        let coords = self.supply_op_coords(idx);
        let style = self.calculate_style(coords, bin.curr_count>0);
        style.draw_at(coords, &bin.op.to_string());

        // Draw count
        let count_txt = format!("{}/{}", bin.curr_count, bin.orig_count);
        draw_text(&count_txt, coords.x + 0.5*self.fr_pos.supply_op_w, coords.y+1.25*self.fr_pos.supply_op_h, self.fr_pos.supply_op_font_sz * 0.25, WHITE);
    }

    fn draw_prog_instr(&self, idx: usize, instr: Option<&Op>)
    {
        let coords = self.prog_instr_coords(idx);
        let style = self.calculate_style(coords, instr.is_some());
        let txt = instr.map_or("".to_string(), Op::to_string);
        style.draw_at(coords, &txt);
    }

    fn interact_prog_instr(&mut self, coding: &mut Coding, idx: usize, instr: Option<&Op>)
    {
        let coords = self.prog_instr_coords(idx);

        if self.is_coding && self.mouse_in(coords.x, coords.y, self.fr_pos.prog_instr_w, self.fr_pos.prog_instr_h) {
            if instr.is_some() && is_mouse_button_pressed(MouseButton::Left) {
                self.drag_prog_instr(coding, idx, mouse_position().0 - coords.x, mouse_position().1 - coords.y);
            } else if is_mouse_button_released(MouseButton::Left){
                self.drop_to_prog(coding, idx);
            }
        }
    }

    fn calculate_style(&self, coords: OpCoords, has_op: bool) -> OpStyle
    {
        let mut style = if self.is_coding {
            if has_op {
                OpStyle::coding()
            } else {
                OpStyle::placeholder()
            }
        } else {
            OpStyle::running()
        };

        let mouse_in = self.mouse_in(coords.x, coords.y, coords.w, coords.h);
        if mouse_in && (has_op || matches!(self.dragging, Dragging::Yes{..}) ) {
            style = OpStyle::highlighted(style);
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
            v_spacing: 0.,
        }
    }

    fn prog_instr_coords(&self, idx: usize) -> OpCoords {
        let fdx = idx as f32;
        let x = self.fr_pos.prog_x + self.fr_pos.prog_w/2. - self.fr_pos.prog_instr_w/2.;
        let y = self.fr_pos.prog_y + self.fr_pos.prog_instr_spacing + fdx * (self.fr_pos.prog_instr_h + self.fr_pos.prog_instr_spacing);

        OpCoords {x, y, w: self.fr_pos.prog_instr_w, h: self.fr_pos.prog_instr_h, v_spacing: self.fr_pos.prog_instr_spacing}
    }

    fn mouse_in(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        let (mx, my) = mouse_position();
        (x..x+w).contains(&mx) && (y..y+h).contains(&my)
    }

    fn drag_supply_op(&mut self, coding: &mut Coding, idx: usize, orig_offset_x: f32, orig_offset_y: f32) {
        // TODO: Test not already dragging?
        let bin = &mut coding.supply.get_mut(idx).unwrap();
        log::debug!("INFO: Dragging {:?} from supply", bin.op);
        self.dragging = if bin.curr_count > 0 {
            bin.curr_count -= 1;
            Dragging::Yes {
                op: bin.op,
                op_ref: InstrRef::Supply { idx },
                orig_offset_x,
                orig_offset_y
            }
        } else {
            Dragging::No
        }
    }

    fn drag_prog_instr(&mut self, coding: &mut Coding, idx: usize, orig_offset_x: f32, orig_offset_y: f32) {
        // TODO: Test not already dragging?
        let op = coding.prog.instrs.remove(idx);
        log::debug!("INFO: Dragging {:?} from prog", op);
        self.dragging = Dragging::Yes {
            op: op,
            op_ref: InstrRef::Prog { idx },
            orig_offset_x,
            orig_offset_y
        }
    }

    fn drop_to_supply_bin(&mut self, coding: &mut Coding, idx: usize) {
        if let Dragging::Yes {op: dragged_op, ..} = self.dragging {
            log::debug!("INFO: Dropping {:?} to supply bin", dragged_op);
            let bin = &mut coding.supply.get_mut(idx).unwrap();
            if bin.op == dragged_op && bin.curr_count < bin.orig_count {
                bin.curr_count += 1;
                self.dragging = Dragging::No;
            }
        }
    }

    fn drop_to_supply(&mut self, coding: &mut Coding) {
        // TODO: For loop to find correct bin.
        // TODO: Handle index errors, or bin overflow errors, without panicking.
        if let Dragging::Yes {op: dragged_op, ..} = self.dragging {
            log::debug!("INFO: Dropping {:?} to supply", dragged_op);
            for bin in &mut coding.supply {
                if bin.op == dragged_op && bin.curr_count < bin.orig_count {
                    bin.curr_count += 1;
                    self.dragging = Dragging::No;
                }
            }
        }
    }

    fn drop_to_prog(&mut self, coding: &mut Coding, idx: usize) {
        if let Dragging::Yes { op, .. } = self.dragging {
            log::debug!("INFO: Dropping {:?} to prog", op);
            coding.prog.instrs.insert(idx, op);
            self.dragging = Dragging::No;
        }
    }

    fn drop_cancel(&mut self, coding: &mut Coding) {
        match self.dragging {
            Dragging::Yes { op, op_ref: InstrRef::Supply { idx }, ..} => {
                log::debug!("INFO: Cancelling drag. Returning {:?} to supply idx {:?}", op, idx);
                self.drop_to_supply_bin(coding, idx);
            },
            Dragging::Yes { op, op_ref: InstrRef::Prog { idx }, ..} => {
                log::debug!("INFO: Cancelling drag. Returning {:?} to supply idx {:?}", op, idx);
                self.drop_to_prog(coding, idx);
            },
            Dragging::No => (),
        }
    }

}
