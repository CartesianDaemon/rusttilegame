use super::*;
use crate::map_coords::MoveCmd;
use crate::for_gamedata;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CodingRunningPhase {
    Coding,
    Running,
    Stopped,
}

// NB: Move into Prog Puzz. Or make into a general multi-widget widget.
#[derive(Clone, Debug)]
pub struct CodingArena<GameLogic : for_gamedata::BaseGameLogic> {
    pub init_arena: Arena<GameLogic>,
    // Maybe move into Running state, not core data?
    pub curr_arena: Option<Arena<GameLogic>>,
    pub coding: Coding,
    phase: CodingRunningPhase,
}

impl<GameLogic : for_gamedata::BaseGameLogic> BaseWidget for CodingArena<GameLogic>
{
    fn advance(&mut self, cmd: MoveCmd) -> PaneContinuation {
        match self.phase {
            CodingRunningPhase::Coding => {
                if cmd == MoveCmd::Stay {
                    log::debug!("Start program running.");

                    // Init interactive arena
                    self.curr_arena = Some(self.init_arena.clone());
                    // Run game-specific logic for sync'ing different panes at start of run.
                    GameLogic::harmonise(self);

                    self.phase = CodingRunningPhase::Running;
                }
            },
            CodingRunningPhase::Running => {
                if cmd == MoveCmd::Stay {
                    log::debug!("Advance bot program.");

                    let conclusion = self.curr_arena.as_mut().unwrap().advance(cmd);
                    if conclusion == std::ops::ControlFlow::Break(for_gamedata::WidgetConclusion::Die) {
                        log::debug!("Ran off end of program. Stopped.");
                        self.phase = CodingRunningPhase::Stopped;
                    } else if conclusion == std::ops::ControlFlow::Break(for_gamedata::WidgetConclusion::Win) {
                        // TODO: Put in a delay and win animation here.
                        log::debug!("Bot found target! Go to next level");
                        return PaneContinuation::Break(WidgetConclusion::Win);
                    } else if conclusion == std::ops::ControlFlow::Continue(()) {
                        log::trace!("Bot advanced normally. Continue executing program.");
                    }
                }
            },
            CodingRunningPhase::Stopped => {
                log::debug!("Returning to coding screen");
                self.phase = CodingRunningPhase::Coding;
                self.curr_arena = None;
            }
        }

        return PaneContinuation::Continue(());
    }

    fn tick_based(&self) -> crate::ui::TickStyle {
        // TODO: Need "stop" to happen at any time. But could trigger bot movement on key, or on tick?
        if self.is_running() {
            crate::ui::TickStyle::TickAutomatically
        } else {
            crate::ui::TickStyle::Continuous
        }
    }
}

impl<GameLogic: for_gamedata::BaseGameLogic> CodingArena<GameLogic>
{
    pub fn new<const HEIGHT: usize>(
        arena: Arena<GameLogic>,
        code: Coding,
    ) -> Self {
        Self {
            init_arena: arena,
            curr_arena: None,
            coding: code,
            phase: CodingRunningPhase::Coding,
        }
    }

    pub fn is_running(&self) -> bool {
        self.phase == CodingRunningPhase::Running
    }

    pub fn is_coding(&self) -> bool {
        self.phase == CodingRunningPhase::Coding
    }
}
