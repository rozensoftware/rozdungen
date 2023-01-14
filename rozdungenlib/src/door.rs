#[derive(Clone, Copy, PartialEq)]
pub struct Door
{
    //An unique id of the door
    pub id: usize,
    /// If true the door is locked
    pub locked: bool,
    //True the door is opened
    pub open: bool,
}