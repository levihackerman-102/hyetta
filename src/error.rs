use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Usage(String),
    Io(std::io::Error),
    Mmap(nix::errno::Errno),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Usage(msg) => write!(f, "usage error: {msg}"),
            AppError::Io(e) => write!(f, "I/O error: {e}"),
            AppError::Mmap(e) => write!(f, "mmap failed: {e}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::Mmap(e) => Some(e),
            AppError::Usage(_) => None,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<nix::errno::Errno> for AppError {
    fn from(e: nix::errno::Errno) -> Self {
        AppError::Mmap(e)
    }
}
