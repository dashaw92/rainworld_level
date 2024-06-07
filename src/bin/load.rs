use rainworld_level::RWLevel;

fn main() {
    let path = std::env::args().skip(1).next().expect("No file?");
    let proj = RWLevel::load(path).unwrap();
    dbg!(proj);
}