use crate::{room::Room, door::Door};

#[derive(Clone, PartialEq)]
pub struct Corridor<'chamber>
{
    pub id: usize,
    pub from_room: &'chamber Room,
    pub to_room: &'chamber Room,
    pub from_room_door: Option<Door>,
    pub to_room_door: Option<Door>,
}

impl<'chamber> Corridor<'chamber>
{
    pub fn new(cid: usize, from: &'chamber Room, to: &'chamber Room, door_from: Option<Door>, door_to: Option<Door>) -> Self
    {
        Self { id: cid, from_room: from, to_room: to, from_room_door: door_from, to_room_door: door_to }
    }
}