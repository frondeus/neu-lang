use crate::core::Name;
use crate::nodes;

nodes! {
    Value,
    Parens,
    Number,
    Boolean,
    String,
    Interpolated,

    StrValue,

    Unary,
    Binary,
    Op,

    Struct,
    Identifier,
    Key,

    Array,
    IdentPath
}
