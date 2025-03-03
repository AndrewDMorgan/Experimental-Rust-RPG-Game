/*

The map rendering is working!!
Light calculations are working!!
User inputs are working!
Entity rendering is working!
    and the camera position/bound box was fixed!

*/

// snake case is bad
#![allow(non_snake_case)]

// importing other scripts that are used in this project
mod levelMaps;
mod userInput;
mod render;
mod player;
mod game;


// the main function
fn main() {
    // initializing the data
    let mut gameData = game::InitializeGameData();

    // running the game
    game::Game(&mut gameData);

    // nothing else is needed here for now
}

