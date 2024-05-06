use clap::{value_parser, Arg, ArgMatches, Command};
use indoc::indoc;

struct Config {
    number: u32,
    hardmode_flag: bool,
    rule_flag: bool,
}

impl Config {
    fn new(number: u32, hardmode_flag: bool, rule_flag: bool) -> Config {
        Config {
            number,
            hardmode_flag,
            rule_flag,
        }
    }
}

fn rules_message(maybe_num: Option<u32>) { 
    let welcome_msg = "Welcome to the Hot-Cold Guess game, copyright 2024.";
    let separator1 = "=".repeat(welcome_msg.len());
    let separator2 = "-".repeat(welcome_msg.len());
    let header = format!("{}\n{}\n{}\n", separator1, welcome_msg, separator2);
    let body = format!(
        indoc! {"
    These are the game rules:
        * I’ll choose a random number in [1,{}]. Your job is to guess that number.
        * From the second guess onward, I’ll tell you if your guess
        is hot (closer than the previous guess) or cold (farther from
        the previous guess).
        * If you wish to quit the game, just type in a negative number.
    Good luck!
    "}, maybe_num.unwrap_or(42));
    println!("{}{}{}", header, body, separator2);
}

fn setup_cli() -> ArgMatches {
    return Command::new("Hot or cold game")
        .author("Vinicius de Lima, (c) 2024")
        .version("0.1")
        .about("Simple game to guess a number between a interval")
        .arg_required_else_help(true)
        .long_about(
            "Given a number you'll have to guess it. The game will tell you if you're hot or cold.",
        )
        .arg(
            Arg::new("number")
                .help("I’ll choose a random number between [1,number]. Your job is to guess that number.")
                .value_parser(value_parser!(u32))
                .index(1),
        )
        .arg(
            Arg::new("hard")
                .long("hard")
                .action(clap::ArgAction::SetTrue)
                .help("Set the game to hard mode"),
        )
        .arg(
            Arg::new("rules")
                .short('r')
                .long("rules")
                .action(clap::ArgAction::SetTrue)
                .help("Show the game rules"),
        )
        .try_get_matches()
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });
}

fn parse_args() -> Option<Config> {
    let matches = setup_cli();

    let rule_flag = matches.get_flag("rules"); 

    if let Some(number) = matches.get_one::<u32>("number") {
        let hard = matches.get_flag("hard");
        if rule_flag {
            rules_message(Some(*number));
        } 
        return Some(Config::new(*number, hard, rule_flag));
    }
    if rule_flag {
        rules_message(None);
    }
    None
}

fn main() {
    if let Some(config) = parse_args() {
        println!(
            "Number: {}, Hard-Mode: {}, Rules: {}",
            config.number, config.hardmode_flag, config.rule_flag
        );
    }
}
