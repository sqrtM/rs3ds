use self::tile::tile;

mod tile;

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

pub const TILE_MAP: [[tile::Tile; 8]; 8] = [
    [
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
    ],
    [
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
    ],
    [
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
    ],
    [
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
    ],
    [
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
    ],
    [
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
    ],
    [
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
    ],
    [
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
        tile(tile::Tiles::Wall, '#'),
    ],
];