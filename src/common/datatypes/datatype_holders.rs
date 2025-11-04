use super::DataTypeDefiner;
use super::DataTypeHolder;
use crate::common::QlError;
use chrono::DateTime;

impl std::fmt::Display for DataTypeHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Integers
                DataTypeHolder::TinyInt { value } => format!("{}", value),
                DataTypeHolder::SmallInt { value } => format!("{}", value),
                DataTypeHolder::Int { value } => format!("{}", value),
                DataTypeHolder::BigInt { value } => format!("{}", value),

                // Unsigned Integers
                DataTypeHolder::UnsignedTinyInt { value } => format!("{}", value),
                DataTypeHolder::UnsignedSmallInt { value } => format!("{}", value),
                DataTypeHolder::UnsignedInt { value } => format!("{}", value),
                DataTypeHolder::UnsignedBigInt { value } => format!("{}", value),

                // Floats
                DataTypeHolder::Float { value } => format!("{}", value),
                DataTypeHolder::Double { value } => format!("{}", value),

                // Texts
                DataTypeHolder::Char { value } => format!("{}", value),
                DataTypeHolder::VarChar { value } => format!("{}", value),
                DataTypeHolder::Text { value } => format!("{}", value),

                // Others
                DataTypeHolder::Boolean { value } =>
                    format!("{}", if *value { "True" } else { "False" }),
                DataTypeHolder::Date { value } => {
                    let dt = DateTime::from_timestamp_secs(*value)
                        .unwrap_or_else(|| panic!("{}", QlError::InvalidDateTime));
                    format!("{}", dt.date_naive())
                }
                DataTypeHolder::DateTime { value } => {
                    let dt = DateTime::from_timestamp_secs(*value)
                        .unwrap_or_else(|| panic!("{}", QlError::InvalidDateTime));
                    format!("{}", dt)
                }
                DataTypeHolder::Blob { value } => format!("{:?}", value),
            }
        )
    }
}

impl DataTypeHolder {
    pub(crate) fn new_from_string(
        data: &str,
        type_definition: &DataTypeDefiner,
    ) -> Result<DataTypeHolder, QlError> {
        let error: Result<_, QlError> = Err(QlError::DataConversionError);
        match type_definition {
            // Integers
            DataTypeDefiner::TinyInt => {
                let value = data.parse::<i8>();
                if let Ok(v) = value {
                    Ok(DataTypeHolder::TinyInt { value: v })
                } else {
                    error
                }
            }
            DataTypeDefiner::SmallInt => {
                let value = data.parse::<i16>();
                if let Ok(v) = value {
                    Ok(DataTypeHolder::SmallInt { value: v })
                } else {
                    error
                }
            }
            DataTypeDefiner::Int => {
                let value = data.parse::<i32>();
                if let Ok(v) = value {
                    Ok(DataTypeHolder::Int { value: v })
                } else {
                    error
                }
            }
            DataTypeDefiner::BigInt => {
                let value = data.parse::<i64>();
                if let Ok(v) = value {
                    Ok(DataTypeHolder::BigInt { value: v })
                } else {
                    error
                }
            }

            // Unsigned Integers
            DataTypeDefiner::UnsignedTinyInt => {
                let value = data.parse::<u8>();
                if let Ok(v) = value {
                    Ok(DataTypeHolder::UnsignedTinyInt { value: v })
                } else {
                    error
                }
            }
            DataTypeDefiner::UnsignedSmallInt => {
                let value = data.parse::<u16>();
                if let Ok(v) = value {
                    Ok(DataTypeHolder::UnsignedSmallInt { value: v })
                } else {
                    error
                }
            }
            DataTypeDefiner::UnsignedInt => {
                let value = data.parse::<u32>();
                if let Ok(v) = value {
                    Ok(DataTypeHolder::UnsignedInt { value: v })
                } else {
                    error
                }
            }
            DataTypeDefiner::UnsignedBigInt => {
                let value = data.parse::<u64>();
                if let Ok(v) = value {
                    Ok(DataTypeHolder::UnsignedBigInt { value: v })
                } else {
                    error
                }
            }

            // Floats
            DataTypeDefiner::Float => {
                let value = data.parse::<f32>();
                if let Ok(v) = value {
                    Ok(DataTypeHolder::Float { value: v })
                } else {
                    error
                }
            }
            DataTypeDefiner::Double => {
                let value = data.parse::<f64>();
                if let Ok(v) = value {
                    Ok(DataTypeHolder::Double { value: v })
                } else {
                    error
                }
            }

            // Texts
            DataTypeDefiner::Char { len } => {
                if data.len() <= *len {
                    Ok(DataTypeHolder::Char {
                        value: data.to_string(),
                    })
                } else {
                    error
                }
            }
            DataTypeDefiner::VarChar { len } => {
                if data.len() <= *len {
                    Ok(DataTypeHolder::VarChar {
                        value: data.to_string(),
                    })
                } else {
                    error
                }
            }
            DataTypeDefiner::Text => Ok(DataTypeHolder::Text {
                value: data.to_string(),
            }),

            // Others
            DataTypeDefiner::Boolean => match data.to_lowercase().as_str() {
                "true" => Ok(DataTypeHolder::Boolean { value: true }),
                "false" => Ok(DataTypeHolder::Boolean { value: false }),
                _ => error,
            },
            DataTypeDefiner::Date => {
                let value = data.parse::<i64>();
                if let Ok(v) = value {
                    Ok(DataTypeHolder::Date { value: v })
                } else {
                    error
                }
            }
            DataTypeDefiner::DateTime => {
                let value = data.parse::<i64>();
                if let Ok(v) = value {
                    Ok(DataTypeHolder::DateTime { value: v })
                } else {
                    error
                }
            }
            DataTypeDefiner::Blob => {
                let bytes = data.as_bytes().to_vec();
                Ok(DataTypeHolder::Blob { value: bytes })
            }
        }
    }

    pub(crate) fn new_from_list(
        data_list: &Vec<String>,
        type_definition_list: &Vec<DataTypeDefiner>,
    ) -> Result<Vec<DataTypeHolder>, QlError> {
        if data_list.len() != type_definition_list.len() {
            return Err(QlError::DataTypeDefinitionError);
        }

        let mut holders: Vec<DataTypeHolder> = Vec::new();
        for i in 0..data_list.len() {
            match DataTypeHolder::new_from_string(&data_list[i], &type_definition_list[i]) {
                Ok(holder) => holders.push(holder),
                Err(e) => return Err(e),
            }
        }
        Ok(holders)
    }
}
