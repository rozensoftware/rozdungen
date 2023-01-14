#[derive(Clone, Copy, PartialEq)]
pub enum ItemType
{
    ///id of the door that key can open
    Key(usize),
    Weapon,
    Armor,
    Potion,
}

#[derive(Clone, PartialEq)]
pub struct Item
{
    /// An unique id of the item
    pub id: usize,
    /// Item type
    pub item_type: ItemType,
    /// Description of the item
    pub desc: String,
}

impl Item
{
    pub fn new(iid: usize, it: ItemType, d: &String) -> Self
    {
        Self { id: iid, item_type: it, desc: d.clone() }
    }
}