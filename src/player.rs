// snake case is bad
#![allow(non_snake_case)]

use crate::levelMaps;
use crate::userInput;
use crate::render;
use crate::items;
use crate::mobs;

// the different sprites for the player (technically, the player should always be lit)
const PLAYER_LIGHT_SPRITES: [&str; 10] = ["!!", "!!", "!!", "!!", "!!", "!!", "!!", "!!", "!!", "!!"];


// storing the data for a player
pub struct Player {
    pub positionX: usize,
    pub positionY: usize,
    pub health: u8,
    pub items: Vec <items::Item>,
    pub hand: usize,  // the index of the item being held
}

impl Player {
    pub fn new (startingPosX: usize, startingPosY: usize) -> Self {
        Player {
            positionX: startingPosX,
            positionY: startingPosY,
            health: 100,  // the base health of the player
            items: vec![
                // starter weapon
                items::Item {
                    name: "Rusty Bat".to_string(),
                    //itemType: items::ItemType::Weapon,
                    itemUsageFunc: items::GetMeleWeaponUsageFunction(10, 1, vec![
                        (items::GetFireEnchantment(0, 0), "Flame".to_string()),
                        (items::GetSharpnessEnchantment(0), "Sharpness".to_string()),
                    ]),
                    itemHeldUpdateFunc: items::GetHeldLightFunction(0),
                    damage: 10,
                    range: 1,
                    enchantments: vec!(),
                },
                // starter torch (the player has a base light level, but this has a greater one)
                items::Item {
                    name: "Charred Torch".to_string(),
                    //itemType: items::ItemType::Light,
                    // empty behavior (torches don't attack)
                    itemUsageFunc: std::sync::Arc::new(move |_player, _monsters, _action, _direction| {}),
                    itemHeldUpdateFunc: items::GetHeldLightFunction(6),
                    damage: 0,
                    range: 0,
                    enchantments: vec!(),
                }
            ],
            hand: 0,
        }
    }

    pub fn Attack (&mut self, damage: u8) {
        if damage > self.health {
            self.health = 0;
        } else {
            self.health -= damage;
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

    // updates the player
    pub fn Update <const NUMBER_OF_ITEMS: usize, const NUMBER_OF_ENCHANTMENTS: usize> (
                  &mut self, tileMap: &mut levelMaps::MapData <'static>,
                  monsters: &mut Vec <mobs::Entity>,
                  chestLootTable: &[(usize, items::Item); NUMBER_OF_ITEMS],
                  enchatnmentsTable: &[(usize, items::ItemEnchantment, String); NUMBER_OF_ENCHANTMENTS],
                  (playerInput, inputDirection): (&userInput::Input, &userInput::Direction)
            ) {
        
        // getting the direction of the action
        let (dirX, dirY): (isize, isize) = userInput::GetDirectionOffsets(inputDirection);

        // moving the player
        match playerInput {
            userInput::Input::Move => {
                let mut newPosX = self.positionY;
                let mut newPosY = self.positionX;

                // bounds check
                if self.positionX >= dirX.unsigned_abs() &&
                   self.positionX as isize + dirX < tileMap.mapSizeX as isize
                   {
                    newPosX = (self.positionX as isize + dirX) as usize;
                }
                if self.positionY >= dirY.unsigned_abs() &&
                   self.positionY as isize + dirY < tileMap.mapSizeY as isize
                   {
                    newPosY = (self.positionY as isize + dirY) as usize;
                }

                // collision check (the final tile and at least one corner tile need to be open)
                if tileMap.CheckTileCollision(newPosX, newPosY) && (
                        tileMap.CheckTileCollision(self.positionX, newPosY) ||
                        tileMap.CheckTileCollision(newPosX,        self.positionY)
                    ) {
                    self.positionX = newPosX;
                    self.positionY = newPosY;
                }
            },
            userInput::Input::Attack => {
                (self.items[self.hand].itemUsageFunc) (self, monsters, playerInput, inputDirection);
            },
            userInput::Input::Interact => {
                // attempting to interact with any chests
                let posX = (self.positionX as isize + dirX).unsigned_abs();
                let posY = (self.positionY as isize + dirY).unsigned_abs();

                levelMaps::TryLootTile(self, tileMap, chestLootTable, posX, posY);
                levelMaps::TryEnchantment(self, tileMap, enchatnmentsTable, posX, posY);
            },
            userInput::Input::HandSelect (newHand) => {
                if *newHand >= 1 && newHand <= &self.items.len()
                    {  self.hand = *newHand - 1;  }
            },
            userInput::Input::Inventory => {},
            _ => {}
        }
        
        // calling the holding update function for any held items
        (self.items[self.hand].itemHeldUpdateFunc) (tileMap, self);

        // updating the player's light on the tilemap
        tileMap.GenerateLightAura(&4, &self.positionX, &self.positionY);
    }

}

