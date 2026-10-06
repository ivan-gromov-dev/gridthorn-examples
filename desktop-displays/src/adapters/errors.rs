use std::{fmt, io};

#[derive(Debug)]
pub(crate) enum PreferenceError {
    Io(io::Error),
    Invalid,
}

impl fmt::Display for PreferenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "adapter preference file: {error}"),
            Self::Invalid => formatter.write_str("invalid adapter preference file"),
        }
    }
}

impl std::error::Error for PreferenceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Invalid => None,
        }
    }
}

impl From<io::Error> for PreferenceError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
