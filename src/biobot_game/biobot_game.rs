use std::collections::HashMap;

use crate::engine;

use crate::engine::*;

use crate::engine::Scene;

// TOOD: Would it be useful to have a levset trait defining the necessary traits,
// even if it doesn't add any other functionality?
#[derive(Clone, Copy, Debug)]
pub enum BiobotSceneId {
    NewGame,
    LevIntro(u16),
    LevPlay(u16),
    LevOutro(u16),
    Retry(u16),
    Win,
}

impl SceneIdBase for BiobotSceneId {
}

impl SceneId for BiobotSceneId {
}

pub struct BiobotGame {
    pub curr_scene: BiobotSceneId,
}

impl Game for BiobotGame {
    type Levstage = BiobotSceneId;

    fn new_game() -> BiobotGame {
        BiobotGame { curr_scene: BiobotSceneId::NewGame }
    }

    fn initial_lev_stage(&self) -> BiobotSceneId {
        BiobotSceneId::NewGame
    }

    fn load_lev_stage_impl(&self, stage: BiobotSceneId) -> Scene {
        let aquarium1_key = HashMap::from([
            // TODO: Combine with obj.char types?
            (' ', vec![ new_floor() ]),
            ('#', vec![ new_floor(), new_wall() ]),
            ('>', vec![ new_floor(), new_fish(CoordDelta::from_xy(1,0)) ]),
            ('<', vec![ new_floor(), new_fish(CoordDelta::from_xy(-1,0)) ]),
            ('g', vec![ new_floor(), new_gawpie(CoordDelta::from_xy(1,0)) ]),
            ('h', vec![ new_floor(), new_hero_crab() ]),
            ('o', vec![ /* new_floor(), */ new_door_win() ]), // TODO: Check win on non-floor tiles
            ('@', vec![ new_floor(), new_door_closed() ]),
            ('_', vec![ new_door_open() ]), // Logically ought to be a floor, but for now draw movs over the 'door'
            /*
            */
        ]);

        match stage {
            // TODO: Can we use idx++ instead of specifying each level number? Not immediately?
            BiobotSceneId::NewGame => biobot_dialogue_splash(
                //"Click or press [enter] to start.".to_string(),
                vec![
                    "Hello!",
                    "Hi!",
                    "I'm a snake!",
                    "I'm crab!",
                ],
                BiobotSceneId::LevIntro(1)
            ),

            BiobotSceneId::LevIntro(1) => biobot_splash("Welcome to level 1!".to_string(), BiobotSceneId::LevPlay(1)),
            BiobotSceneId::LevPlay(1) => biobot_play(1, &[
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
            BiobotSceneId::LevOutro(1) => biobot_splash("Well done!! Goodbye from level 1".to_string(), BiobotSceneId::LevIntro(2)),

            BiobotSceneId::LevIntro(2) => biobot_splash("Ooh, welcome to level 2!".to_string(), BiobotSceneId::LevPlay(2)),
            BiobotSceneId::LevPlay(2) => biobot_play(2, &[
                "################",
                "#              #",
                "#              #",
                "#              #",
                "#       h      #",
                "#              #",
                "#              #",
                "#  g           #",
                "#              #",
                "#        <     #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "####o###########",
            ], aquarium1_key),
            BiobotSceneId::LevOutro(2) => biobot_splash("Wow, well done!! Goodbye from level 2!".to_string(), BiobotSceneId::Win),

            BiobotSceneId::Retry(levno) => biobot_splash("Game Over. Press [enter] to retry.".to_string(), BiobotSceneId::LevPlay(levno)),
            BiobotSceneId::Win => biobot_splash("Congratulations. You win! Press [enter] to play again.".to_string(), BiobotSceneId::LevIntro(1)),

            BiobotSceneId::LevIntro(_) => panic!("Loading LevIntro for level that can't be found."),
            BiobotSceneId::LevPlay(_) => panic!("Loading LevPlay for level that can't be found."),
            BiobotSceneId::LevOutro(_) => panic!("Loading LevOutro for level that can't be found."),
        }
    }
}

///////////
/// Helpers
///
/// Also used by tests

pub fn biobot_splash(txt: String, to_stage: BiobotSceneId) -> Scene {
    Scene::make_splash(txt, Box::new(to_stage))
}

pub fn biobot_dialogue_splash(entries: Vec<&str>, to_stage: BiobotSceneId) -> Scene {
    Scene::make_dialogue(entries, Box::new(to_stage))
}

pub fn biobot_play<const HEIGHT: usize>(levno: u16, ascii_map: &[&str; HEIGHT], map_key: HashMap<char, Vec<engine::Obj>>) -> Scene {
    // Box::new(BiobotLevelNum::LevOutro(levno)),
    Scene::play_from_ascii(
        ascii_map,
        map_key,
        Box::new(BiobotSceneId::LevOutro(levno)),
        Box::new(BiobotSceneId::Retry(levno)))
}
