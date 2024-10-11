/// * Not GPL. Details TBD. OK to use when testing compiling.

use std::collections::HashMap;

use crate::*;

use levset::*; // Less than all?

use play::Play;

#[derive(Clone, Copy, Debug)]
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
            BiobotStage::NewGame => biobot_splash("Press [enter] to start.".to_string(), BiobotStage::LevIntro(1)),

            BiobotStage::LevIntro(1) => biobot_splash("Welcome to level 1!".to_string(), BiobotStage::LevPlay(1)),
            BiobotStage::LevPlay(1) => biobot_levplay(1, &[
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
            BiobotStage::LevOutro(1) => biobot_splash("Well done!! Goodbye from level 1".to_string(), BiobotStage::LevIntro(2)),

            BiobotStage::LevIntro(2) => biobot_splash("Ooh, welcome to level 2!".to_string(), BiobotStage::LevPlay(2)),
            BiobotStage::LevPlay(2) => biobot_levplay(2, &[
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
            BiobotStage::LevOutro(2) => biobot_splash("Wow, well done!! Goodbye from level 2!".to_string(), BiobotStage::Win),

            BiobotStage::Retry(levno) => biobot_splash("Game Over. Press [enter] to retry.".to_string(), BiobotStage::LevPlay(levno)),
            BiobotStage::Win => biobot_splash("Congratulations. You win! Press [enter] to play again.".to_string(), BiobotStage::LevIntro(1)),

            BiobotStage::LevIntro(_) => panic!("Loading LevIntro for level that can't be found."),
            BiobotStage::LevPlay(_) => panic!("Loading LevPlay for level that can't be found."),
            BiobotStage::LevOutro(_) => panic!("Loading LevOutro for level that can't be found."),
        }
    }
}

///////////
/// Helpers
///
/// Also used by tests

pub fn biobot_splash(txt: String, to_stage: levset_biobot::BiobotStage) -> Play {
    Play::make_splash(txt, Box::new(to_stage))
}

pub fn biobot_levplay(levno: u16, ascii_map: &[&str; 16], map_key: HashMap<char, Vec<obj::Obj>>) -> Play {
    // Box::new(BiobotStage::LevOutro(levno)),
    Play::levplay_from_ascii(
        ascii_map,
        map_key,
        Box::new(BiobotStage::LevOutro(levno)),
        Box::new(BiobotStage::Retry(levno)))
}
