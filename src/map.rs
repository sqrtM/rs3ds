pub struct Tile {
    pub name: Tiles,
    pub default_char: char,
}

#[derive(PartialEq)]
pub enum Tiles {
    Wall,
    Floor,
}

pub const fn tile(tile_name: Tiles, char: char) -> Tile {
    return Tile {
        name: tile_name,
        default_char: char
    }
}

const MAP_HEIGHT: usize = 16;
const MAP_WIDTH: usize = 16;


pub const MAP: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

pub const TILE_MAP: [[Tile; MAP_WIDTH]; MAP_HEIGHT] = [
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Floor, '.'),
        tile(Tiles::Wall, '#'),
    ],
    [
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
        tile(Tiles::Wall, '#'),
    ],
];
