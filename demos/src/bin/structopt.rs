use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "CIS198 CLI", about = "A simple CLI example in Rust.")]
struct CLI {
    #[structopt(short, long)]
    name: String,
    #[structopt(short, long, default_value = "Hello")]
    greeting: String,
    #[structopt(short, long)]
    debug: bool,
}

fn main() {
    let args = CLI::from_args();
    println!("{}, {}!", args.greeting, args.name);
    if args.debug {
        dbg!(args);
    }
}
