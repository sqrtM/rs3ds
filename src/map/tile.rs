pub struct Tile {
    pub name: Tiles,
    pub default_char: char,
}

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