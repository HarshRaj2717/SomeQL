//! Common datatypes used across the project

mod datatype_definers;
mod datatype_holders;

/// Definitions for data types used in table creation
/// 
/// Add entry for each new datatype in both Definers and Holders.
/// Also, update the `new` function in datatype_definers.rs
pub(crate) enum DataTypeDefiners {
    // Integers
    TinyInt,
    SmallInt,
    Int,
    BigInt,

    // Unsigned Integers
    UnsignedTinyInt,
    UnsignedSmallInt,
    UnsignedInt,
    UnsignedBigInt,

    // Floats
    Float,
    Double,

    // Texts
    Char { len: usize },
    VarChar { len: usize },
    Text,

    // Others
    Boolean,
    Date,
    DateTime,
    Blob,
}

/// Holders for data types used in table rows
pub(crate) enum DataTypeHolders {
    // Integers
    TinyInt { value: i8 },
    SmallInt { value: i16 },
    Int { value: i32 },
    BigInt { value: i64 },

    // Unsigned Integers
    UnsignedTinyInt { value: u8 },
    UnsignedSmallInt { value: u16 },
    UnsignedInt { value: u32 },
    UnsignedBigInt { value: u64 },

    // Floats
    Float { value: f32 },
    Double { value: f64 },

    // Texts
    Char { len: usize, value: String },
    VarChar { len: usize, value: String },
    Text { value: String },

    // Others
    Boolean { value: bool },
    Date { value: i64 },
    DateTime { value: i64 },
    Blob { value: Vec<u8> },
}
