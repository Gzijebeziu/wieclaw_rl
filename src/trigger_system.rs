use specs::prelude::*;
use super::{EntityMoved, Position, EntryTrigger, Map, Name, effects::*, AreaOfEffect};

pub struct TriggerSystem {}

impl<'a> System<'a> for TriggerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Map>,
                        WriteStorage<'a, EntityMoved>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, EntryTrigger>,
                        ReadStorage<'a, Name>,
                        Entities<'a>,
                        ReadStorage<'a, AreaOfEffect>);

    fn run(&mut self, data : Self::SystemData) {
        let (map, mut entity_moved, position, entry_trigger, names, entities, area_of_effect) = data;

        for (entity, mut _entity_moved, pos) in (&entities, &mut entity_moved, &position).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            crate::spatial::for_each_tile_content(idx, |entity_id| {
                if entity != entity_id {
                    let maybe_trigger = entry_trigger.get(entity_id);
                    match maybe_trigger {
                        None => {},
                        Some(_trigger) => {
                            let name = names.get(entity_id);
                            if let Some(name) = name {
                                crate::gamelog::Logger::new()
                                    .color(rltk::RED)
                                    .append(&name.name)
                                    .color(rltk::WHITE)
                                    .append("uruchamia sie!")
                                    .log();
                            }

                            add_effect(
                                Some(entity),
                                EffectType::TriggerFire{ trigger: entity_id },
                                if let Some(aoe) = area_of_effect.get(entity_id) {
                                    Targets::Tiles{
                                        tiles : aoe_tiles(&*map, rltk::Point::new(pos.x, pos.y), aoe.radius)
                                    }
                                } else {
                                    Targets::Tile{ tile_idx: idx as i32 }
                                }
                            );
                        }
                    }
                }
            });
        }

        entity_moved.clear();
    }
}