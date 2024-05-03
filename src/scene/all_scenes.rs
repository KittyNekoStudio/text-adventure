use lib::{entitys::create_entity::create_player, gamestate::def::GameState};

pub fn first_scene(gamestate: &mut GameState) -> &GameState {
    println!("You wake up with a load yawn. While the first thoughts of the day come and go, only one stands out.");
    println!("What does the future hold for you after today.");
    println!("As you enter the bathroom to get ready for the day you pass by your reflection and see.");
    gamestate.player = create_player();
    println!("You have just customized your character.");
    println!("Now you have entered free room.");
    println!("To move around type the room name you want to move to.");
    println!("To inspect an object, type look at followed by the objects name.");
    println!("To pick something up, type collect followed by the item name.");
    println!("To search the room for clues, type search");
    println!("Type help if you are confused by anything.");
    println!("Now type bedroom to go to the bedroom.");
    return gamestate;
}

pub fn second_scene(gamestate: &mut GameState) -> &GameState {
    println!("Hello World!");
    
    return gamestate;
}