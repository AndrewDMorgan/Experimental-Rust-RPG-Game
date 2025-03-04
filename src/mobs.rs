// snake case is bad
#![allow(non_snake_case)]

use crate::render;
use crate::game;

// the different sprites for zombies
const ZOMBIE_LIGHT_SPRITES: [&str; 10] = ["??", "??", "??", "??", "??", "??", "??", "**", "**", "##"];


// traits or whatever they're called are probably gonna be
// needed to allow standardized behavior between mob types
pub struct Entity {
    pub positionX: usize,
    pub positionY: usize,
    pub health: u8,
    pub damage: u8,
    // functions (so that specific function behaviors can be passed in)
    pub entityAi: fn (&mut Entity, &mut game::GameData, Reaction),
    pub renderer: fn (&Entity) -> render::DynamicRenderFunction,
}


impl Entity {
    // the constructor
    pub fn new (startingPosX: usize, startingPosY: usize, mobHealth: u8, mobDamange: u8, mobAi: fn (&mut Entity, &mut game::GameData, Reaction), mobRenderer: fn (&Entity) -> render::DynamicRenderFunction) -> Self {
        Entity {
            positionX: startingPosX,
            positionY: startingPosY,
            health: mobHealth,  // the base health of the player
            damage: mobDamange,
            entityAi: mobAi,
            renderer: mobRenderer
        }
    }

    pub fn GetRenderer (&self) -> render::DynamicRenderFunction {
        (self.renderer)(self)
    }

    pub fn CheckCollision (&self, x: usize, y: usize) -> bool {
        self.positionX == x && self.positionY == y
    }

    pub fn Attack (&mut self, damage: u8) {
        if damage > self.health {
            // whoever is storing the mob needs to check for mob death
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }

    pub fn Alive (&self) -> bool {
        self.health > 0
    }
}


// reaction/action types (so that a single ai function can be used with a match statement to determine behavior)
pub enum Reaction {
    Attacked,
    Died,  // deals with anything necessary before deletion
}


// ==============================================================================================================
//                                       Mob Behavior & Rendering Functions
// ==============================================================================================================

pub fn ZombieAi (entity: &mut Entity, gameData: &mut game::GameData, reaction: Reaction) {
    
    // moving

    
    // attacking


    // reacting to any events
    match reaction {
        Reaction::Attacked => {
            //
        }
        Reaction::Died     => {
            //
        }
    }
}

// the render function for the zombie
pub fn GetZombieRender (entity: &Entity) -> render::DynamicRenderFunction {
    // generates a function that either:
    // returns None when the position given doesn't match the zombie's position
    let posX = entity.positionX;
    let posY = entity.positionY;

    // returns the static str list when the positions match
    Box::new(move |(x, y)| {
        if x == posX && y ==  posY {
            return Some(ZOMBIE_LIGHT_SPRITES);
        } None
    })
}

