use differ::{Differ, Tag};
use rand::seq::SliceRandom;
use std::{fs, io};
use termion::color;
use wana_kana::to_romaji::*;

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
    let mut entries = fs::read_dir(format!("japanese_texts/{}", file_type.type_to_str()))
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    entries.shuffle(&mut rand::thread_rng());
    fs::read_to_string(&entries[0]).unwrap()
}

pub fn report_error(user_input: &str, expected: &str) {
    let vec_user: Vec<char> = user_input.chars().collect();
    let vec_expected: Vec<char> = expected.chars().collect();

    let diff = find_differences(&vec_user, &vec_expected);
    let mut index = 0;
    let mut error = String::new();
    let mut right = String::new();

    for i in 0..vec_user.len() {
        if index < diff.len() && diff[index] == i {
            index += 1;
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

    println!("Error: {}", to_romaji(&error));
    println!("Right: {}", to_romaji(&right));
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
