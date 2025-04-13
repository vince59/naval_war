#[derive(Debug, Clone, Copy)]
pub struct Player{
    name : &'static str
}

#[derive(Debug, Clone, Copy)]
pub struct Ship{
    pub name: &'static str,
    pub size : u8,
    pub number: u8
}

#[derive(Debug, Clone, Copy)]
pub struct Coord{
    row:u8,
    col:u8
}

#[derive(Debug, Clone, Copy)]
pub enum Direction{
    Horizontal,
    Vertital
}

#[derive(Debug, Clone, Copy)]
pub struct WarShip{
    ship:Ship,
    coord:Coord,
    direction: Direction,
    player: Player
}

pub const SHIPS: [Ship; 5] = [
    Ship { name: "Carrier", size: 5, number: 1 },
    Ship { name: "Battleship", size: 4, number: 1 },
    Ship { name: "Cruiser", size: 3, number: 1 },
    Ship { name: "Submarine", size: 3, number: 2 },
    Ship { name: "Destroyer", size: 2, number: 2 },
];

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Ship(WarShip),
    Hit(WarShip),
    Miss
}

const GRID_SIZE: usize = 10;

type Grid = [[Cell; GRID_SIZE]; GRID_SIZE];

pub fn create_empty_grid() -> Grid {
    [[Cell::Empty; GRID_SIZE]; GRID_SIZE]
}
