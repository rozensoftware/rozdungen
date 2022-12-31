use std::env;
use std::path::PathBuf;
use rozdungenlib::{dungeon::Dungeon, corridor::Corridor};
use ggez::{
    event,  
    glam::*,
    graphics::{self, DrawParam, Image},
    Context, GameResult,
};
use rand::thread_rng;
use rand::Rng;

const RESOURCES_DIR_NAME: &str = "resources";
const RESOURCE_SUBDIRS: [&str; 3] = ["images", "music", "sounds"];

const GAME_ID: &str = "Simple Dungeon Renderer";
const AUTHOR: &str = "Rozen Software";

const WINDOW_TITLE: &str = GAME_ID;
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 800.;
const TILE_SIZE: u16 = 32;
const MAX_ROOM_WIDTH: u16 = 4;
const MAX_ROOM_HEIGHT: u16 = 4;
const MAX_ROOMS_TO_GENERATE: u16 = 5;
const TILE_EMPTY:u8 = 0;
const TILE_WALL:u8 = 1;
const TILE_DUMMY:u8 = 2;

struct MainState     
{
    instances: graphics::InstanceArray,
}

impl MainState
{
    fn new(ctx: &mut Context, d: &Dungeon) -> GameResult<MainState> 
    {
        let image = Image::from_path(ctx, "/wall.png")?;
        let mut inst = graphics::InstanceArray::new(ctx, image);

        let map_width = WINDOW_WIDTH as usize / TILE_SIZE as usize;
        let map_height = WINDOW_HEIGHT as usize / TILE_SIZE as usize;
        let mut map:Vec<Vec<u8>> = vec![vec![0; map_width]; map_height];

        //Clear map with walls
        for y in 0..map_height
        {
            for x in 0..map_width
            {
                map[x][y] = TILE_WALL;
            }
        }

        self::MainState::create_rooms(&d, &mut map);
        self::MainState::create_corridors(&d, &mut map);
        self::MainState::remove_redundant_walls(&mut map);
        self::MainState::create_instances_from_map(&mut inst, &mut map);

        Ok(MainState {
            instances: inst,
        })
    }

    fn create_rooms(dungeon: &Dungeon, walls_pos: &mut Vec<Vec<u8>>)
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

