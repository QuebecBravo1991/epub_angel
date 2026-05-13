use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// The path to the file to be cleaned.
    #[arg(short, long)]
    input: String,

    /// The name of the cleaned e-book.
    #[arg(short, long)]
    output: Option<String>,

    /// The language of the input e-book.
    #[arg(short, long)]
    language: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args)
}
