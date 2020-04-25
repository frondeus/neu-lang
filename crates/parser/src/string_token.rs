use derive_more::Display;

#[derive(Debug, PartialEq, Clone, Copy, Display)]
pub enum StrToken {
    #[display(fmt = "text")]
    Text,

    #[display(fmt = "`${{`")]
    OpenI,

    #[display(fmt = "`}}`")]
    CloseI,

    #[display(fmt = "`\"`")]
    Close,

}

