#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Door
{
    //An unique id of the door
    pub id: usize,
    /// If true the door is locked
    pub locked: bool,
    /// If true the door exists, if not there is no door at this place
    pub exists: bool,
}