#[derive(Clone, Debug)]
pub struct DialogueLine {
    pub tex_path: String,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct Dialogue {
    pub entries: Vec<DialogueLine>,
}
