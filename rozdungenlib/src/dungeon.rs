use crate::door::Door;
use crate::room::Room;
use crate::corridor::Corridor;
use rand::thread_rng;
use rand::Rng;

#[derive(PartialEq)]
pub enum DungeonType
{
    Basement,       //Like one big room with many walls
    SeparateRooms   //Classic dunegon with separate rooms connected with corridors
}

#[derive(Debug)]
pub struct Dungeon
{
    rooms: Vec<Room>,
    corridors: Vec<Corridor>,
}

impl Dungeon
{
    pub fn new() -> Self
    {
        Self { rooms: Vec::new(), corridors: Vec::new() }
    }
    
    /// Gets number of rooms
    pub fn get_rooms_number(&self) -> usize
    {
        self.rooms.len()
    }

    /// Gets number of corridors
    pub fn get_corridors_number(&self) -> usize
    {
        self.corridors.len()
    }

    /// Gets a room by its index in the array
    ///  * 'room_idx' - Room index
    pub fn get_room(&self, room_idx: usize) -> Option<&Room>
    {
        self.rooms.get(room_idx)
    }

    /// Gets a corridor by its index
    /// * 'corridor_idx' - A corridor index
    pub fn get_corridor(&self, corridor_idx: usize) -> Option<&Corridor>
    {
        self.corridors.get(corridor_idx)
    }

    /// Gets all corridors connected to specified room
    /// * 'room' - A room to which all found corridors are connected to
    pub fn get_room_corridors(&self, room: Room) -> Vec<&Corridor>
    {
        let mut corridor_list:Vec<&Corridor> = Vec::new();
        let corridors: &Vec<Corridor>= &self.corridors;

        corridors.into_iter().for_each(|c|{
            if c.from_room == room || c.to_room == room
            {
                corridor_list.push(c);
            }
        });

        corridor_list
    }

    /// Check if the given room interescts with another existing room in the dungeon
    /// * 'room' - A reference to the room we test the intersection with
    fn is_intersect_with_another_room(&self, room: &Room) -> bool
    {
        let x1 = room.x;
        let y1 = room.y;
        let x2 = x1 + room.width;
        let y2 = x2 + room.height;

        for r in &self.rooms
        {
            let xr1 = r.x;
            let yr1 = r.y;
            let xr2 = xr1 + r.width;
            let yr2 = yr1 + r.height;

            if x1 <= xr2 && x2 >= xr1 && y1 <= yr2 && y2 >= yr1
            {
                return true;
            }
        }

        false
    }

