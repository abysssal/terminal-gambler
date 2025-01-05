use std::io;
use clearscreen;

mod casino;

fn main() {
    let money: i32 = 1000;

    clearscreen::clear().expect("Clearscreen did not, in fact, clear the screen. Thanks random crate, very cool.");
    
    println!("Welcome to Terminal Gambler!");
    println!("Type \"help\" for a list of commands");
    println!("");

    loop {
        clearscreen::clear().expect("Clearscreen did not, in fact, clear the screen. Thanks random crate, very cool.");
        
        println!("Please type a command.");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Didn't get that, what?");

        match input.as_str() {
            "casino" => casino::casino(),
            "quit" => {
                println!("Remember, 99% of gamblers quit before they win big.\n\nUntil next time!");
                break;
            },
            "help" => {
                println!("help - brings a list of commands")
            },
            _ => println!("Invalid command. If you need help, please type \"help\""),
        }
    }
}
