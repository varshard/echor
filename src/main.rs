use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    text: Option<Vec<String>>,
}

fn main() {
    let cli = Cli::parse();

    match cli.text {
        Some(text) => {
            println!("{}", text.join(" "));
        },
        None => println!("no argument")
    }
}
