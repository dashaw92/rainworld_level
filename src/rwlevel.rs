use std::path::PathBuf;

pub struct RWLevel {
    name: Option<String>,
    dimensions: (u16, u16),
    geometry: [Geometry; 3],
    tiles: [RWProp; 3],
    items: RWItem,
    effects: ()
}

impl RWLevel {
    pub fn load(path: PathBuf) -> Option<RWLevel> {
        None
    }
}

struct Geometry;
struct RWProp;
struct RWItem;