use specs::prelude::*;
use super::{Targets, add_effect, EffectType, entity_position, targeting};
use crate::{Consumable, gamelog::GameLog, ProvidesFood, Name, RunState, MagicMapper, Map, TownPortal, ProvidesHealing, ProvidesIdentification,
            InflictsDamage, Confusion, Hidden, SingleActivation, TeleportTo, SpawnParticleLine, SpawnParticleBurst, ProvidesRemoveCurse, Duration,
            AttributeBonus, SpellTemplate, Pools, ProvidesMana, TeachesSpell, KnownSpells, KnownSpell, Slow, DamageOverTime};

pub fn item_trigger(creator: Option<Entity>, item: Entity, targets: &Targets, ecs: &mut World) {
    if let Some(c) = ecs.write_storage::<Consumable>().get_mut(item) {
        if c.charges < 1 {
            let mut gamelog = ecs.fetch_mut::<GameLog>();
            gamelog.entries.push(format!("{} nie ma juz ladunków!", ecs.read_storage::<Name>().get(item).unwrap().name));
            return;
        } else {
            c.charges -= 1;
        }
    }

    let did_something = event_trigger(creator, item, targets, ecs);

    if did_something {
        if let Some(c) = ecs.read_storage::<Consumable>().get(item) {
            if c.charges == 0 {
                ecs.entities().delete(item).expect("Delete failed");
            }
        }
    }
}

pub fn spell_trigger(creator : Option<Entity>, spell: Entity, targets : &Targets, ecs: &mut World) {
    if let Some(template) = ecs.read_storage::<SpellTemplate>().get(spell) {
        let mut pools = ecs.write_storage::<Pools>();
        if let Some(caster) = creator {
            if let Some(pool) = pools.get_mut(caster) {
                if template.mana_cost <= pool.mana.current {
                    pool.mana.current -= template.mana_cost;
                }
            }
        }
    }
    event_trigger(creator, spell, targets, ecs);
}

