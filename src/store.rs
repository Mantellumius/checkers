use std::{collections::HashMap, fs};

use crate::{Room};

pub struct Store {}

const PATH: &str = "data/rooms.json";

impl Store {
    pub fn get_rooms() -> HashMap<String, Room> {
        match fs::read_to_string(PATH) {
            Ok(data) => serde_json::from_str::<HashMap<String, Room>>(&data)
                .expect("Failed to parse json"),
            Err(_) => HashMap::new(),
        }
    }

    pub fn get_room(id: String) -> Option<Room> {
        match fs::read_to_string(PATH) {
            Ok(data) => serde_json::from_str::<HashMap<String, Room>>(&data)
                .expect("Failed to parse json")
                .remove(&id),
            Err(_) => None,
        }
    }

    pub fn insert_room(id: String, room: Room) {
        let mut rooms = Self::get_rooms();
        rooms.insert(id, room);
        fs::write(PATH, serde_json::to_string_pretty(&rooms).unwrap())
            .expect("Can't write to file");
    }
}
