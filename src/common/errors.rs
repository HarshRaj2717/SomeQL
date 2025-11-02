pub(crate) enum Error {
    CompilerParseError,
    CompilerUnrecognizedCommand,
    DataTypeDefinitionError,
    InvalidDateTime,
    UnreachablePath,
    // ...more error types come here
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "{}",
            match self {
                Error::CompilerParseError => "Parsing error during query compilation.",
                Error::CompilerUnrecognizedCommand => "Unrecognized command encountered.",
                Error::DataTypeDefinitionError => "Couldn't recognise/define inputted DataType.",
                Error::InvalidDateTime => "Invalid date/time provided.",
                Error::UnreachablePath => "Unreachable path!! Seeing this error must be impossible.",
                // ...more error types come here
            }
        )
    }
}
