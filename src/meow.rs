use include_dir::{include_dir, Dir};
use rand::seq::SliceRandom;

static CAT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/cats");

fn load_cats() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let glob = "*.txt";

    for path in CAT_DIR.find(glob).unwrap() {
        result.push(path.path().display().to_string());
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
    let file = CAT_DIR.get_file(path).unwrap();
    let contents = file.contents_utf8().unwrap().to_string();
    return contents;
}

fn print_cat(cat: &String) {
    println!("{}", cat);
}

fn print_literal(literally: bool){
    if literally {
        println!("I am LITERALLY this cat:");
    }
}

pub fn print_cats(literally: bool, count: u16) {
    let cat_paths = load_cats();

    for _ in 0..count {
        print_literal(literally);
        let cat_path = pick_cat(&cat_paths);
        let cat_art = load_cat(&cat_path);
        print_cat(&cat_art);
    }
}
