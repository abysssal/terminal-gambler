use std::io;
use clearscreen;

mod casino;

struct Stats {
    money: i32,
    luck_exp: i32,
    max_luck_exp: i32,
    luck_level: i32,
    work_exp: i32,
    max_work_exp: i32,
    work_level: i32,
    place: Places
}

enum Places {
    Casino,
    Bank,
    Home,
    KingsBurgers,
    AverageStartup,
    EthicalHacker
}

impl Places {
    fn convert_to_string(place: &Places) -> String {
        match place {
            Places::Home => "home".to_string(),
            Places::Casino => "casino".to_string(),
            Places::Bank => "bank".to_string(),
            Places::KingsBurgers => "kings_burgers".to_string(),
            Places::AverageStartup => "average_startup".to_string(),
            Places::EthicalHacker => "ethical_hacker".to_string()
        }
    }
}

fn main() {
    clearscreen::clear().expect("Clearscreen did not, in fact, clear the screen. Thanks clearscreen, very cool.");

    let help_messages: Vec<&str> = vec![
        "help - brings this menu back up if command is empty, otherwise,",
        "quit - returns you to your terminal",
        "stats - shows your stats",
        "bank - brings you to the bank",
        "casino - brings you to the casino",
        "work - get to work and make money to gamble away"
    ];

    let mut stats: Stats = Stats { 
        money: 1000, 
        luck_exp: 0, 
        max_luck_exp: 100, 
        luck_level: 0, 
        work_exp: 0, 
        max_work_exp: 100,
        work_level: 0,
        place: Places::Home
    };

    let mut first_time: bool = true;

    loop {
        if first_time {
            println!("Welcome to Terminal Gambler!");
            println!("Type \"help\" for a list of commands");
            first_time = false;
        }

        println!("\nInsert a command below");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Didn't get that, what?");

        let mut command= input.split_whitespace();

        clearscreen::clear().expect("Clearscreen did not, in fact, clear the screen. Thanks clearscreen, very cool.");

        match command.next() {
            Some("casino") => {
                stats.place = Places::Casino;
                stats.money = casino::casino(stats.money);
                stats.place = Places::Home;
            },
            Some("quit") => {
                println!("Remember, 99% of gamblers quit before they win big.\nUntil next time!");
                break;
            },
            Some("help") => {
                for message in &help_messages {
                    println!("{}", message);
                }
            },
            Some("stats") => {
                println!("Your stats:");
                println!("-----------------");
                println!("Money: {}", &stats.money);
                println!("Luck EXP: {}/{}", &stats.luck_exp, &stats.max_luck_exp);
                println!("Luck Level: {}", &stats.luck_level);
                println!("Work EXP: {}/{}", &stats.work_exp, &stats.max_work_exp);
                println!("Work Level: {}", &stats.work_level);
                println!("Current Location: {}", Places::convert_to_string(&stats.place));
            }
            _ => println!("Hm, that doesn't seem to work. Type \"help\" for a list of commands."),
        }
    }
}
