/// Common error types used across the project
pub(crate) enum QlError {
    CompilerParseError,
    CompilerUnrecognizedCommand,
    DataConversionError,
    DataTypeDefinitionError,
    InvalidDateTime,
    UnreachablePath,
    // ...more error types come here
}

impl std::fmt::Display for QlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                QlError::CompilerParseError => "Parsing error during query compilation.",
                QlError::CompilerUnrecognizedCommand => "Unrecognized command encountered.",
                QlError::DataConversionError => "Error converting data to specified DataType.",
                QlError::DataTypeDefinitionError => "Couldn't recognise/define inputted DataType.",
                QlError::InvalidDateTime => "Invalid date/time provided.",
                QlError::UnreachablePath =>
                    "Unreachable path!! Seeing this error must be impossible.",
                // ...more error types come here
            }
        )
    }
}
