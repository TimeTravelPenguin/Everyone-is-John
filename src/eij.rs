pub struct Player {
    pub name: String,
    pub willpower: i32,
    pub obsessions: Vec<Obsession>,
}

pub struct Obsession {
    pub name: String,
    pub description: String,
    pub level: ObsessionLevel,
}

pub enum ObsessionLevel {
    One,
    Two,
    Three,
}
