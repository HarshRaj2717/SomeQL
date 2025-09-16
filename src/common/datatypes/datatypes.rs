use super::DataTypes;
use crate::common::Error;
use chrono::DateTime;

impl std::fmt::Display for DataTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataTypes::TinyInt { value } => format!("{}", value),
                DataTypes::SmallInt { value } => format!("{}", value),
                DataTypes::Int { value } => format!("{}", value),
                DataTypes::BigInt { value } => format!("{}", value),
                DataTypes::UnsignedTinyInt { value } => format!("{}", value),
                DataTypes::UnsignedSmallInt { value } => format!("{}", value),
                DataTypes::UnsignedInt { value } => format!("{}", value),
                DataTypes::UnsignedBigInt { value } => format!("{}", value),
                DataTypes::Float { value } => format!("{}", value),
                DataTypes::Double { value } => format!("{}", value),
                DataTypes::Char { len: _, value } => format!("{}", value),
                DataTypes::VarChar { len: _, value } => format!("{}", value),
                DataTypes::Text { value } => format!("{}", value),
                DataTypes::Boolean { value } =>
                    format!("{}", if *value { "True" } else { "False" }),
                DataTypes::Date { value } => {
                    let dt = DateTime::from_timestamp_secs(*value)
                        .unwrap_or_else(|| panic!("{}", Error::InvalidDateTime));
                    format!("{}", dt.date_naive())
                }
                DataTypes::DateTime { value } => {
                    let dt = DateTime::from_timestamp_secs(*value)
                        .unwrap_or_else(|| panic!("{}", Error::InvalidDateTime));
                    format!("{}", dt)
                }
            }
        )
    }
}

impl DataTypes {}
