use std::fmt;

#[derive(Debug)]
pub struct RequestError;

impl fmt::Display for RequestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("expected --scenario NAME --ticks U64 --seed U64")
    }
}

impl std::error::Error for RequestError {}
