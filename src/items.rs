// snake case is bad
#![allow(non_snake_case)]

use crate::userInput;
use crate::levelMaps;
use crate::player;
use crate::mobs;


// the type of item (weapon, ranged, light, etc..)
pub enum ItemType {
    Weapon,
    Light,
}


// a type allias for the dynamic usage; allows different implimentations
// for different types of items (Traits don't seem to be as staight forward for this application?)
pub type ItemUsage <const TOTAL_SIZE_1D: usize> = Box <dyn Fn (
    &player::Player <TOTAL_SIZE_1D>, &mut Vec <mobs::Entity>,
    &userInput::Input, &userInput::Direction)>;
pub type ItemHolding <const TOTAL_SIZE_1D: usize> = Box <dyn Fn (
    &mut levelMaps::MapData <'static, TOTAL_SIZE_1D>,
    &player::Player <TOTAL_SIZE_1D>)>;

pub struct Item <const TOTAL_SIZE_1D: usize> {
    pub name: String,
    pub itemType: ItemType,
    pub itemUsageFunc: ItemUsage <TOTAL_SIZE_1D>,
    pub itemHeldUpdateFunc: ItemHolding <TOTAL_SIZE_1D>,
}


// basic function impl's for the ItemUsage behavior (attacking)
// this system should allow a ton of flexibility
pub fn GetMeleWeaponUsageFunction <const TOTAL_SIZE_1D: usize> (damage: u8, range: usize) -> ItemUsage <TOTAL_SIZE_1D> {
    // generates a function
    Box::new(move |player, monsters, _action, direction| {

        // the position is moved based on attack direction to
        // check all positions for a monster
        let actionPosX = player.positionX;
        let actionPosY = player.positionY;

        // the direction of an action
        let (dirX, dirY) = userInput::GetDirectionOffsets(direction);

        // scaling outward based on the weapons range
        // checking onto of the player incase they're
        // stand ontop of a monster
        for rangeOffset in 0..=range {
            // checking if any mobs are being hit
            for mob in monsters.iter_mut() {
                if mob.CheckCollision(std::cmp::max(
                        actionPosX as isize + dirX * rangeOffset as isize, 0) as usize,
                         std::cmp::max(actionPosY as isize + dirY * rangeOffset as isize, 0) as usize){
                    
                    // attacking the monster
                    mob.Attack(damage);
                }
            }
        }

    })
}

// basic function impl's for the ItemUsage behavior (holding; for things like light)
pub fn GetHeldLightFunction <const TOTAL_SIZE_1D: usize> (lightStrength: usize) -> ItemHolding <TOTAL_SIZE_1D> {
    // generates a function
    Box::new(move |levelMap, player| {
        
        // updating the lighting
        levelMap.GenerateLightAura(&lightStrength, &player.positionX, &player.positionY);

    })
}


