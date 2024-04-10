use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(long, short)]
    text: String,
    #[arg(long, short)]
    rotation: String,
    #[arg(long, short)]
    decipher: bool,
}

fn cipher(text: &String, rot: &String) {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let lower_alphabet = alphabet.to_lowercase();

    let unaltered_key: Vec<char> = alphabet.chars().collect();
    let unaltered_lower_key: Vec<char> = lower_alphabet.chars().collect();

    let mut key: Vec<char> = alphabet.chars().collect();
    let mut lower_key: Vec<char> = lower_alphabet.chars().collect();

    let plain_text: Vec<char> = text.chars().collect();

    let mut encoded_chars: Vec<char> = vec![];

    key.rotate_left(rot.parse::<usize>().unwrap());
    lower_key.rotate_left(rot.parse::<usize>().unwrap());

    for character in plain_text {
        if character.is_uppercase() {
            let position = index(character, unaltered_key.clone());
            encoded_chars.push(key[position.clone().unwrap()]);
        } else if character.is_lowercase() {
            let position = index(character, unaltered_lower_key.clone());
            encoded_chars.push(lower_key[position.clone().unwrap()]);
        } else {
            encoded_chars.push(character);
        }
    }

    let encoded_text = encoded_chars.iter().collect::<String>();
    println!("{}", encoded_text);
}

fn decipher(text: &String, rot: &String) {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let lower_alphabet = alphabet.to_lowercase();

    let unaltered_key: Vec<char> = alphabet.chars().collect();
    let unaltered_lower_key: Vec<char> = lower_alphabet.chars().collect();

    let mut key: Vec<char> = alphabet.chars().collect();
    let mut lower_key: Vec<char> = lower_alphabet.chars().collect();

    let encoded_text: Vec<char> = text.chars().collect();

    let mut decoded_chars: Vec<char> = vec![];

    key.rotate_left(rot.parse::<usize>().unwrap());
    lower_key.rotate_left(rot.parse::<usize>().unwrap());

    for character in encoded_text {
        if character.is_uppercase() {
            let position = index(character, key.clone());
            decoded_chars.push(unaltered_key[position.clone().unwrap()]);
        } else if character.is_lowercase() {
            let position = index(character, lower_key.clone());
            decoded_chars.push(unaltered_lower_key[position.clone().unwrap()]);
        } else {
            decoded_chars.push(character);
        }
    }

    let decoded_text = decoded_chars.iter().collect::<String>();
    println!("{}", decoded_text);
}

fn main() {
    let args = Args::parse();

    if args.decipher {
        decipher(&args.text, &args.rotation);
    } else {
        cipher(&args.text, &args.rotation);
    }
}

fn index(char: char, key: Vec<char>) -> Result<usize, String> {
    for i in 0..=key.len() {
        if key[i] == char {
            return Ok(i);
        }
    }
    Err("Character not found".to_string())
}
