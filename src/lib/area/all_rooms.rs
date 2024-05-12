use crate::{entitys::create_entity::create_dorm_manager, item::def::{EVERYCOLITEM, EVERYINTITEM}};

use super::def::{Area, Room};

/// Struct for player school dorm.
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
                lore: 0,
                times_entered: 0
                }
            }
        }
    }
}
/// Struct for the bathroom.
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
                lore: 1,
                times_entered: 0
                }
            }
        }
    }
}
/// Struct for the hallway.
#[derive(Debug, Clone, PartialEq)]
pub struct Hallway {
    pub area: Area
}

impl Hallway {
    pub fn new_e1() -> Self {
        Self {
            area: Area {
                room: Room {
                    entitys: vec![],
                    collectable_item: vec![],
                    interactable_items: vec![],
                    main_area_name: "school dorms".to_string(),
                    sub_area_names: vec![
                    "bedroom".to_string(),
                    "dorm 1".to_string(),
                    "dorm 2".to_string(),
                    "dorm 3".to_string(),
                    "dorm 4".to_string(),
                    ],
                    lore: 2,
                    times_entered: 0
                }
            }
        }
    }
    pub fn new_e2() -> Self {
        Self {
            area: Area {
                room: Room {
                    entitys: vec![],
                    collectable_item: vec![],
                    interactable_items: vec![],
                    main_area_name: "school dorms".to_string(),
                    sub_area_names: vec![
                    "dorm 5".to_string(),
                    "dorm 6".to_string(),
                    "dorm 7".to_string(),
                    "dorm 8".to_string(),
                    ],
                    lore: 5,
                    times_entered: 0
                }
            }
        }
    }
    pub fn new_w1() -> Self {
        Self {
            area: Area {
                room: Room {
                    entitys: vec![],
                    collectable_item: vec![],
                    interactable_items: vec![],
                    main_area_name: "school dorms".to_string(),
                    sub_area_names: vec![
                    "dorm 9".to_string(),
                    "dorm 10".to_string(),
                    "dorm 11".to_string(),
                    "dorm 12".to_string(),
                    ],
                    lore: 7,
                    times_entered: 0
                }
            }
        }
    }
    pub fn new_w2() -> Self {
        Self {
            area: Area {
                room: Room {
                    entitys: vec![],
                    collectable_item: vec![],
                    interactable_items: vec![],
                    main_area_name: "school dorms".to_string(),
                    sub_area_names: vec![
                    "dorm 13".to_string(),
                    "dorm 14".to_string(),
                    "dorm 15".to_string(),
                    "dorm 16".to_string(),
                    ],
                    lore: 9,
                    times_entered: 0
                }
            }
        }
    }
}
/// Struct for the dorm rooms.
#[derive(Debug, Clone, PartialEq)]
pub struct DormRoom {
    pub area: Area
}

impl DormRoom {
    pub fn new_e1() -> Self {
        Self {
            area: Area {
                room: Room {
                    entitys: vec![],
                    collectable_item: vec![],
                    interactable_items: vec![],
                    main_area_name: "east 1 hallway".to_string(),
                    sub_area_names: vec!["bathroom".to_string(),],
                    lore: 3,
                    times_entered: 0
                }
            }
        }
    } 
    pub fn new_e2() -> Self {
        Self {
            area: Area {
                room: Room {
                    entitys: vec![],
                    collectable_item: vec![],
                    interactable_items: vec![],
                    main_area_name: "east 2 hallway".to_string(),
                    sub_area_names: vec!["bathroom".to_string(),],
                    lore: 6,
                    times_entered: 0
                }
            }
        }
    }
    pub fn new_w1() -> Self {
        Self {
            area: Area {
                room: Room {
                    entitys: vec![],
                    collectable_item: vec![],
                    interactable_items: vec![],
                    main_area_name: "west 1 hallway".to_string(),
                    sub_area_names: vec!["bathroom".to_string(),],
                    lore: 8,
                    times_entered: 0
                }
            }
        }
    }
    pub fn new_w2() -> Self {
        Self {
            area: Area {
                room: Room {
                    entitys: vec![],
                    collectable_item: vec![],
                    interactable_items: vec![],
                    main_area_name: "west 2 hallway".to_string(),
                    sub_area_names: vec!["bathroom".to_string()],
                    lore: 10,
                    times_entered: 0
                }
            }
        }
    }
}
/// Struct form the main school dorm area.
#[derive(Debug, Clone, PartialEq)]
pub struct SchoolDorm {
    pub area: Area
}

