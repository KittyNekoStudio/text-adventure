use crate::map::{map::new_map, movement::match_for_direction};

/// A function to test various code.
pub fn test() {
    let mut map = new_map();
    let direction = match_for_direction();
    map.push(direction);
    map.push(match_for_direction());
    println!("{:?}", map);
}