use clap::Parser;
use regex::Regex;
use std::{
    fs::File,
    io::{Read, Write, read_to_string},
};
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup the inputs from the args
    let args = Args::parse();
    let input_file = args.input;
    let default_output_name = format!(
        "{}_cleaned.epub",
        &input_file.strip_suffix(".epub").unwrap_or(&input_file)
    );
    let output_file = args
        .output
        .as_deref()
        .unwrap_or(&default_output_name)
        .to_string();
    let wordlist_file = format!(
        "wordlists/{}.txt",
        args.language.as_deref().unwrap_or("english")
    );

    // Load the word list for the given langauge
    let wordlist = File::open(&wordlist_file)?;
    let profane_words: Vec<String> = read_to_string(wordlist)?
        .lines()
        .map(|l| l.to_string())
        .collect();

    // Compile the regex
    let raw = profane_words
        .iter()
        .map(|w| format!(r"\b{}\b", w))
        .collect::<Vec<String>>()
        .join("|");
    let re = Regex::new(&raw).unwrap();

    // Unzip the epub
    let book = File::open(&input_file)?;
    let mut archive = ZipArchive::new(book)?;

    // Collect entry names up front to avoid borrow conflicts later
    let names: Vec<String> = (0..archive.len())
        .map(|i| archive.by_index(i).unwrap().name().to_string())
        .collect();

    // Create the output epub
    let out_file = File::create(&output_file)?;
    let mut writer = ZipWriter::new(out_file);
    let options = SimpleFileOptions::default();

    for name in &names {
        let mut entry = archive.by_name(name)?;
        writer.start_file(name, options)?;

        if name.ends_with(".html") || name.ends_with(".xhtml") {
            let mut contents = String::new();
            entry.read_to_string(&mut contents)?;
            let cleaned = re.replace_all(&contents, "****").to_string();
            writer.write_all(cleaned.as_bytes())?;
        } else {
            let mut buf = Vec::new();
            entry.read_to_end(&mut buf)?;
            writer.write_all(&buf)?;
        }
    }

    writer.finish()?;
    println!("Saved cleaned epub to {}", output_file);

    Ok(())
}
