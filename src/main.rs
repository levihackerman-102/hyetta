mod error;
mod mmap;
mod search;

use std::io::{BufWriter, Write};

use error::AppError;
use mmap::MappedFile;
use search::search;

fn run() -> Result<bool, AppError> {
    let args: Vec<String> = std::env::args().collect();
    let (pattern, path) = match args.as_slice() {
        [_prog, pattern, path] => (pattern, path),
        [prog, ..] => {
            return Err(AppError::Usage(format!(
                "expected exactly 2 arguments, got {}\nusage: {prog} <pattern> <file>",
                args.len() - 1
            )));
        }
        [] => unreachable!("argv always contains the program name"),
    };

    let mapped = MappedFile::open(path)?;
    let bytes = mapped.as_bytes();

    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let locations = search(bytes, pattern.as_bytes(), &mut writer)?;
    writer.flush()?;

    eprintln!("{} match(es) in {path}", locations.len());
    for loc in &locations {
        eprintln!("  line {} @ byte {}", loc.line, loc.byte_offset);
    }

    Ok(!locations.is_empty())
}

fn main() {
    match run() {
        Ok(found) => std::process::exit(if found { 0 } else { 1 }),
        Err(e) => {
            eprintln!("hyetta: {e}");
            std::process::exit(2);
        }
    }
}
