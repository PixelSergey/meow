use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Print ASCII cats to your terminal
struct Args {
    /// How many cats to print
    #[arg(short = 'c', long = "count", default_value_t = 1)]
    count: u8,

    /// Are you literally this cat?
    #[arg(short = 'l', long = "literally", action)]
    literally: bool
}

fn main() {
    let args = Args::parse();

    println!("Hello, world!");
}