impl SchoolDorm {
    pub fn new() -> Self {
        Self {
            area: Area {
                room: Room {
                    entitys: vec![],
                    collectable_item: vec![],
                    interactable_items: vec![("map".to_string(), EVERYINTITEM[2])],
                    main_area_name: "campus square".to_string(),
                    sub_area_names: vec![
                    "east 1 hallway".to_string(),
                    "east 2 hallway".to_string(),
                    "west 1 hallway".to_string(),
                    "west 2 hallway".to_string(),
                    "office".to_string()],
                    lore: 4,
                    times_entered: 0
                }
            }
        }
    }
}
/// Struct for the office in the dorms.
#[derive(Debug, Clone, PartialEq)]
pub struct DormOffice {
    pub area: Area
}

impl DormOffice {
    pub fn new() -> Self {
        Self {
            area: Area {
                room: Room {
                    entitys: vec![create_dorm_manager()],
                    collectable_item: vec![],
                    interactable_items: vec![],
                    main_area_name: "school dorms".to_string(),
                    sub_area_names: vec![],
                    lore: 11,
                    times_entered: 0   
                }
            }
        }
    }
}
/// Struct for the school campus square.
#[derive(Debug, Clone, PartialEq)]
pub struct CampusSquare {
    pub area: Area
}

impl CampusSquare {
    pub fn new() -> Self {
        Self {
            area: Area {
                room: Room {
                entitys: vec![],
                collectable_item: vec![],
                interactable_items: vec![("map".to_string(), EVERYINTITEM[3])],
                main_area_name: "school entrance".to_string(),
                sub_area_names: vec!["school dorms".to_string(), "auditorium".to_string()],
                lore: 12,
                times_entered: 0
                }
            }
        }
    }
}
/// Struct for the school auditorium.
#[derive(Debug, Clone, PartialEq)]
pub struct SchoolAuditorium {
    pub area: Area
}

impl SchoolAuditorium {
    pub fn new() -> Self {
        Self {
            area: Area {
                room: Room {
                entitys: vec![],
                collectable_item: vec![],
                interactable_items: vec![],
                main_area_name: "campus square".to_string(),
                sub_area_names: vec!["stage".to_string(), "seat".to_string()],
                lore: 13,
                times_entered: 0
                }
            }
        }
    }
}
/// Struct for the school auditorium stage.
#[derive(Debug, Clone, PartialEq)]
pub struct SchoolStage {
    pub area: Area
}

impl SchoolStage {
    pub fn new() -> Self {
        Self {
            area: Area {
                room: Room {
                entitys: vec![],
                collectable_item: vec![],
                interactable_items: vec![],
                main_area_name: "auditorium".to_string(),
                sub_area_names: vec![],
                lore: 14,
                times_entered: 0
                }
            }
        }
    }
}

/// Struct for the school auditorium seat.
#[derive(Debug, Clone, PartialEq)]
pub struct SchoolAuditoriumSeat {
    pub area: Area
}

impl SchoolAuditoriumSeat {
    pub fn new() -> Self {
        Self {
            area: Area {
                room: Room {
                entitys: vec![],
                collectable_item: vec![],
                interactable_items: vec![],
                main_area_name: "auditorium".to_string(),
                sub_area_names: vec![],
                lore: 15,
                times_entered: 0
                }
            }
        }
    }
}