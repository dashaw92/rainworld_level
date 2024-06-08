use std::{borrow::Cow, fs::read_to_string, path::Path};

use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{value::Index, Value};

#[derive(Debug)]
pub(crate) struct ProjectJson {
    pub(crate) _geom: Value,
    pub(crate) _tiles: Value,
    pub(crate) _effects: Value,
    pub(crate) _lights: Value,
    pub(crate) _settings1: Value,
    pub(crate) _settings2: Value,
    pub(crate) _cams: Value,
    pub(crate) _water: Value,
    pub(crate) _props: Value,
}

pub(super) fn read_to_struct<P: AsRef<Path>>(file: P) -> Option<ProjectJson> {
    let contents = read_to_string(&file).ok()?;
    convert_lines(&contents, '\r')
}

fn convert_lines(contents: &str, newline: char) -> Option<ProjectJson> {
    let Ok(maps): Result<[Value; 9], _> = contents.split(newline)
        .map(convert_to_json)
        .filter_map(|line| serde_json::from_str(&line).ok())
        .collect::<Vec<_>>()
        .try_into() 
    else {
        eprintln!("Invalid level editor project file!");
        return None;
    };

    let [ 
        _geom, _tiles, _effects, 
        _lights, _settings1, _settings2, 
        _cams, _water, _props 
    ] = maps;

    Some(ProjectJson {
        _geom,
        _tiles,
        _effects,
        _lights,
        _settings1,
        _settings2,
        _cams,
        _water,
        _props,
    })
}

static INITIAL_REPLACEMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(#\w+)"#)
        .expect("Failed to compile initial replacement regex")
});

static DATA_NAME_REPLACEMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"((color|point|rect)\([0-9, \-]+\))"#)
        .expect("Failed to compile data type regex")
});

/// Lingo uses exclusively `[` and `]` for structure
/// This function will convert specific uses of this to proper 
/// JSON format, replacing the `[`s with `{` and `]`s with `}`.
/// When a #key is encountered, an inner search is started
/// to find the start of the value associated to that key.
/// If the value uses the `[` format, and the next char
/// is the start of another #key, the `[` is replaced with `{`,
/// and a depth counter is incremented. For every `[` and `]` encountered
/// after that, the depth is incremented and decremented respectively.
/// Once the depth returns back to 0, that closing `]` is replaced with the
/// matching `}` to terminate the newly-formatted JSON object.
/// Quoting of #keys is done in later JSON conversion steps, and can be ignored
/// here. Nested objects are handled via the linear outer iteration of the chars.
fn jsonify_lingo_objects(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    'outer: for idx in 0..chars.len() {
        let c = chars[idx];

        if c == '[' {
            let mut idx2 = idx;

            //Handle nesting: keep traversing `[`s until we find the start
            //of actual data.
            while chars[idx2] == '[' {
                idx2 += 1;
            }

            //Not a nested object, just an array
            if chars[idx2] != '#' {
                continue 'outer;
            }

            //Replace the `[`.
            chars[idx2 - 1] = '{';

            //Depth tracks how many `[` and `]`s have been encountered.
            //Need to iterate through the chars from idx2 to the end until depth
            //returns to 0.
            let mut depth = 1;
            
            //Already replaced the opening `[`. Go to the next char.
            idx2 += 1;

            while idx2 < chars.len() {
                match chars[idx2] {
                    //Increment the depth to indicate we've entered
                    //another nested structure. Processing of nested
                    //objects is not needed, as it'll be fixed in a later
                    //iteration of the 'outer loop.
                    '[' => depth += 1,
                    //Decrement the depth.
                    //If depth is now 0, then this `]` is the matching
                    //bracket for the one starting the object we're
                    //reformatting.
                    ']' => {
                        depth -= 1;
                        if depth == 0 {
                            chars[idx2] = '}';
                            continue 'outer;
                        }
                    }
                    _ => {}
                }

                idx2 += 1;
            }
        }
    }

    chars.into_iter().collect()
}

/// Quote the keys in the input
fn rename_keys(input: &str) -> Cow<'_, str> {
    INITIAL_REPLACEMENT.replace_all(input, "\"$1\"")
}

/// Surround color and point types in quotes
fn fix_color_point(input: &str) -> Cow<'_, str> {
    DATA_NAME_REPLACEMENT.replace_all(input, "\"$1\"")
}

/// Replace the surrounding `[` and `]` with `{` and `}`
fn wrap_in_braces(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    
    if let Some(first) = chars.first_mut() {
        *first = '{';
    } else {
        panic!("Bad input - Couldn't replace the '[' at the start with '{{'");
    }
    
    if let Some(last) = chars.last_mut() {
        *last = '}';
    } else {
        panic!("Bad input - Couldn't replace the ']' at the end with '}}'");
    }

    chars.into_iter().collect()
}

/// Massage the rain world native project format into JSON
fn convert_to_json(input: &str) -> Cow<'_, str> {
    //Don't need to process any further. This line has no JSON-like structures to fix
    if !input.contains("#") {
        return Cow::Borrowed(input);
    }

    let work = jsonify_lingo_objects(input);
    let work = rename_keys(&work);
    let work = fix_color_point(&work);

    return Cow::Owned(wrap_in_braces(&work))
}

