use specs::{prelude::*, saveload::{SimpleMarker, SimpleMarkerAllocator, SerializeComponents, DeserializeComponents, MarkedBuilder}};
use super::components::*;
use std::{fs::File, path::Path, fs::{self}, convert::Infallible};

macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
        $(
        SerializeComponents::<Infallible, SimpleMarker<SerializeMe>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$data.0,
            &$data.1,
            &mut $ser,
        )
        .unwrap();
        )*
    };
}

pub fn save_game(ecs : &mut World) {
    let mapcopy = ecs.get_mut::<super::map::Map>().unwrap().clone();
    let dungeon_master = ecs.get_mut::<super::map::dungeon::MasterDungeonMap>().unwrap().clone();
    let savehelper = ecs
        .create_entity()
        .with(SerializationHelper{ map : mapcopy })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
    let savehelper2 = ecs
        .create_entity()
        .with(DMSerializationHelper{ map : dungeon_master })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    {
        let data = ( ecs.entities(), ecs.read_storage::<SimpleMarker<SerializeMe>>() );

        let writer = File::create("./savegame.json").unwrap();
        let mut serializer = serde_json::Serializer::new(writer);
        serialize_individually!(ecs, serializer, data, Position, Renderable, Player, Viewshed, Name, BlocksTile, SpawnParticleLine,
            WantsToMelee, Item, Consumable, Ranged, InflictsDamage, AreaOfEffect, Confusion, ProvidesHealing, InBackpack, SpawnParticleBurst,
            WantsToPickupItem, WantsToUseItem, WantsToDropItem, SerializationHelper, Equippable, Equipped, MeleeWeapon, Wearable, TownPortal,
            WantsToRemoveItem, ParticleLifetime, HungerClock, ProvidesFood, MagicMapper, Hidden, EntryTrigger, EntityMoved, SingleActivation,
            BlocksVisibility, Door, Quips, Attributes, Skills, Pools, NaturalAttackDefense, LootTable, EquipmentChanged, Vendor, TeleportTo,
            OtherLevelPosition, DMSerializationHelper, LightSource, Initiative, MyTurn, Faction, WantsToApproach, WantsToFlee, MoveMode, Chasing,
            ApplyMove, ApplyTeleport, MagicItem, ObfuscatedName, IdentifiedItem, CursedItem, ProvidesRemoveCurse, ProvidesIdentification,
            AttributeBonus, Duration, StatusEffect, KnownSpells, SpellTemplate, WantsToCastSpell, ProvidesMana, TeachesSpell, Slow, DamageOverTime,
            SpecialAbilities
        );
    }

    ecs.delete_entity(savehelper).expect("Crash on cleanup");
    ecs.delete_entity(savehelper2).expect("Crash on cleanup");
}

pub fn does_save_exist() -> bool {
    Path::new("./savegame.json").exists()
}

macro_rules! deserialize_individually {
    ($ecs:expr, $de:expr, $data:expr, $( $type:ty),*) => {
        $(
        DeserializeComponents::<Infallible, _>::deserialize(
            &mut ( &mut $ecs.write_storage::<$type>(), ),
            &$data.0,
            &mut $data.1,
            &mut $data.2,
            &mut $de,
        )
        .unwrap();
        )*
    };
}

pub fn load_game(ecs: &mut World) {
    {
        let mut to_delete = Vec::new();
        for e in ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            ecs.delete_entity(*del).expect("Deletion failed");
        }
    }

    let data = fs::read_to_string("./savegame.json").unwrap();
    let mut de = serde_json::Deserializer::from_str(&data);

    {
        let mut d = (&mut ecs.entities(), &mut ecs.write_storage::<SimpleMarker<SerializeMe>>(), &mut ecs.write_resource::<SimpleMarkerAllocator<SerializeMe>>());
        
        deserialize_individually!(ecs, de, d, Position, Renderable, Player, Viewshed, Name, BlocksTile, SpawnParticleLine,
            WantsToMelee, Item, Consumable, Ranged, InflictsDamage, AreaOfEffect, Confusion, ProvidesHealing, InBackpack, SpawnParticleBurst,
            WantsToPickupItem, WantsToUseItem, WantsToDropItem, SerializationHelper, Equippable, Equipped, MeleeWeapon, Wearable, TownPortal,
            WantsToRemoveItem, ParticleLifetime, HungerClock, ProvidesFood, MagicMapper, Hidden, EntryTrigger, EntityMoved, SingleActivation,
            BlocksVisibility, Door, Quips, Attributes, Skills, Pools, NaturalAttackDefense, LootTable, EquipmentChanged, Vendor, TeleportTo,
            OtherLevelPosition, DMSerializationHelper, LightSource, Initiative, MyTurn, Faction, WantsToApproach, WantsToFlee, MoveMode, Chasing,
            ApplyMove, ApplyTeleport, MagicItem, ObfuscatedName, IdentifiedItem, CursedItem, ProvidesRemoveCurse, ProvidesIdentification,
            AttributeBonus, Duration, StatusEffect, KnownSpells, SpellTemplate, WantsToCastSpell, ProvidesMana, TeachesSpell, Slow, DamageOverTime,
            SpecialAbilities
        );
    }

    let mut deleteme : Option<Entity> = None;
    let mut deleteme2 : Option<Entity> = None;
    {
        let entities = ecs.entities();
        let helper = ecs.read_storage::<SerializationHelper>();
        let helper2 = ecs.read_storage::<DMSerializationHelper>();
        let player = ecs.read_storage::<Player>();
        let position = ecs.read_storage::<Position>();
        for (e,h) in (&entities, &helper).join() {
            let mut worldmap = ecs.write_resource::<super::map::Map>();
            *worldmap = h.map.clone();
            crate::spatial::set_size((worldmap.height * worldmap.width) as usize);
            deleteme = Some(e);
        }
        for (e,h) in (&entities, &helper2).join() {
            let mut dungeonmaster = ecs.write_resource::<super::map::dungeon::MasterDungeonMap>();
            *dungeonmaster = h.map.clone();
            deleteme2 = Some(e);
        }
        for (e,_p,pos) in (&entities, &player, &position).join() {
            let mut ppos = ecs.write_resource::<rltk::Point>();
            *ppos = rltk::Point::new(pos.x, pos.y);
            let mut player_resource = ecs.write_resource::<Entity>();
            *player_resource = e;
        }
    }
    ecs.delete_entity(deleteme.unwrap()).expect("Unable to delete helper");
    ecs.delete_entity(deleteme2.unwrap()).expect("Unable to delete marker");
}

pub fn delete_save() {
    if Path::new("./savegame.json").exists() { std::fs::remove_file("./savegame.json").expect("Unable to delete file"); }
}