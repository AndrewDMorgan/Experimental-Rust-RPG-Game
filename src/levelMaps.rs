// snake case is bad
#![allow(non_snake_case)]

// an array of solid characters
const SOLID_TILES: [Tiles; 1] = [Tiles::Wall];


// an enum of the different tiles
#[derive(Hash, Eq, PartialEq)]
pub enum Tiles {
    Wall,
    Empty,
}


// stores map data
#[derive(Clone, Copy)]  // so an array of these can be constructed (it'll need to copy it for a unique instance to be placed in each element of the array)
pub struct MapData <'a, const TOTAL_SIZE_1D: usize> {
    pub mapSizeX: usize,
    pub mapSizeY: usize,
    pub tileMap: [&'a Tiles; TOTAL_SIZE_1D],
    pub lightMap: [usize; TOTAL_SIZE_1D],  // light levels aren't that large so u8 should be more memory efficent
}

impl <'a, const TOTAL_SIZE_1D: usize> MapData <'a, TOTAL_SIZE_1D> {
    // basic constructor to prevent the annoyance of trying to initialize the array
    pub fn new (sizeX: usize, sizeY: usize) -> Self {
        MapData {
            mapSizeX: sizeX,
            mapSizeY: sizeY,
            tileMap: [&Tiles::Empty; TOTAL_SIZE_1D],
            lightMap: [9usize; TOTAL_SIZE_1D],
            // light levels go from  0 - 9  with 9 being the darkest (wonderful indexes)
        }
    }

    // adds a value to a row
    pub fn EditTilemapElement (&mut self, tile: &'a Tiles, x: usize, y: usize) {
        if x < self.mapSizeX && y < self.mapSizeY {  // checking for bounds constraints (to prevent any errors)
            self.tileMap[x + y * self.mapSizeX] = tile;  // adding the value
        }
    }

    // checks for a neighbor's light compared to the current light for the GenerateLightAura function
    fn CheckLightNeigbor (&self, lightLevel: usize, pointX: &usize, pointY: &usize) -> bool {
        if *pointX < self.mapSizeX && *pointY < self.mapSizeY {  // this should prevent any errors when directly acsessing lightMap
            if self.lightMap[pointX + pointY * self.mapSizeX] > lightLevel {
                    return true;
                }
        }
        false
    }
    
    // generates a radial light around a point
    pub fn GenerateLightAura (&mut self, lightStrength: &usize, lightPosX: &usize, lightPosY: &usize) {
        // checking if the starting point is valid
        if !self.CheckLightNeigbor(0, lightPosX, lightPosY) {
                return;  // invalid position
                // no need to throw an error as this won't impact any other parts of the system
        }
        
        // start the light, propigate until fused out (till the light map is greater or equal where checking)
        let mut samplePoints: Vec <(usize, usize)> = vec!();
        let mut newSamplePoints: Vec <(usize, usize)> = vec!();  // so the list can copy over after and itteration
        samplePoints.push((*lightPosX, *lightPosY));

        for lightLevel in 0..=*lightStrength {
            for (pointX, pointY) in &samplePoints {
                // adding the light level for the given point
                let index = pointX + pointY * self.mapSizeX;
                self.lightMap[index] = lightLevel;
                if SOLID_TILES.contains(self.tileMap[index]) {
                    continue;  // this member needs it's light set, but it shouldn't proagate beyond
                    // without setting the light, walls won't render correctly, leading to the level's structure
                    // being known by the player before exploring the area
                }

                if pointX > &0 &&
                    Self::CheckLightNeigbor (self, lightLevel, &(pointX - 1), pointY
                ) {
                    newSamplePoints.push((pointX - 1, *pointY));
                }
                if Self::CheckLightNeigbor (self, lightLevel, &(pointX + 1), pointY) {
                    newSamplePoints.push((pointX + 1, *pointY));
                }
                if pointY > &0 && 
                    Self::CheckLightNeigbor (self, lightLevel, pointX, &(pointY - 1)
                ) {
                    newSamplePoints.push((*pointX, pointY - 1));
                }
                if Self::CheckLightNeigbor (self, lightLevel, pointX, &(pointY + 1)) {
                    newSamplePoints.push((*pointX, pointY + 1));
                }
            }

            // moving over the new array without copying any data (no reason to create another array inbetween when it can just be moved over item by item)
            samplePoints.clear();
            while let Some(value) = newSamplePoints.pop() {
                samplePoints.push(value);
            }
            if samplePoints.is_empty() {  break;  }
        }
    }
}


