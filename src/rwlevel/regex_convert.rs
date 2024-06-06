
use std::borrow::Cow;

use once_cell::sync::Lazy;
use regex::Regex;

static INITIAL_REPLACEMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(#\w+)"#)
        .expect("Failed to compile initial replacement regex")
});

static COLOR_POINT_REPLACEMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"((color|point)\([0-9, ]+\))"#)
        .expect("Failed to compile color/point regex")
});

fn rename_keys(input: &str) -> Cow<'_, str> {
    INITIAL_REPLACEMENT.replace_all(input, "\"$1\"")
}

fn fix_color_point(input: &str) -> Cow<'_, str> {
    COLOR_POINT_REPLACEMENT.replace_all(input, "\"$1\"")
}

pub(super) fn convert_to_json(input: &str) -> String {
    let work = rename_keys(input);
    let work = fix_color_point(&work);

    fix_arrays(&work)
}

pub(super) fn fix_arrays(input: &str) -> String {
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

    let string: String = chars.into_iter().collect();
    string
        .replace("[[", "{")
        .replace("]],", "},")
}