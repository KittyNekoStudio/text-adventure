use crate::def::recive_input;


/// The start scene that gives some information to the player.
pub fn start_up_scene() {
    // TODO! figure out a title
    println!("Thank you for playing The Bizarre Things Adam Sees.");
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
pub fn first_scene() {
    println!("");
    println!("You awake with a load yawn. While the first thoughts of the day come and go, only one stands out.");
    println!("What does the future hold for you after today.");
    println!("As you enter the bathroom to get ready for the day you pass by your reflection and see.");
    println!("A skinny man with well kept hair and a clean shaven face.");
    println!("His clothes look well ironed, and he seems to take care of his apperence.");
    println!("");
}
/// The scene in the hallway.
pub fn second_scene() {
    println!("As you leave your room you think of what today means.");
    println!("Today you graduate.");
    println!("Today you leave the safety of the campus and go out into the real world.");
    println!("One must know with your special talents your debtors wouldn't risk you.");
    println!("But you never fully know.");
    println!("Things can go against even your predictions.");
    println!("Although that happens less and less lately.");
    println!("");
}

/// The scene in the campus square.
pub fn third_scene() {
    println!("For the past weeks when you have been divining this day.");
    println!("All you could see was that great change was coming to your life.");
    println!("Which isn't helpful because obviously change will come.");
    println!("As the life you had lived for the past 6 years comes to an end today.");
    println!("Knowing so little doesn't happen often anymore.");
    println!("It's making you feel a little antsty.");
    println!("");
}
/// The scene in the auditorium.
pub fn fourth_scene() {
    println!("You sit down in your seat when the headmaster begins the ceremony.");
    println!("The headmaster starts his speach. \"Thank you all for coming today.\"");
    println!("\"We are here to send of this years seniors. It allways fills me with pride to see how far are students have come.\"");
    println!("\"I don't want to drag on to much as the seniors have a busy day today.\"");
    println!("\"Let's call up our first student, Amy Jones!\"");
    println!("With the call of the first student the ceremony begins.");
    println!("It didn't take long for you to here the shout of \"Adam See!\"");
    println!("You get out of your seat and make your way to the stage.");
    println!("");
}
/// The scene on the stage.
pub fn fifth_scene() {
    println!("As you climb the stairs you think of your future.");
    println!("What does the government want from you?");
    println!("As you are lost in though you get interupted by the headmaster.");
    println!("\"Here is Adam See one of our strongest graduates this year.\"");
    println!("\"Adam please come and shake the professors hands.\"");
    println!("You go down the line shaking your teacher hands until you reach the headmaster.");
    println!("\"Adam I look forward to what you accomplish in the future.\"");
    println!("\"Here is your scroll of recognition.\"");
    println!("He hands you your scroll.");
    println!("\"Now please make your way back to the seats.\"");
    println!("You leave the stage with your thoughts still on the meeting later today.");
    println!("");

}