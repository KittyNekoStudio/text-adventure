use crate::area::def::Room;

/// Checks which field you are looking for
pub fn check_int_item_field(input: &String, room_items: Room) -> bool {
    for i in room_items.interactable_item_name {
        if input == &i {
            return true;
        }
    }
    return false;
}