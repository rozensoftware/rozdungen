use crate::door::Door;

#[derive(Clone, Copy, PartialEq)]
pub struct Corridor
{
    pub id: usize,
    pub from_room_id: usize,
    pub to_room_id: usize,
    pub from_room_door: Option<Door>,
    pub to_room_door: Option<Door>,
}

impl Corridor
{
    pub fn new(cid: usize, from: usize, to: usize, door_from: Option<Door>, door_to: Option<Door>) -> Self
    {
        Self { id: cid, from_room_id: from, to_room_id: to, from_room_door: door_from, to_room_door: door_to }
    }
}