use rand::{thread_rng, Rng};

use crate::{dungeon::Dungeon, corridor::Corridor, door::Door, item::ItemType};

pub enum DungeonTile
{
    TileEmpty = 0,
    TileWall,
    TileDummy,
    TileClosedDoor,
    TileOpenDoor,
    TileChest,
    TileKey,
}

#[derive(Clone)]
pub struct DungeonMap
{
    map_width: usize,
    map_height: usize,
    map: Vec<Vec<u8>>,
}

impl DungeonMap
{
    pub fn new(width: usize, height: usize) -> Self
    {
        let mut m:Vec<Vec<u8>> = vec![vec![0; width]; height];

        (0..height).for_each(|y| {
            (0..width).for_each(|x| {
                m[x][y] = DungeonTile::TileWall as u8;
            });
        });

        Self 
        {
            map_width: width,
            map_height: height,
            map: m,
        }
    }

    fn create_rooms(&mut self, dungeon: &Dungeon)
    {
        let max_rooms = dungeon.get_rooms_number();

        for r in 0..max_rooms
        {
            if let Some(room) = dungeon.get_room(r)
            {
                let width = room.width as usize;
                let height = room.height as usize;

                for y in 0..height
                {
                    let ys = y + room.y as usize;

                    for x in 0..width
                    {
                        let xs = x + room.x as usize;

                        self.map[xs][ys] = DungeonTile::TileEmpty as u8;
                    }    
                }
            }
        }
    }

    fn get_left_wall(&self, corridor: &Corridor, dungeon: &Dungeon) -> (u16, u16, u16)
    {
        let from_room = dungeon.get_room_by_id(corridor.from_room_id).unwrap();
        let to_room = dungeon.get_room_by_id(corridor.to_room_id).unwrap();

        let left_room = if from_room.x + from_room.width <= to_room.x
        {
            (from_room.x + from_room.width, from_room.y, from_room.height)
        }
        else
        {
            (to_room.x, to_room.y, to_room.height)
        };

        left_room
    }

    fn get_right_wall(&self, corridor: &Corridor, dungeon: &Dungeon) -> (u16, u16, u16)
    {
        let from_room = dungeon.get_room_by_id(corridor.from_room_id).unwrap();
        let to_room = dungeon.get_room_by_id(corridor.to_room_id).unwrap();

        let right_room = if from_room.x + from_room.width > to_room.x
        {
            (from_room.x + from_room.width, from_room.y, from_room.height)
        }
        else
        {
            (to_room.x, to_room.y, to_room.height)
        };

        right_room
    }

    fn get_door_from(&self, corridor: &Corridor, dungeon: &Dungeon) -> Option<Door>
    {
        let c = dungeon.get_corridor(corridor.id);
        match c
        {
            Some(x) => x.from_room_door,
            None => None
        }
    }

    fn get_door_to(&self, corridor: &Corridor, dungeon: &Dungeon) -> Option<Door>
    {
        let c = dungeon.get_corridor(corridor.id);
        match c
        {
            Some(x) => x.to_room_door,
            None => None
        }
    }

    fn create_door_from(&mut self, corridor: &Corridor, dungeon: &Dungeon, prev_x: usize, room_wall_y: usize)
    {
        match self.get_door_from(corridor, dungeon)
        {
            Some(x) => 
            {
                let tile = match x.open
                {
                    true => DungeonTile::TileOpenDoor,
                    false => DungeonTile::TileClosedDoor
                };

                self.map[prev_x][room_wall_y] = tile as u8;
            },

            None => self.map[prev_x][room_wall_y] = DungeonTile::TileEmpty as u8
        }
    }

    fn create_door_to(&mut self, corridor: &Corridor, dungeon: &Dungeon, prev_x: usize, room_wall_y: usize)
    {
        match self.get_door_to(corridor, dungeon)
        {
            Some(x) => 
            {
                let tile = match x.open
                {
                    true => DungeonTile::TileOpenDoor,
                    false => DungeonTile::TileClosedDoor
                };

                self.map[prev_x][room_wall_y] = tile as u8;
            },

            None => self.map[prev_x][room_wall_y] = DungeonTile::TileEmpty as u8
        }
    }

