// snake case is bad
#![allow(non_snake_case)]

// importing other scripts to be used in the main script
use crate::levelMaps::Tiles;  // temp
use crate::levelMaps;
use crate::userInput;
use crate::player;
use crate::render;
use crate::items;
use crate::mobs;

// the size and number of all maps
const NUMBER_OF_LEVELS: usize = 10usize;
const MAP_SIZE_X: usize = 25usize;
const MAP_SIZE_Y: usize = 25usize;
pub const TOTAL_SIZE_1D: usize = MAP_SIZE_X * MAP_SIZE_Y;


// stores the data for a game
pub struct GameData {
    pub levelMaps: [levelMaps::MapData <'static>; NUMBER_OF_LEVELS],
    pub player: player::Player,
    pub currentLevel: usize,
    pub monsters: Vec <mobs::Entity>,
}


// creates the data for the game
pub fn InitializeGameData () -> GameData {
    // initializing some basic information that lasts for the scope of the game
    let gameLevelMaps: [levelMaps::MapData
                     <'static>; NUMBER_OF_LEVELS]
                     = [levelMaps::MapData::new(MAP_SIZE_X, MAP_SIZE_Y); NUMBER_OF_LEVELS];
    
    // creating the player
    let mainPlayer = player::Player::new(5usize, 5usize);

    // returning the data
    GameData {
        levelMaps: gameLevelMaps,
        player: mainPlayer,
        currentLevel: 0usize,
        monsters: vec!(),
    }
}


// the main game loop
pub fn Game (gameData: &mut GameData) {
    // chest loot table (chance, item)    chances are 1-100 (higher is more likely)
    // unfortunately this can't be a const because the functions are non-const
    const NUMBER_OF_CHEST_ITEMS: usize = 2;
    let CHEST_LOOT_TABLE: [(usize, items::Item); NUMBER_OF_CHEST_ITEMS] = [
        (25, items::Item {
            name: "Rusted Sword".to_string(),
            //itemType: items::ItemType::Weapon,
            itemUsageFunc: items::GetMeleWeaponUsageFunction(15, 2, vec!()),
            itemHeldUpdateFunc: items::GetHeldLightFunction(0),
            damage: 15,
            range: 2,
            enchantments: vec!(),
        }),
        (50, items::Item {
            name: "Torch".to_string(),
            //itemType: items::ItemType::Light,
            itemUsageFunc: std::sync::Arc::new(move |_player, _monsters, _action, _direction| {}),
            itemHeldUpdateFunc: items::GetHeldLightFunction(8),
            damage: 0,
            range: 0,
            enchantments: vec!(),
        }),
    ];

    const NUMBER_OF_ENCHANTMENTS: usize = 2;
    let ENCHANTMENTS_TABLE: [(usize, items::ItemEnchantment, String); NUMBER_OF_ENCHANTMENTS] = [
        (50, items::GetSharpnessEnchantment(5), "Sharpness".to_string()),
        (15, items::GetFireEnchantment(2, 5), "Flame".to_string()),
    ];
    
    // generating test walls (for light propagation)
    gameData.levelMaps[gameData.currentLevel].tileMap = [
        // ugly temp map (make a file-loading system for this that uses reasonable symbols for the walls and cells)
        &Tiles::Wall, &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall,
        &Tiles::Wall, &Tiles::Chest, &Tiles::Empty, &Tiles::EnchantmentTable, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall , &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall , &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall , &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall , &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall , &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall , &Tiles::Empty, &Tiles::Wall , &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall , &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Empty, &Tiles::Wall,
        &Tiles::Wall, &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall , &Tiles::Wall,
    ];
    
    // generating an initial light (for testing)
    gameData.levelMaps[gameData.currentLevel].GenerateLightAura(&9usize, &5usize, &5usize);
    gameData.levelMaps[gameData.currentLevel].GenerateLightAura(&9usize, &9usize, &12usize);
    gameData.levelMaps[gameData.currentLevel].GenerateLightAura(&9usize, &21usize, &17usize);
    
    gameData.monsters.push(  // (for testing)
       mobs::Entity::new(
            10, 10,
               25,   5,
            mobs::ZombieAi, mobs::GetZombieRender)
    );
    gameData.monsters.push(  // (for testing)
       mobs::Entity::new(
            22, 22,
               25,   5,
            mobs::ZombieAi, mobs::GetZombieRender)
    );

    'main: loop {
        // storing all render functions (passed into the renderer for the game)
        // these also have different sprite (sprite refers to string/&str) versions based on the light level
        let renderFunctions: Vec <render::DynamicRenderFunction> =
                std::iter::once(gameData.player.GetRender())
                        .chain(gameData.monsters.iter().map(|entity| entity.GetRenderer()))
                        .collect();

        // renders the scene and moves the memory of the light buffer so it's destroyed
        render::RenderMap(gameData, &renderFunctions, (gameData.player.positionX, gameData.player.positionY) );
        println!();  // white space to make a seperation between the game and any ui

        let (action, actionDirection) = userInput::GetGameInput();
        
        // updating the player
        gameData.player.Update (
                &mut gameData.levelMaps[gameData.currentLevel],
                &mut gameData.monsters,
                &CHEST_LOOT_TABLE,
                &ENCHANTMENTS_TABLE,
                (&action, &actionDirection)
            );
        
        // updating the mobs
        for mob in &mut gameData.monsters {
            // updating the mob's ai
            (mob.entityAi)(mob, &mut gameData.player,
                                &gameData.levelMaps[gameData.currentLevel]);

            // updating the rest of the mob
            mob.Update();
        }

        // updating the mobs (first removing any dead ones)
        let mut aliveMobs: Vec <mobs::Entity> = vec!();
        while let Some(mob) = gameData.monsters.pop() {
            if mob.Alive() {
                aliveMobs.push(mob);
            }
        }
        gameData.monsters = aliveMobs;

        if matches!(action, userInput::Input::Exit) {
            break 'main;  // ending the loop at some point when the game is finished
        }
    }
}

