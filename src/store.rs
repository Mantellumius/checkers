use std::{collections::HashMap, fs, io};

use crate::{engine::Board, Room};

pub struct Store {}

const PATH: &str = "data/rooms.json";

impl Store {
    pub fn get_rooms() -> io::Result<HashMap<String, Room>> {
        let json_string = fs::read_to_string(PATH)?;
        let rooms = serde_json::from_str::<HashMap<String, Room>>(&json_string)?;
        Ok(rooms)
    }

    pub fn get_room(id: &String) -> io::Result<Room> {
        let mut rooms = Self::get_rooms()?;
        match rooms.remove(id) {
            Some(room) => Ok(room),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "Room not found")),
        }
    }

    pub fn insert_room(id: String, room: Room) -> io::Result<()> {
        let mut rooms = Self::get_rooms()?;
        rooms.insert(id, room);
        let json_string = serde_json::to_string_pretty(&rooms)?;
        fs::write(PATH, json_string)?;
        Ok(())
    }
    
    pub fn update_board(id: String, board: Board) -> io::Result<()> {
        let mut rooms = Self::get_rooms()?;
        rooms.get_mut(&id).unwrap().board = board;
        let json_string = serde_json::to_string_pretty(&rooms)?;
        fs::write(PATH, json_string)?;
        Ok(())
    }
}
