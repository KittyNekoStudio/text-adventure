use colored::Colorize;

/// Get the information of a spell.
fn get_spell_lore(index1: usize, index2: usize) -> String {
    let spell_lore = [
            // 0
        ["Inspect Person".bold(), "Get information of a person".into()],
            // 1
        ["Inspect Object".bold(), "Get information of an object".into()],
            // 2
        ["Enhanced Insight".bold(), "Increases information recieved from divination spells.".into()],
            // 3
        ["See into the Past".bold(), "The mage gains the ability to briefly see into the past.".into()],
            // 4
        ["Danger Sense".bold(), "Gains awareness of danger to directed to the mage.".into()],
            // 5
        ["Wispers of the Touch".bold(), "Get the names of everyone who touched an object.".into()],
            // 6
        ["Hear the Voices".bold(), "Hear the voices that an object heard.".into()]
    ];
    spell_lore[index1][index2].to_string()
}
/// Get information of a spell circle.
fn get_circle_lore(index1: usize, index2: usize) -> String {
    let circle_lore = [
            // 0
        ["Enhanced Inspect".bold(), "Get greater information from a person or an object.".into()],
            // 1
        ["Gaze Into The Past".bold(), "Cast your gaze into the past and truly see it.".into()]
    ];
    circle_lore[index1][index2].to_string()
}
/// Print spell.
pub fn print_spell(spell: &usize) {
    println!("{}", get_spell_lore(*spell, 0));
    println!("{}", get_spell_lore(*spell, 1));
}
/// Print spell circle.
pub fn print_circle(circle: &usize) {
    println!("{}", get_circle_lore(*circle, 0));
    println!("{}", get_circle_lore(*circle, 1));
}