use std::io;

/// Possible errors encountered when parsing user arguments.
#[derive(Debug)]
pub enum CliError<'a> {
    InvalidArg(&'a str),
    InvalidVal(&'a str, &'a str),
    MissingVal(&'a str),
    IOError(io::Error),
}

impl std::error::Error for CliError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            CliError::InvalidArg(_) => None,
            CliError::InvalidVal(_, _) => None,
            CliError::MissingVal(_) => None,
            CliError::IOError(_) => None,
        }
    }
}

impl std::fmt::Display for CliError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CliError::InvalidArg(a) => {
                write!(f, "Invalid argument {} found.", a)
            }
            CliError::MissingVal(a) => {
                write!(f, "Missing value for argument {}", a)
            }
            CliError::InvalidVal(a, v) => {
                write!(f, "Invalid value {} for argument {} found", v, a)
            }
            CliError::IOError(ref err) => err.fmt(f),
        }
    }
}

impl<'a> From<io::Error> for CliError<'a> {
    fn from(err: io::Error) -> CliError<'a> {
        CliError::IOError(err)
    }
}