pub(crate) trait BetterIndexing {
    type Output;

    fn index(&self, path: &[&dyn Index]) -> Option<&Self::Output>;
}

impl BetterIndexing for Value {
    type Output = Value;
    fn index(&self, path: &[&dyn Index]) -> Option<&Self::Output> {
        if path.is_empty() {
            return Some(self)
        }

        self.get(path[0])?.index(&path[1..])
    }
}

#[cfg(test)]
mod tests {
    use crate::rwlevel::lingo_to_json::{convert_to_json, BetterIndexing};

    use super::convert_lines;

    #[test]
    fn test_json_convert() {
        let line = "[#cameras: [point(20, 30)], #selectedCamera: 0, #quads: [[[0, 0], [0, 0], [0, 0], [0, 0]]], #Keys: [#n: 0, #d: 0, #e: 0, #p: 0], #lastKeys: [#n: 0, #d: 0, #e: 0, #p: 0]]";
        let fixed = convert_to_json(line);

        println!("{line}");
        println!("{fixed}");
    }

    #[test]
    fn test_full_convert() {
        let lines = r##"[[[[4, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[2, []], [0, []], [0, []]]], [[[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]]], [[[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]]], [[[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]]], [[[5, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[0, []], [0, []], [0, []]], [[3, []], [0, []], [0, []]]]]
[#lastKeys: [#L: 0, #m1: 0, #m2: 0, #w: 0, #a: 0, #s: 0, #d: 0, #c: 0, #q: 0], #Keys: [#L: 0, #m1: 0, #m2: 0, #w: 0, #a: 0, #s: 0, #d: 0, #c: 0, #q: 0], #workLayer: 1, #lstMsPs: point(8, -1), #tlMatrix: [[[[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]]], [[[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]]], [[[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]]], [[[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]]], [[[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]], [[#tp: "default", #Data: 0], [#tp: "default", #Data: 0], [#tp: "default", #Data: 0]]]], #defaultMaterial: "Concrete", #toolType: "tile", #toolData: "TILE", #tmPos: point(2, 1), #tmSavPosL: [1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 28], #specialEdit: 0]
[#lastKeys: [], #Keys: [], #lstMsPs: point(0, 0), #effects: [], #emPos: point(1, 1), #editEffect: 0, #selectEditEffect: 0, #mode: "createNew", #brushSize: 5]
[#pos: point(567, 695), #rot: 0, #sz: point(50, 70), #col: 1, #Keys: [#m1: 0, #m2: 0, #w: 0, #a: 0, #s: 0, #d: 0, #r: 0, #f: 0, #z: 0, #m: 0], #lastKeys: [#m1: 0, #m2: 0, #w: 0, #a: 0, #s: 0, #d: 0, #r: 0, #f: 0, #z: 0, #m: 0], #lastTm: 301443808, #lightAngle: 180, #flatness: 1, #lightRect: rect(1000, 1000, -1000, -1000), #paintShape: "pxl"]
[#timeLimit: 4800, #defaultTerrain: 1, #maxFlies: 10, #flySpawnRate: 50, #lizards: [], #ambientSounds: [], #music: "NONE", #tags: [], #lightType: "Static", #waterDrips: 1, #lightRect: rect(0, 0, 1040, 800), #Matrix: []]
[#mouse: 1, #lastMouse: 1, #mouseClick: 0, #pal: 1, #pals: [[#detCol: color( 255, 0, 0 )]], #eCol1: 1, #eCol2: 2, #totEcols: 5, #tileSeed: 237, #colGlows: [0, 0], #size: point(5, 5), #extraTiles: [12, 3, 12, 5], #light: 1]
[#cameras: [point(20, 30)], #selectedCamera: 0, #quads: [[[0, 0], [0, 0], [0, 0], [0, 0]]], #Keys: [#n: 0, #d: 0, #e: 0, #p: 0], #lastKeys: [#n: 0, #d: 0, #e: 0, #p: 0]]
[#waterLevel: -1, #waterInFront: 1, #waveLength: 60, #waveAmplitude: 5, #waveSpeed: 10]
[#props: [], #lastKeys: [], #Keys: [], #workLayer: 1, #lstMsPs: point(0, 0), #pmPos: point(1, 1), #pmSavPosL: [], #propRotation: 0, #propStretchX: 1, #propStretchY: 1, #propFlipX: 1, #propFlipY: 1, #depth: 0, #color: 0]"##;

        let json = convert_lines(lines, '\n').unwrap();
        // let _size: Point = dbg!(
        //     dbg!(json._settings2.get("#size"))
        //         .and_then(Value::as_str)
        //         .and_then(|val| val.parse().ok())
        //         .unwrap()
        // );

        // let meta = RWLevelMeta {
        //     dimensions: (_size.fst as usize, _size.snd as usize)
        // };

        // let _tiles = load_tiles(&json, &meta);
        // dbg!(&_tiles[0]);

        dbg!(json._geom.index(&[&0, &1, &1, &1]).unwrap());
    }
}