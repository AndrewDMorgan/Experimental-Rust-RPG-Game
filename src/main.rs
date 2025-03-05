/*

I think my approach to items and for entities would fall under an
ECS-style program system (item data and it's behavior is seperated)
Entity-Component-System (ECS) which is used via arrays of function pointers

The map rendering is working!!
Light calculations are working!!
User inputs are working!
Entity rendering is working!
    and the camera position/bound box was fixed!
Entities can be killed!
Player actions work! & dynamic items were added!
Mob AI!
Enchantments!
Effects!
Chests! (still need to test)

just need:
 * item drops?
 * levels
 * anything else I think of


because of the dynamic function system that seperates items
from their behaviors (and uses closures to do so)
the game totally needs to impliment a precedural item system.
Every single aspect can be done; precedural behavior, name, etc...
It'll just mean that I need to keep implimenting things this way

*/

// snake case is objectively bad
#![allow(non_snake_case)]

// importing other scripts that are used in this project
mod levelMaps;
mod userInput;
mod render;
mod player;
mod items;
mod game;
mod mobs;


// the main function
fn main() {
    // initializing the data
    let mut _gameData = game::InitializeGameData();

    // running the game
    game::Game(&mut _gameData);

    // nothing else is needed here for now
}

