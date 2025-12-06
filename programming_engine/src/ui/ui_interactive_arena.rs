use crate::ui::ui_arena::UiArena;
use crate::widget::Arena;
use crate::gamedata::BaseGameLogic;

use super::*;

pub struct UiInteractiveArena;

impl UiInteractiveArena {
    pub async fn render<GameLogic: BaseGameLogic>(
        state: &Arena<GameLogic>,
        texture_cache: &mut TextureCache,
        // Whole screen, or smaller area, in which to fit a square map.
        draw_area: PRect,
        anim: AnimState,
    ) {
        UiArena::render(state, texture_cache, draw_area, anim).await;
   }
}
