use crate::item::def::{EVERYCOLITEM, EVERYINTITEM};

use super::def::{Area, Room};
#[derive(Debug, Clone, PartialEq)]
pub struct FirstRoom {
    pub area: Area

}

impl FirstRoom {
    pub fn new() -> Self {
    // TODO! create a way to move this into a function
            Self {
                area: Area {
                room: Room {
                entitys: vec![],
                collectable_item: vec![("staff".to_string(), EVERYCOLITEM[0])],
                interactable_items: vec![("bed".to_string(), EVERYINTITEM[0])],
                main_area_name: "hallway".to_string(),
                sub_area_names: vec!["bathroom".to_string()],
                id: 1,
                lore: 0,
                times_entered: 0
                }
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Bathroom {
    pub area: Area
}

impl Bathroom {
    pub fn new() -> Self {
            Self {
                area: Area {
                room: Room {
                entitys: vec![],
                collectable_item: vec![("robe".to_string(), EVERYCOLITEM[2])],
                interactable_items: vec![("mirror".to_string(), EVERYINTITEM[1])],
                main_area_name: "bedroom".to_string(),
                sub_area_names: vec![],
                id: 2,
                lore: 1,
                times_entered: 0
                }
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Hallway {
    pub area: Area
}

impl Hallway {
    pub fn new() -> Self {
        Self {
            area: Area {
                room: Room {
                    entitys: vec![],
                    collectable_item: vec![],
                    interactable_items: vec![],
                    main_area_name: "school dorms".to_string(),
                    sub_area_names: vec!["bedroom".to_string()],
                    id: 3,
                    lore: 2,
                    times_entered: 0
                }
            }
        }
    }
}