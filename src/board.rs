use std::fmt;
use rand::Rng;

#[derive(Copy, Clone)]
pub enum PieceType {
    Empty,
    Water,
    Battleship,
    Cruiser,
    Destroyer,
    Submarine,
    Carrier,
}

impl fmt::Debug for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PieceType::Empty => write!(f, "|  "),
            PieceType::Water => write!(f, "|* "),
            PieceType::Battleship => write!(f, "|B "),
            PieceType::Cruiser => write!(f, "|U "),
            PieceType::Destroyer => write!(f, "|D "),
            PieceType::Submarine => write!(f, "|S "),
            PieceType::Carrier => write!(f, "|A "),
        }
    }
}

pub struct Board {
    board: [PieceType; 256],
}

impl Board {
    pub fn new() -> Self {
        let mut board = [PieceType::Water; 256];

        PieceType::Battleship.position(&mut board);

        for _i in 0..2 {
            PieceType::Cruiser.position(&mut board);
        }

        for _i in 0..3 {
            PieceType::Destroyer.position(&mut board);
        }

        for _i in 0..4 {
            PieceType::Submarine.position(&mut board);
        }

        for _i in 0..5 {
            PieceType::Carrier.position(&mut board);
        }

        Board { board }
    }

    pub fn empty() -> Self {
        Board { 
            board: [PieceType::Empty; 256],
        }
    }

    pub fn set(&mut self, position: usize, elem: PieceType) {
        self.board[position] = elem;
    }

    pub fn get(&self, position: usize) -> PieceType {
        self.board[position]
    }

    pub fn print(&self) {
        println!("   | A | B | C | D | E | F | G | H | I | J | K | L | M | O | P |");
        for i in 0..16 {
            if i < 9 {
                print!(" {} ", i + 1);
            } else {
                print!("{} ", i + 1);
            }

            for j in 0..16 {
                print!("{:?} ", self.board[16 * i + j])
            }
            println!();
        }
    }
}

pub trait Piece {
    fn position(&self, board: &mut[PieceType]);
    fn points(&self) -> i32;
}

impl Piece for PieceType {
    fn position(&self, board: &mut[PieceType]) {
        let mut rng = rand::thread_rng();
        let center = rng.gen_range(0..256);
        
        match *self {
            PieceType::Empty => (),
            PieceType::Water => (),
            PieceType::Battleship => {
                if center % 16 > (center + 4) % 16  {
                    self.position(board);
                    return;
                }
                for i in center..=center + 4 {
                    board[i] = PieceType::Battleship;
                }
            },
            PieceType::Cruiser => {
                if center % 16 > (center + 3) % 16  {
                    self.position(board);
                    return;
                }
                for i in center..=center + 3 {
                    if !matches!(board[i], PieceType::Water) {
                        self.position(board);
                        return;
                    }
                }
                for i in center..center + 4 {
                    board[i] = PieceType::Cruiser;
                }
            }
            PieceType::Destroyer => {
                if center % 16 > (center + 1) % 16  {
                    self.position(board);
                    return;
                }
                for i in center..=center + 1 {
                    if !matches!(board[i], PieceType::Water) {
                        self.position(board);
                        return;
                    }
                }
                for i in center..=center + 1 {
                    board[i] = PieceType::Destroyer;
                }
            },
            PieceType::Submarine => {
                if !matches!(board[center], PieceType::Water) {
                    self.position(board);
                    return;
                }
                board[center] = PieceType::Submarine;
            },
            PieceType::Carrier => {
                if center < 16 || center == 255 || (center - 1) % 16 > (center + 1) % 16 {
                    self.position(board);
                    return;
                }
                for i in [center - 16, center - 1, center + 1] {
                    if !matches!(board[i], PieceType::Water) {
                        self.position(board);
                        return;
                    }
                }
                for i in [center - 16, center - 1, center + 1] {
                    board[i] = PieceType::Carrier;
                }
            },
        }
    }

    fn points(&self) -> i32 {
        match *self {
            PieceType::Empty => 0,
            PieceType::Water => -3,
            PieceType::Battleship => 75,
            PieceType::Cruiser => 100,
            PieceType::Destroyer => 125, 
            PieceType::Submarine => 150,
            PieceType::Carrier => 200,
        }
    }
}
