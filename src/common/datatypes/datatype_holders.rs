use super::DataTypeHolders;
use crate::common::Error;
use chrono::DateTime;

impl std::fmt::Display for DataTypeHolders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Integers
                DataTypeHolders::TinyInt { value } => format!("{}", value),
                DataTypeHolders::SmallInt { value } => format!("{}", value),
                DataTypeHolders::Int { value } => format!("{}", value),
                DataTypeHolders::BigInt { value } => format!("{}", value),

                // Unsigned Integers
                DataTypeHolders::UnsignedTinyInt { value } => format!("{}", value),
                DataTypeHolders::UnsignedSmallInt { value } => format!("{}", value),
                DataTypeHolders::UnsignedInt { value } => format!("{}", value),
                DataTypeHolders::UnsignedBigInt { value } => format!("{}", value),

                // Floats
                DataTypeHolders::Float { value } => format!("{}", value),
                DataTypeHolders::Double { value } => format!("{}", value),

                // Texts
                DataTypeHolders::Char { len: _, value } => format!("{}", value),
                DataTypeHolders::VarChar { len: _, value } => format!("{}", value),
                DataTypeHolders::Text { value } => format!("{}", value),

                // Others
                DataTypeHolders::Boolean { value } =>
                    format!("{}", if *value { "True" } else { "False" }),
                DataTypeHolders::Date { value } => {
                    let dt = DateTime::from_timestamp_secs(*value)
                        .unwrap_or_else(|| panic!("{}", Error::InvalidDateTime));
                    format!("{}", dt.date_naive())
                }
                DataTypeHolders::DateTime { value } => {
                    let dt = DateTime::from_timestamp_secs(*value)
                        .unwrap_or_else(|| panic!("{}", Error::InvalidDateTime));
                    format!("{}", dt)
                }
                DataTypeHolders::Blob { value } => format!("{:?}", value),
            }
        )
    }
}

impl DataTypeHolders {}
