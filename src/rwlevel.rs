pub mod lingo_to_json;
mod lingo_dsl;

use std::{array, path::Path};

use lingo_dsl::Point;
use lingo_to_json::{read_to_struct, BetterIndexing, ProjectJson};
use serde_json::Value;

use crate::{effect::Effect, tile::{Feature, Geometry, Tile}};

#[allow(unused)]
#[derive(Debug)]
pub struct RWLevel {
    /// Based off the filename provided to RWLevel::load
    name: String,
    /// Room settings such as dimensions and medium
    meta: RWLevelMeta,
    /// Tiles composing the room
    /// Each index represents a layer of the level, starting with the foreground at index 0.
    /// Inner vectors are 2D arrays projected into 1D, progressing from Y = 0 to height for each X index
    tiles: [Vec<Tile>; 3],
    effects: Vec<Effect>,
}

#[allow(unused)]
#[derive(Debug)]
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
        let json = read_to_struct(&path)?;

        let name = path.as_ref()
            .to_path_buf()
            .file_stem()
            .and_then(|ostr| ostr.to_owned().into_string().ok())?;

        let size: Point = json._settings2.get("#size")
                .and_then(Value::as_str)
                .and_then(|val| val.parse().ok())
                .unwrap();

        let meta = RWLevelMeta {
            dimensions: (size.fst as usize, size.snd as usize)
        };
        
        let tiles = load_tiles(&json, &meta);
        let effects = load_effects(&json);

        Some(Self {
            name,
            meta,
            tiles,
            effects,
        })
    }
}

fn load_tiles(json: &ProjectJson, meta: &RWLevelMeta) -> [Vec<Tile>; 3] {
    let dim = meta.dimensions.0 * meta.dimensions.1;
    let w = meta.dimensions.0;
    let mut tiles = array::from_fn(|_| vec![Tile::default(); dim]);

    for x in 0..meta.dimensions.0 {
        for y in 0..meta.dimensions.1 {
            for layer in 0..3 {
                let tile = json._geom.index(&[&x, &y, &layer])
                    .and_then(|v| v.as_array())
                    .expect("Bad tile entry in level");

                let _geom_json = tile.get(0).and_then(|v| v.as_u64()).expect("Bad geometry entry in level") as u8;

                let geometry: Geometry = Geometry::from_data(_geom_json).expect("Bad geometry type");
                let features: Vec<Feature> = tile.get(1)
                    .and_then(|v| v.as_array())
                    .expect("Bad feature entry in level")
                    .iter()
                    .filter_map(|val| val.as_u64())
                    .filter_map(|val| val.try_into().ok())
                    .filter_map(Feature::from_data)
                    .collect();

                tiles[layer][y * w + x] = Tile {
                    geometry,
                    features,
                };
            }
        }
    }

    tiles
}

fn load_effects(json: &ProjectJson) -> Vec<Effect> {
    let mut effects = Vec::new();
    let Some(arr) = json._effects.get("#effects")
        .and_then(|v| v.as_array())
    else {
        return effects;
    };

    for entry in arr {
        let name = entry.get("#nm")
            .and_then(|v| v.as_str())
            .expect("Missing effect name #nm")
            .to_owned();
        let mtrx = entry.get("#mtrx")
            .and_then(|v| v.as_array())
            .expect("Missing #mtrx");
        let opts = entry.get("#Options")
            .expect("Missing #Options");

        let matrix = mtrx.iter()
            .map(|v_arr| {
                let pctgs = v_arr.as_array().expect("Array is bad!");

                pctgs.iter()
                    .filter_map(|entry| entry.as_f64())
                    .collect()
            })
            .collect();

        effects.push(Effect {
            name,
            matrix,
            options: opts.clone(),
        });
    }

    effects
}