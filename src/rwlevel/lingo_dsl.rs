use std::str::FromStr;

#[derive(Debug)]
pub(in crate::rwlevel) struct Point {
    fst: isize,
    snd: isize,
}

#[derive(Debug)]
pub(in crate::rwlevel) struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
pub(in crate::rwlevel) struct Rect {
    top: isize,
    left: isize,
    bottom: isize,
    right: isize,
}

fn collect<T: FromStr>(prefix: &str, s: &str, expected_len: usize) -> Option<Vec<T>> {
    let vals: Vec<Option<T>> = s.strip_prefix(prefix)?
        .strip_suffix(")")?
        .split(',')
        .map(|val| val.trim().parse::<T>().ok())
        .collect();

    if vals.len() != expected_len {
        return None
    }

    // FromIterator has a specialization for Vec<Option<T>>
    // to collect into Option<Vec<T>>; Any None will yield
    // None for the entire collection, only returning
    // Some(Vec) if all elements are Some
    vals.into_iter().collect()
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = collect("point(", s, 2).ok_or(())?;

        Ok(Point {
            fst: vals[0],
            snd: vals[1],
        })
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = collect("color(", s, 3).ok_or(())?;

        Ok(Color {
            red: vals[0],
            green: vals[1],
            blue: vals[2],
        })
    }
}

impl FromStr for Rect {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = collect("rect(", s, 4).ok_or(())?;

        Ok(Rect {
            top: vals[0],
            left: vals[1],
            bottom: vals[2],
            right: vals[3]
        })
    }
}