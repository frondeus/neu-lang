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
    pub const Literal: Name = Name::new("Literal");
    pub const Boolean: Name = Name::new("Boolean");
    pub const UnaryOp: Name = Name::new("UnaryOp");
    pub const BinaryOp: Name = Name::new("BinaryOp");
    pub const InnerStringPart: Name = Name::new("InnerStringPart");
    pub const ArticleItem: Name = Name::new("ArticleItem");
    pub const ArticleBodyItem: Name = Name::new("ArticleBodyItem");
    pub const MdValue: Name = Name::new("MdValue");
    pub const Value: Name = Name::new("Value");
    pub const Binary: Name = Name::new("Binary");
    pub const IdentPath: Name = Name::new("IdentPath");
    pub const Unary: Name = Name::new("Unary");
    pub const MdString: Name = Name::new("MdString");
    pub const String: Name = Name::new("String");
    pub const Strukt: Name = Name::new("Strukt");
    pub const Array: Name = Name::new("Array");
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpBang(Red);
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
pub struct OpenI(Red);
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
pub struct OpenP(Red);
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
pub struct CloseP(Red);
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
pub struct OpStar(Red);
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
pub struct OpPlus(Red);
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
pub struct PlusPlus(Red);
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
pub struct PlusPlusEnd(Red);
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
pub struct ThreePlus(Red);
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
pub struct CloseBl(Red);
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
pub struct Comma(Red);
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
pub struct OpMinus(Red);
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
pub struct OpDot(Red);
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
pub struct OpSlash(Red);
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
pub struct OpColon(Red);
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
pub struct OpAssign(Red);
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
pub struct OpDEqual(Red);
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
pub struct OpenB(Red);
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
pub struct OpenBl(Red);
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
pub struct CloseB(Red);
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
pub struct CloseI(Red);
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
pub struct DQuote(Red);
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
pub struct False(Red);
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
pub struct ItemIdent(Red);
impl Ast for ItemIdent {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "item_identifier" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ItemIdent {
    pub fn build() -> TokenBuilder<ItemIdent> {
        TokenBuilder::new("item_identifier")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineEnding(Red);
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
pub struct MdQuote(Red);
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
pub struct Number(Red);
impl Ast for Number {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "number" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Number {
    pub fn build() -> TokenBuilder<Number> {
        TokenBuilder::new("number")
    }
}
impl IntoBuilder<Literal> for TokenBuilder<Number> {
    fn into_builder(self) -> AliasBuilder<Self, Literal> {
        AliasBuilder::new(Nodes::Literal, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Text(Red);
impl Ast for Text {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "text" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Text {
    pub fn build() -> TokenBuilder<Text> {
        TokenBuilder::new("text")
    }
}
impl IntoBuilder<InnerStringPart> for TokenBuilder<Text> {
    fn into_builder(self) -> AliasBuilder<Self, InnerStringPart> {
        AliasBuilder::new(Nodes::InnerStringPart, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct True(Red);
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
pub struct OpenC(Red);
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
pub struct CloseC(Red);
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
pub enum Literal {
    Number(Number),
    Boolean(Boolean),
}
impl From<Number> for Literal {
    fn from(val: Number) -> Self {
        Self::Number(val)
    }
}
impl From<Boolean> for Literal {
    fn from(val: Boolean) -> Self {
        Self::Boolean(val)
    }
}
impl Ast for Literal {
    fn new(node: Red) -> Option<Self> {
        None.or_else(|| Number::new(node.clone()).map(Literal::Number))
            .or_else(|| Boolean::new(node.clone()).map(Literal::Boolean))
    }
    fn red(&self) -> Red {
        match &self {
            Literal::Number(node) => node.red(),
            Literal::Boolean(node) => node.red(),
        }
    }
}
impl Literal {
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
    OpPlus(OpPlus),
    OpSlash(OpSlash),
    OpStar(OpStar),
    OpDEqual(OpDEqual),
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
        None.or_else(|| OpPlus::new(node.clone()).map(BinaryOp::OpPlus))
            .or_else(|| OpSlash::new(node.clone()).map(BinaryOp::OpSlash))
            .or_else(|| OpStar::new(node.clone()).map(BinaryOp::OpStar))
            .or_else(|| OpDEqual::new(node.clone()).map(BinaryOp::OpDEqual))
    }
    fn red(&self) -> Red {
        match &self {
            BinaryOp::OpPlus(node) => node.red(),
            BinaryOp::OpSlash(node) => node.red(),
            BinaryOp::OpStar(node) => node.red(),
            BinaryOp::OpDEqual(node) => node.red(),
        }
    }
}
impl BinaryOp {
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
    Text(Text),
}
impl From<MdLink> for MdValue {
    fn from(val: MdLink) -> Self {
        Self::MdLink(val)
    }
}
impl From<Text> for MdValue {
    fn from(val: Text) -> Self {
        Self::Text(val)
    }
}
impl Ast for MdValue {
    fn new(node: Red) -> Option<Self> {
        None.or_else(|| MdLink::new(node.clone()).map(MdValue::MdLink))
            .or_else(|| Text::new(node.clone()).map(MdValue::Text))
    }
    fn red(&self) -> Red {
        match &self {
            MdValue::MdLink(node) => node.red(),
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
    pub fn as_text(self) -> Option<Text> {
        match self {
            Self::Text(val) => Some(val),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Value(Red);
impl Ast for Value {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Value) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Value {
    pub fn literal(&self) -> Option<Literal> {
        self.0.children().filter_map(Literal::new).next()
    }
    pub fn binary(&self) -> Option<Binary> {
        self.0.children().filter_map(Binary::new).next()
    }
    pub fn ident_path(&self) -> Option<IdentPath> {
        self.0.children().filter_map(IdentPath::new).next()
    }
    pub fn unary(&self) -> Option<Unary> {
        self.0.children().filter_map(Unary::new).next()
    }
    pub fn md_string(&self) -> Option<MdString> {
        self.0.children().filter_map(MdString::new).next()
    }
    pub fn string(&self) -> Option<String> {
        self.0.children().filter_map(String::new).next()
    }
    pub fn strukt(&self) -> Option<Strukt> {
        self.0.children().filter_map(Strukt::new).next()
    }
    pub fn array(&self) -> Option<Array> {
        self.0.children().filter_map(Array::new).next()
    }
    pub fn identifier_token(&self) -> Option<Identifier> {
        self.0.children().filter_map(Identifier::new).next()
    }
    pub fn open_p_token(&self) -> Option<OpenP> {
        self.0.children().filter_map(OpenP::new).next()
    }
    pub fn value(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).next()
    }
    pub fn close_p_token(&self) -> Option<CloseP> {
        self.0.children().filter_map(CloseP::new).next()
    }
    pub fn build<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>(
    ) -> ValueBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
    where
        T0: AstBuilder<T = Literal>,
        T1: AstBuilder<T = Binary>,
        T2: AstBuilder<T = IdentPath>,
        T3: AstBuilder<T = Unary>,
        T4: AstBuilder<T = MdString>,
        T5: AstBuilder<T = String>,
        T6: AstBuilder<T = Strukt>,
        T7: AstBuilder<T = Array>,
        T8: AstBuilder<T = Identifier>,
        T9: AstBuilder<T = OpenP>,
        T10: AstBuilder<T = Value>,
        T11: AstBuilder<T = CloseP>,
    {
        Default::default()
    }
}
pub struct ValueBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
where
    T0: AstBuilder<T = Literal>,
    T1: AstBuilder<T = Binary>,
    T2: AstBuilder<T = IdentPath>,
    T3: AstBuilder<T = Unary>,
    T4: AstBuilder<T = MdString>,
    T5: AstBuilder<T = String>,
    T6: AstBuilder<T = Strukt>,
    T7: AstBuilder<T = Array>,
    T8: AstBuilder<T = Identifier>,
    T9: AstBuilder<T = OpenP>,
    T10: AstBuilder<T = Value>,
    T11: AstBuilder<T = CloseP>,
{
    literal: Option<T0>,
    binary: Option<T1>,
    ident_path: Option<T2>,
    unary: Option<T3>,
    md_string: Option<T4>,
    string: Option<T5>,
    strukt: Option<T6>,
    array: Option<T7>,
    identifier: Option<T8>,
    open_p: Option<T9>,
    value: Option<T10>,
    close_p: Option<T11>,
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Default
    for ValueBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
where
    T0: AstBuilder<T = Literal>,
    T1: AstBuilder<T = Binary>,
    T2: AstBuilder<T = IdentPath>,
    T3: AstBuilder<T = Unary>,
    T4: AstBuilder<T = MdString>,
    T5: AstBuilder<T = String>,
    T6: AstBuilder<T = Strukt>,
    T7: AstBuilder<T = Array>,
    T8: AstBuilder<T = Identifier>,
    T9: AstBuilder<T = OpenP>,
    T10: AstBuilder<T = Value>,
    T11: AstBuilder<T = CloseP>,
{
    fn default() -> Self {
        Self {
            literal: Default::default(),
            binary: Default::default(),
            ident_path: Default::default(),
            unary: Default::default(),
            md_string: Default::default(),
            string: Default::default(),
            strukt: Default::default(),
            array: Default::default(),
            identifier: Default::default(),
            open_p: Default::default(),
            value: Default::default(),
            close_p: Default::default(),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
    ValueBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
where
    T0: AstBuilder<T = Literal>,
    T1: AstBuilder<T = Binary>,
    T2: AstBuilder<T = IdentPath>,
    T3: AstBuilder<T = Unary>,
    T4: AstBuilder<T = MdString>,
    T5: AstBuilder<T = String>,
    T6: AstBuilder<T = Strukt>,
    T7: AstBuilder<T = Array>,
    T8: AstBuilder<T = Identifier>,
    T9: AstBuilder<T = OpenP>,
    T10: AstBuilder<T = Value>,
    T11: AstBuilder<T = CloseP>,
{
    pub fn fill(
        self,
        literal: T0,
        binary: T1,
        ident_path: T2,
        unary: T3,
        md_string: T4,
        string: T5,
        strukt: T6,
        array: T7,
        identifier: T8,
        open_p: T9,
        value: T10,
        close_p: T11,
    ) -> Self {
        Self {
            literal: Some(literal),
            binary: Some(binary),
            ident_path: Some(ident_path),
            unary: Some(unary),
            md_string: Some(md_string),
            string: Some(string),
            strukt: Some(strukt),
            array: Some(array),
            identifier: Some(identifier),
            open_p: Some(open_p),
            value: Some(value),
            close_p: Some(close_p),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> AstBuilder
    for ValueBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
where
    T0: AstBuilder<T = Literal>,
    T1: AstBuilder<T = Binary>,
    T2: AstBuilder<T = IdentPath>,
    T3: AstBuilder<T = Unary>,
    T4: AstBuilder<T = MdString>,
    T5: AstBuilder<T = String>,
    T6: AstBuilder<T = Strukt>,
    T7: AstBuilder<T = Array>,
    T8: AstBuilder<T = Identifier>,
    T9: AstBuilder<T = OpenP>,
    T10: AstBuilder<T = Value>,
    T11: AstBuilder<T = CloseP>,
{
    type T = Value;
    fn build(self, builder: &mut Cache) -> Value {
        let green = AstBuilder::build_green(self, builder);
        Value::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.literal.map(|it| it.build_green(builder)).into_iter())
            .chain(self.binary.map(|it| it.build_green(builder)).into_iter())
            .chain(
                self.ident_path
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(self.unary.map(|it| it.build_green(builder)).into_iter())
            .chain(self.md_string.map(|it| it.build_green(builder)).into_iter())
            .chain(self.string.map(|it| it.build_green(builder)).into_iter())
            .chain(self.strukt.map(|it| it.build_green(builder)).into_iter())
            .chain(self.array.map(|it| it.build_green(builder)).into_iter())
            .chain(
                self.identifier
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(self.open_p.map(|it| it.build_green(builder)).into_iter())
            .chain(self.value.map(|it| it.build_green(builder)).into_iter())
            .chain(self.close_p.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::Value, children)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Binary(Red);
impl Ast for Binary {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Binary) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Binary {
    pub fn left(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).next()
    }
    pub fn binary_op(&self) -> Option<BinaryOp> {
        self.0.children().filter_map(BinaryOp::new).next()
    }
    pub fn right(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).next()
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentPath(Red);
impl Ast for IdentPath {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::IdentPath) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl IdentPath {
    pub fn left(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).next()
    }
    pub fn op_dot_token(&self) -> Option<OpDot> {
        self.0.children().filter_map(OpDot::new).next()
    }
    pub fn right(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).next()
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unary(Red);
impl Ast for Unary {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Unary) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Unary {
    pub fn unary_op(&self) -> Option<UnaryOp> {
        self.0.children().filter_map(UnaryOp::new).next()
    }
    pub fn value(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).next()
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MdString(Red);
impl Ast for MdString {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::MdString) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl MdString {
    pub fn md_quote_token(&self) -> Option<MdQuote> {
        self.0.children().filter_map(MdQuote::new).next()
    }
    pub fn markdown(&self) -> Option<Markdown> {
        self.0.children().filter_map(Markdown::new).next()
    }
    pub fn dquote_token(&self) -> Option<DQuote> {
        self.0.children().filter_map(DQuote::new).next()
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct String(Red);
impl Ast for String {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::String) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl String {
    pub fn left_quote_token(&self) -> Option<DQuote> {
        self.0.children().filter_map(DQuote::new).next()
    }
    pub fn inner_string(&self) -> Option<InnerString> {
        self.0.children().filter_map(InnerString::new).next()
    }
    pub fn right_quote_token(&self) -> Option<DQuote> {
        self.0.children().filter_map(DQuote::new).next()
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Strukt(Red);
impl Ast for Strukt {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Strukt) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Strukt {
    pub fn open_c_token(&self) -> Option<OpenC> {
        self.0.children().filter_map(OpenC::new).next()
    }
    pub fn pairs(&self) -> impl Iterator<Item = StruktPair> + '_ {
        self.0.children().filter_map(StruktPair::new)
    }
    pub fn close_c_token(&self) -> Option<CloseC> {
        self.0.children().filter_map(CloseC::new).next()
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Array(Red);
impl Ast for Array {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Array) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Array {
    pub fn open_b_token(&self) -> Option<OpenB> {
        self.0.children().filter_map(OpenB::new).next()
    }
    pub fn values(&self) -> impl Iterator<Item = Value> + '_ {
        self.0.children().filter_map(Value::new)
    }
    pub fn close_b_token(&self) -> Option<CloseB> {
        self.0.children().filter_map(CloseB::new).next()
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Markdown(Red);
impl Ast for Markdown {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Markdown) {
            return None;
        }
        node.green().as_node()?;
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
pub struct InnerString(Red);
impl Ast for InnerString {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::InnerString) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl InnerString {
    pub fn inner_string_part(&self) -> impl Iterator<Item = InnerStringPart> + '_ {
        self.0.children().filter_map(InnerStringPart::new)
    }
    pub fn build() -> InnerStringBuilder {
        Default::default()
    }
}
pub struct InnerStringBuilder {
    inner_string_part: Vec<Box<dyn AstBuilder<T = InnerStringPart>>>,
}
impl Default for InnerStringBuilder {
    fn default() -> Self {
        Self {
            inner_string_part: Default::default(),
        }
    }
}
impl InnerStringBuilder {
    pub fn fill(self, inner_string_part: Vec<Box<dyn AstBuilder<T = InnerStringPart>>>) -> Self {
        Self { inner_string_part }
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
                self.inner_string_part
                    .into_iter()
                    .map(|it| it.build_boxed_green(builder))
                    .collect::<Vec<_>>()
            })
            .collect();
        builder.node(Nodes::InnerString, children)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Interpolated(Red);
impl Ast for Interpolated {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Interpolated) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Interpolated {
    pub fn open_i_token(&self) -> Option<OpenI> {
        self.0.children().filter_map(OpenI::new).next()
    }
    pub fn value(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).next()
    }
    pub fn close_i_token(&self) -> Option<CloseI> {
        self.0.children().filter_map(CloseI::new).next()
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
pub struct StruktPair(Red);
impl Ast for StruktPair {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::StruktPair) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl StruktPair {
    pub fn key(&self) -> Option<Key> {
        self.0.children().filter_map(Key::new).next()
    }
    pub fn op_assign_token(&self) -> Option<OpAssign> {
        self.0.children().filter_map(OpAssign::new).next()
    }
    pub fn value(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).next()
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
pub struct Key(Red);
impl Ast for Key {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Key) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Key {
    pub fn identifier_token(&self) -> Option<Identifier> {
        self.0.children().filter_map(Identifier::new).next()
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
pub struct MainArticle(Red);
impl Ast for MainArticle {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::MainArticle) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl MainArticle {
    pub fn article_header(&self) -> Option<ArticleHeader> {
        self.0.children().filter_map(ArticleHeader::new).next()
    }
    pub fn article_body(&self) -> Option<ArticleBody> {
        self.0.children().filter_map(ArticleBody::new).next()
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
pub struct SubArticle(Red);
impl Ast for SubArticle {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::SubArticle) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl SubArticle {
    pub fn sub_article_header(&self) -> Option<SubArticleHeader> {
        self.0.children().filter_map(SubArticleHeader::new).next()
    }
    pub fn article_body(&self) -> Option<ArticleBody> {
        self.0.children().filter_map(ArticleBody::new).next()
    }
    pub fn plus_plus_end_token(&self) -> Option<PlusPlusEnd> {
        self.0.children().filter_map(PlusPlusEnd::new).next()
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
pub struct ArticleHeader(Red);
impl Ast for ArticleHeader {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleHeader) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ArticleHeader {
    pub fn start_token(&self) -> Option<ThreePlus> {
        self.0.children().filter_map(ThreePlus::new).next()
    }
    pub fn item_ident_token(&self) -> Option<ItemIdent> {
        self.0.children().filter_map(ItemIdent::new).next()
    }
    pub fn op_colon_token(&self) -> Option<OpColon> {
        self.0.children().filter_map(OpColon::new).next()
    }
    pub fn article_item_id_token(&self) -> Option<ArticleItemId> {
        self.0.children().filter_map(ArticleItemId::new).next()
    }
    pub fn separator_token(&self) -> Option<ThreePlus> {
        self.0.children().filter_map(ThreePlus::new).next()
    }
    pub fn line_ending_token(&self) -> Option<LineEnding> {
        self.0.children().filter_map(LineEnding::new).next()
    }
    pub fn article_header_values(&self) -> Option<ArticleHeaderValues> {
        self.0
            .children()
            .filter_map(ArticleHeaderValues::new)
            .next()
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
pub struct ArticleBody(Red);
impl Ast for ArticleBody {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleBody) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ArticleBody {
    pub fn three_plus_token(&self) -> Option<ThreePlus> {
        self.0.children().filter_map(ThreePlus::new).next()
    }
    pub fn line_ending_token(&self) -> Option<LineEnding> {
        self.0.children().filter_map(LineEnding::new).next()
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
pub struct SubArticleHeader(Red);
impl Ast for SubArticleHeader {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::SubArticleHeader) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl SubArticleHeader {
    pub fn plus_plus_token(&self) -> Option<PlusPlus> {
        self.0.children().filter_map(PlusPlus::new).next()
    }
    pub fn item_ident_token(&self) -> Option<ItemIdent> {
        self.0.children().filter_map(ItemIdent::new).next()
    }
    pub fn op_colon_token(&self) -> Option<OpColon> {
        self.0.children().filter_map(OpColon::new).next()
    }
    pub fn article_item_id_token(&self) -> Option<ArticleItemId> {
        self.0.children().filter_map(ArticleItemId::new).next()
    }
    pub fn three_plus_token(&self) -> Option<ThreePlus> {
        self.0.children().filter_map(ThreePlus::new).next()
    }
    pub fn line_ending_token(&self) -> Option<LineEnding> {
        self.0.children().filter_map(LineEnding::new).next()
    }
    pub fn article_header_values(&self) -> Option<ArticleHeaderValues> {
        self.0
            .children()
            .filter_map(ArticleHeaderValues::new)
            .next()
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
pub struct ArticleHeaderValues(Red);
impl Ast for ArticleHeaderValues {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleHeaderValues) {
            return None;
        }
        node.green().as_node()?;
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
pub struct ArticleRef(Red);
impl Ast for ArticleRef {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleRef) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ArticleRef {
    pub fn open_bl_token(&self) -> Option<OpenBl> {
        self.0.children().filter_map(OpenBl::new).next()
    }
    pub fn item_ident_token(&self) -> Option<ItemIdent> {
        self.0.children().filter_map(ItemIdent::new).next()
    }
    pub fn op_colon_token(&self) -> Option<OpColon> {
        self.0.children().filter_map(OpColon::new).next()
    }
    pub fn article_item_id_token(&self) -> Option<ArticleItemId> {
        self.0.children().filter_map(ArticleItemId::new).next()
    }
    pub fn close_bl_token(&self) -> Option<CloseBl> {
        self.0.children().filter_map(CloseBl::new).next()
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
pub struct MdLink(Red);
impl Ast for MdLink {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::MdLink) {
            return None;
        }
        node.green().as_node()?;
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
        self.0.children().filter_map(MdLinkUrl::new).next()
    }
    pub fn md_link_title_token(&self) -> Option<MdLinkTitle> {
        self.0.children().filter_map(MdLinkTitle::new).next()
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
