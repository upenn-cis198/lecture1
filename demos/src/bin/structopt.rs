use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "CIS198 CLI", about = "A simple CLI example in Rust.")]
struct CLI {
    #[structopt(short, long, default_value = "Hello")]
    greeting: String,
    #[structopt(short, long, required = true)]
    names: Vec<String>,
    #[structopt(short, long)]
    debug: bool,
}

fn main() {
    let args = CLI::from_args();
    for name in &args.names {
        println!("{}, {}!", args.greeting, name);
    }
    if args.debug {
        dbg!(args);
    }
}
