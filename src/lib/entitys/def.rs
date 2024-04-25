pub fn match_entity_field(input: &String) -> usize {
    match &input.to_lowercase() as &str {
        "stats" => 1,
        "inventory" => 2,
        _ => 3
    }
}

pub fn check_entity_field(input: &String) -> bool {
    let fields = ["stats", "inventory"];
    for i in fields {
        if input == i {
            return true;
        }
    }
    return false;
}