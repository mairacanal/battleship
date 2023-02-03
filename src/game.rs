use std::process;

use crate::board::{Board, Piece};

pub struct Game {
    player1: Player,
    player2: Player,
    turn: bool
}

struct Player {
    points: i32,
    id: u8,
    board: Board,
    display_board: Board,
}

impl Player {
    pub fn new(id: u8) -> Self {
        Player {
            points: 0,
            id,
            board: Board::new(),
            display_board: Board::empty(),
        }
    }

    pub fn reset_board(&mut self) {
        self.board = Board::new();
        self.display_board = Board::empty();
    }

    pub fn turn(&mut self, position: &str, board: &Board) {
        let position : Vec<char> = position.chars().collect();
        let column = position[position.len() - 1] as i32 % 65;
        let line;

        match position.len() {
            3 => {
                line = position[0].to_string().parse::<i32>().unwrap() * 10 +
                    position[1].to_string().parse::<i32>().unwrap() - 1;
            },
            2 => {
                line = position[0].to_string().parse::<i32>().unwrap() - 1;
            },
            _ => {
                println!("Invalid position!");
                return;
            }
        }

        let position = (line * 16 + column) as usize;

        let elem = board.get(position);

        self.points += elem.points();
        self.display_board.set(position, elem);

        self.display_board.print();
    }

    pub fn print(&self) {
        println!("================================================================");
        println!("                  Player {} - Points: {}                         ", self.id, self.points);
        println!("================================================================");
    }

    pub fn print_board(&self) {
        self.display_board.print();
    }
}

impl Game {
    pub fn new() -> Self {
        let game = Game {
            player1: Player::new(1),
            player2: Player::new(2),
            turn: true,
        };

        game
    }

    pub fn init(&self) {
        if self.turn {
            self.player1.print();
            self.player1.print_board();
        } else {
            self.player2.print();
            self.player2.print_board();
        }
    }

    pub fn pow(&mut self, position: &str) {
        // Player 1 turn
        if self.turn {
            self.player1.turn(position, &self.player2.board);
        }

        // Player 2 turn
        else {
            self.player2.turn(position, &self.player1.board);
        }

        self.turn = !self.turn;
    }

    pub fn reset(&mut self) {
        self.player1 = Player::new(1);
        self.player2 = Player::new(2);
        self.turn = true;
    }

    pub fn exit(&self) {
        println!("================================================================");
        self.player1.print();
        self.player2.print();
        println!("                      Thanks for playing!                       ");
        println!("================================================================");

        process::exit(1);
    }

    pub fn help(&self) {
        println!("================================================================");
        println!("pow <position> - inform the position you want to hit");
        println!("reset - init a new round, resetting to the initial state");
        println!("help - show the possible game commands");
        println!("exit - leave the game");
        println!("lucky - init a new random board, but save the points");
        println!("================================================================");
    }

    pub fn lucky(&mut self) {
        self.player1.reset_board();
        self.player2.reset_board();
    }
}