    /// Generates a dungeon
    /// * 'max_rooms' - Number of rooms to generate in the dungeon
    /// * 'max_dungeon_width' - Max. dungeon width in internal units
    /// * 'max_dungeon_height' - Max. dungeon height in internal units
    /// * 'max_room_width' - Max. room width in internal units
    /// * 'max_room_height' - Max room height in internal units
    pub fn generate(&mut self, max_rooms: u16, dungeon_type: DungeonType, max_dungeon_width: u16, max_dungeon_height: u16,
        max_room_width: u16, max_room_height: u16) -> Result<&mut Dungeon, String>
    {
        if max_rooms == 0
        {
            return Err("Rooms number must not be a zero!".to_string());
        }
        if max_room_width >= max_dungeon_width - 2 || max_room_height >= max_dungeon_height - 2
        {
            return Err("Room size mismatch dungeon size!".to_string());
        }

        let mut max_room_id = 0;

        let mut rng = thread_rng();
        
        //Create empty rooms
        for _ in 0 .. max_rooms
        {
            let mut max_loop = 10;

            while max_loop > 0
            {
                let x: u16 = rng.gen_range(1..max_dungeon_width - max_room_width - 1);
                let y: u16 = rng.gen_range(1..max_dungeon_height - max_room_height - 1);
                let w: u16 = rng.gen_range(2..max_room_width);
                let h: u16 = rng.gen_range(2..max_room_height);

                const ROOM_SPACE: i16 = 3;

                let r = Room::new(max_room_id, x, y, w, h);
                let r2 = if dungeon_type == DungeonType::SeparateRooms && x as i16 - ROOM_SPACE >= 0 && y as i16 - ROOM_SPACE >= 0
                {
                    Room::new(max_room_id, x - ROOM_SPACE as u16, y - ROOM_SPACE as u16, w + 3, h + 3)
                }
                else
                {
                    r
                };    

                if self.is_intersect_with_another_room(&r2) == false
                {
                    self.rooms.push(r);
                    max_room_id += 1;                            
                    break;
                }

                max_loop -= 1;
            }
        }

        if max_rooms > 1
        {
            //Connect rooms with corridors
            let mut max_corridor_id = 0;

            let rooms_array= &self.rooms;
            let actual_rooms_size = self.rooms.len();

            if dungeon_type == DungeonType::Basement
            {
                rooms_array.into_iter().for_each(|r| {
                    let mut idx: usize;

                    loop 
                    {
                        idx = rng.gen_range(0 .. actual_rooms_size);
                        if idx != r.id
                        {
                            break;
                        }
                    }
    
                    let corridor = Corridor::new(max_corridor_id, r.clone(), self.rooms[idx], None, None);
                    self.corridors.push(corridor);
                    max_corridor_id += 1;    
                });
            }
            else if dungeon_type == DungeonType::SeparateRooms
            {
                let rooms_number = rooms_array.len();

                for i in 0..rooms_number - 1
                {
                    let r1 = self.rooms[i];
                    let r2 = self.rooms[i + 1];

                    let corridor = Corridor::new(max_corridor_id, r1, r2, None, None);
                    self.corridors.push(corridor);
                    max_corridor_id += 1;    
                }
            }
        }

        Ok(self)
    }

    /// Adds random doors in the dungeon. This function must be called after generate function
    pub fn add_doors(&mut self) -> Result<(), String>
    {
        if self.rooms.len() == 1
        {
            return Err("There's only one room in the dungeon. No door is needed.".to_string());
        }

        let mut door_id = 0;
        let mut rng = thread_rng();
        
        const DOOR_CREATION_CHANCE: u8 = 50;
        const DOORS_ON_BOTH_SIDES_CHANCE: u8 = 25;

        for c in &mut self.corridors
        {
            if rng.gen_range(1..=100) <= DOOR_CREATION_CHANCE
            {
                let d1 = Door{id: door_id, locked: false, open: false};

                if rng.gen_range(1..=100) <= DOORS_ON_BOTH_SIDES_CHANCE
                {
                    door_id += 1;
                    let d2 = Door{id: door_id, locked: false, open: false};
                    c.from_room_door = Some(d2);
                }

                door_id += 1;
                c.to_room_door = Some(d1);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]    
    fn create_dungeon_test()
    {
        const MAX_ROOMS:u16 = 10;
        const MAX_DUNGEON_WIDTH: u16 = 100;
        const MAX_DUNGEON_HEIGHT: u16 = 100;
        const MAX_ROOM_WIDTH: u16 = 10;
        const MAX_ROOM_HEIGHT: u16 = 10;

        let mut d = Dungeon::new();
        d.generate(
            MAX_ROOMS, 
            DungeonType::Basement,
            MAX_DUNGEON_WIDTH, 
            MAX_DUNGEON_HEIGHT, MAX_ROOM_WIDTH, 
            MAX_ROOM_HEIGHT).unwrap();

        let count = d.get_rooms_number();
        assert!(count > 0);

        let room = d.rooms[2];
        let corrs = d.get_room_corridors(room);
        assert!(corrs.len() != 0);

        d.add_doors().unwrap();
    }

    #[test]    
    fn create_dungeon_fail_test()
    {
        let mut dungeon = Dungeon::new();
        let res = dungeon.generate(
            10, 
            DungeonType::Basement,
            100, 
            10,
            10, 
            10);
        match res
        {
            Ok(_) =>
            {
                panic!("Bad test!")
            },
            Err(_) =>
            {
            }
        };
    }
}