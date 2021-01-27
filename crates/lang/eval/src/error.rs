use derive_more::Display;
use neu_diagnostics::ToReport;
use neu_syntax::reexport::Red;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "Couldn't find any structure for field access")]
    ContextNotFound,

    #[display(fmt = "Expression is not a struct")]
    ValueNotStruct,

    #[display(fmt = "Field not found")]
    FieldNotFound,

    #[display(fmt  = "Unimplemented eval for AST: {:?}", _0)]
    Unimplemented(Red)
}

impl ToReport for Error {
    fn to_report(&self) -> String {
        self.to_string()
    }
}
