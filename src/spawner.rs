use rltk::{ RGB, RandomNumberGenerator };
use specs::{prelude::*, saveload::{MarkedBuilder, SimpleMarker}};
use super::{CombatStats, Player, Map, TileType, Renderable, Name, Position, Viewshed, Monster, BlocksTile, Rect, 
            map::MAPWIDTH, Item, ProvidesHealing, Consumable, Ranged, InflictsDamage, AreaOfEffect, 
            Confusion, SerializeMe, random_table::RandomTable, Equippable, EquipmentSlot, HungerState, 
            HungerClock, MeleePowerBonus, DefenseBonus, ProvidesFood, MagicMapper, Hidden, EntryTrigger, SingleActivation,
            BlocksVisibility, Door};
use std::collections::HashMap;

const MAX_MONSTERS : i32 = 4;

pub fn player(ecs : &mut World, player_x : i32, player_y : i32) -> Entity {
    ecs
        .create_entity()
        .with(Position{ x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('☺'),
            fg: RGB::from_u8(255, 201, 14),
            bg: RGB::named(rltk::BLACK),
            render_order: 0
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles : Vec::new(), range : 8, dirty : true })
        .with(Name{ name: "Wieclaw".to_string() })
        .with(CombatStats{ max_hp: 30, hp: 30, defense: 2, power: 5 })
        .with(HungerClock{ state: HungerState::WellFed, duration: 20 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}


#[allow(clippy::map_entry)]
pub fn spawn_room(map: &Map, rng: &mut RandomNumberGenerator, room : &Rect, map_depth: i32, spawn_list : &mut Vec<(usize, String)>) {
    let mut possible_targets : Vec<usize> = Vec::new();
    {
        for y in room.y1 + 1 .. room.y2 {
            for x in room.x1 + 1 .. room.x2 {
                let idx = map.xy_idx(x, y);
                if map.tiles[idx] == TileType::Floor {
                    possible_targets.push(idx);
                }
            }
        }
    }

    spawn_region(map, rng, &possible_targets, map_depth, spawn_list);
}


pub fn spawn_region(_map: &Map, rng: &mut RandomNumberGenerator, area : &[usize], map_depth: i32, spawn_list : &mut Vec<(usize, String)>) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points : HashMap<usize, String> = HashMap::new();
    let mut areas : Vec<usize> = Vec::from(area);

    {
        let num_spawns = i32::min(areas.len() as i32, rng.roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1) - 3);
        if num_spawns == 0 { return; }

        for _i in 0 .. num_spawns {
            let array_index = if areas.len() == 1 { 0usize } else { (rng.roll_dice(1, areas.len() as i32)-1) as usize };

            let map_idx = areas[array_index];
            spawn_points.insert(map_idx, spawn_table.roll(rng));
            areas.remove(array_index);
        }
    }

    for spawn in spawn_points.iter() {
        spawn_list.push((*spawn.0, spawn.1.to_string()));
    }
}


pub fn spawn_entity(ecs: &mut World, spawn : &(&usize, &String)) {
    let x = (*spawn.0 % MAPWIDTH) as i32;
    let y = (*spawn.0 / MAPWIDTH) as i32;

    match spawn.1.as_ref() {
        "Gnomon" => gnomon(ecs, x, y),
        "Golem Zoledny" => golem(ecs, x, y),
        "Pasztecik" => health_potion(ecs, x, y),
        "Zwoj Rzutu Kartoflem" => magic_missile_scroll(ecs, x, y),
        "Zwoj z Waznym Pytaniem" => confusion_scroll(ecs, x, y),
        "Zwoj Saznistego Pierdniecia" => fireball_scroll(ecs, x, y),
        "Klapek" => klapek(ecs, x, y),
        "Sandalki" => sandalki(ecs, x, y),
        "Laczek" => laczek(ecs, x, y),
        "Kalosze" => kalosze(ecs, x, y),
        "Surowka" => rations(ecs, x, y),
        "Magic Mapping Scroll" => magic_mapping_scroll(ecs, x, y),
        "Bear Trap" => bear_trap(ecs, x, y),
        "Door" => door(ecs, x, y),
        _ => {}
    }
}


