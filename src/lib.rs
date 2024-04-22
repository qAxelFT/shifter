const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub struct Config {
    pub text: String,
    pub rot: String,
    pub cipher: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 4 {
            return Err("Not enough arguments, use -h or --help");
        }

        let text = args[1].clone();
        let rot = args[2].clone();
        let cipher = args[3].clone();

        Ok(Config { text, rot, cipher })
    }

    pub fn cipher(text: &String, rot: &String) {
        let mut key: Vec<char> = ALPHABET.chars().collect();
        let mut lower_key: Vec<char> = LOWER_ALPHABET.chars().collect();

        let plain_text: Vec<char> = text.chars().collect();

        let mut ciphered_chars: Vec<char> = vec![];

        key.rotate_left(rot.parse::<usize>().unwrap());
        lower_key.rotate_left(rot.parse::<usize>().unwrap());

        for character in plain_text {
            if character.is_uppercase() {
                let position = index(character, &ALPHABET);
                ciphered_chars.push(key[position.clone().unwrap()]);
            } else if character.is_lowercase() {
                let position = index(character, &LOWER_ALPHABET);
                ciphered_chars.push(lower_key[position.clone().unwrap()]);
            } else {
                ciphered_chars.push(character);
            }
        }

        let encoded_text = ciphered_chars.iter().collect::<String>();
        println!("{}", encoded_text);
    }

    pub fn decipher(text: &String, rot: &String) {
        let unaltered_key: Vec<char> = ALPHABET.chars().collect();
        let unaltered_lower_key: Vec<char> = LOWER_ALPHABET.chars().collect();

        let mut key: Vec<char> = ALPHABET.chars().collect();
        let mut lower_key: Vec<char> = LOWER_ALPHABET.chars().collect();

        let encoded_text: Vec<char> = text.chars().collect();

        let mut decoded_chars: Vec<char> = vec![];

        key.rotate_left(rot.parse::<usize>().unwrap());
        lower_key.rotate_left(rot.parse::<usize>().unwrap());

        let key: String = key.iter().collect();
        let lower_key: String = lower_key.iter().collect();
        for character in encoded_text {
            if character.is_uppercase() {
                let position = index(character, &key);
                decoded_chars.push(unaltered_key[position.clone().unwrap()]);
            } else if character.is_lowercase() {
                let position = index(character, &lower_key);
                decoded_chars.push(unaltered_lower_key[position.clone().unwrap()]);
            } else {
                decoded_chars.push(character);
            }
        }

        let decoded_text = decoded_chars.iter().collect::<String>();
        println!("{}", decoded_text);
    }
}

pub fn index(char: char, key: &str) -> Result<usize, String> {
    let vec: Vec<char> = key.chars().collect();

    for i in 0..=vec.len() {
        if vec[i] == char {
            return Ok(i);
        }
    }
    Err("Character not found".to_string())
}

const HELP: &str = "Usage: shifter [TEXT] [ROTATION] [OPTION]
Shifter is a CLI tool for ciphering and deciphering rotation ciphers
Example: shifter ABC 13 -c

Available OPTIONS:
        -c, --cipher: cipher text using the rotation given.
        -d, --decipher: decipher text using the rotation given.
        -h, --help: help menu.
";

pub fn help() {
    println!("{}", HELP);
}
