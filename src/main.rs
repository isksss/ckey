use clap::{Parser, ArgGroup};

#[derive(Debug, Parser)]
#[command(group(ArgGroup::new("").required(true).args(["init", "set", "list"])))]
struct Args {
    #[clap(short = 'i', long = "init", help = "")]
    init: bool,

    #[clap(short = 's', long = "set", help = "")]
    set: bool,

    #[clap(short = 'l', long = "list", help = "")]
    list: bool,
}

fn main() {
    let args = Args::parse();
    let init = args.init;
    if init {
        println!("init");
    } else {
        println!("no init");
    }
}