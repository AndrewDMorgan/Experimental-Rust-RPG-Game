// snake case is bad
#![allow(non_snake_case)]

use crate::userInput;
use crate::levelMaps;
use crate::player;
use crate::mobs;


// the type of item (weapon, ranged, light, etc..)
//#[derive(Clone)]
/*pub enum ItemType {
    Weapon,
    Light,
}*/


// a type allias for the dynamic usage; allows different implimentations
// for different types of items (Traits don't seem to be as staight forward for this application?)
pub type ItemUsage = std::sync::Arc <dyn Fn (
    &player::Player, &mut Vec <mobs::Entity>,
    &userInput::Input, &userInput::Direction
)>;
pub type ItemEnchantment = std::sync::Arc <dyn Fn (
    // takes in the current initial damage value, and the mob
    // (calculates it's own damage/damge effects)
    &u8, &mut mobs::Entity
)>;

pub type ItemHolding = std::sync::Arc <dyn Fn (
    &mut levelMaps::MapData <'static>,
    &player::Player)>;


#[derive(Clone)]
pub struct Item {
    pub name: String,
    //pub itemType: ItemType,
    pub itemUsageFunc: ItemUsage,
    pub itemHeldUpdateFunc: ItemHolding,

    // more weapon specific
    pub damage: u8,
    pub range: usize,
    pub enchantments: Vec <(ItemEnchantment, String)>,
}


// applies the on fire effect
pub fn GetFireEnchantment (strength: u8, durationFrames: usize) -> ItemEnchantment {
    std::sync::Arc::new(move |_damage, mob| {
        mob.mobEffects.push((strength, durationFrames, "On Fire".to_string()));
    })
}

// increases the base damage by a constant
pub fn GetSharpnessEnchantment (strength: u8) -> ItemEnchantment {
    std::sync::Arc::new(move |_damage, mob| {
        mob.Attack(strength);
    })
}


// basic function impl's for the ItemUsage behavior (attacking)
// this system should allow a ton of flexibility
// the enchaments I believe are being moved into the function; shouldn't matter though
pub fn GetMeleWeaponUsageFunction (
        damage: u8, range: usize,
        enchantments: Vec <(ItemEnchantment, String)>
    ) -> ItemUsage {
    
    // generates a function
    std::sync::Arc::new(move |player, monsters, _action, direction| {

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
                    for (enchantment, _name) in &enchantments {
                        (enchantment) (&damage, mob);
                    }
                }
            }
        }

    })
}

// basic function impl's for the ItemUsage behavior (holding; for things like light)
pub fn GetHeldLightFunction (lightStrength: usize) -> ItemHolding {
    // generates a function
    std::sync::Arc::new(move |levelMap, player| {
        
        // updating the lighting
        levelMap.GenerateLightAura(&lightStrength, &player.positionX, &player.positionY);

    })
}


