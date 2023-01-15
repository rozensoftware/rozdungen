use crate::door::Door;
use crate::item::Item;
use crate::item::ItemType;
use crate::room::Room;
use crate::corridor::Corridor;
use rand::thread_rng;
use rand::Rng;

#[derive(PartialEq)]
pub enum DungeonType
{
    Basement,       //Like one big basement with many walls and corridors
    SeparateRooms,  //Classic dungeon with separate rooms connected with corridors
    Grid,           //Rooms are aligned to the grid and connected with corridors
}

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

    /// Gets a room by its id
    ///  * 'room_id' - Room id
    pub fn get_room_by_id(&self, room_id: usize) -> Option<&Room>
    {
        self.rooms.iter().find(|&x| x.id == room_id)
    }

    /// Gets a corridor by its index
    /// * 'corridor_idx' - A corridor index
    pub fn get_corridor(&self, corridor_idx: usize) -> Option<&Corridor>
    {
        self.corridors.get(corridor_idx)
    }

    /// Counts how many doors exist in the dungeon
    pub fn get_doors_number(&self) -> usize
    {
        let mut num: usize = 0;

        for c in self.corridors.iter()
        {
            if c.from_room_door.is_some()
            {
                num += 1;
            }

            if c.to_room_door.is_some()
            {
                num += 1;
            }
        }

        num
    }

    /// Gets all corridors connected to specified room
    /// * 'room' - A room to which all found corridors are connected to
    pub fn get_room_corridors(&self, room: &Room) -> Vec<&Corridor>
    {
        let mut corridor_list:Vec<&Corridor> = Vec::new();
        let corridors: &Vec<Corridor>= &self.corridors;

        corridors.into_iter().for_each(|c|{
            if c.from_room_id == room.id || c.to_room_id == room.id
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

        for r in self.rooms.iter()
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

    fn generate_grid_rooms(&mut self, max_rooms: u16, max_dungeon_width: u16, max_dungeon_height: u16,
        max_room_width: u16, max_room_height: u16)
    {
        let max_grid_y = (max_dungeon_height / max_room_height) as usize;
        let max_grid_x = (max_dungeon_width / max_room_width) as usize;

        let mut grid_y = vec![false; max_grid_y];
        let mut grid_x = vec![false; max_grid_x];

        let mut rng = thread_rng();

        for _ in 0..max_rooms
        {
            let mut count = 10;

            while count > 0
            {
                let y = rng.gen_range(1..max_grid_y - 1);
                let x = rng.gen_range(1..max_grid_x - 1);
    
                if !grid_x[x] && !grid_y[y]
                {
                    grid_x[x] = true; grid_y[y] = true;
                    break;
                }
                
                count -= 1;
            }
        }

        let mut max_room_id = 0;

        for y in 0..max_grid_y
        {
            for x in 0..max_grid_x
            {
                if grid_x[x] && grid_y[y]
                {
                    let r = Room::new(max_room_id, x as u16 * max_room_width, y as u16 * max_room_height,
                         max_room_width - 1, max_room_height - 1);
                    max_room_id += 1;

                    self.rooms.push(r);
                }
            }
        }
    }

    /// Generates a dungeon
    /// * 'max_rooms' - Number of rooms to generate in the dungeon
    /// * 'max_dungeon_width' - Max. dungeon width in internal units
    /// * 'max_dungeon_height' - Max. dungeon height in internal units
    /// * 'max_room_width' - Max. room width in internal units
    /// * 'max_room_height' - Max room height in internal units
    pub fn generate(&mut self, max_rooms: u16, dungeon_type: DungeonType, max_dungeon_width: u16, max_dungeon_height: u16,
        max_room_width: u16, max_room_height: u16) -> Result<&mut Self, String>
    {
        if max_rooms == 0
        {
            return Err("Rooms number must not be a zero!".to_string());
        }
        if max_room_width >= max_dungeon_width - 2 || max_room_height >= max_dungeon_height - 2
        {
            return Err("Room size mismatch dungeon size!".to_string());
        }
        if max_room_width < 3 || max_room_height < 3
        {
            return Err("Room size too small (less than three)!".to_string());
        }

        let mut max_room_id = 0;

        let mut rng = thread_rng();
        
        //Create empty rooms

        if dungeon_type == DungeonType::Grid
        {
            self.generate_grid_rooms(max_rooms, max_dungeon_width, max_dungeon_height, max_room_width, max_room_height);
        }
        else
        {
            for _ in 0 .. max_rooms
            {
                let mut max_loop = 10;
    
                while max_loop > 0
                {
                    let x: u16 = rng.gen_range(1..max_dungeon_width - max_room_width - 1);
                    let y: u16 = rng.gen_range(1..max_dungeon_height - max_room_height - 1);
                    let w: u16 = rng.gen_range(2..max_room_width);
                    let h: u16 = rng.gen_range(2..max_room_height);
    
                    const SPACE_BETWEEN_ROOMS: i16 = 3;
    
                    let r = Room::new(max_room_id, x, y, w, h);
                    let r2 = if dungeon_type == DungeonType::SeparateRooms && x as i16 - SPACE_BETWEEN_ROOMS >= 0 && y as i16 - SPACE_BETWEEN_ROOMS >= 0
                    {
                        Room::new(max_room_id, x - SPACE_BETWEEN_ROOMS as u16, y - SPACE_BETWEEN_ROOMS as u16, w + 3, h + 3)
                    }
                    else
                    {
                        r.clone()
                    };    
    
                    if !self.is_intersect_with_another_room(&r2)
                    {
                        self.rooms.push(r2);
                        max_room_id += 1;                            
                        break;
                    }
    
                    max_loop -= 1;
                }
            }    
        }

        let rooms_number = self.get_rooms_number();

        if max_rooms > 1
        {
            //Connect rooms with corridors
            let mut max_corridor_id = 0;

            if dungeon_type == DungeonType::Basement
            {
                for r_idx in 0..rooms_number
                {
                    let mut idx: usize;
                    let r1 = &self.rooms[r_idx];

                    loop 
                    {
                        idx = rng.gen_range(0 .. rooms_number);
                        if idx != r1.id
                        {
                            break;
                        }
                    }
    
                    let r2 = &self.rooms[idx];
                    let corridor = Corridor::new(max_corridor_id, r1.id, r2.id, None, None);
                    self.corridors.push(corridor);
                    max_corridor_id += 1;        
                }
            }
            else if dungeon_type == DungeonType::SeparateRooms || dungeon_type == DungeonType::Grid
            {
                for i in 0..(rooms_number as isize) - 1
                {
                    let r1 = &self.rooms[i as usize];
                    let r2 = &self.rooms[i as usize + 1];

                    let corridor = Corridor::new(max_corridor_id, r1.id, r2.id, None, None);
                    self.corridors.push(corridor);
                    max_corridor_id += 1;    
                }
            }
        }

        Ok(self)
    }

    /// Populate chambers with items
    /// * 'keys' - if true the routine try to add keys for locked doors. If false no keys will be created
    pub fn add_items(&mut self, keys: bool)
    {
        let mut rng = thread_rng();
        let mut item_id = 0;
        let doors_number = self.get_doors_number();
        let rooms_number = self.get_rooms_number();

        if keys && doors_number > 0
        {
            //Add key to random room. Currently every key open every door
            //It generates number of keys exactly equal to the number of existing doors
            (0..doors_number).for_each(|_| {
                let item = Item::new(item_id, ItemType::Key(0), &"Universal Key".to_string());
                let room_idx= rng.gen_range(0..rooms_number);
                let r = &mut self.rooms[room_idx];
                r.items.push(item);

                item_id += 1;
            });
        }

        //Gererate random items
        let item_type_vec = vec![ItemType::Weapon, ItemType::Armor, ItemType::Potion];
        let number_items_to_generate = rng.gen_range(rooms_number..rooms_number + 2);
        let number_item_type = item_type_vec.len();

        for _ in 0..number_items_to_generate
        {
            let item_type = match rng.gen_range(0..number_item_type)
            {
                0 => ItemType::Weapon,
                1 => ItemType::Armor,
                2 => ItemType::Potion,
                _ => panic!("Unknown item type!")
            };

            let item = Item::new(item_id, item_type, &format!("Item: {}", item_id));
            let room_idx= rng.gen_range(0..rooms_number);
            let r = &mut self.rooms[room_idx];
            r.items.push(item);

            item_id += 1;
        }
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
        
        const DOOR_CREATION_CHANCE: u8 = 75;
        const DOORS_ON_BOTH_SIDES_CHANCE: u8 = 40;

        self.corridors.iter_mut().for_each(|c| {
            if rng.gen_range(1..=100) <= DOOR_CREATION_CHANCE
            {
                let d1 = Door{id: door_id, locked: false, open: rng.gen_range(0..100) > 50};

                if rng.gen_range(1..=100) <= DOORS_ON_BOTH_SIDES_CHANCE
                {
                    door_id += 1;
                    let d2 = Door{id: door_id, locked: false, open: false};
                    c.from_room_door = Some(d2);
                }

                door_id += 1;
                c.to_room_door = Some(d1);
            }
        });

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

        let room = &d.rooms[2];
        let corrs = d.get_room_corridors(&room);
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