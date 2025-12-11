use std::collections::HashMap;

use super::objs::*;

use tile_engine::for_gamedata::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ProgpuzzSceneId {
    LevCodingArena(u16), // Level index from 1 to N. (Not 0 to N-1.)
    Win,
}

#[derive(Debug)]
pub struct ProgpuzzLevset {
    pub current_levid: ProgpuzzSceneId,
}

impl ProgpuzzLevset {
    pub fn new() -> ProgpuzzLevset {
        let s_: Option<String> = tile_engine::infra::get_arg("--start-at=");
        let i_: Option<u16> = s_.map(|s| s.parse::<u16>().ok()).flatten();
        let starting_lev_num : u16 = i_.unwrap_or(1);
        ProgpuzzLevset { current_levid: ProgpuzzSceneId::LevCodingArena(starting_lev_num) }
    }

    pub fn goto_level(&mut self, lev_idx: u16) {
        self.current_levid = ProgpuzzSceneId::LevCodingArena(lev_idx);
    }

    pub fn get_current_level(&self) -> u16 {
        match self.current_levid {
            ProgpuzzSceneId::LevCodingArena(levnum) => levnum,
            ProgpuzzSceneId::Win => self.num_levels()
        }
    }

    pub fn advance_scene(&mut self, continuation: SceneConclusion) {
        self.current_levid = match (self.current_levid, continuation) {
            (ProgpuzzSceneId::LevCodingArena(levnum), SceneConclusion::Succeed) if levnum >= self.levels().len() as u16 => ProgpuzzSceneId::Win,
            (ProgpuzzSceneId::LevCodingArena(levnum), SceneConclusion::Succeed) => ProgpuzzSceneId::LevCodingArena(levnum+1),
            (ProgpuzzSceneId::Win, SceneConclusion::Continue) => Self::new().current_levid,
            _ => panic!()
        };
    }

