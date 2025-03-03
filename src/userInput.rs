// snake case is bad
#![allow(non_snake_case)]


// the different actions/inputs possible
pub enum Input {
    Attack,
    Move,
    Interact,
    Inventory,
    Exit,
    Null,
}

// the directions an action/input can happen in
pub enum Direction {
    Left,
    Up,
    Down,
    Right,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
    Null,  // no direction (many actions don't need a direction; such as inventory)
}


// gets the current input and/or direction for an in game situation
// menus will call another function to get user inputs
pub fn GetGameInput () -> (Input, Direction) {
    let mut direction = Direction::Null;

    // prompting the user
    println!("Input an action (and direction if needed):\n * Directions (pre-fix; optional):\n     - N, NE, E, SE, S, SW, W, NW\n * Actions:\n     - Move\n     - Attack\n     - Interact\n     - Inventory\n     - Exit");
    let mut userTextInput = String::new();
    std::io::stdin().read_line(&mut userTextInput).expect("Failed to read input");

    // parsing the input

    // assuming valid inputs, all directions will have a space while non-directional actions won't
    let mut _userAction = "";
    if userTextInput.contains(" ") {
        // getting the direction and action by splitting it
        let (actionDirection, userActionInput) = userTextInput.split_at(2usize);
        _userAction = userActionInput.trim();  // seperated out the direction

        // getting the direction
        direction = match actionDirection {
            "N" => Direction::Up,
            "E" => Direction::Right,
            "S" => Direction::Down,
            "W" => Direction::Left,
            "NE" => Direction::UpRight,
            "NW" => Direction::UpLeft,
            "SE" => Direction::DownRight,
            "SW" => Direction::DownLeft,
            _ => Direction::Null,
        }
    } else {
        // the input is the action
        _userAction = userTextInput.as_str().trim();
    }

    let usrInput = match _userAction {
        "Attack" => Input::Attack,
        "Move" => Input::Move,
        "Interact" => Input::Interact,
        "Inventory" => Input::Inventory,
        "Exit" => Input::Exit,
        _ => Input::Null,
    };

    (usrInput, direction)  // the output
}

