#[derive(Debug)]
pub struct ItemType {
    pub held: bool,
    pub wearable: bool,
    pub consumable: bool,
    pub throwable: bool
}
#[derive(Debug)]
pub struct ItemEffect {
    pub damage: bool,
    pub recover: bool,
    pub buff: bool,
    pub debuff: bool,
    pub defence: bool
}
#[derive(Debug)]
pub struct Item {
    pub item_type: ItemType,
    pub item_effect: ItemEffect
}

impl Item {
    pub fn new() -> Self {
        Self {
            item_type: ItemType {
                held: false,
                wearable: false,
                consumable: false,
                throwable: false
            },
            item_effect: ItemEffect {
                damage: false,
                recover: false,
                buff: false,
                debuff: false,
                defence: false
                }
        }
    }
}

pub fn check_item_field(item: Item, text: &str) -> (bool, u8) {
    match text {
        "held" => match item {
            Item { item_type, .. } => (item_type.held, 1)
        },
        "consumable" => match item {
            Item { item_type, .. } => (item_type.consumable, 1)
        },
        "wearable" => match item {
            Item { item_type, .. } => (item_type.wearable, 1)
        },
        "throwable" => match item {
            Item { item_type, .. } => (item_type.throwable, 1)
        },
        "damage" => match item {
            Item { item_effect, .. } => (item_effect.damage, 1)
        },
        "healing" => match item {
            Item { item_effect, .. } => (item_effect.recover, 1)
        },
        "buff" => match item {
            Item { item_effect, .. } => (item_effect.buff, 1)
        },
        "debuff" => match item {
            Item { item_effect, .. } => (item_effect.debuff, 1)
        },
        "defence" => match item {
            Item { item_effect, .. } => (item_effect.defence, 1)
        },
        _ => (false, 0)
    }
}