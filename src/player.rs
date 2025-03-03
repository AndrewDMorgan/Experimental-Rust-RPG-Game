// snake case is bad
#![allow(non_snake_case)]

use crate::render;

// the different sprites for the player (technically, the player should always be lit)
const PLAYER_LIGHT_SPRITES: [&str; 10] = ["!!", "!!", "!!", "!!", "!!", "!!", "!!", "!!", "!!", "!!"];


// storing the data for a player
pub struct Player {
    pub positionX: usize,
    pub positionY: usize,
    pub health: u8,
}

impl Player {
    pub fn new (startingPosX: usize, startingPosY: usize) -> Self {
        Player {
            positionX: startingPosX,
            positionY: startingPosY,
            health: 100,  // the base health of the player
        }
    }

    // the render function for the player
    pub fn GetRender (&self) -> render::DynamicRenderFunction {
        // generates a function that either:
        // returns None when the position given doesn't match the player's position
        let posX = self.positionX;
        let posY = self.positionY;

        // returns the static str list when the positions match
        Box::new(move |(x, y)| {
            if x == posX && y ==  posY {
                return Some(PLAYER_LIGHT_SPRITES);
            } None
        })
    }
}

