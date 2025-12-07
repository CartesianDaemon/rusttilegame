use super::*;

#[derive(Clone, Debug)]
pub struct DialogueLine {
    pub tex_path: String,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct Dialogue {
    pub entries: Vec<DialogueLine>,
}

/// Splash message, any key to continue. E.g. New level, game over.
///
/// TODO: A name more like "announcement" or "interstitial" or..?
/// TODO: Merge rendering of single string with dialogue?
#[derive(Clone, Debug)]
pub struct Splash {
    /// Next stage to go to after continue.

    // Text for current interstitial screen. Only in Splash.
    pub splash_text: String,
    pub dialogue: Dialogue, // If this works, will replace splash_text
    ready_for_next_level: Option<SceneConclusion>,
}

impl Splash
{
    pub fn from_string(txt: String) -> Splash {
        Splash {
            splash_text: txt,
            dialogue: Dialogue { entries: vec![]},
            ready_for_next_level: None,
        }
    }

    pub fn from_dialogue(entries: Vec<&str>) -> Splash {
        Splash {
            splash_text: "".to_string(),
            dialogue: Dialogue { entries: entries.iter().map(|x| DialogueLine {tex_path: "".to_string(), text: x.to_string()} ).collect() },
            ready_for_next_level: None,
        }
    }
}

impl BaseScene for Splash {
    fn advance(&mut self, _cmd: InputCmd) {
        self.ready_for_next_level = Some(SceneConclusion::Continue)
    }

    fn ready_for_next_level(&self) -> Option<SceneConclusion> {
        self.ready_for_next_level
    }
}
