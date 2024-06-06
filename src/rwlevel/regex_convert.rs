
use std::borrow::Cow;

use once_cell::sync::Lazy;
use regex::Regex;

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

        match c {
            //The start of a value
            '[' => {
                let mut idx2 = idx;
    
                while chars[idx2] == '[' {
                    idx2 += 1;
                }

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
            _ => {}
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
pub(super) fn convert_to_json(input: &str) -> Cow<'_, str> {
    //Don't need to process any further. This line has no JSON-like structures to fix
    if !input.contains("#") {
        return Cow::Borrowed(input);
    }

    let work = jsonify_lingo_objects(input);
    let work = rename_keys(&work);
    let work = fix_color_point(&work);

    return Cow::Owned(wrap_in_braces(&work))
}