use super::player_character::PlayerCharacter;

pub fn get_player_stat(player: PlayerCharacter, stat: &str) -> (String, u16) {
    match stat {
        "level" => match player {
            PlayerCharacter { stats } => ("Level".to_string(), stats.0)
        },
        "health" => match player {
            PlayerCharacter { stats } => ("Health".to_string(), stats.1)
        },
        "vit" => match player {
            PlayerCharacter { stats } => ("Vitality".to_string(), stats.2)
        },
        "int" => match player {
            PlayerCharacter { stats } => ("Intelligence".to_string(), stats.3)
        },
        "speed" => match player {
            PlayerCharacter { stats } => ("Speed".to_string(), stats.4)
        },
        _ => ("Invalid string".to_string(), 0)
    }
}
