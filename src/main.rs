use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    /// The template repository to be used (do not include the "-template" extension).
    #[arg(short, long)]
    template: String,

    /// The location relative to this folder to place the cloned repository.
    #[arg(short, long)]
    location: String,
}

fn main() {
    let args = Args::parse();

    let clone = String::from("git clone https://github.com/iangaunt/") + &args.template + "-template.git ./" + &args.location;
    let _output = Command::new("cmd")
        .args(["/C", &clone]).output().expect("gg");

    println!("Repository cloned successfully.");
    // assert!(output.status.success());
}