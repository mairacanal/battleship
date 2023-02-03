use std::io;

use battleship::game::Game;

fn main() -> io::Result<()> {
    println!("================================================================");
    println!("                           BATTLESHIP                           ");
    println!("================================================================");
    println!("Type pow <position> to shoot or help to see the commands.");

    let mut game = Game::new();

    loop {
        let mut command = String::new();

        game.init();

        println!("Command: ");
        io::stdin().read_line(&mut command)?;

        let command = command.trim();
        let command : Vec<&str> = command.split(" ").collect();

        match command[0] {
            "pow" => game.pow(command[1]),
            "reset" => game.reset(),
            "exit" => game.exit(),
            "help" => game.help(),
            "lucky" => game.lucky(),
            _ => println!("Invalid command! Try again!"),
        }
    }
}
