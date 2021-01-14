#![allow(clippy::redundant_clone, clippy::wrong_self_convention)]
#![allow(dead_code)]
use microtree::{
    AliasBuilder, Ast, AstBuilder, Cache, Green, IntoBuilder, Name, Red, TokenBuilder,
};

mod handwritten;
pub use handwritten::*;

pub struct Nodes;
#[allow(non_upper_case_globals)]
impl Nodes {
    pub const Root: Name = Name::new("Root");
    pub const Token: Name = Name::new("token");
    pub const Error: Name = Name::new("error");
    pub const Value: Name = Name::new("Value");
    pub const Boolean: Name = Name::new("Boolean");
    pub const UnaryOp: Name = Name::new("UnaryOp");
    pub const BinaryOp: Name = Name::new("BinaryOp");
    pub const InnerStringPart: Name = Name::new("InnerStringPart");
    pub const ArticleItem: Name = Name::new("ArticleItem");
    pub const ArticleBodyItem: Name = Name::new("ArticleBodyItem");
    pub const MdValue: Name = Name::new("MdValue");
    pub const Binary: Name = Name::new("Binary");
    pub const IdentPath: Name = Name::new("IdentPath");
    pub const Unary: Name = Name::new("Unary");
    pub const MdString: Name = Name::new("MdString");
    pub const String: Name = Name::new("String");
    pub const Strukt: Name = Name::new("Strukt");
    pub const Array: Name = Name::new("Array");
    pub const Parens: Name = Name::new("Parens");
    pub const Markdown: Name = Name::new("Markdown");
    pub const InnerString: Name = Name::new("InnerString");
    pub const Interpolated: Name = Name::new("Interpolated");
    pub const StruktPair: Name = Name::new("StruktPair");
    pub const Key: Name = Name::new("Key");
    pub const MainArticle: Name = Name::new("MainArticle");
    pub const SubArticle: Name = Name::new("SubArticle");
    pub const ArticleHeader: Name = Name::new("ArticleHeader");
    pub const ArticleBody: Name = Name::new("ArticleBody");
    pub const SubArticleHeader: Name = Name::new("SubArticleHeader");
    pub const ArticleHeaderValues: Name = Name::new("ArticleHeaderValues");
    pub const ArticleRef: Name = Name::new("ArticleRef");
    pub const MdLink: Name = Name::new("MdLink");
    pub const MdCodeBlock: Name = Name::new("MdCodeBlock");
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpBang(pub(crate) Red);
impl Ast for OpBang {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "!" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl OpBang {
    pub fn build() -> TokenBuilder<OpBang> {
        TokenBuilder::new("!")
    }
}
impl IntoBuilder<UnaryOp> for TokenBuilder<OpBang> {
    fn into_builder(self) -> AliasBuilder<Self, UnaryOp> {
        AliasBuilder::new(Nodes::UnaryOp, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenI(pub(crate) Red);
impl Ast for OpenI {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "${" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl OpenI {
    pub fn build() -> TokenBuilder<OpenI> {
        TokenBuilder::new("${")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenP(pub(crate) Red);
impl Ast for OpenP {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "(" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl OpenP {
    pub fn build() -> TokenBuilder<OpenP> {
        TokenBuilder::new("(")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseP(pub(crate) Red);
impl Ast for CloseP {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != ")" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl CloseP {
    pub fn build() -> TokenBuilder<CloseP> {
        TokenBuilder::new(")")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpStar(pub(crate) Red);
impl Ast for OpStar {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "*" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl OpStar {
    pub fn build() -> TokenBuilder<OpStar> {
        TokenBuilder::new("*")
    }
}
impl IntoBuilder<BinaryOp> for TokenBuilder<OpStar> {
    fn into_builder(self) -> AliasBuilder<Self, BinaryOp> {
        AliasBuilder::new(Nodes::BinaryOp, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpPlus(pub(crate) Red);
impl Ast for OpPlus {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "+" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl OpPlus {
    pub fn build() -> TokenBuilder<OpPlus> {
        TokenBuilder::new("+")
    }
}
impl IntoBuilder<BinaryOp> for TokenBuilder<OpPlus> {
    fn into_builder(self) -> AliasBuilder<Self, BinaryOp> {
        AliasBuilder::new(Nodes::BinaryOp, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlusPlus(pub(crate) Red);
impl Ast for PlusPlus {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "++" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl PlusPlus {
    pub fn build() -> TokenBuilder<PlusPlus> {
        TokenBuilder::new("++")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlusPlusEnd(pub(crate) Red);
impl Ast for PlusPlusEnd {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "++ end ++" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl PlusPlusEnd {
    pub fn build() -> TokenBuilder<PlusPlusEnd> {
        TokenBuilder::new("++ end ++")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThreePlus(pub(crate) Red);
impl Ast for ThreePlus {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "+++" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ThreePlus {
    pub fn build() -> TokenBuilder<ThreePlus> {
        TokenBuilder::new("+++")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseBl(pub(crate) Red);
impl Ast for CloseBl {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "+]" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl CloseBl {
    pub fn build() -> TokenBuilder<CloseBl> {
        TokenBuilder::new("+]")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Comma(pub(crate) Red);
impl Ast for Comma {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "," {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Comma {
    pub fn build() -> TokenBuilder<Comma> {
        TokenBuilder::new(",")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpMinus(pub(crate) Red);
impl Ast for OpMinus {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "-" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl OpMinus {
    pub fn build() -> TokenBuilder<OpMinus> {
        TokenBuilder::new("-")
    }
}
impl IntoBuilder<UnaryOp> for TokenBuilder<OpMinus> {
    fn into_builder(self) -> AliasBuilder<Self, UnaryOp> {
        AliasBuilder::new(Nodes::UnaryOp, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpDot(pub(crate) Red);
impl Ast for OpDot {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "." {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl OpDot {
    pub fn build() -> TokenBuilder<OpDot> {
        TokenBuilder::new(".")
    }
}
impl IntoBuilder<UnaryOp> for TokenBuilder<OpDot> {
    fn into_builder(self) -> AliasBuilder<Self, UnaryOp> {
        AliasBuilder::new(Nodes::UnaryOp, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpSlash(pub(crate) Red);
impl Ast for OpSlash {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "/" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl OpSlash {
    pub fn build() -> TokenBuilder<OpSlash> {
        TokenBuilder::new("/")
    }
}
impl IntoBuilder<BinaryOp> for TokenBuilder<OpSlash> {
    fn into_builder(self) -> AliasBuilder<Self, BinaryOp> {
        AliasBuilder::new(Nodes::BinaryOp, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpColon(pub(crate) Red);
impl Ast for OpColon {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != ":" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl OpColon {
    pub fn build() -> TokenBuilder<OpColon> {
        TokenBuilder::new(":")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpAssign(pub(crate) Red);
impl Ast for OpAssign {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "=" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl OpAssign {
    pub fn build() -> TokenBuilder<OpAssign> {
        TokenBuilder::new("=")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpDEqual(pub(crate) Red);
impl Ast for OpDEqual {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "==" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl OpDEqual {
    pub fn build() -> TokenBuilder<OpDEqual> {
        TokenBuilder::new("==")
    }
}
impl IntoBuilder<BinaryOp> for TokenBuilder<OpDEqual> {
    fn into_builder(self) -> AliasBuilder<Self, BinaryOp> {
        AliasBuilder::new(Nodes::BinaryOp, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenB(pub(crate) Red);
impl Ast for OpenB {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "[" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl OpenB {
    pub fn build() -> TokenBuilder<OpenB> {
        TokenBuilder::new("[")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenBl(pub(crate) Red);
impl Ast for OpenBl {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "[+" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl OpenBl {
    pub fn build() -> TokenBuilder<OpenBl> {
        TokenBuilder::new("[+")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseB(pub(crate) Red);
impl Ast for CloseB {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "]" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl CloseB {
    pub fn build() -> TokenBuilder<CloseB> {
        TokenBuilder::new("]")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fences(pub(crate) Red);
impl Ast for Fences {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "```" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Fences {
    pub fn build() -> TokenBuilder<Fences> {
        TokenBuilder::new("```")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseI(pub(crate) Red);
impl Ast for CloseI {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "close_i" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl CloseI {
    pub fn build() -> TokenBuilder<CloseI> {
        TokenBuilder::new("close_i")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DQuote(pub(crate) Red);
impl Ast for DQuote {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "d_quote" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl DQuote {
    pub fn build() -> TokenBuilder<DQuote> {
        TokenBuilder::new("d_quote")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct False(pub(crate) Red);
impl Ast for False {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "false" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl False {
    pub fn build() -> TokenBuilder<False> {
        TokenBuilder::new("false")
    }
}
impl IntoBuilder<Boolean> for TokenBuilder<False> {
    fn into_builder(self) -> AliasBuilder<Self, Boolean> {
        AliasBuilder::new(Nodes::Boolean, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineEnding(pub(crate) Red);
impl Ast for LineEnding {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "ln" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl LineEnding {
    pub fn build() -> TokenBuilder<LineEnding> {
        TokenBuilder::new("ln")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MdQuote(pub(crate) Red);
impl Ast for MdQuote {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "md_quote" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl MdQuote {
    pub fn build() -> TokenBuilder<MdQuote> {
        TokenBuilder::new("md_quote")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct True(pub(crate) Red);
impl Ast for True {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "true" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl True {
    pub fn build() -> TokenBuilder<True> {
        TokenBuilder::new("true")
    }
}
impl IntoBuilder<Boolean> for TokenBuilder<True> {
    fn into_builder(self) -> AliasBuilder<Self, Boolean> {
        AliasBuilder::new(Nodes::Boolean, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenC(pub(crate) Red);
impl Ast for OpenC {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "{" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl OpenC {
    pub fn build() -> TokenBuilder<OpenC> {
        TokenBuilder::new("{")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseC(pub(crate) Red);
impl Ast for CloseC {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "}" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl CloseC {
    pub fn build() -> TokenBuilder<CloseC> {
        TokenBuilder::new("}")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Number(Number),
    Boolean(Boolean),
    Binary(Binary),
    IdentPath(IdentPath),
    Unary(Unary),
    MdString(MdString),
    String(String),
    Strukt(Strukt),
    Array(Array),
    Identifier(Identifier),
    Parens(Parens),
}
impl From<Number> for Value {
    fn from(val: Number) -> Self {
        Self::Number(val)
    }
}
impl From<Boolean> for Value {
    fn from(val: Boolean) -> Self {
        Self::Boolean(val)
    }
}
impl From<Binary> for Value {
    fn from(val: Binary) -> Self {
        Self::Binary(val)
    }
}
impl From<IdentPath> for Value {
    fn from(val: IdentPath) -> Self {
        Self::IdentPath(val)
    }
}
impl From<Unary> for Value {
    fn from(val: Unary) -> Self {
        Self::Unary(val)
    }
}
impl From<MdString> for Value {
    fn from(val: MdString) -> Self {
        Self::MdString(val)
    }
}
impl From<String> for Value {
    fn from(val: String) -> Self {
        Self::String(val)
    }
}
impl From<Strukt> for Value {
    fn from(val: Strukt) -> Self {
        Self::Strukt(val)
    }
}
impl From<Array> for Value {
    fn from(val: Array) -> Self {
        Self::Array(val)
    }
}
impl From<Identifier> for Value {
    fn from(val: Identifier) -> Self {
        Self::Identifier(val)
    }
}
impl From<Parens> for Value {
    fn from(val: Parens) -> Self {
        Self::Parens(val)
    }
}
impl Ast for Value {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Value) {
            return None;
        }
        None.or_else(|| Number::new(node.clone()).map(Value::Number))
            .or_else(|| Boolean::new(node.clone()).map(Value::Boolean))
            .or_else(|| Binary::new(node.clone()).map(Value::Binary))
            .or_else(|| IdentPath::new(node.clone()).map(Value::IdentPath))
            .or_else(|| Unary::new(node.clone()).map(Value::Unary))
            .or_else(|| MdString::new(node.clone()).map(Value::MdString))
            .or_else(|| String::new(node.clone()).map(Value::String))
            .or_else(|| Strukt::new(node.clone()).map(Value::Strukt))
            .or_else(|| Array::new(node.clone()).map(Value::Array))
            .or_else(|| Identifier::new(node.clone()).map(Value::Identifier))
            .or_else(|| Parens::new(node.clone()).map(Value::Parens))
    }
    fn red(&self) -> Red {
        match &self {
            Value::Number(node) => node.red(),
            Value::Boolean(node) => node.red(),
            Value::Binary(node) => node.red(),
            Value::IdentPath(node) => node.red(),
            Value::Unary(node) => node.red(),
            Value::MdString(node) => node.red(),
            Value::String(node) => node.red(),
            Value::Strukt(node) => node.red(),
            Value::Array(node) => node.red(),
            Value::Identifier(node) => node.red(),
            Value::Parens(node) => node.red(),
        }
    }
}
impl Value {
    pub fn as_number(self) -> Option<Number> {
        match self {
            Self::Number(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_boolean(self) -> Option<Boolean> {
        match self {
            Self::Boolean(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_binary(self) -> Option<Binary> {
        match self {
            Self::Binary(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_identpath(self) -> Option<IdentPath> {
        match self {
            Self::IdentPath(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_unary(self) -> Option<Unary> {
        match self {
            Self::Unary(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_mdstring(self) -> Option<MdString> {
        match self {
            Self::MdString(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_string(self) -> Option<String> {
        match self {
            Self::String(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_strukt(self) -> Option<Strukt> {
        match self {
            Self::Strukt(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_array(self) -> Option<Array> {
        match self {
            Self::Array(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_identifier(self) -> Option<Identifier> {
        match self {
            Self::Identifier(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_parens(self) -> Option<Parens> {
        match self {
            Self::Parens(val) => Some(val),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Boolean {
    True(True),
    False(False),
}
impl From<True> for Boolean {
    fn from(val: True) -> Self {
        Self::True(val)
    }
}
impl From<False> for Boolean {
    fn from(val: False) -> Self {
        Self::False(val)
    }
}
impl Ast for Boolean {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Boolean) {
            return None;
        }
        None.or_else(|| True::new(node.clone()).map(Boolean::True))
            .or_else(|| False::new(node.clone()).map(Boolean::False))
    }
    fn red(&self) -> Red {
        match &self {
            Boolean::True(node) => node.red(),
            Boolean::False(node) => node.red(),
        }
    }
}
impl Boolean {
    pub fn as_true(self) -> Option<True> {
        match self {
            Self::True(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_false(self) -> Option<False> {
        match self {
            Self::False(val) => Some(val),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnaryOp {
    OpMinus(OpMinus),
    OpBang(OpBang),
    OpDot(OpDot),
}
impl From<OpMinus> for UnaryOp {
    fn from(val: OpMinus) -> Self {
        Self::OpMinus(val)
    }
}
impl From<OpBang> for UnaryOp {
    fn from(val: OpBang) -> Self {
        Self::OpBang(val)
    }
}
impl From<OpDot> for UnaryOp {
    fn from(val: OpDot) -> Self {
        Self::OpDot(val)
    }
}
impl Ast for UnaryOp {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::UnaryOp) {
            return None;
        }
        None.or_else(|| OpMinus::new(node.clone()).map(UnaryOp::OpMinus))
            .or_else(|| OpBang::new(node.clone()).map(UnaryOp::OpBang))
            .or_else(|| OpDot::new(node.clone()).map(UnaryOp::OpDot))
    }
    fn red(&self) -> Red {
        match &self {
            UnaryOp::OpMinus(node) => node.red(),
            UnaryOp::OpBang(node) => node.red(),
            UnaryOp::OpDot(node) => node.red(),
        }
    }
}
impl UnaryOp {
    pub fn as_opminus(self) -> Option<OpMinus> {
        match self {
            Self::OpMinus(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_opbang(self) -> Option<OpBang> {
        match self {
            Self::OpBang(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_opdot(self) -> Option<OpDot> {
        match self {
            Self::OpDot(val) => Some(val),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BinaryOp {
    OpMinus(OpMinus),
    OpPlus(OpPlus),
    OpSlash(OpSlash),
    OpStar(OpStar),
    OpDEqual(OpDEqual),
}
impl From<OpMinus> for BinaryOp {
    fn from(val: OpMinus) -> Self {
        Self::OpMinus(val)
    }
}
impl From<OpPlus> for BinaryOp {
    fn from(val: OpPlus) -> Self {
        Self::OpPlus(val)
    }
}
impl From<OpSlash> for BinaryOp {
    fn from(val: OpSlash) -> Self {
        Self::OpSlash(val)
    }
}
impl From<OpStar> for BinaryOp {
    fn from(val: OpStar) -> Self {
        Self::OpStar(val)
    }
}
impl From<OpDEqual> for BinaryOp {
    fn from(val: OpDEqual) -> Self {
        Self::OpDEqual(val)
    }
}
impl Ast for BinaryOp {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::BinaryOp) {
            return None;
        }
        None.or_else(|| OpMinus::new(node.clone()).map(BinaryOp::OpMinus))
            .or_else(|| OpPlus::new(node.clone()).map(BinaryOp::OpPlus))
            .or_else(|| OpSlash::new(node.clone()).map(BinaryOp::OpSlash))
            .or_else(|| OpStar::new(node.clone()).map(BinaryOp::OpStar))
            .or_else(|| OpDEqual::new(node.clone()).map(BinaryOp::OpDEqual))
    }
    fn red(&self) -> Red {
        match &self {
            BinaryOp::OpMinus(node) => node.red(),
            BinaryOp::OpPlus(node) => node.red(),
            BinaryOp::OpSlash(node) => node.red(),
            BinaryOp::OpStar(node) => node.red(),
            BinaryOp::OpDEqual(node) => node.red(),
        }
    }
}
impl BinaryOp {
    pub fn as_opminus(self) -> Option<OpMinus> {
        match self {
            Self::OpMinus(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_opplus(self) -> Option<OpPlus> {
        match self {
            Self::OpPlus(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_opslash(self) -> Option<OpSlash> {
        match self {
            Self::OpSlash(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_opstar(self) -> Option<OpStar> {
        match self {
            Self::OpStar(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_opdequal(self) -> Option<OpDEqual> {
        match self {
            Self::OpDEqual(val) => Some(val),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InnerStringPart {
    Text(Text),
    Interpolated(Interpolated),
}
impl From<Text> for InnerStringPart {
    fn from(val: Text) -> Self {
        Self::Text(val)
    }
}
impl From<Interpolated> for InnerStringPart {
    fn from(val: Interpolated) -> Self {
        Self::Interpolated(val)
    }
}
impl Ast for InnerStringPart {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::InnerStringPart) {
            return None;
        }
        None.or_else(|| Text::new(node.clone()).map(InnerStringPart::Text))
            .or_else(|| Interpolated::new(node.clone()).map(InnerStringPart::Interpolated))
    }
    fn red(&self) -> Red {
        match &self {
            InnerStringPart::Text(node) => node.red(),
            InnerStringPart::Interpolated(node) => node.red(),
        }
    }
}
impl InnerStringPart {
    pub fn as_text(self) -> Option<Text> {
        match self {
            Self::Text(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_interpolated(self) -> Option<Interpolated> {
        match self {
            Self::Interpolated(val) => Some(val),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArticleItem {
    MainArticle(MainArticle),
    SubArticle(SubArticle),
}
impl From<MainArticle> for ArticleItem {
    fn from(val: MainArticle) -> Self {
        Self::MainArticle(val)
    }
}
impl From<SubArticle> for ArticleItem {
    fn from(val: SubArticle) -> Self {
        Self::SubArticle(val)
    }
}
impl Ast for ArticleItem {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleItem) {
            return None;
        }
        None.or_else(|| MainArticle::new(node.clone()).map(ArticleItem::MainArticle))
            .or_else(|| SubArticle::new(node.clone()).map(ArticleItem::SubArticle))
    }
    fn red(&self) -> Red {
        match &self {
            ArticleItem::MainArticle(node) => node.red(),
            ArticleItem::SubArticle(node) => node.red(),
        }
    }
}
impl ArticleItem {
    pub fn as_mainarticle(self) -> Option<MainArticle> {
        match self {
            Self::MainArticle(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_subarticle(self) -> Option<SubArticle> {
        match self {
            Self::SubArticle(val) => Some(val),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArticleBodyItem {
    Markdown(Markdown),
    SubArticle(SubArticle),
    ArticleRef(ArticleRef),
}
impl From<Markdown> for ArticleBodyItem {
    fn from(val: Markdown) -> Self {
        Self::Markdown(val)
    }
}
impl From<SubArticle> for ArticleBodyItem {
    fn from(val: SubArticle) -> Self {
        Self::SubArticle(val)
    }
}
impl From<ArticleRef> for ArticleBodyItem {
    fn from(val: ArticleRef) -> Self {
        Self::ArticleRef(val)
    }
}
impl Ast for ArticleBodyItem {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleBodyItem) {
            return None;
        }
        None.or_else(|| Markdown::new(node.clone()).map(ArticleBodyItem::Markdown))
            .or_else(|| SubArticle::new(node.clone()).map(ArticleBodyItem::SubArticle))
            .or_else(|| ArticleRef::new(node.clone()).map(ArticleBodyItem::ArticleRef))
    }
    fn red(&self) -> Red {
        match &self {
            ArticleBodyItem::Markdown(node) => node.red(),
            ArticleBodyItem::SubArticle(node) => node.red(),
            ArticleBodyItem::ArticleRef(node) => node.red(),
        }
    }
}
impl ArticleBodyItem {
    pub fn as_markdown(self) -> Option<Markdown> {
        match self {
            Self::Markdown(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_subarticle(self) -> Option<SubArticle> {
        match self {
            Self::SubArticle(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_articleref(self) -> Option<ArticleRef> {
        match self {
            Self::ArticleRef(val) => Some(val),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MdValue {
    MdLink(MdLink),
    MdCodeBlock(MdCodeBlock),
    Text(Text),
}
impl From<MdLink> for MdValue {
    fn from(val: MdLink) -> Self {
        Self::MdLink(val)
    }
}
impl From<MdCodeBlock> for MdValue {
    fn from(val: MdCodeBlock) -> Self {
        Self::MdCodeBlock(val)
    }
}
impl From<Text> for MdValue {
    fn from(val: Text) -> Self {
        Self::Text(val)
    }
}
impl Ast for MdValue {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::MdValue) {
            return None;
        }
        None.or_else(|| MdLink::new(node.clone()).map(MdValue::MdLink))
            .or_else(|| MdCodeBlock::new(node.clone()).map(MdValue::MdCodeBlock))
            .or_else(|| Text::new(node.clone()).map(MdValue::Text))
    }
    fn red(&self) -> Red {
        match &self {
            MdValue::MdLink(node) => node.red(),
            MdValue::MdCodeBlock(node) => node.red(),
            MdValue::Text(node) => node.red(),
        }
    }
}
impl MdValue {
    pub fn as_mdlink(self) -> Option<MdLink> {
        match self {
            Self::MdLink(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_mdcodeblock(self) -> Option<MdCodeBlock> {
        match self {
            Self::MdCodeBlock(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_text(self) -> Option<Text> {
        match self {
            Self::Text(val) => Some(val),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Binary(pub(crate) Red);
impl Ast for Binary {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Binary) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Binary {
    pub fn left(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).nth(0usize)
    }
    pub fn binary_op(&self) -> Option<BinaryOp> {
        self.0.children().filter_map(BinaryOp::new).nth(0usize)
    }
    pub fn right(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).nth(1usize)
    }
    pub fn build<T0, T1, T2>() -> BinaryBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = Value>,
        T1: AstBuilder<T = BinaryOp>,
        T2: AstBuilder<T = Value>,
    {
        Default::default()
    }
}
pub struct BinaryBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Value>,
    T1: AstBuilder<T = BinaryOp>,
    T2: AstBuilder<T = Value>,
{
    left: Option<T0>,
    binary_op: Option<T1>,
    right: Option<T2>,
}
impl<T0, T1, T2> Default for BinaryBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Value>,
    T1: AstBuilder<T = BinaryOp>,
    T2: AstBuilder<T = Value>,
{
    fn default() -> Self {
        Self {
            left: Default::default(),
            binary_op: Default::default(),
            right: Default::default(),
        }
    }
}
impl<T0, T1, T2> BinaryBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Value>,
    T1: AstBuilder<T = BinaryOp>,
    T2: AstBuilder<T = Value>,
{
    pub fn fill(self, left: T0, binary_op: T1, right: T2) -> Self {
        Self {
            left: Some(left),
            binary_op: Some(binary_op),
            right: Some(right),
        }
    }
}
impl<T0, T1, T2> AstBuilder for BinaryBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Value>,
    T1: AstBuilder<T = BinaryOp>,
    T2: AstBuilder<T = Value>,
{
    type T = Binary;
    fn build(self, builder: &mut Cache) -> Binary {
        let green = AstBuilder::build_green(self, builder);
        Binary::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.left.map(|it| it.build_green(builder)).into_iter())
            .chain(self.binary_op.map(|it| it.build_green(builder)).into_iter())
            .chain(self.right.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::Binary, children)
    }
}
impl<T0, T1, T2> IntoBuilder<Value> for BinaryBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Value>,
    T1: AstBuilder<T = BinaryOp>,
    T2: AstBuilder<T = Value>,
{
    fn into_builder(self) -> AliasBuilder<Self, Value> {
        AliasBuilder::new(Nodes::Value, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentPath(pub(crate) Red);
impl Ast for IdentPath {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::IdentPath) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl IdentPath {
    pub fn left(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).nth(0usize)
    }
    pub fn op_dot_token(&self) -> Option<OpDot> {
        self.0.children().filter_map(OpDot::new).nth(0usize)
    }
    pub fn right(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).nth(1usize)
    }
    pub fn build<T0, T1, T2>() -> IdentPathBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = Value>,
        T1: AstBuilder<T = OpDot>,
        T2: AstBuilder<T = Value>,
    {
        Default::default()
    }
}
pub struct IdentPathBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Value>,
    T1: AstBuilder<T = OpDot>,
    T2: AstBuilder<T = Value>,
{
    left: Option<T0>,
    op_dot: Option<T1>,
    right: Option<T2>,
}
impl<T0, T1, T2> Default for IdentPathBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Value>,
    T1: AstBuilder<T = OpDot>,
    T2: AstBuilder<T = Value>,
{
    fn default() -> Self {
        Self {
            left: Default::default(),
            op_dot: Default::default(),
            right: Default::default(),
        }
    }
}
impl<T0, T1, T2> IdentPathBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Value>,
    T1: AstBuilder<T = OpDot>,
    T2: AstBuilder<T = Value>,
{
    pub fn fill(self, left: T0, op_dot: T1, right: T2) -> Self {
        Self {
            left: Some(left),
            op_dot: Some(op_dot),
            right: Some(right),
        }
    }
}
impl<T0, T1, T2> AstBuilder for IdentPathBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Value>,
    T1: AstBuilder<T = OpDot>,
    T2: AstBuilder<T = Value>,
{
    type T = IdentPath;
    fn build(self, builder: &mut Cache) -> IdentPath {
        let green = AstBuilder::build_green(self, builder);
        IdentPath::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.left.map(|it| it.build_green(builder)).into_iter())
            .chain(self.op_dot.map(|it| it.build_green(builder)).into_iter())
            .chain(self.right.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::IdentPath, children)
    }
}
impl<T0, T1, T2> IntoBuilder<Value> for IdentPathBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Value>,
    T1: AstBuilder<T = OpDot>,
    T2: AstBuilder<T = Value>,
{
    fn into_builder(self) -> AliasBuilder<Self, Value> {
        AliasBuilder::new(Nodes::Value, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unary(pub(crate) Red);
impl Ast for Unary {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Unary) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Unary {
    pub fn unary_op(&self) -> Option<UnaryOp> {
        self.0.children().filter_map(UnaryOp::new).nth(0usize)
    }
    pub fn value(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).nth(0usize)
    }
    pub fn build<T0, T1>() -> UnaryBuilder<T0, T1>
    where
        T0: AstBuilder<T = UnaryOp>,
        T1: AstBuilder<T = Value>,
    {
        Default::default()
    }
}
pub struct UnaryBuilder<T0, T1>
where
    T0: AstBuilder<T = UnaryOp>,
    T1: AstBuilder<T = Value>,
{
    unary_op: Option<T0>,
    value: Option<T1>,
}
impl<T0, T1> Default for UnaryBuilder<T0, T1>
where
    T0: AstBuilder<T = UnaryOp>,
    T1: AstBuilder<T = Value>,
{
    fn default() -> Self {
        Self {
            unary_op: Default::default(),
            value: Default::default(),
        }
    }
}
impl<T0, T1> UnaryBuilder<T0, T1>
where
    T0: AstBuilder<T = UnaryOp>,
    T1: AstBuilder<T = Value>,
{
    pub fn fill(self, unary_op: T0, value: T1) -> Self {
        Self {
            unary_op: Some(unary_op),
            value: Some(value),
        }
    }
}
impl<T0, T1> AstBuilder for UnaryBuilder<T0, T1>
where
    T0: AstBuilder<T = UnaryOp>,
    T1: AstBuilder<T = Value>,
{
    type T = Unary;
    fn build(self, builder: &mut Cache) -> Unary {
        let green = AstBuilder::build_green(self, builder);
        Unary::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.unary_op.map(|it| it.build_green(builder)).into_iter())
            .chain(self.value.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::Unary, children)
    }
}
impl<T0, T1> IntoBuilder<Value> for UnaryBuilder<T0, T1>
where
    T0: AstBuilder<T = UnaryOp>,
    T1: AstBuilder<T = Value>,
{
    fn into_builder(self) -> AliasBuilder<Self, Value> {
        AliasBuilder::new(Nodes::Value, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MdString(pub(crate) Red);
impl Ast for MdString {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::MdString) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl MdString {
    pub fn md_quote_token(&self) -> Option<MdQuote> {
        self.0.children().filter_map(MdQuote::new).nth(0usize)
    }
    pub fn markdown(&self) -> Option<Markdown> {
        self.0.children().filter_map(Markdown::new).nth(0usize)
    }
    pub fn dquote_token(&self) -> Option<DQuote> {
        self.0.children().filter_map(DQuote::new).nth(0usize)
    }
    pub fn build<T0, T1, T2>() -> MdStringBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = MdQuote>,
        T1: AstBuilder<T = Markdown>,
        T2: AstBuilder<T = DQuote>,
    {
        Default::default()
    }
}
pub struct MdStringBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = MdQuote>,
    T1: AstBuilder<T = Markdown>,
    T2: AstBuilder<T = DQuote>,
{
    md_quote: Option<T0>,
    markdown: Option<T1>,
    dquote: Option<T2>,
}
impl<T0, T1, T2> Default for MdStringBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = MdQuote>,
    T1: AstBuilder<T = Markdown>,
    T2: AstBuilder<T = DQuote>,
{
    fn default() -> Self {
        Self {
            md_quote: Default::default(),
            markdown: Default::default(),
            dquote: Default::default(),
        }
    }
}
impl<T0, T1, T2> MdStringBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = MdQuote>,
    T1: AstBuilder<T = Markdown>,
    T2: AstBuilder<T = DQuote>,
{
    pub fn fill(self, md_quote: T0, markdown: T1, dquote: T2) -> Self {
        Self {
            md_quote: Some(md_quote),
            markdown: Some(markdown),
            dquote: Some(dquote),
        }
    }
}
impl<T0, T1, T2> AstBuilder for MdStringBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = MdQuote>,
    T1: AstBuilder<T = Markdown>,
    T2: AstBuilder<T = DQuote>,
{
    type T = MdString;
    fn build(self, builder: &mut Cache) -> MdString {
        let green = AstBuilder::build_green(self, builder);
        MdString::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.md_quote.map(|it| it.build_green(builder)).into_iter())
            .chain(self.markdown.map(|it| it.build_green(builder)).into_iter())
            .chain(self.dquote.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::MdString, children)
    }
}
impl<T0, T1, T2> IntoBuilder<Value> for MdStringBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = MdQuote>,
    T1: AstBuilder<T = Markdown>,
    T2: AstBuilder<T = DQuote>,
{
    fn into_builder(self) -> AliasBuilder<Self, Value> {
        AliasBuilder::new(Nodes::Value, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct String(pub(crate) Red);
impl Ast for String {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::String) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl String {
    pub fn left_quote_token(&self) -> Option<DQuote> {
        self.0.children().filter_map(DQuote::new).nth(0usize)
    }
    pub fn inner_string(&self) -> Option<InnerString> {
        self.0.children().filter_map(InnerString::new).nth(0usize)
    }
    pub fn right_quote_token(&self) -> Option<DQuote> {
        self.0.children().filter_map(DQuote::new).nth(1usize)
    }
    pub fn build<T0, T1, T2>() -> StringBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = DQuote>,
        T1: AstBuilder<T = InnerString>,
        T2: AstBuilder<T = DQuote>,
    {
        Default::default()
    }
}
pub struct StringBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = DQuote>,
    T1: AstBuilder<T = InnerString>,
    T2: AstBuilder<T = DQuote>,
{
    left_quote: Option<T0>,
    inner_string: Option<T1>,
    right_quote: Option<T2>,
}
impl<T0, T1, T2> Default for StringBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = DQuote>,
    T1: AstBuilder<T = InnerString>,
    T2: AstBuilder<T = DQuote>,
{
    fn default() -> Self {
        Self {
            left_quote: Default::default(),
            inner_string: Default::default(),
            right_quote: Default::default(),
        }
    }
}
impl<T0, T1, T2> StringBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = DQuote>,
    T1: AstBuilder<T = InnerString>,
    T2: AstBuilder<T = DQuote>,
{
    pub fn fill(self, left_quote: T0, inner_string: T1, right_quote: T2) -> Self {
        Self {
            left_quote: Some(left_quote),
            inner_string: Some(inner_string),
            right_quote: Some(right_quote),
        }
    }
}
impl<T0, T1, T2> AstBuilder for StringBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = DQuote>,
    T1: AstBuilder<T = InnerString>,
    T2: AstBuilder<T = DQuote>,
{
    type T = String;
    fn build(self, builder: &mut Cache) -> String {
        let green = AstBuilder::build_green(self, builder);
        String::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(
                self.left_quote
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(
                self.inner_string
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(
                self.right_quote
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .collect();
        builder.node(Nodes::String, children)
    }
}
impl<T0, T1, T2> IntoBuilder<Value> for StringBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = DQuote>,
    T1: AstBuilder<T = InnerString>,
    T2: AstBuilder<T = DQuote>,
{
    fn into_builder(self) -> AliasBuilder<Self, Value> {
        AliasBuilder::new(Nodes::Value, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Strukt(pub(crate) Red);
impl Ast for Strukt {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Strukt) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Strukt {
    pub fn open_c_token(&self) -> Option<OpenC> {
        self.0.children().filter_map(OpenC::new).nth(0usize)
    }
    pub fn pairs(&self) -> impl Iterator<Item = StruktPair> + '_ {
        self.0.children().filter_map(StruktPair::new)
    }
    pub fn close_c_token(&self) -> Option<CloseC> {
        self.0.children().filter_map(CloseC::new).nth(0usize)
    }
    pub fn build<T0, T1, T2>() -> StruktBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = OpenC>,
        T1: AstBuilder<T = Comma>,
        T2: AstBuilder<T = CloseC>,
    {
        Default::default()
    }
}
pub struct StruktBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenC>,
    T1: AstBuilder<T = Comma>,
    T2: AstBuilder<T = CloseC>,
{
    open_c: Option<T0>,
    pairs: Vec<Box<dyn AstBuilder<T = StruktPair>>>,
    comma: Option<T1>,
    close_c: Option<T2>,
}
impl<T0, T1, T2> Default for StruktBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenC>,
    T1: AstBuilder<T = Comma>,
    T2: AstBuilder<T = CloseC>,
{
    fn default() -> Self {
        Self {
            open_c: Default::default(),
            pairs: Default::default(),
            comma: Default::default(),
            close_c: Default::default(),
        }
    }
}
impl<T0, T1, T2> StruktBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenC>,
    T1: AstBuilder<T = Comma>,
    T2: AstBuilder<T = CloseC>,
{
    pub fn fill(
        self,
        open_c: T0,
        pairs: Vec<Box<dyn AstBuilder<T = StruktPair>>>,
        comma: T1,
        close_c: T2,
    ) -> Self {
        Self {
            open_c: Some(open_c),
            pairs,
            comma: Some(comma),
            close_c: Some(close_c),
        }
    }
}
impl<T0, T1, T2> AstBuilder for StruktBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenC>,
    T1: AstBuilder<T = Comma>,
    T2: AstBuilder<T = CloseC>,
{
    type T = Strukt;
    fn build(self, builder: &mut Cache) -> Strukt {
        let green = AstBuilder::build_green(self, builder);
        Strukt::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.open_c.map(|it| it.build_green(builder)).into_iter())
            .chain({
                let delit = self.comma.map(|it| it.build_green(builder));
                self.pairs
                    .into_iter()
                    .flat_map(|it| {
                        Some(it.build_boxed_green(builder))
                            .into_iter()
                            .chain(delit.clone().into_iter())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .chain(self.close_c.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::Strukt, children)
    }
}
impl<T0, T1, T2> IntoBuilder<Value> for StruktBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenC>,
    T1: AstBuilder<T = Comma>,
    T2: AstBuilder<T = CloseC>,
{
    fn into_builder(self) -> AliasBuilder<Self, Value> {
        AliasBuilder::new(Nodes::Value, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Array(pub(crate) Red);
impl Ast for Array {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Array) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Array {
    pub fn open_b_token(&self) -> Option<OpenB> {
        self.0.children().filter_map(OpenB::new).nth(0usize)
    }
    pub fn values(&self) -> impl Iterator<Item = Value> + '_ {
        self.0.children().filter_map(Value::new)
    }
    pub fn close_b_token(&self) -> Option<CloseB> {
        self.0.children().filter_map(CloseB::new).nth(0usize)
    }
    pub fn build<T0, T1, T2>() -> ArrayBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = OpenB>,
        T1: AstBuilder<T = Comma>,
        T2: AstBuilder<T = CloseB>,
    {
        Default::default()
    }
}
pub struct ArrayBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenB>,
    T1: AstBuilder<T = Comma>,
    T2: AstBuilder<T = CloseB>,
{
    open_b: Option<T0>,
    values: Vec<Box<dyn AstBuilder<T = Value>>>,
    comma: Option<T1>,
    close_b: Option<T2>,
}
impl<T0, T1, T2> Default for ArrayBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenB>,
    T1: AstBuilder<T = Comma>,
    T2: AstBuilder<T = CloseB>,
{
    fn default() -> Self {
        Self {
            open_b: Default::default(),
            values: Default::default(),
            comma: Default::default(),
            close_b: Default::default(),
        }
    }
}
impl<T0, T1, T2> ArrayBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenB>,
    T1: AstBuilder<T = Comma>,
    T2: AstBuilder<T = CloseB>,
{
    pub fn fill(
        self,
        open_b: T0,
        values: Vec<Box<dyn AstBuilder<T = Value>>>,
        comma: T1,
        close_b: T2,
    ) -> Self {
        Self {
            open_b: Some(open_b),
            values,
            comma: Some(comma),
            close_b: Some(close_b),
        }
    }
}
impl<T0, T1, T2> AstBuilder for ArrayBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenB>,
    T1: AstBuilder<T = Comma>,
    T2: AstBuilder<T = CloseB>,
{
    type T = Array;
    fn build(self, builder: &mut Cache) -> Array {
        let green = AstBuilder::build_green(self, builder);
        Array::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.open_b.map(|it| it.build_green(builder)).into_iter())
            .chain({
                let delit = self.comma.map(|it| it.build_green(builder));
                self.values
                    .into_iter()
                    .flat_map(|it| {
                        Some(it.build_boxed_green(builder))
                            .into_iter()
                            .chain(delit.clone().into_iter())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .chain(self.close_b.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::Array, children)
    }
}
impl<T0, T1, T2> IntoBuilder<Value> for ArrayBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenB>,
    T1: AstBuilder<T = Comma>,
    T2: AstBuilder<T = CloseB>,
{
    fn into_builder(self) -> AliasBuilder<Self, Value> {
        AliasBuilder::new(Nodes::Value, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parens(pub(crate) Red);
impl Ast for Parens {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Parens) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Parens {
    pub fn open_p_token(&self) -> Option<OpenP> {
        self.0.children().filter_map(OpenP::new).nth(0usize)
    }
    pub fn value(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).nth(0usize)
    }
    pub fn close_p_token(&self) -> Option<CloseP> {
        self.0.children().filter_map(CloseP::new).nth(0usize)
    }
    pub fn build<T0, T1, T2>() -> ParensBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = OpenP>,
        T1: AstBuilder<T = Value>,
        T2: AstBuilder<T = CloseP>,
    {
        Default::default()
    }
}
pub struct ParensBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenP>,
    T1: AstBuilder<T = Value>,
    T2: AstBuilder<T = CloseP>,
{
    open_p: Option<T0>,
    value: Option<T1>,
    close_p: Option<T2>,
}
impl<T0, T1, T2> Default for ParensBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenP>,
    T1: AstBuilder<T = Value>,
    T2: AstBuilder<T = CloseP>,
{
    fn default() -> Self {
        Self {
            open_p: Default::default(),
            value: Default::default(),
            close_p: Default::default(),
        }
    }
}
impl<T0, T1, T2> ParensBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenP>,
    T1: AstBuilder<T = Value>,
    T2: AstBuilder<T = CloseP>,
{
    pub fn fill(self, open_p: T0, value: T1, close_p: T2) -> Self {
        Self {
            open_p: Some(open_p),
            value: Some(value),
            close_p: Some(close_p),
        }
    }
}
impl<T0, T1, T2> AstBuilder for ParensBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenP>,
    T1: AstBuilder<T = Value>,
    T2: AstBuilder<T = CloseP>,
{
    type T = Parens;
    fn build(self, builder: &mut Cache) -> Parens {
        let green = AstBuilder::build_green(self, builder);
        Parens::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.open_p.map(|it| it.build_green(builder)).into_iter())
            .chain(self.value.map(|it| it.build_green(builder)).into_iter())
            .chain(self.close_p.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::Parens, children)
    }
}
impl<T0, T1, T2> IntoBuilder<Value> for ParensBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenP>,
    T1: AstBuilder<T = Value>,
    T2: AstBuilder<T = CloseP>,
{
    fn into_builder(self) -> AliasBuilder<Self, Value> {
        AliasBuilder::new(Nodes::Value, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Markdown(pub(crate) Red);
impl Ast for Markdown {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Markdown) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Markdown {
    pub fn values(&self) -> impl Iterator<Item = MdValue> + '_ {
        self.0.children().filter_map(MdValue::new)
    }
    pub fn build() -> MarkdownBuilder {
        Default::default()
    }
}
pub struct MarkdownBuilder {
    values: Vec<Box<dyn AstBuilder<T = MdValue>>>,
}
impl Default for MarkdownBuilder {
    fn default() -> Self {
        Self {
            values: Default::default(),
        }
    }
}
impl MarkdownBuilder {
    pub fn fill(self, values: Vec<Box<dyn AstBuilder<T = MdValue>>>) -> Self {
        Self { values }
    }
}
impl AstBuilder for MarkdownBuilder {
    type T = Markdown;
    fn build(self, builder: &mut Cache) -> Markdown {
        let green = AstBuilder::build_green(self, builder);
        Markdown::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain({
                self.values
                    .into_iter()
                    .map(|it| it.build_boxed_green(builder))
                    .collect::<Vec<_>>()
            })
            .collect();
        builder.node(Nodes::Markdown, children)
    }
}
impl IntoBuilder<ArticleBodyItem> for MarkdownBuilder {
    fn into_builder(self) -> AliasBuilder<Self, ArticleBodyItem> {
        AliasBuilder::new(Nodes::ArticleBodyItem, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InnerString(pub(crate) Red);
impl Ast for InnerString {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::InnerString) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl InnerString {
    pub fn parts(&self) -> impl Iterator<Item = InnerStringPart> + '_ {
        self.0.children().filter_map(InnerStringPart::new)
    }
    pub fn build() -> InnerStringBuilder {
        Default::default()
    }
}
pub struct InnerStringBuilder {
    parts: Vec<Box<dyn AstBuilder<T = InnerStringPart>>>,
}
impl Default for InnerStringBuilder {
    fn default() -> Self {
        Self {
            parts: Default::default(),
        }
    }
}
impl InnerStringBuilder {
    pub fn fill(self, parts: Vec<Box<dyn AstBuilder<T = InnerStringPart>>>) -> Self {
        Self { parts }
    }
}
impl AstBuilder for InnerStringBuilder {
    type T = InnerString;
    fn build(self, builder: &mut Cache) -> InnerString {
        let green = AstBuilder::build_green(self, builder);
        InnerString::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain({
                self.parts
                    .into_iter()
                    .map(|it| it.build_boxed_green(builder))
                    .collect::<Vec<_>>()
            })
            .collect();
        builder.node(Nodes::InnerString, children)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Interpolated(pub(crate) Red);
impl Ast for Interpolated {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Interpolated) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Interpolated {
    pub fn open_i_token(&self) -> Option<OpenI> {
        self.0.children().filter_map(OpenI::new).nth(0usize)
    }
    pub fn value(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).nth(0usize)
    }
    pub fn close_i_token(&self) -> Option<CloseI> {
        self.0.children().filter_map(CloseI::new).nth(0usize)
    }
    pub fn build<T0, T1, T2>() -> InterpolatedBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = OpenI>,
        T1: AstBuilder<T = Value>,
        T2: AstBuilder<T = CloseI>,
    {
        Default::default()
    }
}
pub struct InterpolatedBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenI>,
    T1: AstBuilder<T = Value>,
    T2: AstBuilder<T = CloseI>,
{
    open_i: Option<T0>,
    value: Option<T1>,
    close_i: Option<T2>,
}
impl<T0, T1, T2> Default for InterpolatedBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenI>,
    T1: AstBuilder<T = Value>,
    T2: AstBuilder<T = CloseI>,
{
    fn default() -> Self {
        Self {
            open_i: Default::default(),
            value: Default::default(),
            close_i: Default::default(),
        }
    }
}
impl<T0, T1, T2> InterpolatedBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenI>,
    T1: AstBuilder<T = Value>,
    T2: AstBuilder<T = CloseI>,
{
    pub fn fill(self, open_i: T0, value: T1, close_i: T2) -> Self {
        Self {
            open_i: Some(open_i),
            value: Some(value),
            close_i: Some(close_i),
        }
    }
}
impl<T0, T1, T2> AstBuilder for InterpolatedBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenI>,
    T1: AstBuilder<T = Value>,
    T2: AstBuilder<T = CloseI>,
{
    type T = Interpolated;
    fn build(self, builder: &mut Cache) -> Interpolated {
        let green = AstBuilder::build_green(self, builder);
        Interpolated::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.open_i.map(|it| it.build_green(builder)).into_iter())
            .chain(self.value.map(|it| it.build_green(builder)).into_iter())
            .chain(self.close_i.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::Interpolated, children)
    }
}
impl<T0, T1, T2> IntoBuilder<InnerStringPart> for InterpolatedBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = OpenI>,
    T1: AstBuilder<T = Value>,
    T2: AstBuilder<T = CloseI>,
{
    fn into_builder(self) -> AliasBuilder<Self, InnerStringPart> {
        AliasBuilder::new(Nodes::InnerStringPart, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StruktPair(pub(crate) Red);
impl Ast for StruktPair {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::StruktPair) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl StruktPair {
    pub fn key(&self) -> Option<Key> {
        self.0.children().filter_map(Key::new).nth(0usize)
    }
    pub fn op_assign_token(&self) -> Option<OpAssign> {
        self.0.children().filter_map(OpAssign::new).nth(0usize)
    }
    pub fn value(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).nth(0usize)
    }
    pub fn build<T0, T1, T2>() -> StruktPairBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = Key>,
        T1: AstBuilder<T = OpAssign>,
        T2: AstBuilder<T = Value>,
    {
        Default::default()
    }
}
pub struct StruktPairBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Key>,
    T1: AstBuilder<T = OpAssign>,
    T2: AstBuilder<T = Value>,
{
    key: Option<T0>,
    op_assign: Option<T1>,
    value: Option<T2>,
}
impl<T0, T1, T2> Default for StruktPairBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Key>,
    T1: AstBuilder<T = OpAssign>,
    T2: AstBuilder<T = Value>,
{
    fn default() -> Self {
        Self {
            key: Default::default(),
            op_assign: Default::default(),
            value: Default::default(),
        }
    }
}
impl<T0, T1, T2> StruktPairBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Key>,
    T1: AstBuilder<T = OpAssign>,
    T2: AstBuilder<T = Value>,
{
    pub fn fill(self, key: T0, op_assign: T1, value: T2) -> Self {
        Self {
            key: Some(key),
            op_assign: Some(op_assign),
            value: Some(value),
        }
    }
}
impl<T0, T1, T2> AstBuilder for StruktPairBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Key>,
    T1: AstBuilder<T = OpAssign>,
    T2: AstBuilder<T = Value>,
{
    type T = StruktPair;
    fn build(self, builder: &mut Cache) -> StruktPair {
        let green = AstBuilder::build_green(self, builder);
        StruktPair::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.key.map(|it| it.build_green(builder)).into_iter())
            .chain(self.op_assign.map(|it| it.build_green(builder)).into_iter())
            .chain(self.value.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::StruktPair, children)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Key(pub(crate) Red);
impl Ast for Key {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Key) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Key {
    pub fn identifier_token(&self) -> Option<Identifier> {
        self.0.children().filter_map(Identifier::new).nth(0usize)
    }
    pub fn build<T0>() -> KeyBuilder<T0>
    where
        T0: AstBuilder<T = Identifier>,
    {
        Default::default()
    }
}
pub struct KeyBuilder<T0>
where
    T0: AstBuilder<T = Identifier>,
{
    identifier: Option<T0>,
}
impl<T0> Default for KeyBuilder<T0>
where
    T0: AstBuilder<T = Identifier>,
{
    fn default() -> Self {
        Self {
            identifier: Default::default(),
        }
    }
}
impl<T0> KeyBuilder<T0>
where
    T0: AstBuilder<T = Identifier>,
{
    pub fn fill(self, identifier: T0) -> Self {
        Self {
            identifier: Some(identifier),
        }
    }
}
impl<T0> AstBuilder for KeyBuilder<T0>
where
    T0: AstBuilder<T = Identifier>,
{
    type T = Key;
    fn build(self, builder: &mut Cache) -> Key {
        let green = AstBuilder::build_green(self, builder);
        Key::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(
                self.identifier
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .collect();
        builder.node(Nodes::Key, children)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MainArticle(pub(crate) Red);
impl Ast for MainArticle {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::MainArticle) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl MainArticle {
    pub fn article_header(&self) -> Option<ArticleHeader> {
        self.0.children().filter_map(ArticleHeader::new).nth(0usize)
    }
    pub fn article_body(&self) -> Option<ArticleBody> {
        self.0.children().filter_map(ArticleBody::new).nth(0usize)
    }
    pub fn build<T0, T1>() -> MainArticleBuilder<T0, T1>
    where
        T0: AstBuilder<T = ArticleHeader>,
        T1: AstBuilder<T = ArticleBody>,
    {
        Default::default()
    }
}
pub struct MainArticleBuilder<T0, T1>
where
    T0: AstBuilder<T = ArticleHeader>,
    T1: AstBuilder<T = ArticleBody>,
{
    article_header: Option<T0>,
    article_body: Option<T1>,
}
impl<T0, T1> Default for MainArticleBuilder<T0, T1>
where
    T0: AstBuilder<T = ArticleHeader>,
    T1: AstBuilder<T = ArticleBody>,
{
    fn default() -> Self {
        Self {
            article_header: Default::default(),
            article_body: Default::default(),
        }
    }
}
impl<T0, T1> MainArticleBuilder<T0, T1>
where
    T0: AstBuilder<T = ArticleHeader>,
    T1: AstBuilder<T = ArticleBody>,
{
    pub fn fill(self, article_header: T0, article_body: T1) -> Self {
        Self {
            article_header: Some(article_header),
            article_body: Some(article_body),
        }
    }
}
impl<T0, T1> AstBuilder for MainArticleBuilder<T0, T1>
where
    T0: AstBuilder<T = ArticleHeader>,
    T1: AstBuilder<T = ArticleBody>,
{
    type T = MainArticle;
    fn build(self, builder: &mut Cache) -> MainArticle {
        let green = AstBuilder::build_green(self, builder);
        MainArticle::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(
                self.article_header
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(
                self.article_body
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .collect();
        builder.node(Nodes::MainArticle, children)
    }
}
impl<T0, T1> IntoBuilder<ArticleItem> for MainArticleBuilder<T0, T1>
where
    T0: AstBuilder<T = ArticleHeader>,
    T1: AstBuilder<T = ArticleBody>,
{
    fn into_builder(self) -> AliasBuilder<Self, ArticleItem> {
        AliasBuilder::new(Nodes::ArticleItem, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubArticle(pub(crate) Red);
impl Ast for SubArticle {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::SubArticle) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl SubArticle {
    pub fn sub_article_header(&self) -> Option<SubArticleHeader> {
        self.0
            .children()
            .filter_map(SubArticleHeader::new)
            .nth(0usize)
    }
    pub fn article_body(&self) -> Option<ArticleBody> {
        self.0.children().filter_map(ArticleBody::new).nth(0usize)
    }
    pub fn plus_plus_end_token(&self) -> Option<PlusPlusEnd> {
        self.0.children().filter_map(PlusPlusEnd::new).nth(0usize)
    }
    pub fn build<T0, T1, T2>() -> SubArticleBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = SubArticleHeader>,
        T1: AstBuilder<T = ArticleBody>,
        T2: AstBuilder<T = PlusPlusEnd>,
    {
        Default::default()
    }
}
pub struct SubArticleBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = SubArticleHeader>,
    T1: AstBuilder<T = ArticleBody>,
    T2: AstBuilder<T = PlusPlusEnd>,
{
    sub_article_header: Option<T0>,
    article_body: Option<T1>,
    plus_plus_end: Option<T2>,
}
impl<T0, T1, T2> Default for SubArticleBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = SubArticleHeader>,
    T1: AstBuilder<T = ArticleBody>,
    T2: AstBuilder<T = PlusPlusEnd>,
{
    fn default() -> Self {
        Self {
            sub_article_header: Default::default(),
            article_body: Default::default(),
            plus_plus_end: Default::default(),
        }
    }
}
impl<T0, T1, T2> SubArticleBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = SubArticleHeader>,
    T1: AstBuilder<T = ArticleBody>,
    T2: AstBuilder<T = PlusPlusEnd>,
{
    pub fn fill(self, sub_article_header: T0, article_body: T1, plus_plus_end: T2) -> Self {
        Self {
            sub_article_header: Some(sub_article_header),
            article_body: Some(article_body),
            plus_plus_end: Some(plus_plus_end),
        }
    }
}
impl<T0, T1, T2> AstBuilder for SubArticleBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = SubArticleHeader>,
    T1: AstBuilder<T = ArticleBody>,
    T2: AstBuilder<T = PlusPlusEnd>,
{
    type T = SubArticle;
    fn build(self, builder: &mut Cache) -> SubArticle {
        let green = AstBuilder::build_green(self, builder);
        SubArticle::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(
                self.sub_article_header
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(
                self.article_body
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(
                self.plus_plus_end
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .collect();
        builder.node(Nodes::SubArticle, children)
    }
}
impl<T0, T1, T2> IntoBuilder<ArticleItem> for SubArticleBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = SubArticleHeader>,
    T1: AstBuilder<T = ArticleBody>,
    T2: AstBuilder<T = PlusPlusEnd>,
{
    fn into_builder(self) -> AliasBuilder<Self, ArticleItem> {
        AliasBuilder::new(Nodes::ArticleItem, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArticleHeader(pub(crate) Red);
impl Ast for ArticleHeader {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleHeader) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ArticleHeader {
    pub fn start_token(&self) -> Option<ThreePlus> {
        self.0.children().filter_map(ThreePlus::new).nth(0usize)
    }
    pub fn item_ident_token(&self) -> Option<ItemIdent> {
        self.0.children().filter_map(ItemIdent::new).nth(0usize)
    }
    pub fn op_colon_token(&self) -> Option<OpColon> {
        self.0.children().filter_map(OpColon::new).nth(0usize)
    }
    pub fn article_item_id_token(&self) -> Option<ArticleItemId> {
        self.0.children().filter_map(ArticleItemId::new).nth(0usize)
    }
    pub fn separator_token(&self) -> Option<ThreePlus> {
        self.0.children().filter_map(ThreePlus::new).nth(1usize)
    }
    pub fn line_ending_token(&self) -> Option<LineEnding> {
        self.0.children().filter_map(LineEnding::new).nth(0usize)
    }
    pub fn article_header_values(&self) -> Option<ArticleHeaderValues> {
        self.0
            .children()
            .filter_map(ArticleHeaderValues::new)
            .nth(0usize)
    }
    pub fn build<T0, T1, T2, T3, T4, T5, T6>() -> ArticleHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
    where
        T0: AstBuilder<T = ThreePlus>,
        T1: AstBuilder<T = ItemIdent>,
        T2: AstBuilder<T = OpColon>,
        T3: AstBuilder<T = ArticleItemId>,
        T4: AstBuilder<T = ThreePlus>,
        T5: AstBuilder<T = LineEnding>,
        T6: AstBuilder<T = ArticleHeaderValues>,
    {
        Default::default()
    }
}
pub struct ArticleHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleHeaderValues>,
{
    start: Option<T0>,
    item_ident: Option<T1>,
    op_colon: Option<T2>,
    article_item_id: Option<T3>,
    separator: Option<T4>,
    line_ending: Option<T5>,
    article_header_values: Option<T6>,
}
impl<T0, T1, T2, T3, T4, T5, T6> Default for ArticleHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleHeaderValues>,
{
    fn default() -> Self {
        Self {
            start: Default::default(),
            item_ident: Default::default(),
            op_colon: Default::default(),
            article_item_id: Default::default(),
            separator: Default::default(),
            line_ending: Default::default(),
            article_header_values: Default::default(),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> ArticleHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleHeaderValues>,
{
    pub fn fill(
        self,
        start: T0,
        item_ident: T1,
        op_colon: T2,
        article_item_id: T3,
        separator: T4,
        line_ending: T5,
        article_header_values: T6,
    ) -> Self {
        Self {
            start: Some(start),
            item_ident: Some(item_ident),
            op_colon: Some(op_colon),
            article_item_id: Some(article_item_id),
            separator: Some(separator),
            line_ending: Some(line_ending),
            article_header_values: Some(article_header_values),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> AstBuilder for ArticleHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleHeaderValues>,
{
    type T = ArticleHeader;
    fn build(self, builder: &mut Cache) -> ArticleHeader {
        let green = AstBuilder::build_green(self, builder);
        ArticleHeader::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.start.map(|it| it.build_green(builder)).into_iter())
            .chain(
                self.item_ident
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(self.op_colon.map(|it| it.build_green(builder)).into_iter())
            .chain(
                self.article_item_id
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(self.separator.map(|it| it.build_green(builder)).into_iter())
            .chain(
                self.line_ending
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(
                self.article_header_values
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .collect();
        builder.node(Nodes::ArticleHeader, children)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArticleBody(pub(crate) Red);
impl Ast for ArticleBody {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleBody) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ArticleBody {
    pub fn three_plus_token(&self) -> Option<ThreePlus> {
        self.0.children().filter_map(ThreePlus::new).nth(0usize)
    }
    pub fn line_ending_token(&self) -> Option<LineEnding> {
        self.0.children().filter_map(LineEnding::new).nth(0usize)
    }
    pub fn items(&self) -> impl Iterator<Item = ArticleBodyItem> + '_ {
        self.0.children().filter_map(ArticleBodyItem::new)
    }
    pub fn build<T0, T1>() -> ArticleBodyBuilder<T0, T1>
    where
        T0: AstBuilder<T = ThreePlus>,
        T1: AstBuilder<T = LineEnding>,
    {
        Default::default()
    }
}
pub struct ArticleBodyBuilder<T0, T1>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = LineEnding>,
{
    three_plus: Option<T0>,
    line_ending: Option<T1>,
    items: Vec<Box<dyn AstBuilder<T = ArticleBodyItem>>>,
}
impl<T0, T1> Default for ArticleBodyBuilder<T0, T1>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = LineEnding>,
{
    fn default() -> Self {
        Self {
            three_plus: Default::default(),
            line_ending: Default::default(),
            items: Default::default(),
        }
    }
}
impl<T0, T1> ArticleBodyBuilder<T0, T1>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = LineEnding>,
{
    pub fn fill(
        self,
        three_plus: T0,
        line_ending: T1,
        items: Vec<Box<dyn AstBuilder<T = ArticleBodyItem>>>,
    ) -> Self {
        Self {
            three_plus: Some(three_plus),
            line_ending: Some(line_ending),
            items,
        }
    }
}
impl<T0, T1> AstBuilder for ArticleBodyBuilder<T0, T1>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = LineEnding>,
{
    type T = ArticleBody;
    fn build(self, builder: &mut Cache) -> ArticleBody {
        let green = AstBuilder::build_green(self, builder);
        ArticleBody::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(
                self.three_plus
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(
                self.line_ending
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain({
                self.items
                    .into_iter()
                    .map(|it| it.build_boxed_green(builder))
                    .collect::<Vec<_>>()
            })
            .collect();
        builder.node(Nodes::ArticleBody, children)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubArticleHeader(pub(crate) Red);
impl Ast for SubArticleHeader {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::SubArticleHeader) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl SubArticleHeader {
    pub fn plus_plus_token(&self) -> Option<PlusPlus> {
        self.0.children().filter_map(PlusPlus::new).nth(0usize)
    }
    pub fn item_ident_token(&self) -> Option<ItemIdent> {
        self.0.children().filter_map(ItemIdent::new).nth(0usize)
    }
    pub fn op_colon_token(&self) -> Option<OpColon> {
        self.0.children().filter_map(OpColon::new).nth(0usize)
    }
    pub fn article_item_id_token(&self) -> Option<ArticleItemId> {
        self.0.children().filter_map(ArticleItemId::new).nth(0usize)
    }
    pub fn three_plus_token(&self) -> Option<ThreePlus> {
        self.0.children().filter_map(ThreePlus::new).nth(0usize)
    }
    pub fn line_ending_token(&self) -> Option<LineEnding> {
        self.0.children().filter_map(LineEnding::new).nth(0usize)
    }
    pub fn article_header_values(&self) -> Option<ArticleHeaderValues> {
        self.0
            .children()
            .filter_map(ArticleHeaderValues::new)
            .nth(0usize)
    }
    pub fn build<T0, T1, T2, T3, T4, T5, T6>() -> SubArticleHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
    where
        T0: AstBuilder<T = PlusPlus>,
        T1: AstBuilder<T = ItemIdent>,
        T2: AstBuilder<T = OpColon>,
        T3: AstBuilder<T = ArticleItemId>,
        T4: AstBuilder<T = ThreePlus>,
        T5: AstBuilder<T = LineEnding>,
        T6: AstBuilder<T = ArticleHeaderValues>,
    {
        Default::default()
    }
}
pub struct SubArticleHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = PlusPlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleHeaderValues>,
{
    plus_plus: Option<T0>,
    item_ident: Option<T1>,
    op_colon: Option<T2>,
    article_item_id: Option<T3>,
    three_plus: Option<T4>,
    line_ending: Option<T5>,
    article_header_values: Option<T6>,
}
impl<T0, T1, T2, T3, T4, T5, T6> Default for SubArticleHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = PlusPlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleHeaderValues>,
{
    fn default() -> Self {
        Self {
            plus_plus: Default::default(),
            item_ident: Default::default(),
            op_colon: Default::default(),
            article_item_id: Default::default(),
            three_plus: Default::default(),
            line_ending: Default::default(),
            article_header_values: Default::default(),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> SubArticleHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = PlusPlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleHeaderValues>,
{
    pub fn fill(
        self,
        plus_plus: T0,
        item_ident: T1,
        op_colon: T2,
        article_item_id: T3,
        three_plus: T4,
        line_ending: T5,
        article_header_values: T6,
    ) -> Self {
        Self {
            plus_plus: Some(plus_plus),
            item_ident: Some(item_ident),
            op_colon: Some(op_colon),
            article_item_id: Some(article_item_id),
            three_plus: Some(three_plus),
            line_ending: Some(line_ending),
            article_header_values: Some(article_header_values),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> AstBuilder for SubArticleHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = PlusPlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleHeaderValues>,
{
    type T = SubArticleHeader;
    fn build(self, builder: &mut Cache) -> SubArticleHeader {
        let green = AstBuilder::build_green(self, builder);
        SubArticleHeader::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.plus_plus.map(|it| it.build_green(builder)).into_iter())
            .chain(
                self.item_ident
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(self.op_colon.map(|it| it.build_green(builder)).into_iter())
            .chain(
                self.article_item_id
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(
                self.three_plus
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(
                self.line_ending
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(
                self.article_header_values
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .collect();
        builder.node(Nodes::SubArticleHeader, children)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArticleHeaderValues(pub(crate) Red);
impl Ast for ArticleHeaderValues {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleHeaderValues) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ArticleHeaderValues {
    pub fn values(&self) -> impl Iterator<Item = StruktPair> + '_ {
        self.0.children().filter_map(StruktPair::new)
    }
    pub fn build<T0>() -> ArticleHeaderValuesBuilder<T0>
    where
        T0: AstBuilder<T = LineEnding>,
    {
        Default::default()
    }
}
pub struct ArticleHeaderValuesBuilder<T0>
where
    T0: AstBuilder<T = LineEnding>,
{
    values: Vec<Box<dyn AstBuilder<T = StruktPair>>>,
    line_ending: Option<T0>,
}
impl<T0> Default for ArticleHeaderValuesBuilder<T0>
where
    T0: AstBuilder<T = LineEnding>,
{
    fn default() -> Self {
        Self {
            values: Default::default(),
            line_ending: Default::default(),
        }
    }
}
impl<T0> ArticleHeaderValuesBuilder<T0>
where
    T0: AstBuilder<T = LineEnding>,
{
    pub fn fill(self, values: Vec<Box<dyn AstBuilder<T = StruktPair>>>, line_ending: T0) -> Self {
        Self {
            values,
            line_ending: Some(line_ending),
        }
    }
}
impl<T0> AstBuilder for ArticleHeaderValuesBuilder<T0>
where
    T0: AstBuilder<T = LineEnding>,
{
    type T = ArticleHeaderValues;
    fn build(self, builder: &mut Cache) -> ArticleHeaderValues {
        let green = AstBuilder::build_green(self, builder);
        ArticleHeaderValues::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain({
                let delit = self.line_ending.map(|it| it.build_green(builder));
                self.values
                    .into_iter()
                    .flat_map(|it| {
                        Some(it.build_boxed_green(builder))
                            .into_iter()
                            .chain(delit.clone().into_iter())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        builder.node(Nodes::ArticleHeaderValues, children)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArticleRef(pub(crate) Red);
impl Ast for ArticleRef {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleRef) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ArticleRef {
    pub fn open_bl_token(&self) -> Option<OpenBl> {
        self.0.children().filter_map(OpenBl::new).nth(0usize)
    }
    pub fn item_ident_token(&self) -> Option<ItemIdent> {
        self.0.children().filter_map(ItemIdent::new).nth(0usize)
    }
    pub fn op_colon_token(&self) -> Option<OpColon> {
        self.0.children().filter_map(OpColon::new).nth(0usize)
    }
    pub fn article_item_id_token(&self) -> Option<ArticleItemId> {
        self.0.children().filter_map(ArticleItemId::new).nth(0usize)
    }
    pub fn close_bl_token(&self) -> Option<CloseBl> {
        self.0.children().filter_map(CloseBl::new).nth(0usize)
    }
    pub fn build<T0, T1, T2, T3, T4>() -> ArticleRefBuilder<T0, T1, T2, T3, T4>
    where
        T0: AstBuilder<T = OpenBl>,
        T1: AstBuilder<T = ItemIdent>,
        T2: AstBuilder<T = OpColon>,
        T3: AstBuilder<T = ArticleItemId>,
        T4: AstBuilder<T = CloseBl>,
    {
        Default::default()
    }
}
pub struct ArticleRefBuilder<T0, T1, T2, T3, T4>
where
    T0: AstBuilder<T = OpenBl>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = CloseBl>,
{
    open_bl: Option<T0>,
    item_ident: Option<T1>,
    op_colon: Option<T2>,
    article_item_id: Option<T3>,
    close_bl: Option<T4>,
}
impl<T0, T1, T2, T3, T4> Default for ArticleRefBuilder<T0, T1, T2, T3, T4>
where
    T0: AstBuilder<T = OpenBl>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = CloseBl>,
{
    fn default() -> Self {
        Self {
            open_bl: Default::default(),
            item_ident: Default::default(),
            op_colon: Default::default(),
            article_item_id: Default::default(),
            close_bl: Default::default(),
        }
    }
}
impl<T0, T1, T2, T3, T4> ArticleRefBuilder<T0, T1, T2, T3, T4>
where
    T0: AstBuilder<T = OpenBl>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = CloseBl>,
{
    pub fn fill(
        self,
        open_bl: T0,
        item_ident: T1,
        op_colon: T2,
        article_item_id: T3,
        close_bl: T4,
    ) -> Self {
        Self {
            open_bl: Some(open_bl),
            item_ident: Some(item_ident),
            op_colon: Some(op_colon),
            article_item_id: Some(article_item_id),
            close_bl: Some(close_bl),
        }
    }
}
impl<T0, T1, T2, T3, T4> AstBuilder for ArticleRefBuilder<T0, T1, T2, T3, T4>
where
    T0: AstBuilder<T = OpenBl>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = CloseBl>,
{
    type T = ArticleRef;
    fn build(self, builder: &mut Cache) -> ArticleRef {
        let green = AstBuilder::build_green(self, builder);
        ArticleRef::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.open_bl.map(|it| it.build_green(builder)).into_iter())
            .chain(
                self.item_ident
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(self.op_colon.map(|it| it.build_green(builder)).into_iter())
            .chain(
                self.article_item_id
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(self.close_bl.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::ArticleRef, children)
    }
}
impl<T0, T1, T2, T3, T4> IntoBuilder<ArticleBodyItem> for ArticleRefBuilder<T0, T1, T2, T3, T4>
where
    T0: AstBuilder<T = OpenBl>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = CloseBl>,
{
    fn into_builder(self) -> AliasBuilder<Self, ArticleBodyItem> {
        AliasBuilder::new(Nodes::ArticleBodyItem, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MdLink(pub(crate) Red);
impl Ast for MdLink {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::MdLink) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl MdLink {
    pub fn values(&self) -> impl Iterator<Item = MdValue> + '_ {
        self.0.children().filter_map(MdValue::new)
    }
    pub fn md_link_url_token(&self) -> Option<MdLinkUrl> {
        self.0.children().filter_map(MdLinkUrl::new).nth(0usize)
    }
    pub fn md_link_title_token(&self) -> Option<MdLinkTitle> {
        self.0.children().filter_map(MdLinkTitle::new).nth(0usize)
    }
    pub fn build<T1, T2>() -> MdLinkBuilder<T1, T2>
    where
        T1: AstBuilder<T = MdLinkUrl>,
        T2: AstBuilder<T = MdLinkTitle>,
    {
        Default::default()
    }
}
pub struct MdLinkBuilder<T1, T2>
where
    T1: AstBuilder<T = MdLinkUrl>,
    T2: AstBuilder<T = MdLinkTitle>,
{
    values: Vec<Box<dyn AstBuilder<T = MdValue>>>,
    md_link_url: Option<T1>,
    md_link_title: Option<T2>,
}
impl<T1, T2> Default for MdLinkBuilder<T1, T2>
where
    T1: AstBuilder<T = MdLinkUrl>,
    T2: AstBuilder<T = MdLinkTitle>,
{
    fn default() -> Self {
        Self {
            values: Default::default(),
            md_link_url: Default::default(),
            md_link_title: Default::default(),
        }
    }
}
impl<T1, T2> MdLinkBuilder<T1, T2>
where
    T1: AstBuilder<T = MdLinkUrl>,
    T2: AstBuilder<T = MdLinkTitle>,
{
    pub fn fill(
        self,
        values: Vec<Box<dyn AstBuilder<T = MdValue>>>,
        md_link_url: T1,
        md_link_title: T2,
    ) -> Self {
        Self {
            values,
            md_link_url: Some(md_link_url),
            md_link_title: Some(md_link_title),
        }
    }
}
impl<T1, T2> AstBuilder for MdLinkBuilder<T1, T2>
where
    T1: AstBuilder<T = MdLinkUrl>,
    T2: AstBuilder<T = MdLinkTitle>,
{
    type T = MdLink;
    fn build(self, builder: &mut Cache) -> MdLink {
        let green = AstBuilder::build_green(self, builder);
        MdLink::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain({
                self.values
                    .into_iter()
                    .map(|it| it.build_boxed_green(builder))
                    .collect::<Vec<_>>()
            })
            .chain(
                self.md_link_url
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(
                self.md_link_title
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .collect();
        builder.node(Nodes::MdLink, children)
    }
}
impl<T1, T2> IntoBuilder<MdValue> for MdLinkBuilder<T1, T2>
where
    T1: AstBuilder<T = MdLinkUrl>,
    T2: AstBuilder<T = MdLinkTitle>,
{
    fn into_builder(self) -> AliasBuilder<Self, MdValue> {
        AliasBuilder::new(Nodes::MdValue, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MdCodeBlock(pub(crate) Red);
impl Ast for MdCodeBlock {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::MdCodeBlock) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl MdCodeBlock {
    pub fn start_token(&self) -> Option<Fences> {
        self.0.children().filter_map(Fences::new).nth(0usize)
    }
    pub fn md_code_block_lang_token(&self) -> Option<MdCodeBlockLang> {
        self.0
            .children()
            .filter_map(MdCodeBlockLang::new)
            .nth(0usize)
    }
    pub fn end_token(&self) -> Option<Fences> {
        self.0.children().filter_map(Fences::new).nth(1usize)
    }
    pub fn build<T0, T1, T2>() -> MdCodeBlockBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = Fences>,
        T1: AstBuilder<T = MdCodeBlockLang>,
        T2: AstBuilder<T = Fences>,
    {
        Default::default()
    }
}
pub struct MdCodeBlockBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Fences>,
    T1: AstBuilder<T = MdCodeBlockLang>,
    T2: AstBuilder<T = Fences>,
{
    start: Option<T0>,
    md_code_block_lang: Option<T1>,
    end: Option<T2>,
}
impl<T0, T1, T2> Default for MdCodeBlockBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Fences>,
    T1: AstBuilder<T = MdCodeBlockLang>,
    T2: AstBuilder<T = Fences>,
{
    fn default() -> Self {
        Self {
            start: Default::default(),
            md_code_block_lang: Default::default(),
            end: Default::default(),
        }
    }
}
impl<T0, T1, T2> MdCodeBlockBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Fences>,
    T1: AstBuilder<T = MdCodeBlockLang>,
    T2: AstBuilder<T = Fences>,
{
    pub fn fill(self, start: T0, md_code_block_lang: T1, end: T2) -> Self {
        Self {
            start: Some(start),
            md_code_block_lang: Some(md_code_block_lang),
            end: Some(end),
        }
    }
}
impl<T0, T1, T2> AstBuilder for MdCodeBlockBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Fences>,
    T1: AstBuilder<T = MdCodeBlockLang>,
    T2: AstBuilder<T = Fences>,
{
    type T = MdCodeBlock;
    fn build(self, builder: &mut Cache) -> MdCodeBlock {
        let green = AstBuilder::build_green(self, builder);
        MdCodeBlock::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.start.map(|it| it.build_green(builder)).into_iter())
            .chain(
                self.md_code_block_lang
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(self.end.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::MdCodeBlock, children)
    }
}
impl<T0, T1, T2> IntoBuilder<MdValue> for MdCodeBlockBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Fences>,
    T1: AstBuilder<T = MdCodeBlockLang>,
    T2: AstBuilder<T = Fences>,
{
    fn into_builder(self) -> AliasBuilder<Self, MdValue> {
        AliasBuilder::new(Nodes::MdValue, self)
    }
}
