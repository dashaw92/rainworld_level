mod regex_convert;

use std::{fs::read_to_string, path::{Path, PathBuf}};

use crate::tile::Tile;

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

pub struct RWLevelMeta {
    /// (Width, Height) dimensions of the level
    dimensions: (u16, u16),
}

impl RWLevel {

    /// Helper constant for indexing into the layer 1 (foreground) of tiles
    pub const L1_FG: usize = 0;
    /// Helper constant for indexing into the layer 2 (midground) of tiles
    pub const L2_MG: usize = 1;
    /// Helper constant for indexing into the layer 3 (background) of tiles
    pub const L3_BG: usize = 2;


    pub fn load<P: AsRef<Path>>(path: P) -> Option<Self> {
        let lines: Vec<_> = read_to_string(&path).ok()?
            .split("\n")
            .collect();

        //Convert project input to json
        //Get room dimensions from json
        //Deserialize tiles from json
        //???
        //profit

        let name = path.as_ref().to_path_buf().file_stem().and_then(|ostr| ostr.to_owned().into_string().ok())?;
        let meta = RWLevelMeta {
            dimensions: (0, 0)
        };
        let tiles = [vec![], vec![], vec![]];

        Some(Self {
            name,
            meta,
            tiles,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::rwlevel::regex_convert::convert_to_json;

    #[test]
    fn test_regex() {
        let line = "[#cameras: [point(20, 30)], #selectedCamera: 0, #quads: [[[0, 0], [0, 0], [0, 0], [0, 0]]], #Keys: [#n: 0, #d: 0, #e: 0, #p: 0], #lastKeys: [#n: 0, #d: 0, #e: 0, #p: 0]]";
        let fixed = convert_to_json(line);

        println!("{line}");
        println!("{fixed}");
    }
}