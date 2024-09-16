use std::fs;
use rand::seq::SliceRandom;

fn load_cats() -> Vec<String> {
    let paths = fs::read_dir("./cats").unwrap();
    let mut result: Vec<String> = Vec::new();
    
    for path in paths {
        result.push(path.unwrap().path().display().to_string());
    }

    return result;
}

fn pick_cat(options: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let result = options.choose(&mut rng);

    match result {
        None => return "Meow".to_string(),
        Some(path) => return path.to_string(),
    }
}

fn load_cat(path: &String) -> String {
    let contents = fs::read_to_string(path)
        .expect("Unable to read catfile");
    return contents;
}

fn print_cat(cat: &String) {
    println!("{}", cat);
}

pub fn print_cats(literally: bool, count: u16) {
    let cat_paths = load_cats();

    if literally {
        println!("I am LITERALLY this cat:");
    }

    for _ in 0..count {
        let cat_path = pick_cat(&cat_paths);
        let cat_art = load_cat(&cat_path);
        print_cat(&cat_art);
    }
}
