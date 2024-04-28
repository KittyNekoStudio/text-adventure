/// If check_entity_field() is true call this the get usable data.
pub fn match_entity_field(input: &String) -> usize {
    match &input.to_lowercase() as &str {
        "stats" => 1,
        "inventory" => 2,
        _ => 3
    }
}
/// Checks which field you are looking for
pub fn check_entity_field(input: &String) -> bool {
    let fields = ["stats", "inventory"];
    for i in fields {
        if input == i {
            return true;
        }
    }
    return false;
}