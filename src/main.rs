use clap::Parser;
use clap_num::number_range;
mod meow;

/// Helper function to validate the command-line numeric argument
fn valid_cat_count(s: &str) -> Result<u64, String> {
    number_range(s, 0, std::u64::MAX)
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Print ASCII cats to your terminal
struct Args {
    /// How many cats to print
    #[arg(short = 'c', long = "count", default_value_t = 1, value_parser = valid_cat_count)]
    count: u64,

    /// Are you literally this cat?
    #[arg(short = 'l', long = "literally", action)]
    literally: bool,
}

/// Prints ASCII cats depending on command-line parameters
fn main() {
    let args: Args = Args::parse();

    meow::print_cats(args.literally, args.count);
}
