pub const TALKKEYWORDS: [&str; 1] = ["talk to "];

pub fn npc_interaction(input: &String) -> bool {
    for i in TALKKEYWORDS {
        if input.contains(i) {
            return true;
        }
    }
    return false;
}