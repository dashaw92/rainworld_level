#[allow(unused)]
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
    SlopeNE = 4,
    /// Slope with the northwest half solid
    SlopeNW = 5,
    /// Solid, but can be dropped through
    Floor = 6,
    /// Invisible wall
    Glass = 9,
}

#[allow(unused)]
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
    features: Vec<Feature>,
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
}