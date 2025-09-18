mod datatypes;

pub(crate) enum DataTypesDefiner  {
    
}

pub(crate) enum DataTypes {
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
}
