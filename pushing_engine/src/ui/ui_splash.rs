use macroquad::prelude::*;

use crate::scene::Splash;

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
