use std::io;
use std::process::exit;

use wana_kana::to_hiragana::to_hiragana;
use wana_kana::to_katakana::to_katakana;

use japanese_read::{Char, read_random_file_kana, read_random_file_kanji, report_error, stop};

fn run_kana(file_type: Char) {
    let file = read_random_file_kana();
    for line in file.lines() {
        let mut user_input = String::new();
        match file_type {
            Char::Hiragana => println!("{}", line),
            Char::Katakana => println!("{}", to_katakana(line)),
            _ => {}
        }
        // read the user input
        io::stdin()
            .read_line(&mut user_input)
            .expect("Cannot read line");
        user_input.pop(); // delete the new-line character (`\n`)
        if user_input == "exit" {
            // if the user wants to exit, let him do this
            stop!();
        }
        user_input = to_hiragana(&user_input);
        if user_input != line {
            // if the two periods are not the same, then there is an error (from the user)
            // we need to report this error
            println!();
            report_error(&user_input, line);
        }
        println!();
    }
}

fn run_kanji() {
    let files = read_random_file_kanji();
    for line in files.1.lines() {
        let mut user_input = String::new();
        println!("{}", files.0.lines().next().unwrap());
        // read the user input
        io::stdin()
            .read_line(&mut user_input)
            .expect("Cannot read line");
        user_input.pop(); // delete the new-line character (`\n`)
        if user_input == "exit" {
            // if the user wants to exit, let him do this
            stop!();
        }
        user_input = to_hiragana(&user_input);
        if user_input != line {
            // if the two periods are not the same, then there is an error (from the user)
            // we need to report this error
            println!();
            report_error(&user_input, line);
        }
        println!();
    }
}

fn main() {
    // This is a workaround to enable terminal colors in Windows
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();
    loop {
        println!("Choose:\n0. Exit\n1. Hiragana\n2. Katakana\n3. Kanji");
        // Read the choice
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<usize>().unwrap();
        let file_type = match choice {
            // compute the choice
            0 => stop!(),
            1 => Char::Hiragana,
            2 => Char::Katakana,
            3 => Char::Kanji,
            _ => {
                println!("Invalid choice");
                continue;
            }
        };
        // if the choice is valid, pass `file_type` with his ownership (we don't need it anymore)
        match file_type {
            Char::Hiragana => run_kana(file_type),
            Char::Katakana => run_kana(file_type),
            Char::Kanji => run_kanji(),
        }
    }
}
