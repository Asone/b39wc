mod dictionaries;

use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;

use clap::{App, Arg};
use colour::green_ln;
use colour::red_ln;
use dictionaries::langs::{
    CZECH, 
    FRENCH, 
    ITALIAN, 
    ENGLISH, 
    JAPANESE, 
    KOREAN, 
    PORTUGUESE, 
    SPANISH, 
    CHINESE_SIMPLIFIED, 
    CHINESE_TRADITIONAL
};
use regex::Regex;
use std::fs;
use std::process::exit;
use std::str::SplitWhitespace;

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

#[test]
fn test_get_words(){
    let test_seed: &str = "abandon toto tata";
    let results: Vec<&str> = get_words(test_seed).collect();
    assert_eq!("abandon",results[0]);
    assert_eq!("toto",results[1]);
    assert_ne!("abandon",results[1]);
    assert_eq!("tata",results[2]);
}

fn process_paths(paths: &str) -> Vec<&str> {
    return paths.split(',').collect();
}

#[test]
fn test_process_paths(){
    let single_path: &str = "./my/path.txt";
    let multiple_paths: &str = "./my/path1.txt,./my/path2.txt";
    assert_eq!(1,process_paths(single_path).iter().count());
    assert_eq!(2,process_paths(multiple_paths).iter().count());
}

fn check_words_number(content: &str) -> Result<bool,usize> {
    let splitted_content = content.split_whitespace();
    // let re = Regex::new(r"[a-zA-Z]+$").unwrap();

    if splitted_content.clone().count() != 24 {
        return Err(splitted_content.clone().count());
    }
       return Ok(true);
}

#[test]
fn test_check_words_number(){
    let test_seed: &str = "erupt quit sphere taxi air decade vote mixed life elevator mammal search empower rabbit barely indoor crush grid slide correct scatter deal tenant verb";
    let test_seed_failure: &str = "erupt quit sphere taxi air decade vote mixed life elevator mammal search empower rabbit barely indoor crush grid";
    let result= check_words_number(test_seed);
    assert!(result.is_ok());
    assert_eq!(true,result.unwrap());
    let result = check_words_number(test_seed_failure);
    assert!(result.is_err());
    
}

fn check_from_builtin_dictionaries(words: SplitWhitespace) -> (bool,Vec<String>) {
    let mut missing: Vec<String> = Vec::new();
    let mut all_found = true;

    '_outer: for word in words.into_iter() {
        let mut is_found = false;
        '_inner: for dictionary in [ENGLISH, FRENCH, CZECH, ITALIAN, SPANISH, PORTUGUESE, JAPANESE, KOREAN, CHINESE_TRADITIONAL, CHINESE_SIMPLIFIED].to_vec() {
            match self::find_in_dictionary(dictionary.to_vec(), word) {
                true => {
                    is_found = true;
                }
                false => {}
            }

        }

        if !is_found {
            all_found = false;
            missing.push(word.clone().to_string());
        }
    }

    return (all_found, missing);
}

fn load_dictionary(path: &str) -> Result<String,&str> {
    match fs::read_to_string(path){
        Ok(data) => Ok(data),
        Err(_) => Err(path)
    }
}

#[test]
fn test_load_dictionary(){

    let invalid_path: &str = "resources/test/invalid_dictionary.txt";
    assert!(load_dictionary(invalid_path).is_err());

    
    let valid_path: &str = "resources/test/test_dictionary.txt";
    let result: Result<String,&str> = load_dictionary(valid_path);
    assert!(result.is_ok());
    assert!(result.unwrap().as_str().contains("loterie"));
    // assert_eq!("hello world",result.unwrap().as_str());
    
}

fn check_from_external_dictionaries(paths: Vec<&str>, words: SplitWhitespace) -> (bool,Vec<String>) {
    let mut missing: Vec<String> = Vec::new();
    let mut dictionaries: Vec<String> = Vec::new();
    for path in paths {
        match self::load_dictionary(path){
            Ok(dictionary) => {
                dictionaries.push(dictionary);
            },
            Err(failed_path) => {
                red_ln!("Error loading dictionary with path : {}",failed_path);
                exit(1);
            }
        }
    }

    let mut all_found = true;

    '_outer: for word in words.into_iter() {
        let mut is_found = false;
        '_inner: for dictionary in dictionaries.iter() {
            let regex_string = format!(r"{}", &word);
            let regex = Regex::new(regex_string.as_str()).unwrap();

            match regex.is_match(&dictionary.as_str()) {
                true => {
                    is_found = true;
                }
                false => {}
            }
        }

        if !is_found {
            missing.push(word.clone().to_string());
            all_found = false;
        }
    }

    return (all_found, missing);
}

