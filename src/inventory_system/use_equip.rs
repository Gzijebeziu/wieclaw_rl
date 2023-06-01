use specs::prelude::*;
use super::{Name, InBackpack, gamelog::GameLog, WantsToUseItem, Equippable, Equipped, EquipmentChanged,
            IdentifiedItem, CursedItem};

pub struct ItemEquipOnUse {}

impl<'a> System<'a> for ItemEquipOnUse {
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        WriteStorage<'a, WantsToUseItem>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, Equippable>,
                        WriteStorage<'a, Equipped>,
                        WriteStorage<'a, InBackpack>,
                        WriteStorage<'a, EquipmentChanged>,
                        WriteStorage<'a, IdentifiedItem>,
                        ReadStorage<'a, CursedItem>
                    );

    #[allow(clippy::cognitive_complexity)]
    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut gamelog, entities, mut wants_use, names, equippable, mut equipped,
            mut backpack, mut dirty, mut identified_item, cursed) = data;

        let mut remove_use : Vec<Entity> = Vec::new();
        for (target, useitem) in (&entities, &wants_use).join() {
            if let Some(can_equip) = equippable.get(useitem.item) {
                let target_slot = can_equip.slot;

                let mut can_equip = true;
                let mut log_entries : Vec<String> = Vec::new();
                let mut to_unequip : Vec<Entity> = Vec::new();
                for (item_entity, already_equipped, name) in (&entities, &equipped, &names).join() {
                    if already_equipped.owner == target && already_equipped.slot == target_slot {
                        if cursed.get(item_entity).is_some() {
                            can_equip = false;
                            gamelog.entries.push(format!("Wieclaw nie moze zdjac {}, albowiem ma na sobie heksa.", name.name));
                        } else {
                            to_unequip.push(item_entity);
                            if target == *player_entity {
                                log_entries.push(format!("Wieclaw zdejmuje {}.", name.name));
                            }
                        }
                    }
                }
                if can_equip {
                    if target == *player_entity {
                        identified_item.insert(target, IdentifiedItem{ name: names.get(useitem.item).unwrap().name.clone() })
                            .expect("Unable to insert");
                    }

                    for item in to_unequip.iter() {
                        equipped.remove(*item);
                        backpack.insert(*item, InBackpack{ owner: target }).expect("Unable to insert backpack entry");
                    }

                    for le in log_entries.iter() {
                        gamelog.entries.push(le.to_string());
                    }

                    equipped.insert(useitem.item, Equipped{ owner: target, slot: target_slot }).expect("Unable to insert equipped component");
                    backpack.remove(useitem.item);
                    if target == *player_entity {
                        gamelog.entries.push(format!("Wieclaw zaklada {}.", names.get(useitem.item).unwrap().name));
                    }
                }

                remove_use.push(target);
            }
        }

        remove_use.iter().for_each(|e| {
            dirty.insert(*e, EquipmentChanged{}).expect("Unable to insert");
            wants_use.remove(*e).expect("Unable to remove");
        });
    }
}