    fn create_corridors(&mut self, dungeon: &Dungeon)
    {
        const MIN_CORRIDOR_LENGTH_FOR_DOOR: u16 = 3;

        let mut rng = thread_rng();
        let corridors_number = dungeon.get_corridors_number();

        for c in 0..corridors_number
        {
            if let Some(corridor) = dungeon.get_corridor(c)
            {
                let left_room = self.get_left_wall(&corridor, dungeon);
                let right_room = self.get_right_wall(&corridor, dungeon);

                //Find random right place in the wall of the left room to start drawing a corridor from
                let left_room_wall_y = rng.gen_range(0..left_room.2) + left_room.1;

                //..and the end point in the right wall of the second room
                let right_room_wall_y = rng.gen_range(0..right_room.2) + right_room.1;

                let pos_x0 = left_room.0;                
                let corridor_x_len = right_room.0 - pos_x0;
                
                let mut prev_x:usize = pos_x0 as usize;

                for x in 0..=corridor_x_len
                {
                    //Create door 1 if it does exist and has the correct length
                    if x == 1 && corridor_x_len >= MIN_CORRIDOR_LENGTH_FOR_DOOR
                    {
                        prev_x = (pos_x0 + x) as usize;                    
                        self.create_door_from(corridor, dungeon, prev_x, left_room_wall_y as usize);
                    }
                    else
                    {
                        prev_x = (pos_x0 + x) as usize;                    
                        self.map[prev_x][left_room_wall_y as usize] = DungeonTile::TileEmpty as u8;    
                    }
                }

                let corridor_y_len = left_room_wall_y.abs_diff(right_room_wall_y);
                let incr:isize = if left_room_wall_y >= right_room_wall_y
                {
                    -1
                }
                else
                {
                    1
                };

                for y in 0..=corridor_y_len
                {
                    self.map[prev_x][(left_room_wall_y as isize + y as isize * incr) as usize] = DungeonTile::TileEmpty as u8
                }

                //Create door 2 if it does exist and the length is right
                if corridor_y_len >= MIN_CORRIDOR_LENGTH_FOR_DOOR
                {
                    self.create_door_to(corridor, dungeon, prev_x, (left_room_wall_y as isize + (corridor_y_len - 2) as isize * incr) as usize);
                }
            }
        }
    }

    fn create_items(&mut self, dungeon: &Dungeon)
    {
        let mut rng = thread_rng();

        let rooms_number = dungeon.get_rooms_number();
        for r in 0..rooms_number
        {
            let room = dungeon.get_room(r).unwrap();
            let item_iter = room.items.iter();
            for i in item_iter
            {
                let mut loop_number = 10;

                while loop_number > 0
                {
                    let r_x = rng.gen_range(room.x..room.x + room.width) as usize;
                    let r_y = rng.gen_range(room.y..room.y + room.height) as usize;
    
                    let tile = self.map[r_x][r_y];
                    if tile != DungeonTile::TileChest as u8 && tile != DungeonTile::TileKey as u8
                    {
                        //Currently we won't store any item information
                        if i.item_type == ItemType::Key(0)
                        {
                            self.map[r_x][r_y] = DungeonTile::TileKey as u8;    
                        }
                        else
                        {
                            self.map[r_x][r_y] = DungeonTile::TileChest as u8;    
                        }

                        break;
                    }

                    loop_number -= 1;
                }
            }
        }
    }

    fn get_field(&self, x: isize, y: isize) -> Option<u8>
    {
        if x < 0 
            || y < 0 
            || x >= self.map_width as isize 
            || y >= self.map_height as isize
        {
            return None;
        }

        Some(self.map[x as usize][y as usize])
    }

    fn has_wall(&self, x: isize, y: isize) -> bool
    {
        match self.get_field(x, y)
        {
            Some(x) =>
            {
                return x == DungeonTile::TileWall as u8 || x == DungeonTile::TileDummy as u8;
            },
            None =>
            {
                return true;
            }
        }
    }

    fn has_walls_around(&self, x: isize, y: isize) -> bool
    {
        self.has_wall(x, y) 
            && self.has_wall(x - 1, y)
            && self.has_wall(x + 1, y)
            && self.has_wall(x, y + 1)
            && self.has_wall(x, y - 1)
            && self.has_wall(x - 1, y - 1)
            && self.has_wall(x + 1, y + 1)
            && self.has_wall(x - 1, y + 1)
            && self.has_wall(x + 1, y - 1)
    }

    fn remove_redundant_walls(&mut self)
    {
        (0..self.map_height as isize).for_each(|y| {
            (0..self.map_width as isize).for_each(|x| {
                if self.has_walls_around(x, y) == true
                {
                    self.map[x as usize][y as usize] = DungeonTile::TileDummy as u8;
                }
            });
        });
        (0..self.map_height).for_each(|y| {
            (0..self.map_width).for_each(|x| {
                if self.map[x][y] == DungeonTile::TileDummy as u8
                {
                    self.map[x][y] = DungeonTile::TileEmpty as u8;
                }
            });
        });
    }

    fn is_valid_door_position(&self, x: isize, y: isize) -> bool
    {
        (self.has_wall(x, y - 1)
        && self.has_wall(x, y + 1)
        && !self.has_wall(x + 1, y)
        && !self.has_wall(x - 1, y))

        ||

        (self.has_wall(x - 1, y)
        && self.has_wall(x + 1, y)
        && !self.has_wall(x, y - 1)
        && !self.has_wall(x, y + 1))
    }

    fn remove_not_useful_doors(&mut self)
    {
        (0..self.map_height).for_each(|y| {
            (0..self.map_width).for_each(|x| {
                let tile = self.map[x][y];

                if tile == DungeonTile::TileClosedDoor as u8
                {
                    if !self.is_valid_door_position(x as isize, y as isize)
                    {
                        self.map[x][y] = DungeonTile::TileEmpty as u8;
                    }
                }
            });
        });
    }

    pub fn create_map(&mut self, d: &Dungeon) -> &Vec<Vec<u8>>
    {
        self.create_rooms(&d);
        self.create_corridors(&d);
        self.remove_redundant_walls();
        self.remove_not_useful_doors();
        self.create_items(&d);        

        &self.map
    }
}