fn main() {
    let matches = App::new("b39wc")
        .version("1.0")
        .author("Nelson Herbin <nelson@herbin.info>")
        .about("Minimalist seed words checker")
        .arg(
            Arg::with_name("skipcount")
                .short("s")
                .long("skip-count")
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
    
    let skip_count: bool = matches.is_present("skipcount");

    match skip_count{
        true => {},
        false => {        

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
    }

    let words = self::get_words(seed);

    let result: (bool, Vec<String>);

    // Handles wether or not we have to use built-in dictionaries
    match matches.value_of("dictionaries") {
        Some(paths_value) => {
            let paths: Vec<&str> = self::process_paths(paths_value.trim_start());
            result = self::check_from_external_dictionaries(paths, words);
        }
        None => {
            result = self::check_from_builtin_dictionaries(words);
        }
    }

    match result {
        (true, _) => {
            green_ln!("Provided words were all found in dictionary");
            exit(0);
        }
        (false, missing) => {
            let missing_str: String = missing.join(",");
            red_ln!("One or many words were not found in dictionary: {}",missing_str);
            exit(1);
        }
    }
}

#[test]
fn test_successful_program_without_options() -> Result<(), Box<dyn std::error::Error>> {
    let test_seed: &str = "erupt quit sphere taxi air decade vote mixed life elevator mammal search empower rabbit barely indoor crush grid slide correct scatter deal tenant verb";
    let mut cmd = Command::cargo_bin("b39wc")?;
    cmd.arg(test_seed);
    cmd.assert().success();

    Ok(())
}

#[test]
fn test_unsuccessful_program_without_options() -> Result<(), Box<dyn std::error::Error>> {
    let test_seed: &str = "cswisafraud quit sphere taxi air decade vote mixed life elevator mammal search empower rabbit barely indoor crush grid slide correct scatter deal tenant verb";
    let mut cmd = Command::cargo_bin("b39wc")?;
    cmd.arg(test_seed);
    cmd.assert().failure();

    Ok(())
}

#[test]
fn test_unsuccessful_program_with_word_count() -> Result<(), Box<dyn std::error::Error>> {
    let test_seed: &str = "cswisafraud quit";
    let mut cmd = Command::cargo_bin("b39wc")?;
    cmd.arg(test_seed);
    cmd.assert().failure();

    Ok(())
}

#[test]
fn test_successful_program_with_word_count_skip() -> Result<(), Box<dyn std::error::Error>> {
    let test_seed: &str = "erupt quit";
    let mut cmd = Command::cargo_bin("b39wc")?;
    cmd.arg("--skip-count");
    cmd.arg(test_seed);
    cmd.assert().success();

    Ok(())
}

#[test]
fn test_successful_program_with_external_dictionary() -> Result<(), Box<dyn std::error::Error>> {
     let test_seed: &str = "loterie batterie érosion immobile marqueur sembler malice farceur défensif caresser avenir trivial ouvrage ozone union palmarès impact facette diluer faiblir radieux spacieux naufrage lampe";
     let mut cmd = Command::cargo_bin("b39wc")?;

    let dictionary_argument: String = format!("-d {}","resources/test/test_dictionary.txt");
    cmd.arg(dictionary_argument.as_str());
    cmd.arg(test_seed);
    cmd.assert().success();

    Ok(())
}

#[test]
fn test_unsuccessful_program_with_external_dictionary_seed_failure() -> Result<(), Box<dyn std::error::Error>> {
     let test_seed: &str = "bitpoint batterie érosion immobile marqueur sembler malice farceur défensif caresser avenir trivial ouvrage ozone union palmarès impact facette diluer faiblir radieux spacieux naufrage lampe";
     let mut cmd = Command::cargo_bin("b39wc")?;

    let dictionary_argument: String = format!("-d {}","resources/test/test_dictionary.txt");
    cmd.arg(dictionary_argument.as_str());
    cmd.arg(test_seed);
    cmd.assert().failure();

    Ok(())
}

#[test]
fn test_unsuccessful_program_with_external_dictionary_load_failure() -> Result<(), Box<dyn std::error::Error>> {
     let test_seed: &str = "loterie batterie érosion immobile marqueur sembler malice farceur défensif caresser avenir trivial ouvrage ozone union palmarès impact facette diluer faiblir radieux spacieux naufrage lampe";
     let mut cmd = Command::cargo_bin("b39wc")?;

    let dictionary_argument: String = format!("-d {}","resources/test/invalid_dictionary.txt");
    cmd.arg(dictionary_argument.as_str());
    cmd.arg(test_seed);
    cmd.assert().failure();

    Ok(())
}

#[test]
fn test_unsuccessful_program_with_external_dictionary_count_failure() -> Result<(), Box<dyn std::error::Error>> {
     let test_seed: &str = "loterie batterie érosion";
     let mut cmd = Command::cargo_bin("b39wc")?;

    let dictionary_argument: String = format!("-d {}","resources/test/test_dictionary.txt");
    cmd.arg(dictionary_argument.as_str());
    cmd.arg(test_seed);
    cmd.assert().failure();

    Ok(())
}

#[test]
fn test_successful_program_with_external_dictionary_and_skip_count() -> Result<(), Box<dyn std::error::Error>> {
     let test_seed: &str = "loterie batterie érosion";
     let mut cmd = Command::cargo_bin("b39wc")?;

    let dictionary_argument: String = format!("-d {}","resources/test/test_dictionary.txt");
    cmd.arg(dictionary_argument.as_str());
    cmd.arg("--skip-count");
    cmd.arg(test_seed);
    cmd.assert().success();

    Ok(())
}



