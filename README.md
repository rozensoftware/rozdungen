# Rozdungen

![dungeon pic 1](https://github.com/rozensoftware/rozdungen/blob/master/dungeon.png)

Schema without decorations

![dungeon pic 2](https://github.com/rozensoftware/rozdungen/blob/master/dungeon2.png)

Dungeon with keys, doors and chests

![dungeon pic 3](https://github.com/rozensoftware/rozdungen/blob/master/dungeon3.png)

Dungeon type grid

## Brief

Rozdungenlib is a library for generating simple dungeons. Chambers are filled with items, monsters and other data you can use for building your own dungeon. Below are generation type modes: 

```
DungeonType::Basement, DungeonType::Grid and DungeonType::SeparateRooms.
``` 

They give slightly different dungeon schemas.

In the example a simple dungeon renderer is used. It doesn't mean the created maze looks exactly as it is presented. It might look differently according to your data visualization.
The dungeon interface provides you everything you need to create such labirynth and populate it with monsters, treasure, keys, doors and other stuff like that.

## Purpose

This library can be used in games based on the dungeons generated content like rougelikes or in any other applications where such mazes are needed.

## Use

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

    //Create random doors
    dungeon.add_doors().unwrap();
    
    //Get number of generated rooms
    let count = dungeon.get_rooms_number();

    //Get room with index 2
    let room = dungeon.rooms[2];

    //Get corridors connected with room of index 2
    let corrs = dungeon.get_room_corridors(&room);
```

## Building

The library must be added to your project (currently there is no version that can be added as an external crate). The example dungeon renderer uses ggez (https://github.com/ggez/ggez).

## Project future

The plan is to add some of the following features:

- Interior decoration of chambers
- Monsters
- Entering and exiting the dungeon
