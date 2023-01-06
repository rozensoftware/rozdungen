use crate::{room::Room, door::Door};

#[derive(Debug, Clone, Copy)]
pub struct Corridor
{
    pub id: usize,
    pub from_room: Room,
    pub to_room: Room,
    pub from_room_door: Option<Door>,
    pub to_room_door: Option<Door>,
}

impl Corridor
{
    pub fn new(cid: usize, from: Room, to: Room, door_from: Option<Door>, door_to: Option<Door>) -> Self
    {
        Self { id: cid, from_room: from, to_room: to, from_room_door: door_from, to_room_door: door_to }
    }
}