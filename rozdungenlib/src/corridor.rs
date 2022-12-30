use crate::room::Room;

#[derive(Debug)]
pub struct Corridor<'a>
{
    pub id: usize,
    pub from_room: &'a Room,
    pub to_room: &'a Room,
}

impl<'a> Corridor<'a>
{
    pub fn new(cid: usize, from: &'a Room, to: &'a Room) -> Self
    {
        Self { id: cid, from_room: from, to_room: to }
    }
}