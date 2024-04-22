use shifter::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprint!("Problem parsing aguments: {}", err);
        process::exit(1);
    });

    let command = &args[3];
    match &command[..] {
        "-d" | "--decipher" => Config::decipher(&config.text, &config.rot),
        "-c" | "--cipher" => Config::cipher(&config.text, &config.rot),
        _ => shifter::help(),
    }
}
