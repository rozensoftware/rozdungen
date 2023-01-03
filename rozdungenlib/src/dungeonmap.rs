use rand::{thread_rng, Rng};

use crate::{dungeon::Dungeon, corridor::Corridor};

pub enum DungeonTile
{
    TileEmpty = 0,
    TileWall,
    TileDummy
}

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

        for y in 0..height
        {
            for x in 0..width
            {
                m[x][y] = DungeonTile::TileWall as u8;
            }
        }

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

    fn get_left_wall(&self, corridor: &Corridor) -> (u16, u16, u16)
    {
        let left_room = if corridor.from_room.x + corridor.from_room.width <= corridor.to_room.x
        {
            (corridor.from_room.x + corridor.from_room.width, corridor.from_room.y, corridor.from_room.height)
        }
        else
        {
            (corridor.to_room.x,  corridor.to_room.y, corridor.to_room.height)
        };

        left_room
    }

    fn get_right_wall(&self, corridor: &Corridor) -> (u16, u16, u16)
    {
        let right_room = if corridor.from_room.x + corridor.from_room.width > corridor.to_room.x
        {
            (corridor.from_room.x + corridor.from_room.width, corridor.from_room.y, corridor.from_room.height)
        }
        else
        {
            (corridor.to_room.x,  corridor.to_room.y, corridor.to_room.height)
        };

        right_room
    }

    fn create_corridors(&mut self, dungeon: &Dungeon)
    {
        let mut rng = thread_rng();
        let corridors_number = dungeon.get_corridors_number();

        for c in 0..corridors_number
        {
            if let Some(corridor) = dungeon.get_corridor(c)
            {
                let left_room = self.get_left_wall(&corridor);
                let right_room = self.get_right_wall(&corridor);

                //Find random right place in the wall of the left room to start drawing a corridor from
                let left_room_wall_y = rng.gen_range(0..left_room.2) + left_room.1;

                //..and the end point in the right wall of the second room
                let right_room_wall_y = rng.gen_range(0..right_room.2) + right_room.1;

                let pos_x0 = left_room.0;
                let corridor_x_length = right_room.0 - pos_x0;
                
                let mut prev_x:usize = 0;

                for x in 0..=corridor_x_length
                {
                    prev_x = (pos_x0 + x) as usize;
                    self.map[prev_x][left_room_wall_y as usize] = DungeonTile::TileEmpty as u8;
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
        for y in 0..self.map_height as isize
        {
            for x in 0..self.map_width as isize
            {
                if self.has_walls_around(x, y) == true
                {
                    self.map[x as usize][y as usize] = DungeonTile::TileDummy as u8;
                }
            }
        }
        for y in 0..self.map_height
        {
            for x in 0..self.map_width
            {
                if self.map[x][y] == DungeonTile::TileDummy as u8
                {
                    self.map[x][y] = DungeonTile::TileEmpty as u8;
                }
            }
        }
    }

    pub fn create_map(&mut self, d: &Dungeon) -> &Vec<Vec<u8>>
    {
        self.create_rooms(&d);
        self.create_corridors(&d);
        self.remove_redundant_walls();

        &self.map
    }
}