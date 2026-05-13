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

fn unzip(input_name: &String) {}

fn clean(lang: &Option<String>) {}

fn rezip(output_name: &Option<String>, input_name: &String) -> String {
    let default = format!(
        "{}_cleaned.epub",
        input_name.strip_suffix(".epub").unwrap_or(input_name)
    );
    return output_name.as_deref().unwrap_or(&default).to_string();
}

fn main() {
    let args = Args::parse();

    unzip(&args.input);
    clean(&args.language);
    rezip(&args.output, &args.input);
}
