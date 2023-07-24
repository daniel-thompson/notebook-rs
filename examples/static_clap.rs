use clap::Parser;
use static_init::dynamic;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

// If you prefer lazy_static ARGS could have been spelled:
//   lazy_static! {
//       pub static ref ARGS: Args = Args::parse();
//   }
#[dynamic]
pub static ARGS: Args = Args::parse();

fn main() {
    println!("{}", ARGS.verbose);
}
