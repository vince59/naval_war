#[derive(Debug, Clone, Copy,PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Ship {
    pub name: &'static str,
    pub size: u8,
    pub number: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    row: u8,
    col: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertital,
}

#[derive(Debug, Clone, Copy)]
pub struct WarShip {
    ship: Ship,
    coord: Coord,
    direction: Direction,
    player: Player,
}

pub const SHIPS: [Ship; 5] = [
    Ship {
        name: "Carrier",
        size: 5,
        number: 1,
    },
    Ship {
        name: "Battleship",
        size: 4,
        number: 1,
    },
    Ship {
        name: "Cruiser",
        size: 3,
        number: 1,
    },
    Ship {
        name: "Submarine",
        size: 3,
        number: 2,
    },
    Ship {
        name: "Destroyer",
        size: 2,
        number: 2,
    },
];

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Ship(WarShip),
    Hit(WarShip),
    Miss(Player),
}

const GRID_SIZE: usize = 10;

type Grid = [[Cell; GRID_SIZE]; GRID_SIZE];

#[derive(Debug, Clone, Copy)]
pub struct Board {
    grid: Grid,
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[Cell::Empty; GRID_SIZE]; GRID_SIZE],
        }
    }

    pub fn display(self,player: &Player) {
        println!("Grille de {}",player.name);
        print!("    ");
        for x in 0..GRID_SIZE {
            print!("{:<3} ", (b'A' + x as u8) as char);
        }
        println!();
        for (y, row) in self.grid.iter().enumerate() {
            print!("{:2} ", y + 1); 
            for cell in row.iter() {
                let symbol = match cell {
                    Cell::Empty => "🌊",
                    Cell::Ship(s) => if s.player==*player {"🛳️"} else {"🌊"},
                    Cell::Hit(s) => if s.player==*player {"💥"} else {"❌"},
                    Cell::Miss(p) => if p==player {"⭘"} else {"☔"},
                };
                print!("{:<3}", symbol);
            }
            println!();
        }
    }
}
