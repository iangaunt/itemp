use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    t: String,

    #[arg(short, long)]
    l: String,
}

fn main() {
    let args = Args::parse();

    let clone = String::from("git clone https://github.com/iangaunt/") + &args.t + "-template.git ./" + &args.l;
    let output = Command::new("cmd")
        .args(["/C", &clone]).output().expect("gg");

    println!("Repository cloned successfully.");
    assert!(output.status.success());
}