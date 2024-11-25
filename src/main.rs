
use std::io::Read;
use std::io::{self, Write};
use clap::Parser;

mod ciphers;
mod caesar;
mod vigenere;
mod enigma;

use ciphers::*;

fn escape_string(input: &str) -> String {
    let mut key = String::new();

    let mut pos = 0;
    while pos < input.len() {
        if pos == input.len() - 1 {
            key.push_str(&input[pos..pos+1]);
            break;
        }
        match &input[pos..pos+2] {
            "\\\\" => key.push_str("\\"),
            "\\x" => {
                let c = u8::from_str_radix(&input[pos+2..pos+4], 16).unwrap();
                key.push(c as char);
                pos += 2;
            },
            _ => key.push_str(&input[pos..pos+2]),
        };
        pos += 2;
    }
    return key;
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    cipher: String,
    #[arg(short, long)]
    key: String,
    #[arg(short, long)]
    decode: bool,
}

fn main() {
    let args = Args::parse();
    let key_str = escape_string(&args.key);
    if key_str.len() == 0 {
        eprint!("key must be atleast 1 character");
        return;
    }

    let input: Result<Vec<_>,_> = io::stdin().bytes().collect();
    let input: Vec<u8> = input.expect("failed to read input");

    let output: Vec<u8> = match args.cipher.as_str() {
        "caeser" => {
            if key_str.len() != 1 {
                eprint!("caesar key can not be longer than one byte");
                return;
            }
            let key = *key_str.as_bytes().get(0).unwrap();
            let ceaser = caesar::Ceaser::new(key);

            if args.decode {
                ceaser.decode(&input)
            } else {
                ceaser.encode(&input)
            }
        },
        "vigenere" => {
            let key = key_str.bytes().collect();
            let vigenere = vigenere::Vigenere::new(key);

            if args.decode {
                vigenere.decode(&input)
            } else {
                vigenere.encode(&input)
            }
        },  
        "enigma" => {
            let plugboard: [u8; 256] = (0..=255).collect::<Vec<_>>().try_into().expect("wrong size"); // identity plugboard
            let enigma = enigma::Enigma::new(vec![0,1], vec![27,15], &plugboard).expect("failed to create enigma");

            if args.decode {
                enigma.decode(&input)
            } else {
                enigma.encode(&input)
            }
        },  
        _ => panic!("invalid cipher"),
    };
    let _ = std::io::stdout().write_all(&output);
}

