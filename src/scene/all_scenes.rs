use lib::{def::recive_input, gamestate::def::GameState};

/// The start scene that gives some information to the player.
pub fn start_up_scene() {
    // TODO! figure out a title
    println!("Thank you for playing PLACEHOLDER.");
    println!("This is my first game and I would appreciate any feedback you have.");
    println!("This is a text adventure game about a recent graduate of a magical school");
    println!("doing jobs to repay his school debt.");
    println!("The controls are through typing text into the console.");
    println!("Capitalization does not matter.");
    println!("Type help if you need any help.");
    println!("To start the game type yes into the terminal.");
    println!("Type no to quit out of the game.");
    let mut input = recive_input().to_lowercase();
    while input.as_str() != "yes" {
        if input.as_str() == "no" {
            std::process::exit(0);
        } else {
            println!("Please type Yes or No.");
            input = recive_input().to_lowercase();
        }
    }
}
/// The waking up scene.
pub fn first_scene(gamestate: &mut GameState) -> &GameState {
    println!("");
    println!("You awake with a load yawn. While the first thoughts of the day come and go, only one stands out.");
    println!("What does the future hold for you after today.");
    println!("As you enter the bathroom to get ready for the day you pass by your reflection and see.");
    println!("A skinny man with well kept hair and a clean shaven face.");
    println!("His clothes look well ironed, and he seems to take care of his apperence.");
    return gamestate;
}
/// The scene in the hallway.
pub fn second_scene(gamestate: &mut GameState) -> &GameState {
    println!("");
    println!("As you leave your room you think of what today means.");
    println!("Today you graduate.");
    println!("Today you leave the safety of the campus and go out into the real world.");
    println!("One must know with your special talents your debtors wouldn't risk you.");
    println!("But you never fully know.");
    println!("Things can go against even your predictions.");
    println!("Although that happens less and less lately.");
    return gamestate;
}