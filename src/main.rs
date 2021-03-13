use japanese_read::{read_random_file, report_error, Char};
use std::io;
use std::process::exit;
use wana_kana::to_hiragana::to_hiragana;
use wana_kana::to_katakana::to_katakana;

fn main() {
    println!("Choose:\n1. Hiragana\n2. Katakana");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim().parse::<usize>().unwrap();
    let file_type: Char;
    match choice {
        1 => file_type = Char::Hiragana,
        2 => file_type = Char::Katakana,
        _ => exit(1),
    }
    let file = read_random_file(&file_type);
    for line in file.lines() {
        let mut user_input = String::new();
        println!("{}", line);
        io::stdin()
            .read_line(&mut user_input)
            .expect("Cannot read line");
        user_input.pop();
        match file_type {
            Char::Hiragana => user_input = to_hiragana(&user_input),
            Char::Katakana => user_input = to_katakana(&user_input),
        }
        if &user_input != line {
            report_error(&user_input, line);
        }
    }
}
