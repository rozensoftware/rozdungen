use std::{env, rc::Rc};
use std::path::PathBuf;
use ggez::graphics::{Canvas, Color};
use rozdungenlib::{dungeon::Dungeon, dungeonmap::{DungeonMap, DungeonTile}};
use ggez::{
    event,  
    glam::*,
    graphics::{self, DrawParam, Image},
    Context, GameResult,
};
const RESOURCES_DIR_NAME: &str = "resources";
const RESOURCE_SUBDIRS: [&str; 3] = ["images", "music", "sounds"];

const GAME_ID: &str = "Simple Dungeon Renderer";
const AUTHOR: &str = "Rozen Software";

const WINDOW_TITLE: &str = GAME_ID;
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 800.;
const TILE_SIZE: u16 = 32;
const MAX_ROOM_WIDTH: u16 = 5;
const MAX_ROOM_HEIGHT: u16 = 4;
const MAX_ROOMS_TO_GENERATE: u16 = 6;

struct MainState     
{
    instances: graphics::InstanceArray,
    map: Rc<Vec<Vec<u8>>>,
    open_door_image: graphics::Image,
    closed_door_image: graphics::Image,
}

impl MainState
{
    fn new(ctx: &mut Context, d: &mut Dungeon) -> GameResult<MainState> 
    {
        let image = Image::from_path(ctx, "/wall.png")?;
        let mut inst = graphics::InstanceArray::new(ctx, image);

        let map_width = WINDOW_WIDTH as usize / TILE_SIZE as usize;
        let map_height = WINDOW_HEIGHT as usize / TILE_SIZE as usize;

        let mut dm = DungeonMap::new(map_width, map_height);
        let map = dm.create_map(d);

        self::MainState::create_instances_from_map(&mut inst, map);

        Ok(MainState {
            instances: inst,
            map: Rc::new(map.clone()),
            open_door_image: graphics::Image::from_path(ctx, "/door_open.png")?,
            closed_door_image: graphics::Image::from_path(ctx, "/door_closed.png")?,
        })
    }

    fn create_instances_from_map(inst: &mut graphics::InstanceArray, map: &Vec<Vec<u8>>)
    {
        for y in 0..WINDOW_HEIGHT as usize / TILE_SIZE as usize
        {
            let tile_y = y * TILE_SIZE as usize;

            for x in 0..WINDOW_WIDTH as usize / TILE_SIZE as usize
            {
                let tile_x = x * TILE_SIZE as usize;

                if map[x][y] == DungeonTile::TileWall as u8
                {
                    inst.push(DrawParam::new()
                    .dest(Vec2::new(tile_x as f32, tile_y as f32))
                    .scale(Vec2::new(1.0, 1.0))
                    .rotation(0.0));  
                }
            }
        }
    }

    pub fn draw_doors(&self, canvas: &mut Canvas)
    {
        let color = Color::from((255, 255, 255, 255));

        for y in 0..WINDOW_HEIGHT as usize / TILE_SIZE as usize
        {
            let tile_y = y * TILE_SIZE as usize;

            for x in 0..WINDOW_WIDTH as usize / TILE_SIZE as usize
            {
                let tile_x = x * TILE_SIZE as usize;
                let tile = self.map[x][y];

                if tile == DungeonTile::TileOpenDoor as u8
                {
                    canvas.draw(&self.open_door_image, DrawParam::new()
                        .dest(Vec2::new(tile_x as f32, tile_y as f32))
                        .color(color));                
                }
                else if tile == DungeonTile::TileClosedDoor as u8
                {
                    canvas.draw(&self.closed_door_image, DrawParam::new()
                        .dest(Vec2::new(tile_x as f32, tile_y as f32))
                        .color(color));                
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

        self.draw_doors(&mut canvas);

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

    dungeon.add_doors().unwrap();

    let (mut context, event_loop) = context_builder.build()?;
    let state = MainState::new(&mut context, dungeon).unwrap();

    event::run(context, event_loop, state)    
}