fn event_trigger(creator: Option<Entity>, entity: Entity, targets: &Targets, ecs: &mut World) -> bool {
    let mut did_something = false;
    let mut gamelog = ecs.fetch_mut::<GameLog>();

    if let Some(part) = ecs.read_storage::<SpawnParticleLine>().get(entity) {
        if let Some(start_pos) = targeting::find_item_position(ecs, entity, creator) {
            match targets {
                Targets::Tile{tile_idx} => spawn_line_particles(ecs, start_pos, *tile_idx, part),
                Targets::Tiles{tiles} => tiles.iter().for_each(|tile_idx| spawn_line_particles(ecs, start_pos, *tile_idx, part)),
                Targets::Single{ target } => {
                    if let Some(end_pos) = entity_position(ecs, *target) {
                        spawn_line_particles(ecs, start_pos, end_pos, part);
                    }
                }
                Targets::TargetList{ targets } => {
                    targets.iter().for_each(|target| {
                        if let Some(end_pos) = entity_position(ecs, *target) {
                            spawn_line_particles(ecs, start_pos, end_pos, part);
                        }
                    });
                }
            }
        }
    }

    if let Some(part) = ecs.read_storage::<SpawnParticleBurst>().get(entity) {
        add_effect(
            creator,
            EffectType::Particle{
                glyph: part.glyph,
                fg: part.color,
                bg: rltk::RGB::named(rltk::BLACK),
                lifespan: part.lifetime_ms
            },
            targets.clone()
        );
    }

    if ecs.read_storage::<ProvidesFood>().get(entity).is_some() {
        add_effect(creator, EffectType::WellFed, targets.clone());
        let names = ecs.read_storage::<Name>();
        gamelog.entries.push(format!("Wieclaw zjada {}.", names.get(entity).unwrap().name));
        did_something = true;
    }

    if ecs.read_storage::<MagicMapper>().get(entity).is_some() {
        let mut runstate = ecs.fetch_mut::<RunState>();
        gamelog.entries.push("Wieclaw odkryl cala mape!".to_string());
        *runstate = RunState::MagicMapReveal{row: 0};
        did_something = true;
    }

    if ecs.read_storage::<ProvidesIdentification>().get(entity).is_some() {
        let mut runstate = ecs.fetch_mut::<RunState>();
        *runstate = RunState::ShowIdentify;
        did_something = true;
    }

    if ecs.read_storage::<ProvidesRemoveCurse>().get(entity).is_some() {
        let mut runstate = ecs.fetch_mut::<RunState>();
        *runstate = RunState::ShowRemoveCurse;
        did_something = true;
    }

    if ecs.read_storage::<TownPortal>().get(entity).is_some() {
        let map = ecs.fetch::<Map>();
        if map.depth == 1 {
            gamelog.entries.push("Wieclaw juz jest w miescie, wiec zwój nie dziala.".to_string());
        } else {
            gamelog.entries.push("Wieclaw teleportuje sie do miasta!".to_string());
            let mut runstate = ecs.fetch_mut::<RunState>();
            *runstate = RunState::TownPortal;
            did_something = true;
        }
    }

    if let Some(heal) = ecs.read_storage::<ProvidesHealing>().get(entity) {
        add_effect(creator, EffectType::Healing{amount: heal.heal_amount}, targets.clone());
        let names = ecs.read_storage::<Name>();
        gamelog.entries.push(format!("Wieclaw konsumuje {} i odzyskuje {} HP.", names.get(entity).unwrap().name, heal.heal_amount));
        did_something = true;
    }

    if let Some(mana) = ecs.read_storage::<ProvidesMana>().get(entity) {
        add_effect(creator, EffectType::Mana{amount: mana.mana_amount}, targets.clone());
        let names = ecs.read_storage::<Name>();
        gamelog.entries.push(format!("Wieclaw konsumuje {} i odzyskuje {} MP.", names.get(entity).unwrap().name, mana.mana_amount));
        did_something = true;
    }

    if let Some(damage) = ecs.read_storage::<InflictsDamage>().get(entity) {
        add_effect(creator, EffectType::Damage{amount: damage.damage}, targets.clone());
        did_something = true;
    }

    if let Some(_confusion) = ecs.read_storage::<Confusion>().get(entity) {
        if let Some(duration) = ecs.read_storage::<Duration>().get(entity) {
            add_effect(creator, EffectType::Confusion{ turns: duration.turns }, targets.clone());
            did_something = true;
        }

    }

    if let Some(teleport) = ecs.read_storage::<TeleportTo>().get(entity) {
        add_effect(
            creator,
            EffectType::TeleportTo{
                x: teleport.x,
                y: teleport.y,
                depth: teleport.depth,
                player_only: teleport.player_only
            },
            targets.clone()
        );
        did_something = true;
    }

    if let Some(attr) = ecs.read_storage::<AttributeBonus>().get(entity) {
        add_effect(
            creator,
            EffectType::AttributeEffect{
                bonus : attr.clone(),
                duration : 10,
                name : ecs.read_storage::<Name>().get(entity).unwrap().name.clone()
            },
            targets.clone()
        );
        did_something = true;
    }

    if let Some(spell) = ecs.read_storage::<TeachesSpell>().get(entity) {
        if let Some(known) = ecs.write_storage::<KnownSpells>().get_mut(creator.unwrap()) {
            if let Some(spell_entity) = crate::raws::find_spell_entity(ecs, &spell.spell) {
                if let Some(spell_info) = ecs.read_storage::<SpellTemplate>().get(spell_entity) {
                    let mut already_known = false;
                    known.spells.iter().for_each(|s| if s.display_name == spell.spell { already_known = true });
                    if !already_known {
                        known.spells.push(KnownSpell{ display_name: spell.spell.clone(), mana_cost : spell_info.mana_cost });
                    }
                }
            }
        }

        did_something = true;
    }

    if let Some(slow) = ecs.read_storage::<Slow>().get(entity) {
        add_effect(creator, EffectType::Slow{ initiative_penalty : slow.initiative_penalty }, targets.clone());
        did_something = true;
    }

    if let Some(damage) = ecs.read_storage::<DamageOverTime>().get(entity) {
        add_effect(creator, EffectType::DamageOverTime{ damage: damage.damage }, targets.clone());
        did_something = true;
    }

    did_something
}

pub fn trigger(creator : Option<Entity>, trigger: Entity, targets: &Targets, ecs: &mut World) {
    ecs.write_storage::<Hidden>().remove(trigger);

    let did_something = event_trigger(creator, trigger, targets, ecs);

    if did_something && ecs.read_storage::<SingleActivation>().get(trigger).is_some() {
        ecs.entities().delete(trigger).expect("Delete failed");
    }
}

fn spawn_line_particles(ecs: &World, start: i32, end: i32, part: &SpawnParticleLine) {
    let map = ecs.fetch::<Map>();
    let start_pt = rltk::Point::new(start % map.width, end / map.width);
    let end_pt = rltk::Point::new(end % map.width, end / map.width);
    let line = rltk::line2d(rltk::LineAlg::Bresenham, start_pt, end_pt);
    for pt in line.iter() {
        add_effect(
            None,
            EffectType::Particle{
                glyph: part.glyph,
                fg: part.color,
                bg: rltk::RGB::named(rltk::BLACK),
                lifespan: part.lifetime_ms
            },
            Targets::Tile{tile_idx : map.xy_idx(pt.x, pt.y) as i32}
        );
    }
}