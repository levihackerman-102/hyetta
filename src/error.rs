use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Usage(String),
    Open { path: String, source: std::io::Error },
    Mmap { path: String, source: nix::errno::Errno },
    Io(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Usage(msg) => write!(f, "usage error: {msg}"),
            AppError::Open { path, source } => write!(f, "cannot open '{path}': {source}"),
            AppError::Mmap { path, source } => {
                write!(f, "cannot map '{path}' into memory: {source}")
            }
            AppError::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Open { source, .. } => Some(source),
            AppError::Mmap { source, .. } => Some(source),
            AppError::Io(e) => Some(e),
            AppError::Usage(_) => None,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}
