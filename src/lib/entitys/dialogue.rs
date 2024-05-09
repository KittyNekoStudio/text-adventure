use super::npcs::NPC;

pub fn print_dialogue(npc: NPC) {
    match npc.name.as_str() {
        "manager" => dorm_manager_dia(npc),
        _ => ()
    }
}

fn dorm_manager_dia(manager: NPC) {
    if manager.talked_to == 0 {
        println!("");
        println!("{}", manager.dialogue[0]);
    } else {
        println!("");
        println!("{}", manager.dialogue[1]);
    }
}