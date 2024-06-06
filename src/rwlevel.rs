mod regex_convert;

use std::{fs::read_to_string, path::Path};

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
        let _lines: Vec<_> = read_to_string(&path).ok()?
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
    fn test_json_convert() {
        let line = "[#cameras: [point(20, 30)], #selectedCamera: 0, #quads: [[[0, 0], [0, 0], [0, 0], [0, 0]]], #Keys: [#n: 0, #d: 0, #e: 0, #p: 0], #lastKeys: [#n: 0, #d: 0, #e: 0, #p: 0]]";
        let fixed = convert_to_json(line);

        println!("{line}");
        println!("{fixed}");
    }

    #[test]
    fn test_full_convert() {
        let lines = r##"[[[[4, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[1, []], [0, []], [0, []]]], [[[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, [10, 2]], [0, []], [0, []]]], [[[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]]], [[[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]]], [[[5, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[9, []], [0, []], [0, []]]]]
[#lastKeys: [#L: 0, #m1: 0, #m2: 0, #w: 0, #a: 0, #s: 0, #d: 0, #c: 0, #q: 0], #Keys: [#L: 0, #m1: 0, #m2: 0, #w: 0, #a: 0, #s: 0, #d: 0, #c: 0, #q: 0], #workLayer: 1, #lstMsPs: point(8, -1), #tlMatrix: [[[[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]]], [[[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]]], [[[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]]], [[[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]]], [[[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]]]], #defaultMaterial: "Concrete", #toolType: "tile", #toolData: "TILE", #tmPos: point(2, 1), #tmSavPosL: [1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 28], #specialEdit: 0]
[#lastKeys: [], #Keys: [], #lstMsPs: point(0, 0), #effects: [], #emPos: point(1, 1), #editEffect: 0, #selectEditEffect: 0, #mode: "createNew", #brushSize: 5]
[#pos: point(567, 695), #rot: 0, #sz: point(50, 70), #col: 1, #Keys: [#m1: 0, #m2: 0, #w: 0, #a: 0, #s: 0, #d: 0, #r: 0, #f: 0, #z: 0, #m: 0], #lastKeys: [#m1: 0, #m2: 0, #w: 0, #a: 0, #s: 0, #d: 0, #r: 0, #f: 0, #z: 0, #m: 0], #lastTm: 301443808, #lightAngle: 180, #flatness: 1, #lightRect: rect(1000, 1000, -1000, -1000), #paintShape: "pxl"]
[#timeLimit: 4800, #defaultTerrain: 1, #maxFlies: 10, #flySpawnRate: 50, #lizards: [], #ambientSounds: [], #music: "NONE", #tags: [], #lightType: "Static", #waterDrips: 1, #lightRect: rect(0, 0, 1040, 800), #Matrix: []]
[#mouse: 1, #lastMouse: 1, #mouseClick: 0, #pal: 1, #pals: [[#detCol: color( 255, 0, 0 )]], #eCol1: 1, #eCol2: 2, #totEcols: 5, #tileSeed: 237, #colGlows: [0, 0], #size: point(5, 5), #extraTiles: [12, 3, 12, 5], #light: 1]
[#cameras: [point(20, 30)], #selectedCamera: 0, #quads: [[[0, 0], [0, 0], [0, 0], [0, 0]]], #Keys: [#n: 0, #d: 0, #e: 0, #p: 0], #lastKeys: [#n: 0, #d: 0, #e: 0, #p: 0]]
[#waterLevel: -1, #waterInFront: 1, #waveLength: 60, #waveAmplitude: 5, #waveSpeed: 10]
[#props: [], #lastKeys: [], #Keys: [], #workLayer: 1, #lstMsPs: point(0, 0), #pmPos: point(1, 1), #pmSavPosL: [], #propRotation: 0, #propStretchX: 1, #propStretchY: 1, #propFlipX: 1, #propFlipY: 1, #depth: 0, #color: 0]"##;

        lines.split('\n')
            .map(convert_to_json)
            .for_each(|line| println!("{line}"));
    }
}