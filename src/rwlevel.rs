use std::path::PathBuf;

use crate::tile::Tile;

pub struct RWLevel {
    name: Option<String>,
    meta: RWLevelMeta,
    tiles: [Tile; 3],
}

pub struct RWLevelMeta {
    dimensions: (u16, u16),
}

impl RWLevel {
    pub fn load(path: PathBuf) -> Option<RWLevel> {
        None
    }
}