    fn levels(&self) -> Vec<CodingArena<super::game_logic::ProgpuzzGameLogic>> {
        let progpuzz_key = HashMap::from([
            // NB: Better to move this into obj? Combined with obj.char types?
            (' ', vec![ new_floor() ]),
            ('#', vec![ new_floor(), new_wall() ]),
            ('^', vec![ new_floor(), new_progbot(CoordDelta::from_xy(0, -1)) ]),
            ('>', vec![ new_floor(), new_progbot(CoordDelta::from_xy(1, 0)) ]),
            ('v', vec![ new_floor(), new_progbot(CoordDelta::from_xy(0, 1)) ]),
            ('<', vec![ new_floor(), new_progbot(CoordDelta::from_xy(-1, 0)) ]),
            ('w', vec![ new_door_win() ]),
            /*
            */
        ]);

        // TODO: Separate debug levels..?
        let lev1_coding = if std::env::args().collect::<Vec<_>>().contains(&"--debug-coding=A".to_string()) {
            let mut coding;
            {
                use supply_ops::*;
                coding = Coding::from_vec(&[
                    (F, 1),
                    (L, 1),
                    (R, 1),
                    (group, 1),
                    (x2, 1),
                ]);
            }

            {
                use prog_ops::*;
                coding.prog = Prog::from(vec![group, x2]);
                coding.prog.instrs[0] = Instr::Parent(ParentOpcode::group, Prog::from("R"));
                coding.prog.instrs[1] = Instr::Parent(ParentOpcode::x2, Prog::from("x2"));
                coding.prog.instrs[1][0] = Instr::Parent(ParentOpcode::x2, Prog::from("F"));
            }

            coding
        } else if std::env::args().collect::<Vec<_>>().contains(&"--debug-coding=B".to_string()) {
            use supply_ops::*;
            Coding::from_vec(&[
                (F, 2),
                (L, 2),
                (R, 2),
                (group, 2),
                (x2, 2),
                (LOOP, 2),
                (Else, 2),
            ])
        } else {
            Coding::from_vec(&[(F, 2), (L, 0), (R, 0)])
        };

        use supply_ops::*;
        vec![
            // TODO: Avoid needing to specify HEIGHT explicitly.
            CodingArena::new::<16>(
                Arena::from_map_and_key(&[
                    "################",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#     w        #",
                    "#              #",
                    "#     ^        #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "################",
                ], progpuzz_key.clone()),
                lev1_coding,
            ),
            CodingArena::new::<16>(
                Arena::from_map_and_key(&[
                    "################",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#       w      #",
                    "#              #",
                    "#              #",
                    "#     ^        #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "################",
                ], progpuzz_key.clone()),
                Coding::from_vec(&[(F, 6), (L, 3), (R, 3)]),
            ),
            CodingArena::new::<16>(
                Arena::from_map_and_key(&[
                    "################",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#     #  w     #",
                    "#              #",
                    "#              #",
                    "#     ^        #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "################",
                ], progpuzz_key.clone()),
                Coding::from_vec(&[(F, 6), (L, 3), (R, 3)]),
            ),
            CodingArena::new::<16>(
                Arena::from_map_and_key(&[
                    "################",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#        w     #",
                    "#       #      #",
                    "#              #",
                    "#      #       #",
                    "#              #",
                    "#     #        #",
                    "#              #",
                    "#              #",
                    "#     ^        #",
                    "#              #",
                    "#              #",
                    "################",
                ], progpuzz_key.clone()),
                Coding::from_vec(&[(F, 6), (L, 3), (R, 3), (LOOP, 1)]),
            ),
            CodingArena::new::<16>(
                Arena::from_map_and_key(&[
                    "################",
                    "#              #",
                    "#              #",
                    "#              #",
                    "# # ### #      #",
                    "#^#     #      #",
                    " #  # # #      #",
                    " w#   # #      #",
                    "# ###          #",
                    "#       #      #",
                    " # #### #      #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "################",
                ], progpuzz_key.clone()),
                Coding::from_vec(&[(F, 9), (L, 1), (R, 1), (LOOP, 1)]),
            ),



            CodingArena::new::<16>(
                Arena::from_map_and_key(&[
                    "                ",
                    " ############## ",
                    " ############## ",
                    " ############## ",
                    " ##### ######## ",
                    " #####   ###### ",
                    " ####  #  ##### ",
                    " ####^### ##### ",
                    " ######w  ##### ",
                    " ######  ###### ",
                    " ############## ",
                    " ############## ",
                    " ############## ",
                    " ############## ",
                    " ############## ",
                    "                ",
                ], progpuzz_key.clone()),
                Coding::from_vec(&[(F, 5), (L, 2), (R, 2), (LOOP, 1)]),
            ),
            CodingArena::new::<16>(
                Arena::from_map_and_key(&[
                    "################",
                    "#              #",
                    "#   #          #",
                    "#         #    #",
                    "#              #",
                    "#   #          #",
                    "# #           w#",
                    "#      #########",
                    "#  #           #",
                    "#     #        #",
                    "#          #   #",
                    "#  #           #",
                    "#^             #",
                    "#        #     #",
                    "#              #",
                    "################",
                ], progpuzz_key.clone()),
                Coding::from_vec(&[(F, 13), (L, 2), (R, 2), (LOOP, 1)]),
            ),
            CodingArena::new::<16>(
                Arena::from_map_and_key(&[
                    "################",
                    "#              #",
                    "##             #",
                    "#              #",
                    "#   #   # #    #",
                    "#          #   #",
                    "#     ##       #",
                    "#  #     #     #",
                    "#    #^# #     #",
                    "#  #     #     #",
                    "#  #  ###  #   #",
                    "#              #",
                    "#   ###   #    #",
                    "####     #     #",
                    "#w             #",
                    "################",
                ], progpuzz_key.clone()),
                Coding::from_vec(&[(F, 15), (L, 3), (R, 3), (LOOP, 1)]),
            ),
            CodingArena::new::<16>(
                Arena::from_map_and_key(&[
                    "################",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#    >         #",
                    "#     #        #",
                    "#     w        #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "################",
                ], progpuzz_key.clone()),
                Coding::from_vec(&[(F, 3), (L, 0), (R, 2), (x2, 3)]), // Still need to tweak
            ),
            CodingArena::new::<16>(
                Arena::from_map_and_key(&[
                    "################",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "###            #",
                    "# #            #",
                    "#w#   v        #",
                    "# #            #",
                    "# #            #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "################",
                ], progpuzz_key.clone()),
                Coding::from_vec(&[(F, 6), (L, 3), (R, 0), (x2, 5)]),
            ),



            CodingArena::new::<16>(
                Arena::from_map_and_key(&[
                    "################",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "# #            #",
                    "# #            #",
                    "#w#   v        #",
                    "# #            #",
                    "###            #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "################",
                ], progpuzz_key.clone()),
                Coding::from_vec(&[(F, 6), (L, 3), (R, 0), (x2, 5)]),
            ),
            CodingArena::new::<16>(
                Arena::from_map_and_key(&[
                    "################",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#       #      #",
                    "#       #      #",
                    "#       #      #",
                    "#   ^   #w     #",
                    "#       #      #",
                    "#       #      #",
                    "#       #      #",
                    "#       #      #",
                    "#       #      #",
                    "#       #      #",
                    "#       #      #",
                    "################",
                ], progpuzz_key.clone()),
                Coding::from_vec(&[(F, 2), (L, 2), (R, 2), (Else, 1), (LOOP, 1)]),
            ),
        ]
    }

    pub fn num_levels(&self) -> u16 {
        self.levels().len() as u16
    }

    pub fn load_scene(&self) -> Scene<super::game_logic::ProgpuzzGameLogic> {
        // NB: Would like to implement thin walls between squares, not walls filling whole squares.
        match self.current_levid {
            ProgpuzzSceneId::LevCodingArena(n) => Scene::CodingArena(self.levels()[n as usize -1].clone()),
            ProgpuzzSceneId::Win => {
                Scene::from_splash_string("Congratulations. You've completed all the levels. Press [enter] to play through again".to_string())
            },
        }
    }
}
