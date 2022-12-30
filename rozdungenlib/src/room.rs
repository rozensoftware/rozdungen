
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Room
{
    pub id: usize,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Room
{
    pub fn new(rid: usize, xp: u16, yp: u16, w: u16, h: u16) -> Self
    {
        Self { id: rid, x: xp, y: yp, width: w, height: h }
    }
}