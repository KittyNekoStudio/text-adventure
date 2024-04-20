use std::io;
/// Take input from the user
pub fn recive_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input not recived");
    let input_clean: String = input.trim().parse().expect("Input no cleaned");
    return input_clean;
}

