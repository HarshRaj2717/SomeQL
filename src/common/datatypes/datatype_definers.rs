use super::DataTypeDefiner;
use crate::common::{constants::DATATYPE_DEFINERS_REGEX, errors::QlError};
use regex::Regex;

impl DataTypeDefiner {
    pub(crate) fn new(type_name: &str) -> Result<DataTypeDefiner, QlError> {
        let name_and_legth_regex = Regex::new(DATATYPE_DEFINERS_REGEX).unwrap();
        let length_r: Option<usize>;
        let type_name_r: String;
        if let Some(captures) = name_and_legth_regex.captures(type_name) {
            type_name_r = captures[1].to_string();
            if captures.len() > 2 {
                match captures[2].parse::<usize>() {
                    Ok(len) => length_r = Some(len),
                    Err(_) => length_r = None,
                }
            } else {
                length_r = None;
            }
        } else {
            return Err(QlError::DataTypeDefinitionError);
        }
        println!("Parsing data type definer: {}", type_name);

        match type_name_r.as_ref() {
            // Integers
            "TinyInt" => Ok(DataTypeDefiner::TinyInt),
            "SmallInt" => Ok(DataTypeDefiner::SmallInt),
            "Int" => Ok(DataTypeDefiner::Int),
            "BigInt" => Ok(DataTypeDefiner::BigInt),

            // Unsigned Integers
            "UnsignedTinyInt" => Ok(DataTypeDefiner::UnsignedTinyInt),
            "UnsignedSmallInt" => Ok(DataTypeDefiner::UnsignedSmallInt),
            "UnsignedInt" => Ok(DataTypeDefiner::UnsignedInt),
            "UnsignedBigInt" => Ok(DataTypeDefiner::UnsignedBigInt),

            // Floats
            "Float" => Ok(DataTypeDefiner::Float),
            "Double" => Ok(DataTypeDefiner::Double),

            // Texts
            "Char" => Ok(DataTypeDefiner::Char {
                len: length_r.ok_or(QlError::DataTypeDefinitionError)?,
            }),
            "VarChar" => Ok(DataTypeDefiner::VarChar {
                len: length_r.ok_or(QlError::DataTypeDefinitionError)?,
            }),
            "Text" => Ok(DataTypeDefiner::Text),

            // Others
            "Boolean" => Ok(DataTypeDefiner::Boolean),
            "Date" => Ok(DataTypeDefiner::Date),
            "DateTime" => Ok(DataTypeDefiner::DateTime),
            "Blob" => Ok(DataTypeDefiner::Blob),
            _ => Err(QlError::DataTypeDefinitionError),
        }
    }

    pub(crate) fn new_from_list(
        type_name_list: &Vec<String>,
    ) -> Result<Vec<DataTypeDefiner>, QlError> {
        let mut definers: Vec<DataTypeDefiner> = Vec::new();
        for type_name in type_name_list {
            definers.push(DataTypeDefiner::new(type_name)?);
        }
        Ok(definers)
    }
}
