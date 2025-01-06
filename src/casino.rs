use std::{io, thread, time::{self, Duration}};
use rand::{self, Rng};

pub fn casino(money: i32) -> i32 {
    let mut new_money: i32 = money;
    
    println!("Welcome to the casino!");

    println!("\nUse \"play [game] [bet]\" to play a game. Leave [game] empty for a list of games.");
    println!("Type \"leave\" to leave.\n");

    loop {
        println!("Please insert a command below:");
    
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Didn't get that, what?");

        let mut command= input.split_whitespace();

        match command.next() {
            Some("play") => {
                new_money = play(command.next(), command.next(), new_money);
            },
            Some("leave") => break,
            None | Some(&_) => {
                println!("Uh, what?\n");
            }
        }
    }

    new_money
}

fn play(game: Option<&str>, bet: Option<&str>, money: i32) -> i32 {
    let mut local_bet: i32 = 0;

    match bet {
        Some(_) => {
            if bet.unwrap().to_string().parse::<i32>().is_err() {
                println!("Bet was not a number.");
                return -2
            } else {
                local_bet = bet.unwrap().to_string().parse::<i32>().expect("the what");
            }
        },
        None => local_bet = 0
    }
    
    match game {
        Some("wheel") => return play_wheel(money, local_bet),
        Some("slots") => return play_slots(money, local_bet),
        Some("blackjack") => return play_blackjack(money, local_bet),
        Some("coinflip") => return play_coinflip(money, local_bet),
        None | Some(_) => {
            println!("List of games:");
            println!("coinflip - min bet 5, max bet 100");
            println!("blackjack - min bet 20, max bet 500");
            println!("slots - min bet 50, max bet 5000");
            println!("wheel - min bet 200, no max bet");
            0
        }
    }
}

pub fn play_wheel(money: i32, bet: i32) -> i32 {
    0
}

pub fn play_slots(money: i32, bet: i32) -> i32 {
    0
}

pub fn play_blackjack(money: i32, bet: i32) -> i32 {
    0
}

pub fn play_coinflip(money: i32, bet: i32) -> i32 {
    if bet < 5 {
        println!("C'mon man, do more than that. At least $5.\n");
        return 0;
    }

    let mut input: String = String::new();
    let mut choice: bool;

    println!("Alright, ${bet} is on the line. Heads or tails?\n");

    io::stdin()
            .read_line(&mut input)
            .expect("Didn't get that, what?");

    match input.to_lowercase().trim() {
        "heads" => choice = true,
        "tails" => choice = false,
        _ => {
            println!("Yeah, no, that's not heads or tails.\n");
            return 0;
        }
    }

    println!("\nAlright, you chose {}", input);

    let flip: bool = rand::thread_rng().gen_range(0..=1) == 0;

    thread::sleep(Duration::from_millis(2000));

    if flip == choice {
        println!("\nYou won! You earned {bet}.");
        return money + bet;
    } else {
        println!("\nBad luck! You lost {bet}.");
        return money - bet;
    }
}