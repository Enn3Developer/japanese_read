use differ::{Differ, Tag};
use rand::seq::SliceRandom;
use std::{fs, io};
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
    let mut entries = fs::read_dir(format!("japanese_texts/{}", file_type.type_to_str()))
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    entries.shuffle(&mut rand::thread_rng());
    // if there aren't any file then the program should crash before we try
    // to access a non-existent address (`entries[0]`)
    fs::read_to_string(&entries[0]).unwrap()
}

pub fn report_error(user_input: &str, expected: &str) {
    // &str -> Vec<char>
    let vec_user: Vec<char> = user_input.chars().collect();
    let vec_expected: Vec<char> = expected.chars().collect();

    let diff = find_differences(&vec_user, &vec_expected);
    let mut index = 0;
    let mut error = String::new();
    let mut right = String::new();

    for i in 0..vec_user.len() {
        if index < diff.len() && diff[index] == i {
            index += 1;
            // Work on the errors
            error.push_str(&format!(
                "{}{}{}",
                color::Fg(color::Red),
                vec_user[i],
                color::Fg(color::Reset)
            ));
        } else {
            error.push(vec_user[i]);
        }
    }

    index = 0;

    for i in 0..vec_expected.len() {
        if index < diff.len() && diff[index] == i {
            index += 1;
            // The same as above but the right version
            right.push_str(&format!(
                "{}{}{}",
                color::Fg(color::Green),
                vec_expected[i],
                color::Fg(color::Reset)
            ));
        } else {
            right.push(vec_expected[i]);
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

fn find_differences(vec_user: &Vec<char>, vec_expected: &Vec<char>) -> Vec<usize> {
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
