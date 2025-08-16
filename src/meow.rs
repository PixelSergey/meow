//! Cat processing functions
//! 
//! This module provides functions to:
//! - Load a list of catpaths from the `cats` directory
//! - Pick a random path out of the list
//! - Load the data from a certain catfile
//! - Print the catfile data and the literal string

use include_dir::{include_dir, Dir};
use rand::seq::IndexedRandom;

static CAT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/cats");

/// Load a vector of cat paths from the `CAT_DIR` folder
fn load_cat_paths() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let glob: &'static str = "*.txt";

    for path in CAT_DIR.find(glob).unwrap() {
        result.push(path.path().display().to_string());
    }

    result
}

/// Pick a cat path from a vector of cat paths
/// 
/// # Arguments
/// 
/// * `options` - A vector containing string paths to catfiles
/// 
/// # Returns
/// 
/// One of the strings in `options`, randomly picked
fn pick_cat(options: &Vec<String>) -> String {
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let result: Option<&String> = options.choose(&mut rng);

    match result {
        None => "Meow".to_string(),
        Some(path) => path.to_string(),
    }
}

/// Load data from a catfile
/// 
/// # Arguments
/// 
/// * `path` - A string path to a catfile
/// 
/// # Returns
/// 
/// A string containing the contents of the catfile
fn load_cat(path: &String) -> String {
    let file: &include_dir::File<'_> = CAT_DIR.get_file(path).unwrap();
    let contents: String = file.contents_utf8().unwrap().to_string();
    contents
}

/// Print cat data to the screen
/// 
/// # Arguments
/// 
/// * `cat` - Cat data
/// * `newline` - Whether to print an extra newline after the cat data
fn print_cat(cat: &String, newline: bool) {
    println!("{cat}");
    if newline { println!(); }
}

/// Print the literal string
/// 
/// # Arguments
/// 
/// * `literally` - A boolean, whether to print the literal string or not
fn print_literal(literally: bool){
    if literally {
        println!("I am LITERALLY this cat:\n");
    }
}

/// Print a certain number of randomly picked cats with a literal string
/// 
/// # Arguments
/// 
/// * `literally` - A boolean, whether to print the literal string or not
/// * `count` - How many cats to print
pub fn print_cats(literally: bool, count: u16) {
    let cat_paths: Vec<String> = load_cat_paths();

    for i in 0..count {
        print_literal(literally);
        let cat_path: String = pick_cat(&cat_paths);
        let cat_art: String = load_cat(&cat_path);
        print_cat(&cat_art, i<count-1);
    }
}
