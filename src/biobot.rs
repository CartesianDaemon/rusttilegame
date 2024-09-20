/// * Not GPL. Details TBD. OK to use when testing compiling.

use std::collections::HashMap;

use crate::*;

use load::*; // Less than all?

use play::Play;

#[derive(Clone, Copy)]
pub enum BiobotStage {
    NewGame,
    LevIntro(u16),
    LevPlay(u16),
    LevOutro(u16),
    Retry(u16),
    Win,
}

impl LevstageBase for BiobotStage {
}

impl LevstageDerived for BiobotStage {
}

pub struct BiobotLevSet {
    // No state needed
}

impl LevSet for BiobotLevSet {
    type Levstage = BiobotStage;

    fn initial_lev_stage(&self) -> BiobotStage {
        BiobotStage::NewGame
    }

    fn _load_lev_stage(&self, stage: BiobotStage) -> Play {
        let aquarium1_key = HashMap::from([
            (' ', vec![ new_floor() ]),
            ('#', vec![ new_floor(), new_wall() ]),
            ('>', vec![ new_floor(), new_snake((1,0)) ]),
            ('<', vec![ new_floor(), new_snake((-1,0)) ]),
            ('h', vec![ new_floor(), new_hero_crab() ]),
            ('o', vec![ /* new_floor(), */ new_door_win() ]), // TODO: Check win on non-floor tiles
            ('@', vec![ new_floor(), new_door_closed() ]),
            ('_', vec![ new_floor(), new_door_open() ]),
            /*
            */
        ]);

        match stage {
            // TODO: Can we use idx++ instead of specifying each level number? Not immediately?
            BiobotStage::NewGame => make_splash("Press [enter] to start.".to_string(), BiobotStage::LevIntro(1)),

            BiobotStage::LevIntro(1) => make_splash("Welcome to level 1!".to_string(), BiobotStage::LevPlay(1)),
            BiobotStage::LevPlay(1) => make_levplay(1, &[
                "#            # #",
                "#####@####@###@#",
                "@              #",
                "#####_########_#",
                "#            # #",
                "#            # #",
                "#  >         @ @",
                "#            # #",
                "#            # #",
                "#       h    # #",
                "#            # o",
                "#            # #",
                "#            # #",
                "##############@#",
                "#            # #",
                "#            @ #",
            ], aquarium1_key),
            BiobotStage::LevOutro(1) => make_splash("Well done!! Goodbye from level 1".to_string(), BiobotStage::LevIntro(2)),

            BiobotStage::LevIntro(2) => make_splash("Ooh, welcome to level 2!".to_string(), BiobotStage::LevPlay(2)),
            BiobotStage::LevPlay(2) => make_levplay(2, &[
                "################",
                "#              #",
                "#              #",
                "#              #",
                "#       h      #",
                "#              #",
                "#              #",
                "#  >           #",
                "#              #",
                "#        <     #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "####o###########",
            ], aquarium1_key),
            BiobotStage::LevOutro(2) => make_splash("Wow, well done!! Goodbye from level 2!".to_string(), BiobotStage::Win),

            BiobotStage::Retry(levno) => make_splash("Game Over. Press [enter] to retry.".to_string(), BiobotStage::LevPlay(levno)),
            BiobotStage::Win => make_splash("Congratulations. You win! Press [enter] to play again.".to_string(), BiobotStage::LevIntro(1)),

            BiobotStage::LevIntro(_) => panic!("Loading LevIntro for level that can't be found."),
            BiobotStage::LevPlay(_) => panic!("Loading LevPlay for level that can't be found."),
            BiobotStage::LevOutro(_) => panic!("Loading LevOutro for level that can't be found."),
        }
    }
}
