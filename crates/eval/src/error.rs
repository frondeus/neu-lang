use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "Couldn't find any structure for field access")]
    ContextNotFound,

    #[display(fmt = "Expression is not a struct")]
    ValueNotStruct,

    #[display(fmt = "Field not found")]
    FieldNotFound,
}