fn room_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .add("Gnomon", 10)
        .add("Golem Zoledny", 1 + map_depth)
        .add("Pasztecik", 7)
        .add("Zwoj Rzutu Kartoflem", 4)
        .add("Zwoj z Waznym Pytaniem", 2 + map_depth)
        .add("Zwoj Saznistego Pierdniecia", 2 + map_depth)
        .add("Klapek", 3)
        .add("Sandalki", 3)
        .add("Laczek", map_depth - 1)
        .add("Kalosze", map_depth - 1)
        .add("Surowka", 10)
        .add("Magic Mapping Scroll", 2)
        .add("Bear Trap", 2)
}

fn golem(ecs: &mut World, x: i32, y: i32) { monster(ecs, x, y, rltk::to_cp437('G'), "Golem Zoledny"); }
fn gnomon(ecs: &mut World, x: i32, y: i32) { monster(ecs, x, y, rltk::to_cp437('g'), "Gnomon"); }

fn monster<S : ToString>(ecs: &mut World, x: i32, y:i32, glyph : rltk::FontCharType, name : S) {
    ecs.create_entity()
            .with(Position{ x, y })
            .with(Renderable{
                glyph,
                fg: RGB::named(rltk::GREEN3),
                bg: RGB::named(rltk::BLACK),
                render_order: 1
            })
            .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
            .with(Monster{})
            .with(Name{ name : name.to_string() })
            .with(BlocksTile{})
            .with(CombatStats{ max_hp: 16, hp: 16, defense: 1, power: 4 })
            .marked::<SimpleMarker<SerializeMe>>()
            .build();
}


fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('∞'),
            fg: RGB::from_u8(180, 125, 0),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Pasztecik".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(ProvidesHealing{ heal_amount: 8 })
        .with(ProvidesFood{})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}


fn magic_missile_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('π'),
            fg: RGB::from_u8(175, 130, 90),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Zwój Rzutu Kartoflem".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}


fn fireball_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('π'),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name: "Zwój Saznistego Pierdniecia".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 20 })
        .with(AreaOfEffect{ radius: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('π'),
            fg: RGB::named(rltk::PINK),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name: "Zwój z Waznym Pytaniem".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(Ranged { range: 6 })
        .with(Confusion{ turns: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn klapek(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('Θ'),
            fg: RGB::from_u8(189, 188, 56),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Klapek".to_string() })
        .with(Item{})
        .with(Equippable{ slot : EquipmentSlot::Melee })
        .with(MeleePowerBonus{ power : 2 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn sandalki(ecs: &mut World, x: i32, y:i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('╚'),
            fg: RGB::from_u8(255, 4, 131),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Sandalki".to_string() })
        .with(Item{})
        .with(Equippable{ slot : EquipmentSlot::Armor })
        .with(DefenseBonus{ defense : 1 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn laczek(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('Θ'),
            fg: RGB::from_u8(240, 95, 56),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Laczek".to_string() })
        .with(Item{})
        .with(Equippable{ slot : EquipmentSlot::Melee })
        .with(MeleePowerBonus{ power: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn kalosze(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('╚'),
            fg: RGB::from_u8(53, 217, 234),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Kalosze".to_string() })
        .with(Item{})
        .with(Equippable{ slot : EquipmentSlot::Armor })
        .with(DefenseBonus{ defense: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn rations(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('%'),
            fg: RGB::named(rltk::GREEN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Surówka Grzeskowiak".to_string() })
        .with(Item{})
        .with(ProvidesFood{})
        .with(Consumable{})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn magic_mapping_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('π'),
            fg: RGB::named(rltk::CYAN3),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Zwój Malego Odkrywcy".to_string() })
        .with(Item{})
        .with(MagicMapper{})
        .with(Consumable{})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn bear_trap(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('^'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Pulapka na mazowszan".to_string() })
        .with(Hidden{})
        .with(EntryTrigger{})
        .with(InflictsDamage{ damage: 6 })
        .with(SingleActivation{})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn door(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('+'),
            fg: RGB::named(rltk::CHOCOLATE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Drzwi".to_string() })
        .with(BlocksTile{})
        .with(BlocksVisibility{})
        .with(Door{open: false})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}