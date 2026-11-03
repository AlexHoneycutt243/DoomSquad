use std::{collections::HashMap, fs, path::Path};

use crate::{
    GameAction, audio::audio_handler::Audio, game::{
        entities::{Entity, EntityEvent::*},
        entity_behaviour::death_behaviour,
        interactables::Interactable,
        map_grid::MapGrid,
    }, keys::key_assigned::load_key_bindings_file_parse, parser::map_parser::parse_map, render::RendererData,
};

use super::map::Map;
use super::player::Player;
use crate::game::damage_calculation::damage_check;
use crate::game::interactables::InteractableEvent::*;
use crate::parser::entities_parser::parse_entities;
use crate::parser::interactables_parser::*;
use crate::parser::player_parser::parse_player_position;
use minifb::{Key, Window};

const DESPAWN_TIME: i32 = 300;
const MAP_GRID_CELL_SIZE: f64 = 64.0;
pub const INTERACTABLE_RANGE: f64 = 30.0;

#[derive(Clone)]
pub struct Game {
    pub player: Player,
    pub entities: Vec<Entity>,
    pub interactables: Vec<Interactable>,
    pub map: Map,
    pub despawn_timer: i32,
    pub map_grid: MapGrid,
    pub projectile_that_hit: Vec<usize>,
    pub map_index: usize,
    pub last_map_index: usize,
    pub key_map: HashMap<GameAction, Key>,
}

impl Game {
    pub fn new_game(renderer_data: &RendererData) -> Option<Self> {
        let path = Path::new("assets/maps/ggb");
        let entries_result = fs::read_dir(path);
        let mut entries: Vec<_> = match entries_result {
            Ok(read_dir) => read_dir.filter_map(Result::ok).collect(),
            Err(e) => {
                eprintln!("Error when reading directory: {}", e);
                return None;
            }
        };
        entries.sort_by_key(|e| e.path());
        if let Some(entry) = entries.first() {
            // we initially load the first map
            let path_buf = entry.path();

            let path = match path_buf.to_str() {
                Some(p) => p,
                None => {
                    eprintln!("Invalid path");
                    return None;
                }
            };
            let key_map: HashMap<GameAction, Key> = load_key_bindings_file_parse().unwrap_or_else(|_| {
                let mut default_map = HashMap::default();
                default_map.insert(GameAction::Forwards, Key::W);
                default_map.insert(GameAction::Left, Key::A);
                default_map.insert(GameAction::Backwards, Key::S);
                default_map.insert(GameAction::Right, Key::D);
                default_map.insert(GameAction::Sprint, Key::LeftShift);
                default_map.insert(GameAction::Jump, Key::Space);
                default_map.insert(GameAction::Crouch, Key::C);
                default_map.insert(GameAction::RocketJump, Key::R);
                default_map.insert(GameAction::Interact, Key::F);
                default_map.insert(GameAction::ToggleFineAiming,Key::E);
                default_map.insert(GameAction::CrosshairUp, Key::Up);
                default_map.insert(GameAction::TurnLeft, Key::Left);
                default_map.insert(GameAction::CrosshairDown, Key::Down);
                default_map.insert(GameAction::TurnRight, Key::Right);
                default_map.insert(GameAction::FireBullet, Key::RightCtrl);
                default_map.insert(GameAction::FireArrow, Key::RightShift);
                default_map.insert(GameAction::EnterGodmode, Key::G);
                default_map.insert(GameAction::LockEnemies, Key::L);
                default_map.insert(GameAction::PauseGame, Key::Escape);
                default_map
            });

            let game = Self {
                player: Player::new_with_position(
                    parse_player_position(path.to_string(), renderer_data).ok()?,
                ),
                entities: parse_entities(path.to_string(), renderer_data).ok()?,
                interactables: parse_interactables(path.to_string(), renderer_data).ok()?,
                map: parse_map(path.to_string()).ok()?, // TODO remove unwrap
                despawn_timer: DESPAWN_TIME,
                map_grid: MapGrid::new(MAP_GRID_CELL_SIZE),
                projectile_that_hit: Vec::new(),
                map_index: 0,
                last_map_index: 0,
                key_map,
            };

            Some(game)
        } else {
            None
        }
    }

    pub fn update(
        &mut self, 
        window: &mut Window, 
        renderer_data: &RendererData, 
        audio: &mut Audio, 
        //key_map: &mut HashMap<GameAction, Key>
    ) {
        //update all interactables and add possible spawn events
        let mut interactables = std::mem::take(&mut self.interactables);
        let mut interactables_spawns = Vec::new();

        for interactable in &mut interactables {
            let event = interactable.update(window, renderer_data, self, audio);
            interactables_spawns.extend(event);
        }

        self.interactables = interactables;


        /////////////////////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////////////////////
        let mut entity_spawns = Vec::new();
        //update player
        let event = self.player.update(window, &self.map, renderer_data, audio, &mut self.key_map);
        //add possible spawn events
        entity_spawns.extend(event);

        /////////////////////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////////////////////
        //update all entites and add possible spawn events
        for entity in &mut self.entities {
            let event = entity.update(window, &self.map, &self.player.mover, renderer_data, audio, &mut self.key_map);
            entity_spawns.extend(event);
        }

        //deal damage
        damage_check(self, audio);

        //on death behaviour
        for entity in &mut self.entities {
            if entity.hp <= 0.0 {
                entity_spawns.extend(death_behaviour(
                    entity,
                    renderer_data,
                    self.player.mover.position,
                    audio,
                ));
            }
        }

        //despawn everything that has 0 hp
        self.entities.retain(|entity| entity.hp > 0.0);

        //spawn new entities
        for event in entity_spawns {
            match event {
                Spawn(entity) => self.entities.push(entity),
            }
        }
        //spawn new map
        for interactables_event in interactables_spawns {
            match interactables_event {
                SpawnMap(index) => self.map_swap(renderer_data, index),
            }
        }
    }
    pub fn map_swap(&mut self, renderer_data: &RendererData, new_map_index: usize) {
        let path = Path::new("assets/maps/ggb");
        let entries_result = fs::read_dir(path);
        let mut entries: Vec<_> = match entries_result {
            Ok(read_dir) => read_dir.filter_map(Result::ok).collect(),
            Err(e) => {
                eprintln!("Error when reading directory: {}", e);
                return;
            }
        };
        entries.sort_by_key(|e| e.path());
        if let Some(entry) = entries.get(new_map_index) {
            let path_buf = entry.path();

            let path = match path_buf.to_str() {
                Some(p) => p,
                None => {
                    eprintln!("Invalid path");
                    return;
                }
            };

            if let Ok(map) = parse_map(path.to_string()) {
                self.map = map;
            } else {
                eprintln!("Error parsing map");
            }
            if let Ok(entities) = parse_entities(path.to_string(), renderer_data) {
                self.entities = entities;
            } else {
                eprintln!("Error parsing entities");
            }
            if let Ok(interactables) = parse_interactables(path.to_string(), renderer_data) {
                self.interactables = interactables;
            } else {
                eprintln!("Error parsing interactables");
            }
            if let Ok(player_position) = parse_player_position(path.to_string(), renderer_data) {
                self.player.mover.position = player_position;
            } else {
                eprintln!("Error parsing player_pos");
            }
            self.last_map_index = self.map_index;
            self.map_index = new_map_index;
        }
    }
}
