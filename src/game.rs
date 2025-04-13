use rand::Rng;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Ship {
    pub name: &'static str,
    pub size: usize,
    pub number: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    pub fn new_random()->Self {
        let mut rng = rand::rng();
        Self {
            col: rng.random_range(0..GRID_SIZE),
            row: rng.random_range(0..GRID_SIZE)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    // Fonction qui retourne une direction aléatoire
    pub fn random() -> Self {
        let random_num = rand::rng().random_range(0..2);

        match random_num {
            0 => Direction::Horizontal,
            _ => Direction::Vertical,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WarShip {
    ship: Ship,
    coord: Coord,
    direction: Direction,
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

#[derive(Debug, Clone)]
pub enum Cell {
    Empty,
    Ship(Rc<WarShip>),
    Hit(Rc<WarShip>),
    Miss(),
}

const GRID_SIZE: usize = 10;

type Grid = [[Cell; GRID_SIZE]; GRID_SIZE];

#[derive(Debug, Clone)]
pub struct Board {
    grid: Grid,
    player: Player,
}

impl Board {
    pub fn new(player: Player) -> Self {
        let grid = std::array::from_fn(|_| std::array::from_fn(|_| Cell::Empty));

        Self { grid, player }
    }

    pub fn display(self, player: &Player) {
        println!("Grille de {}", player.name);
        print!("   ");
        for x in 0..GRID_SIZE {
            print!("{} ", (b'A' + x as u8) as char);
        }
        println!();
        for (y, row) in self.grid.iter().enumerate() {
            print!("{:2} ", y + 1);
            for cell in row.iter() {
                let symbol = match cell {
                    Cell::Empty => ".",
                    Cell::Ship(_) => "S",
                    Cell::Hit(_) => "X",
                    Cell::Miss() => "o",
                };
                print!("{} ", symbol);
            }
            println!();
        }
    }

    pub fn place_ship(&mut self, war_ship: Rc<WarShip>) {
        let Coord { row, col } = war_ship.coord;
        let ship_size = war_ship.ship.size;
        let (dr, dc) = match war_ship.direction {
            Direction::Horizontal => (0, 1),
            Direction::Vertical => (1, 0),
        };
    
        for i in 0..ship_size {
            let r = row + i * dr;
            let c = col + i * dc;
            self.grid[r][c] = Cell::Ship(Rc::clone(&war_ship));
        }
    }

    pub fn is_place_ship_free(&self, war_ship: &WarShip) -> bool {
        let Coord { row, col } = war_ship.coord;
        let ship_size = war_ship.ship.size;
        let (dr, dc) = match war_ship.direction {
            Direction::Horizontal => (0, 1),
            Direction::Vertical => (1, 0),
        };

        (0..ship_size).all(|i| {
            let r = row + i * dr;
            let c = col + i * dc;
            r<GRID_SIZE && c<GRID_SIZE && matches!(self.grid[r][c], Cell::Empty)
        })
    }

    pub fn random_ship(&mut self) {

        for ship in SHIPS.iter()
        {
            let mut war_ship = WarShip {
                ship : *ship,
                coord: Coord::new_random(),
                direction: Direction::random(),
            };
            while !self.is_place_ship_free(&war_ship) {
                war_ship.coord=Coord::new_random();
                war_ship.direction=Direction::random();
            }
            self.place_ship(Rc::new(war_ship));
        }
    }

    pub fn random_placement(&mut self) {
        self.random_ship();
    }
}
