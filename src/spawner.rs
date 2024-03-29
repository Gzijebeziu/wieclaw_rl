use rltk::RGB;
use specs::{prelude::*, saveload::{MarkedBuilder, SimpleMarker}};
use super::{Player, Map, TileType, Renderable, Name, Position, Viewshed, Rect, MasterDungeonMap, OtherLevelPosition,
            SerializeMe, random_table::MasterTable, HungerState, HungerClock, raws::*, Attributes, EntryTrigger, SingleActivation,
            Attribute, attr_bonus, Skills, Skill, Pools, Pool, player_hp_at_level, mana_at_level, LightSource, TeleportTo,
            Initiative, Faction, EquipmentChanged, StatusEffect, Duration, AttributeBonus, KnownSpells};
use std::collections::HashMap;

const MAX_MONSTERS : i32 = 4;

fn room_table(map_depth: i32) -> MasterTable {
    get_spawn_table_for_depth(&RAWS.lock().unwrap(), map_depth)
}

pub fn player(ecs : &mut World, player_x : i32, player_y : i32) -> Entity {
    spawn_all_spells(ecs);
    let mut skills = Skills{ skills: HashMap::new() };
    skills.skills.insert(Skill::Melee, 1);
    skills.skills.insert(Skill::Defense, 1);
    skills.skills.insert(Skill::Magic, 1);

    let player = ecs
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
        .with(HungerClock{ state: HungerState::WellFed, duration: 20 })
        .with(Attributes{
            might: Attribute{ base: 11, modifiers: 0, bonus: attr_bonus(11) },
            fitness: Attribute { base: 11, modifiers: 0, bonus: attr_bonus(11) },
            quickness: Attribute { base: 11, modifiers: 0, bonus: attr_bonus(11) },
            intelligence: Attribute { base: 11, modifiers: 0, bonus: attr_bonus(11) }
        })
        .with(skills)
        .with(Pools{
            hit_points : Pool{
                current: player_hp_at_level(11, 1),
                max: player_hp_at_level(11, 1)
            },
            mana: Pool{
                current: mana_at_level(11, 1),
                max: mana_at_level(11, 1)
            },
            xp: 0,
            level: 1,
            total_weight: 0.0,
            total_initiative_penalty: 0.0,
            gold: 0.0,
            god_mode: false
        })
        .with(LightSource{ color: rltk::RGB::from_f32(1.0, 1.0, 0.5), range: 8 })
        .with(Initiative{ current: 0 })
        .with(Faction{name : "Player".to_string()})
        .with(EquipmentChanged{})
        .with(KnownSpells{ spells : Vec::new() })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    spawn_named_entity(&RAWS.lock().unwrap(), ecs, "Dywan", SpawnType::Equipped{ by: player });
    spawn_named_entity(&RAWS.lock().unwrap(), ecs, "Smardz", SpawnType::Carried{ by: player });
    spawn_named_entity(&RAWS.lock().unwrap(), ecs, "Soczek marchewkowy", SpawnType::Carried{ by: player });
    spawn_named_entity(&RAWS.lock().unwrap(), ecs, "Koszulka 'Baciary'", SpawnType::Equipped{ by: player });
    spawn_named_entity(&RAWS.lock().unwrap(), ecs, "Obdarte pantalony", SpawnType::Equipped{ by: player });
    spawn_named_entity(&RAWS.lock().unwrap(), ecs, "Papcie", SpawnType::Equipped{ by: player });
    spawn_named_entity(&RAWS.lock().unwrap(), ecs, "Kapiszonowiec", SpawnType::Carried{ by: player });

    ecs.create_entity()
        .with(StatusEffect{ target: player })
        .with(Duration{ turns: 10 })
        .with(Name{ name: "Pan Kacy".to_string() })
        .with(AttributeBonus{
            might : Some(-1),
            fitness : None,
            quickness : Some(-1),
            intelligence : Some(-1)
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    player
}


#[allow(clippy::map_entry)]
pub fn spawn_room(map: &Map, room : &Rect, map_depth: i32, spawn_list : &mut Vec<(usize, String)>) {
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

    spawn_region(map, &possible_targets, map_depth, spawn_list);
}


pub fn spawn_region(_map: &Map, area : &[usize], map_depth: i32, spawn_list : &mut Vec<(usize, String)>) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points : HashMap<usize, String> = HashMap::new();
    let mut areas : Vec<usize> = Vec::from(area);

    {
        let num_spawns = i32::min(areas.len() as i32, crate::rng::roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1) - 3);
        if num_spawns == 0 { return; }

        for _i in 0 .. num_spawns {
            let array_index = if areas.len() == 1 { 0usize } else { (crate::rng::roll_dice(1, areas.len() as i32)-1) as usize };

            let map_idx = areas[array_index];
            spawn_points.insert(map_idx, spawn_table.roll());
            areas.remove(array_index);
        }
    }

    for spawn in spawn_points.iter() {
        spawn_list.push((*spawn.0, spawn.1.to_string()));
    }
}


pub fn spawn_entity(ecs: &mut World, spawn : &(&usize, &String)) {
    let map = ecs.fetch::<Map>();
    let width = map.width as usize;
    let x = (*spawn.0 % width) as i32;
    let y = (*spawn.0 / width) as i32;
    std::mem::drop(map);

    let spawn_result = spawn_named_entity(&RAWS.lock().unwrap(), ecs, &spawn.1, SpawnType::AtPosition { x, y });
    if spawn_result.is_some() {
        return;
    }
    
    if spawn.1 != "None" {
        rltk::console::log(format!("WARNING: don't know how to spawn [{}]!", spawn.1));
    }
}

pub fn spawn_town_portal(ecs: &mut World) {
    let map = ecs.fetch::<Map>();
    let player_depth = map.depth;
    let player_pos = ecs.fetch::<rltk::Point>();
    let player_x = player_pos.x;
    let player_y = player_pos.y;
    std::mem::drop(player_pos);
    std::mem::drop(map);

    let dm = ecs.fetch::<MasterDungeonMap>();
    let town_map = dm.get_map(1).unwrap();
    let mut stairs_idx = 0;
    for (idx, tt) in town_map.tiles.iter().enumerate() {
        if *tt == TileType::DownStairs {
            stairs_idx = idx;
        }
    }
    let portal_x = (stairs_idx as i32 % town_map.width)-2;
    let portal_y = stairs_idx as i32 / town_map.width;

    std::mem::drop(dm);

    ecs.create_entity()
        .with(OtherLevelPosition { x: portal_x, y: portal_y, depth: 1 })
        .with(Renderable {
            glyph: rltk::to_cp437('♥'),
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 0
        })
        .with(EntryTrigger{})
        .with(TeleportTo{ x: player_x, y: player_y, depth: player_depth, player_only: true })
        .with(Name{ name: "Drzwi bez domu".to_string() })
        .with(SingleActivation{})
        .build();
}