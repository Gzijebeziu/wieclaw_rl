use std::collections::{HashMap, HashSet};
use specs::prelude::*;
use serde::{Serialize, Deserialize};
use super::{Map, TileType, super::{Viewshed, Position, map_builders::level_builder, OtherLevelPosition}};
use rltk::Point;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct MasterDungeonMap {
    maps : HashMap<i32, Map>,
    pub identified_items : HashSet<String>,
    pub scroll_mappings : HashMap<String, String>,
    pub potion_mappings : HashMap<String, String>
}

impl MasterDungeonMap {
    pub fn new() -> MasterDungeonMap {
        let mut dm = MasterDungeonMap{
            maps: HashMap::new(),
            identified_items: HashSet::new(),
            scroll_mappings: HashMap::new(),
            potion_mappings: HashMap::new()
        };

        for scroll_tag in crate::raws::get_scroll_tags().iter() {
            let masked_name = make_scroll_name();
            dm.scroll_mappings.insert(scroll_tag.to_string(), masked_name);
        }

        let mut used_potion_names : HashSet<String> = HashSet::new();
        for potion_tag in crate::raws::get_potion_tags().iter() {
            let masked_name = make_potion_name(&mut used_potion_names);
            dm.potion_mappings.insert(potion_tag.to_string(), masked_name);
        }

        dm
    }

    pub fn store_map(&mut self, map : &Map) {
        self.maps.insert(map.depth, map.clone());
    }

    pub fn get_map(&self, depth : i32) -> Option<Map> {
        if self.maps.contains_key(&depth) {
            let result = self.maps[&depth].clone();
            Some(result)
        } else {
            None
        }
    }
}

pub fn level_transition(ecs : &mut World, new_depth: i32, offset: i32) -> Option<Vec<Map>> {
    let dungeon_master = ecs.read_resource::<MasterDungeonMap>();

    if dungeon_master.get_map(new_depth).is_some() {
        std::mem::drop(dungeon_master);
        transition_to_existing_map(ecs, new_depth, offset);
        None
    } else {
        std::mem::drop(dungeon_master);
        Some(transition_to_new_map(ecs, new_depth))
    }
}

fn transition_to_new_map(ecs: &mut World, new_depth: i32) -> Vec<Map> {
    let mut builder = level_builder(new_depth, 80, 50);
    builder.build_map();
    if new_depth > 1 {
        if let Some(pos) = &builder.build_data.starting_position {
            let up_idx = builder.build_data.map.xy_idx(pos.x, pos.y);
            builder.build_data.map.tiles[up_idx] = TileType::UpStairs;
        }
    }
    let mapgen_history = builder.build_data.history.clone();
    let player_start;
    {
        let mut worldmap_resource = ecs.write_resource::<Map>();
        *worldmap_resource = builder.build_data.map.clone();
        player_start = builder.build_data.starting_position.as_mut().unwrap().clone();
    }

    builder.spawn_entities(ecs);

    let (player_x, player_y) = (player_start.x, player_start.y);
    let mut player_position = ecs.write_resource::<Point>();
    *player_position = Point::new(player_x, player_y);
    let mut position_components = ecs.write_storage::<Position>();
    let player_entity = ecs.fetch::<Entity>();
    let player_pos_comp = position_components.get_mut(*player_entity);
    if let Some(player_pos_comp) = player_pos_comp {
        player_pos_comp.x = player_x;
        player_pos_comp.y = player_y;
    }

    let mut viewshed_components = ecs.write_storage::<Viewshed>();
    let vs = viewshed_components.get_mut(*player_entity);
    if let Some(vs) = vs {
        vs.dirty = true;
    }

    let mut dungeon_master = ecs.write_resource::<MasterDungeonMap>();
    dungeon_master.store_map(&builder.build_data.map);

    mapgen_history
}

