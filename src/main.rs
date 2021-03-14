use japanese_read::{read_random_file, report_error, stop, Char};
use std::io;
use std::process::exit;
use wana_kana::to_hiragana::to_hiragana;
use wana_kana::to_katakana::to_katakana;

fn run(file_type: Char) {
    let file = read_random_file(&file_type);
    for line in file.lines() {
        let mut user_input = String::new();
        println!("{}", line);
        // read the user input
        io::stdin()
            .read_line(&mut user_input)
            .expect("Cannot read line");
        user_input.pop(); // delete the new-line character (`\n`)
        if user_input == "exit" {
            // if the user wants to exit, let him do this
            stop!();
        }
        match file_type {
            // translate the user input to what he chose
            Char::Hiragana => user_input = to_hiragana(&user_input),
            Char::Katakana => user_input = to_katakana(&user_input),
        }
        if &user_input != line {
            // if the two periods are not the same, then there is an error (from the user)
            // we need to report this error
            println!();
            report_error(&user_input, line);
        }
        println!();
    }
}

fn main() {
    loop {
        println!("Choose:\n0. Exit\n1. Hiragana\n2. Katakana");
        // Read the choice
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<usize>().unwrap();
        let file_type: Char;
        match choice {
            // compute the choice
            0 => stop!(),
            1 => file_type = Char::Hiragana,
            2 => file_type = Char::Katakana,
            _ => {
                println!("Invalid choice");
                continue;
            }
        }
        // if the choice is valid, pass `file_type` with his ownership (we don't need it anymore)
        run(file_type);
    }
}