                        walls_pos[xs][ys] = TILE_EMPTY;
                    }    
                }
            }
        }
    }

    fn get_left_wall(corridor: &Corridor) -> (u16, u16, u16)
    {
        let left_room = if corridor.from_room.x + corridor.from_room.width <= corridor.to_room.x
        {
            (corridor.from_room.x + corridor.from_room.width,  corridor.from_room.y, corridor.from_room.height)
        }
        else
        {
            (corridor.to_room.x,  corridor.to_room.y, corridor.to_room.height)
        };

        left_room
    }

    fn get_right_wall(corridor: &Corridor) -> (u16, u16, u16)
    {
        let right_room = if corridor.from_room.x + corridor.from_room.width > corridor.to_room.x
        {
            (corridor.from_room.x + corridor.from_room.width,  corridor.from_room.y, corridor.from_room.height)
        }
        else
        {
            (corridor.to_room.x,  corridor.to_room.y, corridor.to_room.height)
        };

        right_room
    }

    fn create_corridors(dungeon: &Dungeon, map: &mut Vec<Vec<u8>>)
    {
        let mut rng = thread_rng();
        let corridors_number = dungeon.get_corridors_number();

        for c in 0..corridors_number
        {
            if let Some(corridor) = dungeon.get_corridor(c)
            {
                let left_room = self::MainState::get_left_wall(&corridor);
                let right_room = self::MainState::get_right_wall(&corridor);

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
                    map[prev_x][left_room_wall_y as usize] = TILE_EMPTY;
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
                    map[prev_x][(left_room_wall_y as isize + y as isize * incr) as usize] = TILE_EMPTY
                }
            }

        }
    }

    fn get_field(x: isize, y: isize, map: &Vec<Vec<u8>>) -> Option<u8>
    {
        if x < 0 
            || y < 0 
            || x >= WINDOW_WIDTH as isize / TILE_SIZE as isize 
            || y >= WINDOW_HEIGHT as isize / TILE_SIZE as isize
        {
            return None;
        }

        Some(map[x as usize][y as usize])
    }

    fn has_wall(x: isize, y: isize, map: &Vec<Vec<u8>>) ->bool
    {
        match self::MainState::get_field(x, y, map)
        {
            Some(x) =>
            {
                return x == TILE_WALL || x == TILE_DUMMY;
            },
            None =>
            {
                return true;
            }
        }
    }

    fn has_walls_around(x: isize, y: isize, map: &Vec<Vec<u8>>) ->bool
    {
        self::MainState::has_wall(x, y, map) 
            && self::MainState::has_wall(x - 1, y, map)
            && self::MainState::has_wall(x + 1, y, map)
            && self::MainState::has_wall(x, y + 1, map)
            && self::MainState::has_wall(x, y - 1, map)
            && self::MainState::has_wall(x - 1, y - 1, map)
            && self::MainState::has_wall(x + 1, y + 1, map)
            && self::MainState::has_wall(x - 1, y + 1, map)
            && self::MainState::has_wall(x + 1, y - 1, map)
    }

    fn remove_redundant_walls(map: &mut Vec<Vec<u8>>)
    {
        for y in 0..WINDOW_HEIGHT as isize / TILE_SIZE as isize
        {
            for x in 0..WINDOW_WIDTH as isize / TILE_SIZE as isize
            {
                if self::MainState::has_walls_around(x, y, map) == true
                {
                    map[x as usize][y as usize] = TILE_DUMMY;
                }
            }
        }
        for y in 0..WINDOW_HEIGHT as usize / TILE_SIZE as usize
        {
            for x in 0..WINDOW_WIDTH as usize / TILE_SIZE as usize
            {
                if map[x][y] == TILE_DUMMY
                {
                    map[x][y] = TILE_EMPTY;
                }
            }
        }
    }

    fn create_instances_from_map(inst: &mut graphics::InstanceArray, map: &mut Vec<Vec<u8>>)
    {
        for y in 0..WINDOW_HEIGHT as usize / TILE_SIZE as usize
        {
            let tile_y = y * TILE_SIZE as usize;

            for x in 0..WINDOW_WIDTH as usize / TILE_SIZE as usize
            {
                let tile_x = x * TILE_SIZE as usize;
                if map[x][y] == TILE_WALL
                {
                    inst.push(DrawParam::new()
                    .dest(Vec2::new(tile_x as f32, tile_y as f32))
                    .scale(Vec2::new(1.0, 1.0))
                    .rotation(0.0));  
                }
            }
        }
    }

}

impl event::EventHandler<ggez::GameError> for MainState
{
    fn update(&mut self, _ctx: &mut Context) -> GameResult 
    {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult 
    {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.instances, DrawParam::new().dest(Vec2::new(0.0, 0.0)));

        canvas.finish(ctx)?;

        Ok(())
    }
}

fn get_resource_dirs() -> Vec<PathBuf> 
{
    let resources_root_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") 
    {
        let mut path = PathBuf::from(manifest_dir);
        path.push(RESOURCES_DIR_NAME);
        path
    } 
    else 
    {
        PathBuf::from(RESOURCES_DIR_NAME)
    };

    //Here we have three folders but currently we're using only images folder
    RESOURCE_SUBDIRS
        .iter()
        .map(|subdir| resources_root_dir.join(subdir).canonicalize().unwrap())
        .collect()
}

pub fn main() -> GameResult 
{
    let resource_dirs = get_resource_dirs();

    let mut context_builder = ggez::ContextBuilder::new(GAME_ID, AUTHOR)
        .window_setup(ggez::conf::WindowSetup::default().title(WINDOW_TITLE))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));

    for dir in resource_dirs 
    {
        context_builder = context_builder.add_resource_path(dir);
    }

    let mut d = Dungeon::new();

    let dungeon = match d.generate(MAX_ROOMS_TO_GENERATE, rozdungenlib::dungeon::DungeonType::SeparateRooms,
        WINDOW_WIDTH as u16 / TILE_SIZE, WINDOW_HEIGHT as u16 / TILE_SIZE, 
        MAX_ROOM_WIDTH, MAX_ROOM_HEIGHT)
    {
        Ok(x) => x,
        Err(y) => panic!("{}", y)
    };

    let (mut context, event_loop) = context_builder.build()?;
    let state = MainState::new(&mut context, &dungeon).unwrap();

    event::run(context, event_loop, state)    
}