fn transition_to_existing_map(ecs: &mut World, new_depth: i32, offset: i32) {
    let dungeon_master = ecs.read_resource::<MasterDungeonMap>();
    let map = dungeon_master.get_map(new_depth).unwrap();
    let mut worldmap_resource = ecs.write_resource::<Map>();
    let player_entity = ecs.fetch::<Entity>();

    let w = map.width;
    let stair_type = if offset < 0 { TileType::DownStairs } else { TileType::UpStairs };
    for (idx, tt) in map.tiles.iter().enumerate() {
        if *tt == stair_type {
            let mut player_position = ecs.write_resource::<Point>();
            *player_position = Point::new(idx as i32 % w, idx as i32 / w);
            let mut position_components = ecs.write_storage::<Position>();
            let player_pos_comp = position_components.get_mut(*player_entity);
            if let Some(player_pos_comp) = player_pos_comp {
                player_pos_comp.x = idx as i32 % w;
                player_pos_comp.y = idx as i32 / w;
            }
        }
    }

    *worldmap_resource = map;

    let mut viewshed_components = ecs.write_storage::<Viewshed>();
    let vs = viewshed_components.get_mut(*player_entity);
    if let Some(vs) = vs {
        vs.dirty = true;
    }
}

pub fn freeze_level_entities(ecs: &mut World) {
    let entities = ecs.entities();
    let mut positions = ecs.write_storage::<Position>();
    let mut other_level_positions = ecs.write_storage::<OtherLevelPosition>();
    let player_entity = ecs.fetch::<Entity>();
    let map_depth = ecs.fetch::<Map>().depth;

    let mut pos_to_delete : Vec<Entity> = Vec::new();
    for (entity, pos) in (&entities, &positions).join() {
        if entity != *player_entity {
            other_level_positions.insert(entity, OtherLevelPosition{ x: pos.x, y: pos.y, depth: map_depth }).expect("Insert fail");
            pos_to_delete.push(entity);
        }
    }

    for p in pos_to_delete.iter() {
        positions.remove(*p);
    }
}

pub fn thaw_level_entities(ecs: &mut World) {
    let entities = ecs.entities();
    let mut positions = ecs.write_storage::<Position>();
    let mut other_level_positions = ecs.write_storage::<OtherLevelPosition>();
    let player_entity = ecs.fetch::<Entity>();
    let map_depth = ecs.fetch::<Map>().depth;

    let mut pos_to_delete : Vec<Entity> = Vec::new();
    for (entity, pos) in (&entities, &other_level_positions).join() {
        if entity != *player_entity && pos.depth == map_depth {
            positions.insert(entity, Position{ x: pos.x, y: pos.y }).expect("Insert fail");
            pos_to_delete.push(entity);
        }
    }

    for p in pos_to_delete.iter() {
        other_level_positions.remove(*p);
    }
}

fn make_scroll_name() -> String {
    let length = 4 + crate::rng::roll_dice(1, 8);
    let mut name = "Zwój ".to_string();

    for i in 0..length {
        if i % 2 == 0 {
            name += match crate::rng::roll_dice(1, 27) {
                1 => "b",
                2 => "c",
                3 => "ch",
                4 => "cz",
                5 => "d",
                6 => "dz",
                7 => "f",
                8 => "g",
                9 => "h",
                10 => "j",
                11 => "k",
                12 => "l",
                13 => "m",
                14 => "n",
                15 => "p",
                16 => "r",
                17 => "rz",
                18 => "s",
                19 => "sz",
                20 => "t",
                21 => "w",
                22 => "z",
                23 => "dzi",
                24 => "si",
                25 => "zi",
                26 => "ni",
                _ => "ci"
            }
        } else {
            name += match crate::rng::roll_dice(1, 8) {
                1 => "a",
                2 => "e",
                3 => "en",
                4 => "i",
                5 => "o",
                6 => "on",
                7 => "u",
                _ => "y"
            }
        }
    }

    name
}

const POTION_ADJECTIVES1: &[&str] = &["Zlocisty", "Brazowy", "Ladny", "Paskudny", "Spalony", "Nadgnity", "Pulchny"];
const POTION_ADJECTIVES2: &[&str] = &["pachnacy", "smierdzacy", "smaczny", "obrzydliwy", "zachecajacy", "elegancki", "zepsuty"];

fn make_potion_name(used_names: &mut HashSet<String>) -> String {
    loop {
        let mut name : String = POTION_ADJECTIVES1[crate::rng::roll_dice(1, POTION_ADJECTIVES1.len() as i32) as usize -1].to_string();
        name += " ";
        name += POTION_ADJECTIVES2[crate::rng::roll_dice(1, POTION_ADJECTIVES2.len() as i32) as usize -1];
        name += " wypiek";

        if !used_names.contains(&name) {
            used_names.insert(name.clone());
            return name;
        }
    }
}