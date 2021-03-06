use differ::{Differ, Tag};
use rand::seq::SliceRandom;
use rust_embed::RustEmbed;
use termion::color;
use wana_kana::to_romaji::*;

#[macro_export]
macro_rules! stop {
    () => {
        // when called with 0 arguments (`stop!()`)
        {
            // creates this
            println!("Exiting...");
            exit(0);
        }
    };
}

#[derive(RustEmbed)]
#[folder = "japanese_texts"]
struct Asset;

pub enum Char {
    Hiragana,
    Katakana,
}

impl Char {
    fn type_to_str(&self) -> &str {
        match self {
            Char::Hiragana => "hiragana",
            Char::Katakana => "katakana",
        }
    }
}

pub fn read_random_file(file_type: &Char) -> String {
    // Read a random file given the file_type
    let mut entries = Vec::new();
    for file in Asset::iter() {
        if file.contains(file_type.type_to_str()) {
            entries.push(file);
        }
    }
    assert!(!entries.is_empty());
    entries.shuffle(&mut rand::thread_rng());
    // if there aren't any file then the program should crash before we try
    // to access a non-existent address (`entries[0]`)
    String::from_utf8(Asset::get(&entries[0]).unwrap().as_ref().to_vec()).unwrap()
}

pub fn report_error(user_input: &str, expected: &str) {
    // &str -> Vec<char>
    let vec_user: Vec<char> = user_input.chars().collect();
    let vec_expected: Vec<char> = expected.chars().collect();

    let diff = find_differences(&vec_user, &vec_expected);
    let mut index = 0;
    let mut error = String::new();
    let mut right = String::new();

    for (i, character) in vec_user.iter().enumerate() {
        if index < diff.len() && diff[index] == i {
            index += 1;
            // Work on the errors
            error.push_str(&format!(
                "{}{}{}",
                color::Fg(color::Red),
                character,
                color::Fg(color::Reset)
            ));
        } else {
            error.push(*character);
        }
    }

    index = 0;

    for (i, character) in vec_expected.iter().enumerate() {
        if index < diff.len() && diff[index] == i {
            index += 1;
            // The same as above but the right version
            right.push_str(&format!(
                "{}{}{}",
                color::Fg(color::Green),
                character,
                color::Fg(color::Reset)
            ));
        } else {
            right.push(*character);
        }
    }

    println!(
        "{}Error: {}{}",
        color::Fg(color::Red),
        color::Fg(color::Reset),
        to_romaji(&error)
    );
    println!(
        "{}Right: {}{}",
        color::Fg(color::Green),
        color::Fg(color::Reset),
        to_romaji(&right)
    );
}

fn find_differences(vec_user: &[char], vec_expected: &[char]) -> Vec<usize> {
    let mut diff: Vec<usize> = vec![];
    let differ = Differ::new(vec_expected, vec_user);
    for span in differ.spans() {
        if let Tag::Replace = span.tag {
            for i in span.b_start..span.b_end {
                diff.push(i);
            }
        }
    }

    diff
}
