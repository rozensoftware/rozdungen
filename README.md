# Rozdungen

Rozdungenlib is a library for generating simple dungeons. Currently, the dungeons consist of empty chambers connected by corridors. Two generation modes are available: 

```rust
DungeonType::Basement and DungeonType::SeparateRooms.
``` 

They give slightly different dungeon schemas.


## Purpose

This library can be used in games based on the dungeons generated content like rougelikes or in any other applications where such mazes are needed.

## Uses

Code usage example:

```rust
    let mut d = Dungeon::new();
    
    let res = d.generate(
        MAX_ROOMS, 
        DungeonType::Basement,  //or SeparateRooms
        MAX_DUNGEON_WIDTH, 
        MAX_DUNGEON_HEIGHT, 
        MAX_ROOM_WIDTH, 
        MAX_ROOM_HEIGHT);

    let dungeon  = match res
    {
        Ok(x) =>
        {
            x
        },
        Err(y) =>
        {
            panic!("{}", y);
        }
    };

    //Get number of generated rooms
    let count = dungeon.get_rooms_number();

    //Get room woth index 2
    let room = dungeon.rooms[2];

    //Get corridors connected with room of index 2
    let corrs = dungeon.get_room_corridors(&room);
```

## Building

The library must be added to your project (currently there is no version that can be added as an external crate). The example dungeon renderer uses ggez (https://github.com/ggez/ggez).

## Project future

The library will be expanded in future. The plan is to add some of the following features:

- Interior decoration of chambers
- Monsters
- Doors and keys
- Treasures
- Entering and exiting the dungeon