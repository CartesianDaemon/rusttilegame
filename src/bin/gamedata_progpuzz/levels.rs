use std::collections::HashMap;

use super::objs::*;

use crate::engine::for_gamedata::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BiobotSceneId {
    NewGame,
    LevIntro(u16),
    LevPlay(u16),
    LevOutro(u16),
    LevRetry(u16),
    Win,
}

#[derive(Debug)]
pub struct ProgpuzzGamedata {
    // TODO: Move type into mod.rs somehow.
    pub current_sceneid: BiobotSceneId,
}

impl BaseGamedata for ProgpuzzGamedata {
    // Try to move GameData into mod.rs and leave Levels as a separate member type.
    type Scripts = super::super::scripts_progpuzz::ProgpuzzScripts;

    fn new_game() -> ProgpuzzGamedata {
        ProgpuzzGamedata { current_sceneid: BiobotSceneId::NewGame }
    }

    fn advance_scene(&mut self, continuation: SceneEnding) {
        self.current_sceneid = match (self.current_sceneid, continuation) {
            (BiobotSceneId::NewGame, SceneEnding::SplashNext) => BiobotSceneId::LevIntro(1),
            (BiobotSceneId::LevIntro(levnum), SceneEnding::SplashNext) => BiobotSceneId::LevPlay(levnum),
            (BiobotSceneId::LevPlay(levnum), SceneEnding::PlayWin) => BiobotSceneId::LevOutro(levnum),
            (BiobotSceneId::LevPlay(levnum), SceneEnding::PlayDie) => BiobotSceneId::LevRetry(levnum),
            (BiobotSceneId::LevRetry(levnum), SceneEnding::SplashNext) => BiobotSceneId::LevPlay(levnum),
            // TODO: Get max levnum from list of levels?
            (BiobotSceneId::LevOutro(2), SceneEnding::SplashNext) => BiobotSceneId::Win,
            (BiobotSceneId::LevOutro(levnum), SceneEnding::SplashNext) => BiobotSceneId::LevOutro(levnum+1),
            (BiobotSceneId::Win, SceneEnding::SplashNext) => BiobotSceneId::NewGame,
            _ => panic!()
        };
    }

    fn load_scene(&self) -> Scene {
        let aquarium1_key = HashMap::from([
            // TODO: Combine with obj.char types?
            (' ', vec![ new_floor() ]),
            ('#', vec![ new_floor(), new_wall() ]),
            ('^', vec![ new_floor(), new_hero_crab() ]),
            /*
            */
        ]);

        match self.current_sceneid {
            // TODO: Can we use idx++ instead of specifying each level number? Not immediately?
            BiobotSceneId::NewGame => Scene::from_splash_dialogue(
                //"Click or press [enter] to start.".to_string(),
                vec![
                    "Welcome to programming bot game!",
                ]
            ),

            BiobotSceneId::LevIntro(1) => {
                Scene::from_splash_string("Welcome to level 1!".to_string())
            },
            BiobotSceneId::LevPlay(1) => Scene::from_play_ascii_map(&[
                "################",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "################",
            ], aquarium1_key),
            BiobotSceneId::LevOutro(1) => {
                Scene::from_splash_string("Well done!! Goodbye from level 1".to_string())
            },

            BiobotSceneId::LevRetry(_levno) => {
                Scene::from_splash_string("Press [enter] to restart.".to_string())
            },
            BiobotSceneId::Win => {
                Scene::from_splash_string("Congratulations. You've completed all the levels. Press [enter] to play through again".to_string())
            },

            BiobotSceneId::LevIntro(_) => panic!("Loading LevIntro for level that can't be found."),
            BiobotSceneId::LevPlay(_) => panic!("Loading LevPlay for level that can't be found."),
            BiobotSceneId::LevOutro(_) => panic!("Loading LevOutro for level that can't be found."),
        }
    }
}
