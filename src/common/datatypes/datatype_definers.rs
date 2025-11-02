use super::DataTypeDefiners;
use crate::common::{constants::DATATYPE_DEFINERS_REGEX, errors::Error::DataTypeDefinitionError};
use regex::Regex;

impl DataTypeDefiners {
    pub(crate) fn new(type_name: &str) -> DataTypeDefiners {
        let name_and_legth_regex = Regex::new(DATATYPE_DEFINERS_REGEX).unwrap();
        let length_r: Option<usize>;
        let type_name_r: String;
        if let Some(captures) = name_and_legth_regex.captures(type_name) {
            type_name_r = captures[1].to_string();
            length_r = Option::Some(captures[2].parse().unwrap());
        } else {
            panic!("{}", DataTypeDefinitionError);
        }

        match type_name_r.as_ref() {
            // Integers
            "TinyInt" => DataTypeDefiners::TinyInt,
            "SmallInt" => DataTypeDefiners::SmallInt,
            "Int" => DataTypeDefiners::Int,
            "BigInt" => DataTypeDefiners::BigInt,

            // Unsigned Integers
            "UnsignedTinyInt" => DataTypeDefiners::UnsignedTinyInt,
            "UnsignedSmallInt" => DataTypeDefiners::UnsignedSmallInt,
            "UnsignedInt" => DataTypeDefiners::UnsignedInt,
            "UnsignedBigInt" => DataTypeDefiners::UnsignedBigInt,

            // Floats
            "Float" => DataTypeDefiners::Float,
            "Double" => DataTypeDefiners::Double,

            // Texts
            "Char" => DataTypeDefiners::Char {
                len: length_r.expect(DataTypeDefinitionError.to_string().as_str()),
            },
            "VarChar" => DataTypeDefiners::VarChar {
                len: length_r.expect(DataTypeDefinitionError.to_string().as_str()),
            },
            "Text" => DataTypeDefiners::Text,

            // Others
            "Boolean" => DataTypeDefiners::Boolean,
            "Date" => DataTypeDefiners::Date,
            "DateTime" => DataTypeDefiners::DateTime,
            "Blob" => DataTypeDefiners::Blob,
            _ => panic!("{}", DataTypeDefinitionError),
        }
    }

    pub(crate) fn new_from_list(type_name_list: &Vec<String>) -> Vec<DataTypeDefiners> {
        let mut definers: Vec<DataTypeDefiners> = Vec::new();
        for type_name in type_name_list {
            definers.push(DataTypeDefiners::new(type_name));
        }
        definers
    }
}
