/// If check_entity_field() is true call this the get usable data.
pub fn match_entity_field(input: &String) -> usize {
    match &input.to_lowercase() as &str {
        "spells" => 1,
        "inventory" => 2,
        "circles" => 3,
        "mana" => 4,
        _ => 0
    }
}
/// Checks which field you are looking for.
pub fn check_entity_field(input: &String) -> bool {
    let fields = ["spells", "inventory", "circles", "mana"];
    for i in fields {
        if input == i {
            return true;
        }
    }
    return false;
}