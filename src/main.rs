mod error;
mod mmap;

use error::AppError;
use mmap::MappedFile;

fn run() -> Result<(), AppError> {
    let args: Vec<String> = std::env::args().collect();
    let (pattern, path) = match args.as_slice() {
        [_prog, pattern, path] => (pattern, path),
        [prog, ..] => {
            return Err(AppError::Usage(format!(
                "expected exactly 2 arguments, got {}\nusage: {prog} <pattern> <file>",
                args.len() - 1
            )))
        }
        [] => unreachable!("argv always contains the program name"),
    };

    let mapped = MappedFile::open(path)?;
    let bytes = mapped.as_bytes();

    println!(
        "mapped {} bytes from {path} (pattern: {pattern:?})",
        bytes.len()
    );

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("hyetta: {e}");
        std::process::exit(2);
    }
}
