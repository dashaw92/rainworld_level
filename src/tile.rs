#[allow(unused)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Geometry {
    /// Passable tile
    Air = 0,
    /// Solid wall
    Wall = 1,
    /// Slope with the southwest half solid
    SlopeSW = 2,
    /// Slope with the southeast half solid
    SlopeSE = 3,
    /// Slope with the northeast half solid
    SlopeNW = 4,
    /// Slope with the northwest half solid
    SlopeNE = 5,
    /// Solid, but can be dropped through
    Floor = 6,
    /// Shortcut entrance?
    ShortcutEntrance = 7,
    /// Invisible wall
    Glass = 9,
}

#[allow(unused)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Feature {
    /// Horizontal pole that can be climbed on
    HPole = 1,
    /// Vertical pole that can be climbed on
    VPole = 2,
    /// Batflies can use these to travel across the room quickly
    Hive = 3,
    /// Small triangle pipe for traversing the room quickly. Accessible to slugcats
    ShortcutEntrance = 4,
    /// Connects shortcut entrances together
    ShortcutPathDot = 5,
    /// Pipe for travelling from one room to another
    RoomEntrance = 6,
    /// Spawns creatures based off the parent region's entries for the room
    CreatureDen = 7,
    /// Give a 60% chance of spawning a rock here
    Rock = 9,
    /// Give a 60% chance of spawning a spear here
    Spear = 10,
    /// Renders as a rough crack in the tile. Traversable by slugcats
    Fissure = 11,
    /// Forbid batflies from hanging here
    ForbidBatflyChain = 12,
    /// Spawn a garbage worm here
    GarbageWormDen = 13,
    /// Causes a waterfall to appear, beginning with this tile
    Waterfall = 18,
    /// Creature-exclusive shortcuts. Inaccessible to slugcats
    GopherHole = 19,
    /// Place wormgrass here
    WormGrass = 20,
    /// Used by all scavengers in the region to traverse rooms
    ScavengerHole = 21,
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct Tile {
    pub geometry: Geometry,
    pub features: Vec<Feature>,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            geometry: Geometry::Wall,
            features: vec![],
        }
    }
}

impl Tile {
    pub fn add_features(&mut self, features: &[Feature]) {
        for feature in features {
            if !self.features.contains(&feature) {
                self.features.push(*feature);
            }
        }
    }

    pub fn remove_features(&mut self, features: &[Feature]) {
        self.features.retain(|f| !features.contains(f));
    }

    pub fn features(&self) -> &[Feature] {
        &self.features
    }
}

impl Geometry {
    pub fn to_tile(&self) -> Tile {
        Tile {
            geometry: *self,
            features: vec![],
        }
    }

    pub fn from_data(data: u8) -> Option<Self> {
        Some(match data {
            0 => Self::Air,
            1 => Self::Wall,
            2 => Self::SlopeSW,
            3 => Self::SlopeSE,
            4 => Self::SlopeNW,
            5 => Self::SlopeNE,
            6 => Self::Floor,
            7 => Self::ShortcutEntrance,
            9 => Self::Glass,
            _ => return None,
        })
    }
}

impl Feature {
    pub fn from_data(data: u8) -> Option<Self> {
        Some(match data {
            1 => Self::HPole,
            2 => Self::VPole,
            3 => Self::Hive,
            4 => Self::ShortcutEntrance,
            5 => Self::ShortcutPathDot,
            6 => Self::RoomEntrance,
            7 => Self::CreatureDen,
            9 => Self::Rock,
            10 => Self::Spear,
            11 => Self::Fissure,
            12 => Self::ForbidBatflyChain,
            13 => Self::GarbageWormDen,
            18 => Self::Waterfall,
            19 => Self::GopherHole,
            20 => Self::WormGrass,
            21 => Self::ScavengerHole,
            _ => return None,
        })
    }
}