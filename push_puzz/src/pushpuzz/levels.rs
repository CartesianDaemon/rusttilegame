use std::collections::HashMap;

use super::objs::*;

use tile_engine::for_gamedata::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PushpuzzSceneId {
    NewGame,
    LevIntro(u16),
    LevArena(u16),
    LevOutro(u16),
    LevRetry(u16),
    Win,
}

#[derive(Debug)]
pub struct PushpuzzLevset {
    pub current_sceneid: PushpuzzSceneId,
}

impl PushpuzzLevset {
    pub fn new() -> PushpuzzLevset {
        PushpuzzLevset { current_sceneid: PushpuzzSceneId::NewGame }
    }

    pub fn advance_scene(&mut self, continuation: SceneConclusion) {
        self.current_sceneid = match (self.current_sceneid, continuation) {
            (PushpuzzSceneId::NewGame, SceneConclusion::SplashContinue) => PushpuzzSceneId::LevIntro(1),
            (PushpuzzSceneId::LevIntro(levnum), SceneConclusion::SplashContinue) => PushpuzzSceneId::LevArena(levnum),
            (PushpuzzSceneId::LevArena(levnum), SceneConclusion::Win) => PushpuzzSceneId::LevOutro(levnum),
            (PushpuzzSceneId::LevArena(levnum), SceneConclusion::Die) => PushpuzzSceneId::LevRetry(levnum),
            (PushpuzzSceneId::LevRetry(levnum), SceneConclusion::SplashContinue) => PushpuzzSceneId::LevArena(levnum),
            // TODO: Get max levnum from list of levels?
            (PushpuzzSceneId::LevOutro(2), SceneConclusion::SplashContinue) => PushpuzzSceneId::Win,
            (PushpuzzSceneId::LevOutro(levnum), SceneConclusion::SplashContinue) => PushpuzzSceneId::LevOutro(levnum+1),
            (PushpuzzSceneId::Win, SceneConclusion::SplashContinue) => PushpuzzSceneId::NewGame,
            _ => panic!()
        };
    }

    pub fn load_scene(&self) -> Scene<super::PushpuzzMovementLogic> {
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

        match self.current_sceneid {
            // TODO: Can we use idx++ instead of specifying each level number? Not immediately?
            PushpuzzSceneId::NewGame => Scene::from_splash_dialogue(
                //"Click or press [enter] to start.".to_string(),
                vec![
                    "Hello!",
                    "Hi!",
                    "I'm a snake!",
                    "I'm crab!",
                ]
            ),

            PushpuzzSceneId::LevIntro(1) => {
                Scene::from_splash_string("Welcome to level 1!".to_string())
            },
            PushpuzzSceneId::LevArena(1) => Scene::from_play_ascii_map(&[
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
            PushpuzzSceneId::LevOutro(1) => {
                Scene::from_splash_string("Well done!! Goodbye from level 1".to_string())
            },

            PushpuzzSceneId::LevIntro(2) => {
                Scene::from_splash_string("Ooh, welcome to level 2!".to_string())
            },
            PushpuzzSceneId::LevArena(2) => Scene::from_play_ascii_map(&[
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
            PushpuzzSceneId::LevOutro(2) => {
                Scene::from_splash_string("Wow, well done!! Goodbye from level 2!".to_string())
            },

            PushpuzzSceneId::LevRetry(_levno) => {
                Scene::from_splash_string("Game Over. Press [enter] to retry.".to_string())
            },
            PushpuzzSceneId::Win => {
                Scene::from_splash_string("Congratulations. You win! Press [enter] to play again.".to_string())
            },

            PushpuzzSceneId::LevIntro(_) => panic!("Loading LevIntro for level that can't be found."),
            PushpuzzSceneId::LevArena(_) => panic!("Loading LevArena for level that can't be found."),
            PushpuzzSceneId::LevOutro(_) => panic!("Loading LevOutro for level that can't be found."),
        }
    }
}
