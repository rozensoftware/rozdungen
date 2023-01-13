use crate::item::Item;


#[derive(Clone, PartialEq)]
pub struct Room
{
    pub id: usize,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub items: Vec<Item>,
}

impl Room
{
    pub fn new(rid: usize, xp: u16, yp: u16, w: u16, h: u16) -> Self
    {
        Self { id: rid, x: xp, y: yp, width: w, height: h,
            items: Vec::<Item>::new() }
    }
}