// snake case is bad
#![allow(non_snake_case)]

use crate::levelMaps;
use crate::player;
use crate::render;

// the different sprites for zombies
const ZOMBIE_LIGHT_SPRITES: [&str; 10] = ["??", "??", "??", "??", "??", "??", "??", "**", "**", "##"];

pub type EntityAI = fn (
    &mut Entity, &mut player::Player,
    &levelMaps::MapData);


// traits or whatever they're called are probably gonna be
// needed to allow standardized behavior between mob types
pub struct Entity {
    pub positionX: usize,
    pub positionY: usize,
    pub health: u8,
    pub damage: u8,
    // functions (so that specific function behaviors can be passed in)
    pub entityAi: EntityAI,
    pub renderer: fn (&Entity) -> render::DynamicRenderFunction,
    pub mobEffects: Vec <(u8, usize, String)>,  // damage, lifetime left, name
}


impl Entity {
    // the constructor
    pub fn new (
            startingPosX: usize,
            startingPosY: usize,
            mobHealth: u8, mobDamange: u8,
            mobAi: EntityAI,
            mobRenderer: fn (&Entity) -> render::DynamicRenderFunction
        ) -> Self {
        
        Entity {
            positionX: startingPosX,
            positionY: startingPosY,
            health: mobHealth,  // the base health of the player
            damage: mobDamange,
            entityAi: mobAi,
            renderer: mobRenderer,
            mobEffects: vec!(),
        }
    }

    pub fn Update (&mut self) {
        // applying any effects
        let mut accumulativeDamage = 0u8;
        let mut validEffects: Vec <(u8, usize, String)> = vec!();
        for (damage, duration, _name) in &self.mobEffects {
            if *duration >= 1 {  validEffects.push((*damage, duration - 1, _name.clone()));  }
            accumulativeDamage += *damage;
        }

        // applying all the damage done
        self.Attack(accumulativeDamage);
        
        // moving the valid effects over to the entities active effects
        self.mobEffects.clear();
        while let Some(effect) = validEffects.pop() {
            self.mobEffects.push(effect);
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


// ==============================================================================================================
//                                       Mob Behavior & Rendering Functions
// ==============================================================================================================

pub fn ZombieAi (
        entity: &mut Entity,
        player: &mut player::Player,
        map: &levelMaps::MapData)
    {
    
    // moving

    // evaluating the quality of each direction
    // (9 - light) * player_dst   (i think this should work?)
    let mut bestQuality = None;  // lower is better
    let (mut bestOffsetX, mut bestOffsetY) = (0isize, 0isize);
    for (offsetX, offsetY) in [
        (-1isize, 0isize), (1isize, 0isize), (0isize, -1isize), (0isize, 1isize),
        (1isize, 1isize), (-1isize, 1isize), (1isize, -1isize), (-1isize, -1isize)
    ] {
        // checking if the position is valid
        if entity.positionX <= offsetX.unsigned_abs() || entity.positionY <= offsetY.unsigned_abs() {
            continue;
        }

        let posX = (entity.positionX as isize + offsetX) as usize;
        let posY = (entity.positionY as isize + offsetY) as usize;
        if !map.CheckTileCollision(posX, posY) { continue; }
        if posX == player.positionX && posY == player.positionY{
            // attacking the player and then not moving
            player.Attack(entity.damage);

            bestQuality = None;  // the zombie won't move
            break;  // the mob shouldn't walk onto the player
        }

        let difX = (player.positionX as isize - posX as isize).unsigned_abs();
        let difY = (player.positionY as isize - posY as isize).unsigned_abs();
        let playerDst = difX*difX + difY*difY;  // does this need a sqrt?

        if let Some(light) = map.GetLightLevel(
            posX,
            posY,
        ) {
            // i think this works for now?
            let quality = playerDst * (9 - light);  // does light need to be square? (9 - light)(9 - light)

            // updating the best quality
            if quality < bestQuality.unwrap_or(999999usize) {
                bestQuality = Some(quality);
                bestOffsetX = offsetX;
                bestOffsetY = offsetY;
            }
        }
    }

    // moving the zombie
    if let Some(_quality) = bestQuality {
        // these have already been varified to be unsigned (the conversion won't crash)
        entity.positionX = (entity.positionX as isize + bestOffsetX) as usize;
        entity.positionY = (entity.positionY as isize + bestOffsetY) as usize;
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

