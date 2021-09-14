mod dictionaries;

use clap::{App, Arg};
use colour::green_ln;
use colour::red_ln;
use dictionaries::{ENGLISH, FRENCH};
use regex::Regex;
use std::fs;
use std::process::exit;
use std::str::SplitWhitespace;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;

fn find_in_dictionary(dictionary: Vec<&str>, word: &str) -> bool {
    let mut result: bool = false;
    if dictionary.iter().any(|&w| w == word) {
        result = true;
    }

    return result;
}

/**
 * Splits the seed string into an iterable of seperated words
 */
fn get_words(content: &str) -> SplitWhitespace<'_> {
    let splitted_content = content.split_whitespace();
    let s_re = Regex::new(r"\s").unwrap();

    println!("words to be searched : {}", s_re.replace_all(content, ", "));

    return splitted_content;
}

fn process_paths(paths: &str) -> Vec<&str> {
    return paths.split(',').collect();
}

fn check_words_number(content: &str) -> Result<bool,usize> {
    let splitted_content = content.split_whitespace();
    // let re = Regex::new(r"[a-zA-Z]+$").unwrap();

    if splitted_content.clone().count() != 24 {
        return Err(splitted_content.clone().count());
    }
       return Ok(true);
}

fn check_from_builtin_dictionaries(words: SplitWhitespace) -> bool {
    let mut all_found = true;

    '_outer: for word in words.into_iter() {
        '_inner: for dictionary in [ENGLISH, FRENCH].to_vec() {
            let mut is_found = false;
            match self::find_in_dictionary(dictionary.to_vec(), word) {
                true => {
                    is_found = true;
                }
                false => {}
            }
            if !is_found {
                all_found = false;
                break '_outer;
            }
        }
    }

    return all_found;
}

fn load_dictionary(path: &str) -> String {
    let content = fs::read_to_string(path).expect("Something went wrong reading the file");

    return content;
}

fn check_from_external_dictionaries(paths: Vec<&str>, words: SplitWhitespace) -> bool {
    let mut dictionaries: Vec<String> = Vec::new();
    for path in paths {
        let dictionary = self::load_dictionary(path).clone();
        dictionaries.push(dictionary);
    }

    let mut all_found = true;

    '_outer: for word in words.into_iter() {
        '_inner: for dictionary in dictionaries.iter() {
            let mut is_found = false;
            let regex_string = format!(r"{}", &word);
            let regex = Regex::new(regex_string.as_str()).unwrap();

            match regex.is_match(&dictionary.as_str()) {
                true => {
                    is_found = true;
                }
                false => {}
            }

            if !is_found {
                all_found = false;
                break '_outer;
            }
        }
    }

    return all_found;
}

fn main() {
    let matches = App::new("smwc")
        .version("1.0")
        .author("Nelson Herbin <nelson@herbin.info>")
        .about("Minimalist seed words checker")
        .arg(
            Arg::with_name("skipcount")
                .short("s")
                .long("skip-count")
                .value_name("sc")
                .help("Skips count of words number")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("dictionaries")
                .short("d")
                .long("dictionaries")
                .value_name("d")
                .help("Dictionaries files to use (separated with coma)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("seed")
                .index(1)
                .help("The 24 words for seed generation"),
        )
        .get_matches();

    let seed: &str;
    match matches.value_of("seed") {
        Some(s) => {
            seed = s;
        }
        None => {
            red_ln!("You must provide a seed. None found.");
            exit(1);
        }
    }

    // Skips the count checker of the provided seed
    if let None = matches.value_of("skip-count") {
        

        match self::check_words_number(seed){
            Ok(_) => {},
            Err(count) => {
                red_ln!(
                    "Provided mnemonic seed is invalid. 24 words expected, {} found",
                    count
                );
                exit(1)
            }   
        }
    }

    let words = self::get_words(seed);

    let result: bool;

    // Handles wether or not we have to use built-in dictionaries
    match matches.value_of("dictionaries") {
        Some(paths_value) => {
            let paths: Vec<&str> = self::process_paths(paths_value);
            result = self::check_from_external_dictionaries(paths, words);
        }
        None => {
            result = self::check_from_builtin_dictionaries(words);
        }
    }

    match result {
        true => {
            green_ln!("Provided words were all found in dictionary");
            exit(0);
        }
        false => {
            red_ln!("One or many words were not found in dictionary");
            exit(1);
        }
    }
}



#[test]
fn test_get_words(){
    let test_seed: &str = "abandon toto tata";
    let results: Vec<&str> = get_words(test_seed).collect();
    assert_eq!("abandon",results[0]);
    assert_eq!("toto",results[1]);
    assert_ne!("abandon",results[1]);
    assert_eq!("tata",results[2]);
}

#[test]
fn test_words_number(){
    let test_seed: &str = "erupt quit sphere taxi air decade vote mixed life elevator mammal search empower rabbit barely indoor crush grid slide correct scatter deal tenant verb";
    let test_seed_failure: &str = "erupt quit sphere taxi air decade vote mixed life elevator mammal search empower rabbit barely indoor crush grid";
    let result= check_words_number(test_seed);
    assert!(result.is_ok());
    assert_eq!(true,result.unwrap());
    let result = check_words_number(test_seed_failure);
    assert!(result.is_err());
    
}