use std::error::Error;
use std::fmt::{self, Display, Formatter};

const DEFAULT_FIXED_STEPS: u32 = 3;

/// Parsed command-line configuration for the headless run.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct HeadlessArguments {
    pub(crate) fixed_steps: u32,
}

impl HeadlessArguments {
    /// Parse `--ticks <count>` or use the example default.
    pub(crate) fn parse(
        arguments: impl IntoIterator<Item = String>,
    ) -> Result<Self, ArgumentError> {
        let mut arguments = arguments.into_iter();
        let Some(flag) = arguments.next() else {
            return Ok(Self {
                fixed_steps: DEFAULT_FIXED_STEPS,
            });
        };
        if flag != "--ticks" {
            return Err(ArgumentError::UnknownArgument(flag));
        }
        let count = arguments.next().ok_or(ArgumentError::MissingTickCount)?;
        if let Some(argument) = arguments.next() {
            return Err(ArgumentError::UnknownArgument(argument));
        }
        let fixed_steps = count
            .parse()
            .map_err(|_| ArgumentError::InvalidTickCount(count))?;
        Ok(Self { fixed_steps })
    }
}

/// Invalid command-line configuration for the headless example.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ArgumentError {
    /// `--ticks` was provided without its value.
    MissingTickCount,
    /// A tick count was not an unsigned 32-bit integer.
    InvalidTickCount(String),
    /// The example received an unsupported argument.
    UnknownArgument(String),
}

impl Display for ArgumentError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingTickCount => formatter.write_str("--ticks requires a fixed-step count"),
            Self::InvalidTickCount(value) => {
                write!(formatter, "invalid --ticks value `{value}`; expected u32")
            }
            Self::UnknownArgument(argument) => {
                write!(
                    formatter,
                    "unknown argument `{argument}`; expected --ticks <count>"
                )
            }
        }
    }
}

impl Error for ArgumentError {}

#[cfg(test)]
mod test;
