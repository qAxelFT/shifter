use std::env;
use std::process;

struct Cipher {
    text: String,
    rotation: String,
    option: String,
}

impl Cipher {
    fn new(args: &[String]) -> Result<Cipher, &str> {
        if args.len() > 4 {
            return Err("Not Enough Arguments");
        }
        let text = args[1].clone();
        let rotation = args[2].clone();
        let option = args[3].clone();

        Ok(Cipher {
            text,
            rotation,
            option,
        })
    }
    fn encode(text: &String, rot: &String) -> String {
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
        encoded_text
    }

    fn decode(text: &String, rot: &String) -> String {
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
        decoded_text
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cipher = Cipher::new(&args).unwrap_or_else(|err| {
        println!("Something went wrong while parsing arguments: {}", err);
        process::exit(1);
    });

    if cipher.option == "-d" {
        let result = Cipher::decode(&cipher.text, &cipher.rotation);
        println!("{}", result);
    } else if cipher.option == "-e" {
        let result = Cipher::encode(&cipher.text, &cipher.rotation);
        println!("{}", result)
    } else {
        println!("Unkown flag. please use -d to decode or -e to encode");
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
