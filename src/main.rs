mod infection;
use infection::{field::*, analyzer::*};

mod bot;
use bot::{Bot, Game};

use std::io::{self, Write};
use std::{thread::sleep, time::Duration};


enum Mode {
    StdOutVsAi,
    HumanVsAi,
}


fn parse_analyzer_type<S: AsRef<str>>(s: S) -> Box<dyn Analyzer> {
    match s.as_ref() {
        "basic" => Box::new(Basic),
        "surrounder" => Box::new(Surrounder),
        &_ => Box::new(Basic),
    }
}


fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let ord = args.last().unwrap();
    let is_player2 = ord == "1";
    if ord != "0" && ord != "1" { return Ok(()) }

    let mut mode = Mode::StdOutVsAi;
    if args.contains(&String::from("--human")) {
        mode = Mode::HumanVsAi;
    }
    
    let mut depth = 3;
    let mut analyzer: Box<(dyn Analyzer + 'static)> = Box::new(Basic);
    for arg in args.windows(2) {
        if arg[0] == String::from("-d") {
            let Ok(new_depth) = str::parse::<u32>(&arg[1]) else { continue };
            depth = new_depth;
        }

        if arg[0] == String::from("-a") {
            analyzer = parse_analyzer_type(&arg[1]);
        }
    }


    let player_id = if is_player2 {2} else {1};
    match mode {
        Mode::StdOutVsAi => {
            stdout_vs_ai(depth, analyzer, player_id)
        },

        Mode::HumanVsAi => {
            human_vs_ai(depth, analyzer, player_id)
        },
    }
}



fn stdout_vs_ai(depth: u32, analyzer: Box<dyn Analyzer>, player_id: u8) -> io::Result<()> {
    let mut field = Field::default();
    println!("{field}");
    
    let bot = Bot::new(depth, analyzer);
    
    loop {
        let is_our_turn = field.current_player_id == player_id;
        let m = if is_our_turn {
            bot.decide(field).unwrap()
        } else {
            read_move_funny(field)?
        };

        let result = field.make_move(m);
        println!("{m}");
        println!("{field}");

        if is_our_turn {
            io::stderr().write_all((m.to_string() + "\n").as_bytes())?;
        }

        if let Some(winner) = result.unwrap() {
            if winner == 0 {
                std::process::exit(4);
            }

            std::process::exit(if winner == player_id {0} else {3});
        }
    }
}


fn read_move_funny(field: Field) -> io::Result<Move> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let s = buffer.trim();
    let Some(m) = Move::from_string(s) else {
        std::process::exit(0); // Rofls
    };
    
    if !field.is_valid_move(m) {
        std::process::exit(0); // Rofls
    }

    Ok(m)
}


fn human_vs_ai(depth: u32, analyzer: Box<dyn Analyzer>, player_id: u8) -> io::Result<()> {
    let mut field = Field::default();
    println!("{field}");
    
    let bot = Bot::new(depth, analyzer);

    let mut history = Vec::new();
    let mut winner = None;
    loop {
        if field.current_player_id != player_id {
            let command_option;
            loop {
                command_option = read_move_human(field, history.len() > 0)?;
                if command_option.is_none() && winner.is_some() { return Ok(()) }
                break;
            }

            match command_option.unwrap() {
                Command::Undo => {
                    field = history.pop().unwrap();
                    println!("Undone.");
                    println!("{field}");
                    continue;
                },

                Command::Move(m) => {
                    history.push(field);
                    winner = field.make_move(m).unwrap();
                    println!("{field}");
                    sleep(Duration::from_secs_f32(1.0));
                    if let Some(w_id) = winner {
                        println!("Player {} wins!", w_id);
                        continue
                    }
                }
            }
        } else {
            let m = bot.decide(field).expect("Valid moves _are_ present, but bot got stuck :(");
            winner = field.make_move(m).expect("Bot is retarded");
            println!("{field}");
            sleep(Duration::from_secs_f32(1.0));
            if let Some(w_id) = winner {
                println!("Player {} wins!", w_id);
                continue
            }
        }
    }
}


enum Command {
    Move(Move),
    Undo,
}


fn read_move_human(field: Field, allow_undo: bool) -> io::Result<Option<Command>> {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let s = buffer.trim();
        if s.trim().len() == 0 { return Ok(None) }
        if allow_undo && s == "u1" { return Ok(Some(Command::Undo)) }
        let Some(m) = Move::from_string(s) else { continue };
        println!("{m}");
        if field.is_valid_move(m) {
            return Ok(Some(Command::Move(m)));
        }
    }
}
