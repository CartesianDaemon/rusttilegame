use macroquad::prelude::*;

use crate::gamedata::BaseGameLogic;

use crate::widget::*;

enum InstrRef {
    Supply {
        _idx: usize
    },
    Flowchart{
        _idx: usize,
    },
}

enum Dragging {
    No,
    Yes{
        orig_offset_x: f32,
        orig_offset_y: f32,
        instr_ref: InstrRef,
    },
}

struct FrameCoords {
    supply_x: f32,
    supply_y: f32,
    supply_w: f32,
    supply_h: f32,

    supply_instr_w: f32,
    supply_instr_h: f32,
    supply_instr_font_sz: f32,
    supply_instr_spacing: f32,

    flowchart_x: f32,
    flowchart_y: f32,
    flowchart_w: f32,
    flowchart_h: f32,

    flowchart_instr_w: f32,
    flowchart_instr_h: f32,
    flowchart_instr_font_sz: f32,
    flowchart_instr_spacing: f32,
}

pub struct UiCodingArena {
    fr_pos: FrameCoords,
    dragging: Dragging,
}

impl UiCodingArena
{
    pub fn new() -> Self {
        Self {
            fr_pos: FrameCoords {
                supply_x: 0.,
                supply_y: 0.,
                supply_w: 0.,
                supply_h: 0.,
                supply_instr_w: 0.,
                supply_instr_h: 0.,
                supply_instr_font_sz: 0.,
                supply_instr_spacing: 0.,
                flowchart_x: 0.,
                flowchart_y: 0.,
                flowchart_w: 0.,
                flowchart_h: 0.,
                flowchart_instr_w: 0.,
                flowchart_instr_h: 0.,
                flowchart_instr_font_sz: 0.,
                flowchart_instr_spacing: 0.,
            },
            dragging: Dragging::No,
        }

    }

    pub fn initialise_frame_coords(&mut self) {
        // Game
        // let game_w = screen_width().min(screen_height());
        // let game_h = screen_width().min(screen_height());
        // let game_x = (screen_width() - game_w)/2.;
        // let game_y = (screen_height() - game_h)/2.;

        // Arena
        let arena_w = screen_height().min(screen_width() * 0.6);

        // Supply
        let supply_x = arena_w;
        let supply_y = 0.;
        let supply_w = screen_width() - arena_w;
        let supply_h = screen_height() * 0.3;

        // Supply instr
        let spacing_pc = 0.5;
        let flow_n = 2.;
        let supply_instr_w = (supply_h * 0.8).min(supply_w / (spacing_pc + flow_n*(1.+spacing_pc)));
        let supply_instr_h = supply_instr_w;
        let supply_instr_font_sz = supply_instr_h * 1.35;
        let supply_instr_spacing = supply_instr_w * spacing_pc;

        // Flowchart
        let flowchart_x = arena_w;
        let flowchart_y = supply_h;
        let flowchart_w = screen_width() - arena_w;
        let flowchart_h = screen_height() - supply_h;

        // Flowchart instrs
        let prog_n = 6.;
        let flowchart_instr_h = (flowchart_w * 0.8).min(flowchart_h / (spacing_pc + prog_n*(1.+spacing_pc)));
        let flowchart_instr_w = flowchart_instr_h;
        let flowchart_instr_font_sz = flowchart_instr_w * 1.35;
        let flowchart_instr_spacing =  flowchart_instr_w * spacing_pc;

        self.fr_pos = FrameCoords {
            supply_x,
            supply_y,
            supply_w,
            supply_h,
            supply_instr_w,
            supply_instr_h,
            supply_instr_font_sz,
            supply_instr_spacing,
            flowchart_x,
            flowchart_y,
            flowchart_w,
            flowchart_h,
            flowchart_instr_w,
            flowchart_instr_h,
            flowchart_instr_font_sz,
            flowchart_instr_spacing,
        }

    }

    pub fn render<GameLogic: BaseGameLogic>(&mut self, split: &mut CodingArena<GameLogic>) {
        let _arena = &split.arena;
        let _code = &split.code;

        self.initialise_frame_coords();

        crate::ui::clear_background_for_current_platform(LIGHTGRAY);

        draw_text(format!("Level: 1", ).as_str(), 10., 20., 20., DARKGRAY);

        draw_rectangle_lines(self.fr_pos.supply_x, self.fr_pos.supply_y, self.fr_pos.supply_w, self.fr_pos.supply_h+1., 2., WHITE);
        self.draw_supply_instr(0, "F", 2);
        self.draw_supply_instr(1, "L", 2);

        draw_rectangle_lines(self.fr_pos.flowchart_x, self.fr_pos.flowchart_y, self.fr_pos.flowchart_w, self.fr_pos.flowchart_h, 2., WHITE);
        self.draw_flowchart_instr(0, "F");
        self.draw_flowchart_instr(1, "F");
        self.draw_flowchart_instr(2, "R");
        self.draw_flowchart_instr(3, "L");
        self.draw_flowchart_instr(4, "L");
        self.draw_flowchart_instr(5, "");

        // If mouse is released anywhere non-actionable, forget any dragging.
        if !is_mouse_button_down(MouseButton::Left) {
            self.dragging = Dragging::No;
        }

        if let Dragging::Yes{orig_offset_x, orig_offset_y, instr_ref,..} = &self.dragging {
            let (mx, my) = mouse_position();
            let (x,y) = (mx - orig_offset_x, my - orig_offset_y);
            // TODO: get txt from original instr via InstrRef
            let txt = "?";
            match instr_ref {
                InstrRef::Supply{..} => self.draw_supply_instr_at(x, y, txt, 0),
                InstrRef::Flowchart{..} => self.draw_flowchart_instr_at(x, y, txt, 1.),
            }
        }
    }

