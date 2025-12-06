use super::*;
use crate::for_gamedata;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CodingRunningPhase {
    Coding,
    Running,
    Died,
    Won,
}

// NB: Move into Prog Puzz. Or make into a general multi-widget widget.
#[derive(Clone, Debug)]
pub struct CodingArena<GameLogic : for_gamedata::BaseGameLogic> {
    pub init_arena: Arena<GameLogic>,
    // Maybe move into Running state, not core data?
    pub curr_arena: Option<Arena<GameLogic>>,
    pub coding: Coding,
    pub phase: CodingRunningPhase,
}

impl<GameLogic : for_gamedata::BaseGameLogic> BaseWidget for CodingArena<GameLogic>
{
    fn advance(&mut self, cmd: MoveCmd) -> WidgetContinuation {
        match self.phase {
            CodingRunningPhase::Coding => {
                if cmd == MoveCmd::NextPhase {
                    self.start_execution();
                }
            },
            CodingRunningPhase::Running => {
                if cmd == MoveCmd::Tick {
                    log::debug!("Advance bot program.");

                    let conclusion = self.curr_arena.as_mut().unwrap().advance(cmd);
                    if conclusion == std::ops::ControlFlow::Break(for_gamedata::WidgetConclusion::Die) {
                        log::debug!("Ran off end of program. Stopped.");
                        self.phase = CodingRunningPhase::Died;
                    } else if conclusion == std::ops::ControlFlow::Break(for_gamedata::WidgetConclusion::Win) {
                        log::debug!("Bot found target!");
                        self.phase = CodingRunningPhase::Won;
                    } else if conclusion == std::ops::ControlFlow::Continue(()) {
                        log::trace!("Bot advanced normally. Continue executing program.");
                    }
                } else {
                    log::debug!("Cancelling execution.");
                    self.cancel_execution();
                }
            },
            CodingRunningPhase::Won => {
                return WidgetContinuation::Break(WidgetConclusion::Win);
            },
            CodingRunningPhase::Died => {
                self.cancel_execution();
            },
        }

        return WidgetContinuation::Continue(());
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

    pub fn start_execution(&mut self) {
        log::debug!("Start program running.");

        // Init interactive arena
        self.curr_arena = Some(self.init_arena.clone());
        // Run game-specific logic for sync'ing different panes at start of run.
        GameLogic::harmonise(self);

        self.phase = CodingRunningPhase::Running;
    }

    pub fn cancel_execution(&mut self) {
        log::debug!("Returning to coding screen");
        self.phase = CodingRunningPhase::Coding;
        self.curr_arena = None;
    }

}
