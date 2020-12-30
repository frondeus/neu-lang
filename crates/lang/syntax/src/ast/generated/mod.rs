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
    pub const ArticleBody: Name = Name::new("ArticleBody");
    pub const Value: Name = Name::new("Value");
    pub const Binary: Name = Name::new("Binary");
    pub const IdentPath: Name = Name::new("IdentPath");
    pub const Unary: Name = Name::new("Unary");
    pub const Markdown: Name = Name::new("Markdown");
    pub const String: Name = Name::new("String");
    pub const Strukt: Name = Name::new("Strukt");
    pub const Array: Name = Name::new("Array");
    pub const InnerMarkdown: Name = Name::new("InnerMarkdown");
    pub const Md_Value: Name = Name::new("Md_Value");
    pub const InnerString: Name = Name::new("InnerString");
    pub const Interpolated: Name = Name::new("Interpolated");
    pub const StruktPair: Name = Name::new("StruktPair");
    pub const Key: Name = Name::new("Key");
    pub const MainItem: Name = Name::new("MainItem");
    pub const MainItemHeader: Name = Name::new("MainItemHeader");
    pub const MainItemBody: Name = Name::new("MainItemBody");
    pub const ArticleItemId: Name = Name::new("ArticleItemId");
    pub const ArticleItemValues: Name = Name::new("ArticleItemValues");
    pub const ArticleItem: Name = Name::new("ArticleItem");
    pub const ArticleItemHeader: Name = Name::new("ArticleItemHeader");
    pub const ArticleItemBody: Name = Name::new("ArticleItemBody");
    pub const ArticleRef: Name = Name::new("ArticleRef");
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ItemId(Red);
impl Ast for ItemId {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Token) {
            return None;
        }
        let green = node.green();
        let tok = green.as_token()?;
        if tok.value != "item_id" {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ItemId {
    pub fn build() -> TokenBuilder<ItemId> {
        TokenBuilder::new("item_id")
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ArticleBody {
    Text(Text),
    ArticleItem(ArticleItem),
    ArticleRef(ArticleRef),
}
impl From<Text> for ArticleBody {
    fn from(val: Text) -> Self {
        Self::Text(val)
    }
}
impl From<ArticleItem> for ArticleBody {
    fn from(val: ArticleItem) -> Self {
        Self::ArticleItem(val)
    }
}
impl From<ArticleRef> for ArticleBody {
    fn from(val: ArticleRef) -> Self {
        Self::ArticleRef(val)
    }
}
impl Ast for ArticleBody {
    fn new(node: Red) -> Option<Self> {
        None.or_else(|| Text::new(node.clone()).map(ArticleBody::Text))
            .or_else(|| ArticleItem::new(node.clone()).map(ArticleBody::ArticleItem))
            .or_else(|| ArticleRef::new(node.clone()).map(ArticleBody::ArticleRef))
    }
    fn red(&self) -> Red {
        match &self {
            ArticleBody::Text(node) => node.red(),
            ArticleBody::ArticleItem(node) => node.red(),
            ArticleBody::ArticleRef(node) => node.red(),
        }
    }
}
impl ArticleBody {
    pub fn as_text(self) -> Option<Text> {
        match self {
            Self::Text(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_articleitem(self) -> Option<ArticleItem> {
        match self {
            Self::ArticleItem(val) => Some(val),
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

#[derive(Debug)]
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
    pub fn markdown(&self) -> Option<Markdown> {
        self.0.children().filter_map(Markdown::new).next()
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
        T4: AstBuilder<T = Markdown>,
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
    T4: AstBuilder<T = Markdown>,
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
    markdown: Option<T4>,
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
    T4: AstBuilder<T = Markdown>,
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
            markdown: Default::default(),
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
    T4: AstBuilder<T = Markdown>,
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
        markdown: T4,
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
            markdown: Some(markdown),
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
    T4: AstBuilder<T = Markdown>,
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
            .chain(self.markdown.map(|it| it.build_green(builder)).into_iter())
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
    pub fn md_quote_token(&self) -> Option<MdQuote> {
        self.0.children().filter_map(MdQuote::new).next()
    }
    pub fn inner_markdown(&self) -> Option<InnerMarkdown> {
        self.0.children().filter_map(InnerMarkdown::new).next()
    }
    pub fn dquote_token(&self) -> Option<DQuote> {
        self.0.children().filter_map(DQuote::new).next()
    }
    pub fn build<T0, T1, T2>() -> MarkdownBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = MdQuote>,
        T1: AstBuilder<T = InnerMarkdown>,
        T2: AstBuilder<T = DQuote>,
    {
        Default::default()
    }
}
pub struct MarkdownBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = MdQuote>,
    T1: AstBuilder<T = InnerMarkdown>,
    T2: AstBuilder<T = DQuote>,
{
    md_quote: Option<T0>,
    inner_markdown: Option<T1>,
    dquote: Option<T2>,
}
impl<T0, T1, T2> Default for MarkdownBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = MdQuote>,
    T1: AstBuilder<T = InnerMarkdown>,
    T2: AstBuilder<T = DQuote>,
{
    fn default() -> Self {
        Self {
            md_quote: Default::default(),
            inner_markdown: Default::default(),
            dquote: Default::default(),
        }
    }
}
impl<T0, T1, T2> MarkdownBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = MdQuote>,
    T1: AstBuilder<T = InnerMarkdown>,
    T2: AstBuilder<T = DQuote>,
{
    pub fn fill(self, md_quote: T0, inner_markdown: T1, dquote: T2) -> Self {
        Self {
            md_quote: Some(md_quote),
            inner_markdown: Some(inner_markdown),
            dquote: Some(dquote),
        }
    }
}
impl<T0, T1, T2> AstBuilder for MarkdownBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = MdQuote>,
    T1: AstBuilder<T = InnerMarkdown>,
    T2: AstBuilder<T = DQuote>,
{
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
            .chain(self.md_quote.map(|it| it.build_green(builder)).into_iter())
            .chain(
                self.inner_markdown
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(self.dquote.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::Markdown, children)
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct InnerMarkdown(Red);
impl Ast for InnerMarkdown {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::InnerMarkdown) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl InnerMarkdown {
    pub fn md_value(&self) -> impl Iterator<Item = Md_Value> + '_ {
        self.0.children().filter_map(Md_Value::new)
    }
    pub fn build() -> InnerMarkdownBuilder {
        Default::default()
    }
}
pub struct InnerMarkdownBuilder {
    md_value: Vec<Box<dyn AstBuilder<T = Md_Value>>>,
}
impl Default for InnerMarkdownBuilder {
    fn default() -> Self {
        Self {
            md_value: Default::default(),
        }
    }
}
impl InnerMarkdownBuilder {
    pub fn fill(self, md_value: Vec<Box<dyn AstBuilder<T = Md_Value>>>) -> Self {
        Self { md_value }
    }
}
impl AstBuilder for InnerMarkdownBuilder {
    type T = InnerMarkdown;
    fn build(self, builder: &mut Cache) -> InnerMarkdown {
        let green = AstBuilder::build_green(self, builder);
        InnerMarkdown::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain({
                self.md_value
                    .into_iter()
                    .map(|it| it.build_boxed_green(builder))
                    .collect::<Vec<_>>()
            })
            .collect();
        builder.node(Nodes::InnerMarkdown, children)
    }
}

#[derive(Debug)]
pub struct Md_Value(Red);
impl Ast for Md_Value {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Md_Value) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl Md_Value {
    pub fn text_token(&self) -> Option<Text> {
        self.0.children().filter_map(Text::new).next()
    }
    pub fn build<T0>() -> Md_ValueBuilder<T0>
    where
        T0: AstBuilder<T = Text>,
    {
        Default::default()
    }
}
pub struct Md_ValueBuilder<T0>
where
    T0: AstBuilder<T = Text>,
{
    text: Option<T0>,
}
impl<T0> Default for Md_ValueBuilder<T0>
where
    T0: AstBuilder<T = Text>,
{
    fn default() -> Self {
        Self {
            text: Default::default(),
        }
    }
}
impl<T0> Md_ValueBuilder<T0>
where
    T0: AstBuilder<T = Text>,
{
    pub fn fill(self, text: T0) -> Self {
        Self { text: Some(text) }
    }
}
impl<T0> AstBuilder for Md_ValueBuilder<T0>
where
    T0: AstBuilder<T = Text>,
{
    type T = Md_Value;
    fn build(self, builder: &mut Cache) -> Md_Value {
        let green = AstBuilder::build_green(self, builder);
        Md_Value::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.text.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::Md_Value, children)
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct MainItem(Red);
impl Ast for MainItem {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::MainItem) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl MainItem {
    pub fn main_item_header(&self) -> Option<MainItemHeader> {
        self.0.children().filter_map(MainItemHeader::new).next()
    }
    pub fn main_item_body(&self) -> Option<MainItemBody> {
        self.0.children().filter_map(MainItemBody::new).next()
    }
    pub fn build<T0, T1>() -> MainItemBuilder<T0, T1>
    where
        T0: AstBuilder<T = MainItemHeader>,
        T1: AstBuilder<T = MainItemBody>,
    {
        Default::default()
    }
}
pub struct MainItemBuilder<T0, T1>
where
    T0: AstBuilder<T = MainItemHeader>,
    T1: AstBuilder<T = MainItemBody>,
{
    main_item_header: Option<T0>,
    main_item_body: Option<T1>,
}
impl<T0, T1> Default for MainItemBuilder<T0, T1>
where
    T0: AstBuilder<T = MainItemHeader>,
    T1: AstBuilder<T = MainItemBody>,
{
    fn default() -> Self {
        Self {
            main_item_header: Default::default(),
            main_item_body: Default::default(),
        }
    }
}
impl<T0, T1> MainItemBuilder<T0, T1>
where
    T0: AstBuilder<T = MainItemHeader>,
    T1: AstBuilder<T = MainItemBody>,
{
    pub fn fill(self, main_item_header: T0, main_item_body: T1) -> Self {
        Self {
            main_item_header: Some(main_item_header),
            main_item_body: Some(main_item_body),
        }
    }
}
impl<T0, T1> AstBuilder for MainItemBuilder<T0, T1>
where
    T0: AstBuilder<T = MainItemHeader>,
    T1: AstBuilder<T = MainItemBody>,
{
    type T = MainItem;
    fn build(self, builder: &mut Cache) -> MainItem {
        let green = AstBuilder::build_green(self, builder);
        MainItem::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(
                self.main_item_header
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(
                self.main_item_body
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .collect();
        builder.node(Nodes::MainItem, children)
    }
}

#[derive(Debug)]
pub struct MainItemHeader(Red);
impl Ast for MainItemHeader {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::MainItemHeader) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl MainItemHeader {
    pub fn start_token(&self) -> Option<ThreePlus> {
        self.0.children().filter_map(ThreePlus::new).next()
    }
    pub fn item_ident_token(&self) -> Option<ItemIdent> {
        self.0.children().filter_map(ItemIdent::new).next()
    }
    pub fn op_colon_token(&self) -> Option<OpColon> {
        self.0.children().filter_map(OpColon::new).next()
    }
    pub fn article_item_id(&self) -> Option<ArticleItemId> {
        self.0.children().filter_map(ArticleItemId::new).next()
    }
    pub fn separator_token(&self) -> Option<ThreePlus> {
        self.0.children().filter_map(ThreePlus::new).next()
    }
    pub fn line_ending_token(&self) -> Option<LineEnding> {
        self.0.children().filter_map(LineEnding::new).next()
    }
    pub fn article_item_values(&self) -> Option<ArticleItemValues> {
        self.0.children().filter_map(ArticleItemValues::new).next()
    }
    pub fn build<T0, T1, T2, T3, T4, T5, T6>() -> MainItemHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
    where
        T0: AstBuilder<T = ThreePlus>,
        T1: AstBuilder<T = ItemIdent>,
        T2: AstBuilder<T = OpColon>,
        T3: AstBuilder<T = ArticleItemId>,
        T4: AstBuilder<T = ThreePlus>,
        T5: AstBuilder<T = LineEnding>,
        T6: AstBuilder<T = ArticleItemValues>,
    {
        Default::default()
    }
}
pub struct MainItemHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleItemValues>,
{
    start: Option<T0>,
    item_ident: Option<T1>,
    op_colon: Option<T2>,
    article_item_id: Option<T3>,
    separator: Option<T4>,
    line_ending: Option<T5>,
    article_item_values: Option<T6>,
}
impl<T0, T1, T2, T3, T4, T5, T6> Default for MainItemHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleItemValues>,
{
    fn default() -> Self {
        Self {
            start: Default::default(),
            item_ident: Default::default(),
            op_colon: Default::default(),
            article_item_id: Default::default(),
            separator: Default::default(),
            line_ending: Default::default(),
            article_item_values: Default::default(),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> MainItemHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleItemValues>,
{
    pub fn fill(
        self,
        start: T0,
        item_ident: T1,
        op_colon: T2,
        article_item_id: T3,
        separator: T4,
        line_ending: T5,
        article_item_values: T6,
    ) -> Self {
        Self {
            start: Some(start),
            item_ident: Some(item_ident),
            op_colon: Some(op_colon),
            article_item_id: Some(article_item_id),
            separator: Some(separator),
            line_ending: Some(line_ending),
            article_item_values: Some(article_item_values),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> AstBuilder for MainItemHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleItemValues>,
{
    type T = MainItemHeader;
    fn build(self, builder: &mut Cache) -> MainItemHeader {
        let green = AstBuilder::build_green(self, builder);
        MainItemHeader::new(Red::root(green)).unwrap()
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
                self.article_item_values
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .collect();
        builder.node(Nodes::MainItemHeader, children)
    }
}

#[derive(Debug)]
pub struct MainItemBody(Red);
impl Ast for MainItemBody {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::MainItemBody) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl MainItemBody {
    pub fn three_plus_token(&self) -> Option<ThreePlus> {
        self.0.children().filter_map(ThreePlus::new).next()
    }
    pub fn line_ending_token(&self) -> Option<LineEnding> {
        self.0.children().filter_map(LineEnding::new).next()
    }
    pub fn article_body(&self) -> Option<ArticleBody> {
        self.0.children().filter_map(ArticleBody::new).next()
    }
    pub fn build<T0, T1, T2>() -> MainItemBodyBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = ThreePlus>,
        T1: AstBuilder<T = LineEnding>,
        T2: AstBuilder<T = ArticleBody>,
    {
        Default::default()
    }
}
pub struct MainItemBodyBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = LineEnding>,
    T2: AstBuilder<T = ArticleBody>,
{
    three_plus: Option<T0>,
    line_ending: Option<T1>,
    article_body: Option<T2>,
}
impl<T0, T1, T2> Default for MainItemBodyBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = LineEnding>,
    T2: AstBuilder<T = ArticleBody>,
{
    fn default() -> Self {
        Self {
            three_plus: Default::default(),
            line_ending: Default::default(),
            article_body: Default::default(),
        }
    }
}
impl<T0, T1, T2> MainItemBodyBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = LineEnding>,
    T2: AstBuilder<T = ArticleBody>,
{
    pub fn fill(self, three_plus: T0, line_ending: T1, article_body: T2) -> Self {
        Self {
            three_plus: Some(three_plus),
            line_ending: Some(line_ending),
            article_body: Some(article_body),
        }
    }
}
impl<T0, T1, T2> AstBuilder for MainItemBodyBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = LineEnding>,
    T2: AstBuilder<T = ArticleBody>,
{
    type T = MainItemBody;
    fn build(self, builder: &mut Cache) -> MainItemBody {
        let green = AstBuilder::build_green(self, builder);
        MainItemBody::new(Red::root(green)).unwrap()
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
            .chain(
                self.article_body
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .collect();
        builder.node(Nodes::MainItemBody, children)
    }
}

#[derive(Debug)]
pub struct ArticleItemId(Red);
impl Ast for ArticleItemId {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleItemId) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ArticleItemId {
    pub fn item_id_token(&self) -> Option<ItemId> {
        self.0.children().filter_map(ItemId::new).next()
    }
    pub fn build<T0>() -> ArticleItemIdBuilder<T0>
    where
        T0: AstBuilder<T = ItemId>,
    {
        Default::default()
    }
}
pub struct ArticleItemIdBuilder<T0>
where
    T0: AstBuilder<T = ItemId>,
{
    item_id: Option<T0>,
}
impl<T0> Default for ArticleItemIdBuilder<T0>
where
    T0: AstBuilder<T = ItemId>,
{
    fn default() -> Self {
        Self {
            item_id: Default::default(),
        }
    }
}
impl<T0> ArticleItemIdBuilder<T0>
where
    T0: AstBuilder<T = ItemId>,
{
    pub fn fill(self, item_id: T0) -> Self {
        Self {
            item_id: Some(item_id),
        }
    }
}
impl<T0> AstBuilder for ArticleItemIdBuilder<T0>
where
    T0: AstBuilder<T = ItemId>,
{
    type T = ArticleItemId;
    fn build(self, builder: &mut Cache) -> ArticleItemId {
        let green = AstBuilder::build_green(self, builder);
        ArticleItemId::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(self.item_id.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::ArticleItemId, children)
    }
}

#[derive(Debug)]
pub struct ArticleItemValues(Red);
impl Ast for ArticleItemValues {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleItemValues) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ArticleItemValues {
    pub fn values(&self) -> impl Iterator<Item = StruktPair> + '_ {
        self.0.children().filter_map(StruktPair::new)
    }
    pub fn build<T0>() -> ArticleItemValuesBuilder<T0>
    where
        T0: AstBuilder<T = LineEnding>,
    {
        Default::default()
    }
}
pub struct ArticleItemValuesBuilder<T0>
where
    T0: AstBuilder<T = LineEnding>,
{
    values: Vec<Box<dyn AstBuilder<T = StruktPair>>>,
    line_ending: Option<T0>,
}
impl<T0> Default for ArticleItemValuesBuilder<T0>
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
impl<T0> ArticleItemValuesBuilder<T0>
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
impl<T0> AstBuilder for ArticleItemValuesBuilder<T0>
where
    T0: AstBuilder<T = LineEnding>,
{
    type T = ArticleItemValues;
    fn build(self, builder: &mut Cache) -> ArticleItemValues {
        let green = AstBuilder::build_green(self, builder);
        ArticleItemValues::new(Red::root(green)).unwrap()
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
        builder.node(Nodes::ArticleItemValues, children)
    }
}

#[derive(Debug)]
pub struct ArticleItem(Red);
impl Ast for ArticleItem {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleItem) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ArticleItem {
    pub fn article_item_header(&self) -> Option<ArticleItemHeader> {
        self.0.children().filter_map(ArticleItemHeader::new).next()
    }
    pub fn article_item_body(&self) -> Option<ArticleItemBody> {
        self.0.children().filter_map(ArticleItemBody::new).next()
    }
    pub fn plus_plus_end_token(&self) -> Option<PlusPlusEnd> {
        self.0.children().filter_map(PlusPlusEnd::new).next()
    }
    pub fn build<T0, T1, T2>() -> ArticleItemBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = ArticleItemHeader>,
        T1: AstBuilder<T = ArticleItemBody>,
        T2: AstBuilder<T = PlusPlusEnd>,
    {
        Default::default()
    }
}
pub struct ArticleItemBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = ArticleItemHeader>,
    T1: AstBuilder<T = ArticleItemBody>,
    T2: AstBuilder<T = PlusPlusEnd>,
{
    article_item_header: Option<T0>,
    article_item_body: Option<T1>,
    plus_plus_end: Option<T2>,
}
impl<T0, T1, T2> Default for ArticleItemBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = ArticleItemHeader>,
    T1: AstBuilder<T = ArticleItemBody>,
    T2: AstBuilder<T = PlusPlusEnd>,
{
    fn default() -> Self {
        Self {
            article_item_header: Default::default(),
            article_item_body: Default::default(),
            plus_plus_end: Default::default(),
        }
    }
}
impl<T0, T1, T2> ArticleItemBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = ArticleItemHeader>,
    T1: AstBuilder<T = ArticleItemBody>,
    T2: AstBuilder<T = PlusPlusEnd>,
{
    pub fn fill(self, article_item_header: T0, article_item_body: T1, plus_plus_end: T2) -> Self {
        Self {
            article_item_header: Some(article_item_header),
            article_item_body: Some(article_item_body),
            plus_plus_end: Some(plus_plus_end),
        }
    }
}
impl<T0, T1, T2> AstBuilder for ArticleItemBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = ArticleItemHeader>,
    T1: AstBuilder<T = ArticleItemBody>,
    T2: AstBuilder<T = PlusPlusEnd>,
{
    type T = ArticleItem;
    fn build(self, builder: &mut Cache) -> ArticleItem {
        let green = AstBuilder::build_green(self, builder);
        ArticleItem::new(Red::root(green)).unwrap()
    }
    fn build_boxed_green(self: Box<Self>, builder: &mut Cache) -> Green {
        AstBuilder::build_green(*self, builder)
    }
    fn build_green(self, builder: &mut Cache) -> Green {
        let children = None
            .into_iter()
            .chain(
                self.article_item_header
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(
                self.article_item_body
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .chain(
                self.plus_plus_end
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .collect();
        builder.node(Nodes::ArticleItem, children)
    }
}
impl<T0, T1, T2> IntoBuilder<ArticleBody> for ArticleItemBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = ArticleItemHeader>,
    T1: AstBuilder<T = ArticleItemBody>,
    T2: AstBuilder<T = PlusPlusEnd>,
{
    fn into_builder(self) -> AliasBuilder<Self, ArticleBody> {
        AliasBuilder::new(Nodes::ArticleBody, self)
    }
}

#[derive(Debug)]
pub struct ArticleItemHeader(Red);
impl Ast for ArticleItemHeader {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleItemHeader) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ArticleItemHeader {
    pub fn plus_plus_token(&self) -> Option<PlusPlus> {
        self.0.children().filter_map(PlusPlus::new).next()
    }
    pub fn item_ident_token(&self) -> Option<ItemIdent> {
        self.0.children().filter_map(ItemIdent::new).next()
    }
    pub fn op_colon_token(&self) -> Option<OpColon> {
        self.0.children().filter_map(OpColon::new).next()
    }
    pub fn article_item_id(&self) -> Option<ArticleItemId> {
        self.0.children().filter_map(ArticleItemId::new).next()
    }
    pub fn three_plus_token(&self) -> Option<ThreePlus> {
        self.0.children().filter_map(ThreePlus::new).next()
    }
    pub fn line_ending_token(&self) -> Option<LineEnding> {
        self.0.children().filter_map(LineEnding::new).next()
    }
    pub fn article_item_values(&self) -> Option<ArticleItemValues> {
        self.0.children().filter_map(ArticleItemValues::new).next()
    }
    pub fn build<T0, T1, T2, T3, T4, T5, T6>(
    ) -> ArticleItemHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
    where
        T0: AstBuilder<T = PlusPlus>,
        T1: AstBuilder<T = ItemIdent>,
        T2: AstBuilder<T = OpColon>,
        T3: AstBuilder<T = ArticleItemId>,
        T4: AstBuilder<T = ThreePlus>,
        T5: AstBuilder<T = LineEnding>,
        T6: AstBuilder<T = ArticleItemValues>,
    {
        Default::default()
    }
}
pub struct ArticleItemHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = PlusPlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleItemValues>,
{
    plus_plus: Option<T0>,
    item_ident: Option<T1>,
    op_colon: Option<T2>,
    article_item_id: Option<T3>,
    three_plus: Option<T4>,
    line_ending: Option<T5>,
    article_item_values: Option<T6>,
}
impl<T0, T1, T2, T3, T4, T5, T6> Default for ArticleItemHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = PlusPlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleItemValues>,
{
    fn default() -> Self {
        Self {
            plus_plus: Default::default(),
            item_ident: Default::default(),
            op_colon: Default::default(),
            article_item_id: Default::default(),
            three_plus: Default::default(),
            line_ending: Default::default(),
            article_item_values: Default::default(),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> ArticleItemHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = PlusPlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleItemValues>,
{
    pub fn fill(
        self,
        plus_plus: T0,
        item_ident: T1,
        op_colon: T2,
        article_item_id: T3,
        three_plus: T4,
        line_ending: T5,
        article_item_values: T6,
    ) -> Self {
        Self {
            plus_plus: Some(plus_plus),
            item_ident: Some(item_ident),
            op_colon: Some(op_colon),
            article_item_id: Some(article_item_id),
            three_plus: Some(three_plus),
            line_ending: Some(line_ending),
            article_item_values: Some(article_item_values),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> AstBuilder for ArticleItemHeaderBuilder<T0, T1, T2, T3, T4, T5, T6>
where
    T0: AstBuilder<T = PlusPlus>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ArticleItemId>,
    T4: AstBuilder<T = ThreePlus>,
    T5: AstBuilder<T = LineEnding>,
    T6: AstBuilder<T = ArticleItemValues>,
{
    type T = ArticleItemHeader;
    fn build(self, builder: &mut Cache) -> ArticleItemHeader {
        let green = AstBuilder::build_green(self, builder);
        ArticleItemHeader::new(Red::root(green)).unwrap()
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
                self.article_item_values
                    .map(|it| it.build_green(builder))
                    .into_iter(),
            )
            .collect();
        builder.node(Nodes::ArticleItemHeader, children)
    }
}

#[derive(Debug)]
pub struct ArticleItemBody(Red);
impl Ast for ArticleItemBody {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleItemBody) {
            return None;
        }
        node.green().as_node()?;
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
impl ArticleItemBody {
    pub fn three_plus_token(&self) -> Option<ThreePlus> {
        self.0.children().filter_map(ThreePlus::new).next()
    }
    pub fn line_ending_token(&self) -> Option<LineEnding> {
        self.0.children().filter_map(LineEnding::new).next()
    }
    pub fn article_body(&self) -> impl Iterator<Item = ArticleBody> + '_ {
        self.0.children().filter_map(ArticleBody::new)
    }
    pub fn build<T0, T1>() -> ArticleItemBodyBuilder<T0, T1>
    where
        T0: AstBuilder<T = ThreePlus>,
        T1: AstBuilder<T = LineEnding>,
    {
        Default::default()
    }
}
pub struct ArticleItemBodyBuilder<T0, T1>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = LineEnding>,
{
    three_plus: Option<T0>,
    line_ending: Option<T1>,
    article_body: Vec<Box<dyn AstBuilder<T = ArticleBody>>>,
}
impl<T0, T1> Default for ArticleItemBodyBuilder<T0, T1>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = LineEnding>,
{
    fn default() -> Self {
        Self {
            three_plus: Default::default(),
            line_ending: Default::default(),
            article_body: Default::default(),
        }
    }
}
impl<T0, T1> ArticleItemBodyBuilder<T0, T1>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = LineEnding>,
{
    pub fn fill(
        self,
        three_plus: T0,
        line_ending: T1,
        article_body: Vec<Box<dyn AstBuilder<T = ArticleBody>>>,
    ) -> Self {
        Self {
            three_plus: Some(three_plus),
            line_ending: Some(line_ending),
            article_body,
        }
    }
}
impl<T0, T1> AstBuilder for ArticleItemBodyBuilder<T0, T1>
where
    T0: AstBuilder<T = ThreePlus>,
    T1: AstBuilder<T = LineEnding>,
{
    type T = ArticleItemBody;
    fn build(self, builder: &mut Cache) -> ArticleItemBody {
        let green = AstBuilder::build_green(self, builder);
        ArticleItemBody::new(Red::root(green)).unwrap()
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
                self.article_body
                    .into_iter()
                    .map(|it| it.build_boxed_green(builder))
                    .collect::<Vec<_>>()
            })
            .collect();
        builder.node(Nodes::ArticleItemBody, children)
    }
}

#[derive(Debug)]
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
    pub fn item_id_token(&self) -> Option<ItemId> {
        self.0.children().filter_map(ItemId::new).next()
    }
    pub fn close_bl_token(&self) -> Option<CloseBl> {
        self.0.children().filter_map(CloseBl::new).next()
    }
    pub fn build<T0, T1, T2, T3, T4>() -> ArticleRefBuilder<T0, T1, T2, T3, T4>
    where
        T0: AstBuilder<T = OpenBl>,
        T1: AstBuilder<T = ItemIdent>,
        T2: AstBuilder<T = OpColon>,
        T3: AstBuilder<T = ItemId>,
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
    T3: AstBuilder<T = ItemId>,
    T4: AstBuilder<T = CloseBl>,
{
    open_bl: Option<T0>,
    item_ident: Option<T1>,
    op_colon: Option<T2>,
    item_id: Option<T3>,
    close_bl: Option<T4>,
}
impl<T0, T1, T2, T3, T4> Default for ArticleRefBuilder<T0, T1, T2, T3, T4>
where
    T0: AstBuilder<T = OpenBl>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ItemId>,
    T4: AstBuilder<T = CloseBl>,
{
    fn default() -> Self {
        Self {
            open_bl: Default::default(),
            item_ident: Default::default(),
            op_colon: Default::default(),
            item_id: Default::default(),
            close_bl: Default::default(),
        }
    }
}
impl<T0, T1, T2, T3, T4> ArticleRefBuilder<T0, T1, T2, T3, T4>
where
    T0: AstBuilder<T = OpenBl>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ItemId>,
    T4: AstBuilder<T = CloseBl>,
{
    pub fn fill(
        self,
        open_bl: T0,
        item_ident: T1,
        op_colon: T2,
        item_id: T3,
        close_bl: T4,
    ) -> Self {
        Self {
            open_bl: Some(open_bl),
            item_ident: Some(item_ident),
            op_colon: Some(op_colon),
            item_id: Some(item_id),
            close_bl: Some(close_bl),
        }
    }
}
impl<T0, T1, T2, T3, T4> AstBuilder for ArticleRefBuilder<T0, T1, T2, T3, T4>
where
    T0: AstBuilder<T = OpenBl>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ItemId>,
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
            .chain(self.item_id.map(|it| it.build_green(builder)).into_iter())
            .chain(self.close_bl.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::ArticleRef, children)
    }
}
impl<T0, T1, T2, T3, T4> IntoBuilder<ArticleBody> for ArticleRefBuilder<T0, T1, T2, T3, T4>
where
    T0: AstBuilder<T = OpenBl>,
    T1: AstBuilder<T = ItemIdent>,
    T2: AstBuilder<T = OpColon>,
    T3: AstBuilder<T = ItemId>,
    T4: AstBuilder<T = CloseBl>,
{
    fn into_builder(self) -> AliasBuilder<Self, ArticleBody> {
        AliasBuilder::new(Nodes::ArticleBody, self)
    }
}
