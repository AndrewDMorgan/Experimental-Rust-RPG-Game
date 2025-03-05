// snake case is bad
#![allow(non_snake_case)]

use std::collections::HashMap;

// importing other scripts to be used in the main script
use crate::levelMaps;
//use crate::player;
use crate::game;

// maximum distance (square shaped diameter) that can be viewed
const MAX_VIEW_DISTANCE: usize = 15usize;  // can be even or odd with how it's setup

// every character and it's appearance in different light levels
lazy_static::lazy_static! {
    static ref LIGHT_CHARS: HashMap <levelMaps::Tiles, [&'static str; 10]> = {
        HashMap::from([
            (levelMaps::Tiles::Empty, ["  ", "  ", "  ", "..", "..", "\"\"", ";;", "++", "**", "##"]),
            (levelMaps::Tiles::Wall , ["==", "==", "==", "==", "==", "==", "##", "##", "##", "##"]),
            (levelMaps::Tiles::Chest , ["[]", "[]", "[]", "[]", "[]", "[]", "[]", "[]", "##", "##"]),
            (levelMaps::Tiles::EnchantmentTable , ["()", "()", "()", "()", "()", "()", "()", "##", "##", "##"]),
        ])};
}

// a type allias for the dynamic render function because it's really complicated
pub type DynamicRenderFunction = Box <dyn Fn ((usize, usize)) -> Option<[&'static str; 10]>>;


// functions for rendering
pub fn RenderMap (
            gameData: &game::GameData,
            renderFunctions: &Vec <DynamicRenderFunction>,
            (cameraPosX, cameraPosY): (usize, usize)
        ) {
    // lights directly edit the lightmap, so this should never need to know where lights actively are
    // sense they perminately leave an area lit
    let mut renderWidth = MAX_VIEW_DISTANCE - {
        if cameraPosX + MAX_VIEW_DISTANCE >= gameData.levelMaps[gameData.currentLevel].mapSizeX {
            std::cmp::min(cameraPosX + MAX_VIEW_DISTANCE - 
                gameData.levelMaps[gameData.currentLevel].mapSizeX,
                MAX_VIEW_DISTANCE)
                // making sure the result isn't negative (using unsigned int)
        } else {
            0usize  // within bounds
        }};
    let mut renderHeight = MAX_VIEW_DISTANCE - {
        if cameraPosY + MAX_VIEW_DISTANCE >= gameData.levelMaps[gameData.currentLevel].mapSizeY {
            std::cmp::min(cameraPosY + MAX_VIEW_DISTANCE - 
                gameData.levelMaps[gameData.currentLevel].mapSizeY,
                MAX_VIEW_DISTANCE)
                // making sure the result isn't negative (using unsigned int)
        } else {
            0usize  // within bounds
        }};
    
    // the starting position x for the camera view area
    // is the + 1 after renderW/H correct?
    let startX = cameraPosX - std::cmp::min(renderWidth /2 + 1, cameraPosX);
    let startY = cameraPosY - std::cmp::min(renderHeight/2 + 1, cameraPosY);
    renderWidth -= 1;
    renderHeight -= 1;

    // itterating over the viewing box and retreaving the results
    let mut output = "".to_string();  // accumulates the results (allows a controled printing at the end)
    for y in startY..cameraPosY+renderHeight {
        for x in startX..cameraPosX+renderWidth {
            // getting any sprites that need to be rendered
            let mut sprite: Option <[&'static str; 10]> = None;
            for function in renderFunctions {
                if let Some(sprt) = function((x, y)) {
                    sprite = Some(sprt);  // the sprite is good
                    break;
                }
            }

            // getting the character being rendered
            let charLightSlice = LIGHT_CHARS.get(
                                              // based on the tile, getting the set of char sprites
                                              gameData.levelMaps[gameData.currentLevel].tileMap.get
                                              (x + y * gameData.levelMaps[gameData.currentLevel].mapSizeX)
                                              .expect("Failed to unwrap tile"))
                                          .expect("Failed to unwrap tile character");
                                          // these should never fail with how it's setup (if it does than something is broken and needs fixing)
            
            // unpacking the sprite if there is one
            if let Some(sprt) = sprite {
                output.push_str(
                    sprt.get(
                            *gameData.levelMaps[gameData.currentLevel].lightMap.get(
                            x + y * (gameData.levelMaps[gameData.currentLevel].mapSizeX)
                            ).expect("Failed to unwrap light level")
                        ).expect("Failed to unwrap entity light character")
                        // this should never never fail, and if it does there's an error somewhere in the code
                        // could be a light level that's too high, or an incorrectly formed array (or something here?)
                );
            } else {
                // getting the correct char based on the light level
                output.push_str(
                    charLightSlice.get(
                            *gameData.levelMaps[gameData.currentLevel].lightMap.get(
                            x + y * (gameData.levelMaps[gameData.currentLevel].mapSizeX)
                            ).expect("Failed to unwrap light level")
                        ).expect("Failed to unwrap tile light character")
                        // this should never never fail, and if it does there's an error somewhere in the code
                        // could be a light level that's too high, or an incorrectly formed array (or something here?)
                );
            }
        }
        output.push('\n');
    }

    // printing the results to the terminal
    print!("{}", output);
}