    fn mouse_in(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        let (mx, my) = mouse_position();
        (x..x+w).contains(&mx) && (y..y+h).contains(&my)
    }

    fn border_width_col(&self, highlight: bool) -> (f32, Color) {
        // TODO: Settings for mouseover highlight, dragged-from highlight, mid-drag, normal...
        if highlight {
            (4., YELLOW)
        } else {
            (2., WHITE)
        }
    }

    fn draw_supply_instr_at(&mut self, x: f32, y: f32, txt: &str, _curr_count: usize) {
        let (border_width, border_col) = self.border_width_col(self.mouse_in(x, y, self.fr_pos.supply_instr_w, self.fr_pos.supply_instr_h));

        // Draw square interior. Covers over background when dragging, or excess connecting line.
        draw_rectangle(x, y, self.fr_pos.supply_instr_w, self.fr_pos.supply_instr_h, Color{r: 0., g:0., b:0., a:0.5 });

        // Draw outline
        draw_rectangle_lines(x, y, self.fr_pos.supply_instr_w, self.fr_pos.supply_instr_h, border_width, border_col);

        // Text
        draw_text(txt, x + 0.2*self.fr_pos.supply_instr_w, y+0.85*self.fr_pos.supply_instr_h, self.fr_pos.supply_instr_font_sz, WHITE);

        // Count
        //draw_text(txt, x + 0.2*self.frame_coords.supply_instr_w, y+0.85*self.frame_coords.supply_instr_h, self.frame_coords.supply_instr_font_sz, WHITE);
    }

    fn draw_supply_instr(&mut self, idx: usize, txt: &str, curr_count: usize)
    {
        let fdx = idx as f32;
        let _curr_count = curr_count as f32;

        let x = self.fr_pos.supply_x + self.fr_pos.supply_instr_spacing + fdx * (self.fr_pos.supply_instr_w + self.fr_pos.supply_instr_spacing);
        let y = self.fr_pos.supply_y + self.fr_pos.supply_h/2. - self.fr_pos.supply_instr_h/2.;

        if is_mouse_button_pressed(MouseButton::Left) && self.mouse_in(x, y, self.fr_pos.supply_instr_w, self.fr_pos.supply_instr_h) {
            let orig_offset_x = mouse_position().0 - x;
            let orig_offset_y = mouse_position().1 - y;
            self.dragging = Dragging::Yes{orig_offset_x, orig_offset_y, instr_ref: InstrRef::Supply{_idx: idx}};
        }

        self.draw_supply_instr_at(x, y, txt, curr_count);
    }

    fn draw_flowchart_instr_at(&mut self, orig_x: f32, orig_y: f32, txt: &str, scale: f32) {
        let shrink_by = 1. - scale;
        let x = orig_x + self.fr_pos.flowchart_instr_w * shrink_by / 2.;
        let y = orig_y - self.fr_pos.flowchart_instr_h * shrink_by / 2.;
        let w = self.fr_pos.flowchart_instr_w * scale;
        let h = self.fr_pos.flowchart_instr_h * scale;

        let mouse_in = self.mouse_in(x, y, w, h);
        let highlight = mouse_in && (scale==1.0 || matches!(self.dragging, Dragging::Yes{..}) );
        let (border_width, border_col) = self.border_width_col(highlight);

        // Draw square interior. Covers over background when dragging, or excess connecting line.
        let fill_col = if scale==1.0 {Color{r: 0., g:0., b:0., a:0.5 }} else {BLACK};
        draw_rectangle(x, y, w, h, fill_col);

        // Draw outline
        draw_rectangle_lines(x, y, w, h, border_width, border_col);

        // Draw text
        draw_text(txt, x + 0.2*w, y+0.85*h, self.fr_pos.flowchart_instr_font_sz, WHITE);
    }

    fn draw_flowchart_instr(&mut self, idx: usize, txt: &str)
    {
        // TODO: Still drawing too often on windows compared to pushpuzz??
        let fdx = idx as f32;

        let x = self.fr_pos.flowchart_x + self.fr_pos.flowchart_w/2. - self.fr_pos.flowchart_instr_w/2.;
        let y = self.fr_pos.flowchart_y + self.fr_pos.flowchart_instr_spacing + fdx * (self.fr_pos.flowchart_instr_h + self.fr_pos.flowchart_instr_spacing);

        let scale = if txt=="" {0.6} else {1.};

        self.draw_flowchart_instr_at(x, y, txt, scale);

        if txt!="" {
            if is_mouse_button_pressed(MouseButton::Left) && self.mouse_in(x, y, self.fr_pos.flowchart_w, self.fr_pos.flowchart_instr_h) {
                let orig_offset_x = mouse_position().0 - x;
                let orig_offset_y = mouse_position().1 - y;
                self.dragging = Dragging::Yes{orig_offset_x, orig_offset_y, instr_ref: InstrRef::Flowchart{_idx: idx}};
            }

            // Connection to next instr
            draw_line(x+self.fr_pos.flowchart_instr_w/2., y+self.fr_pos.flowchart_instr_h, x+self.fr_pos.flowchart_instr_w/2., y+self.fr_pos.flowchart_instr_h+self.fr_pos.flowchart_instr_spacing, 2., LIGHTGRAY);
        }
    }
}
