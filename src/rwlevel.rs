mod lingo_to_json;
mod lingo_dsl;

use std::path::Path;

use lingo_dsl::Point;
use lingo_to_json::read_to_struct;
use serde_json::Value;

use crate::tile::Tile;

#[allow(unused)]
pub struct RWLevel {
    /// Based off the filename provided to RWLevel::load
    name: String,
    /// Room settings such as dimensions and medium
    meta: RWLevelMeta,
    /// Tiles composing the room
    /// Each index represents a layer of the level, starting with the foreground at index 0.
    /// Inner vectors are 2D arrays projected into 1D, progressing from Y = 0 to height for each X index
    tiles: [Vec<Tile>; 3],
}

#[allow(unused)]
pub struct RWLevelMeta {
    /// (Width, Height) dimensions of the level
    dimensions: (usize, usize),
}

impl RWLevel {

    /// Helper constant for indexing into the layer 1 (foreground) of tiles
    pub const L1_FG: usize = 0;
    /// Helper constant for indexing into the layer 2 (midground) of tiles
    pub const L2_MG: usize = 1;
    /// Helper constant for indexing into the layer 3 (background) of tiles
    pub const L3_BG: usize = 2;


    pub fn load<P: AsRef<Path>>(path: P) -> Option<Self> {
        let _json = read_to_struct(&path)?;

        let name = path.as_ref()
            .to_path_buf()
            .file_stem()
            .and_then(|ostr| ostr.to_owned().into_string().ok())?;

        let size: Point = _json._settings2.get("#size")
                .and_then(Value::as_str)
                .and_then(|val| val.parse().ok())
                .unwrap();

        let meta = RWLevelMeta {
            dimensions: (size.fst as usize, size.snd as usize)
        };
        let dim = meta.dimensions.0 * meta.dimensions.1;
        let tiles = [
            vec![Tile::default(); dim],
            vec![Tile::default(); dim],
            vec![Tile::default(); dim],
        ];

        Some(Self {
            name,
            meta,
            tiles,
        })
    }
}