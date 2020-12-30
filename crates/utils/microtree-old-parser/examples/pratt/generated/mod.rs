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
    pub const Op: Name = Name::new("Op");
    pub const Value: Name = Name::new("Value");
    pub const Unary: Name = Name::new("Unary");
    pub const Binary: Name = Name::new("Binary");
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
pub struct Star(Red);
impl Ast for Star {
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
impl Star {
    pub fn build() -> TokenBuilder<Star> {
        TokenBuilder::new("*")
    }
}
impl IntoBuilder<Op> for TokenBuilder<Star> {
    fn into_builder(self) -> AliasBuilder<Self, Op> {
        AliasBuilder::new(Nodes::Op, self)
    }
}

#[derive(Debug)]
pub struct Plus(Red);
impl Ast for Plus {
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
impl Plus {
    pub fn build() -> TokenBuilder<Plus> {
        TokenBuilder::new("+")
    }
}
impl IntoBuilder<Op> for TokenBuilder<Plus> {
    fn into_builder(self) -> AliasBuilder<Self, Op> {
        AliasBuilder::new(Nodes::Op, self)
    }
}

#[derive(Debug)]
pub struct Minus(Red);
impl Ast for Minus {
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
impl Minus {
    pub fn build() -> TokenBuilder<Minus> {
        TokenBuilder::new("-")
    }
}
impl IntoBuilder<Op> for TokenBuilder<Minus> {
    fn into_builder(self) -> AliasBuilder<Self, Op> {
        AliasBuilder::new(Nodes::Op, self)
    }
}

#[derive(Debug)]
pub struct Slash(Red);
impl Ast for Slash {
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
impl Slash {
    pub fn build() -> TokenBuilder<Slash> {
        TokenBuilder::new("/")
    }
}
impl IntoBuilder<Op> for TokenBuilder<Slash> {
    fn into_builder(self) -> AliasBuilder<Self, Op> {
        AliasBuilder::new(Nodes::Op, self)
    }
}

#[derive(Debug)]
pub enum Op {
    Minus(Minus),
    Plus(Plus),
    Star(Star),
    Slash(Slash),
}
impl From<Minus> for Op {
    fn from(val: Minus) -> Self {
        Self::Minus(val)
    }
}
impl From<Plus> for Op {
    fn from(val: Plus) -> Self {
        Self::Plus(val)
    }
}
impl From<Star> for Op {
    fn from(val: Star) -> Self {
        Self::Star(val)
    }
}
impl From<Slash> for Op {
    fn from(val: Slash) -> Self {
        Self::Slash(val)
    }
}
impl Ast for Op {
    fn new(node: Red) -> Option<Self> {
        None.or_else(|| Minus::new(node.clone()).map(Op::Minus))
            .or_else(|| Plus::new(node.clone()).map(Op::Plus))
            .or_else(|| Star::new(node.clone()).map(Op::Star))
            .or_else(|| Slash::new(node.clone()).map(Op::Slash))
    }
    fn red(&self) -> Red {
        match &self {
            Op::Minus(node) => node.red(),
            Op::Plus(node) => node.red(),
            Op::Star(node) => node.red(),
            Op::Slash(node) => node.red(),
        }
    }
}
impl Op {
    pub fn as_minus(self) -> Option<Minus> {
        match self {
            Self::Minus(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_plus(self) -> Option<Plus> {
        match self {
            Self::Plus(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_star(self) -> Option<Star> {
        match self {
            Self::Star(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_slash(self) -> Option<Slash> {
        match self {
            Self::Slash(val) => Some(val),
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
    pub fn number_token(&self) -> Option<Number> {
        self.0.children().filter_map(Number::new).next()
    }
    pub fn unary(&self) -> Option<Unary> {
        self.0.children().filter_map(Unary::new).next()
    }
    pub fn binary(&self) -> Option<Binary> {
        self.0.children().filter_map(Binary::new).next()
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
    pub fn build<T0, T1, T2, T3, T4, T5>() -> ValueBuilder<T0, T1, T2, T3, T4, T5>
    where
        T0: AstBuilder<T = Number>,
        T1: AstBuilder<T = Unary>,
        T2: AstBuilder<T = Binary>,
        T3: AstBuilder<T = OpenP>,
        T4: AstBuilder<T = Value>,
        T5: AstBuilder<T = CloseP>,
    {
        Default::default()
    }
}
pub struct ValueBuilder<T0, T1, T2, T3, T4, T5>
where
    T0: AstBuilder<T = Number>,
    T1: AstBuilder<T = Unary>,
    T2: AstBuilder<T = Binary>,
    T3: AstBuilder<T = OpenP>,
    T4: AstBuilder<T = Value>,
    T5: AstBuilder<T = CloseP>,
{
    number: Option<T0>,
    unary: Option<T1>,
    binary: Option<T2>,
    open_p: Option<T3>,
    value: Option<T4>,
    close_p: Option<T5>,
}
impl<T0, T1, T2, T3, T4, T5> Default for ValueBuilder<T0, T1, T2, T3, T4, T5>
where
    T0: AstBuilder<T = Number>,
    T1: AstBuilder<T = Unary>,
    T2: AstBuilder<T = Binary>,
    T3: AstBuilder<T = OpenP>,
    T4: AstBuilder<T = Value>,
    T5: AstBuilder<T = CloseP>,
{
    fn default() -> Self {
        Self {
            number: Default::default(),
            unary: Default::default(),
            binary: Default::default(),
            open_p: Default::default(),
            value: Default::default(),
            close_p: Default::default(),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5> ValueBuilder<T0, T1, T2, T3, T4, T5>
where
    T0: AstBuilder<T = Number>,
    T1: AstBuilder<T = Unary>,
    T2: AstBuilder<T = Binary>,
    T3: AstBuilder<T = OpenP>,
    T4: AstBuilder<T = Value>,
    T5: AstBuilder<T = CloseP>,
{
    pub fn fill(
        self,
        number: T0,
        unary: T1,
        binary: T2,
        open_p: T3,
        value: T4,
        close_p: T5,
    ) -> Self {
        Self {
            number: Some(number),
            unary: Some(unary),
            binary: Some(binary),
            open_p: Some(open_p),
            value: Some(value),
            close_p: Some(close_p),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5> AstBuilder for ValueBuilder<T0, T1, T2, T3, T4, T5>
where
    T0: AstBuilder<T = Number>,
    T1: AstBuilder<T = Unary>,
    T2: AstBuilder<T = Binary>,
    T3: AstBuilder<T = OpenP>,
    T4: AstBuilder<T = Value>,
    T5: AstBuilder<T = CloseP>,
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
            .chain(self.number.map(|it| it.build_green(builder)).into_iter())
            .chain(self.unary.map(|it| it.build_green(builder)).into_iter())
            .chain(self.binary.map(|it| it.build_green(builder)).into_iter())
            .chain(self.open_p.map(|it| it.build_green(builder)).into_iter())
            .chain(self.value.map(|it| it.build_green(builder)).into_iter())
            .chain(self.close_p.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::Value, children)
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
    pub fn minus_token(&self) -> Option<Minus> {
        self.0.children().filter_map(Minus::new).next()
    }
    pub fn value(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).next()
    }
    pub fn build<T0, T1>() -> UnaryBuilder<T0, T1>
    where
        T0: AstBuilder<T = Minus>,
        T1: AstBuilder<T = Value>,
    {
        Default::default()
    }
}
pub struct UnaryBuilder<T0, T1>
where
    T0: AstBuilder<T = Minus>,
    T1: AstBuilder<T = Value>,
{
    minus: Option<T0>,
    value: Option<T1>,
}
impl<T0, T1> Default for UnaryBuilder<T0, T1>
where
    T0: AstBuilder<T = Minus>,
    T1: AstBuilder<T = Value>,
{
    fn default() -> Self {
        Self {
            minus: Default::default(),
            value: Default::default(),
        }
    }
}
impl<T0, T1> UnaryBuilder<T0, T1>
where
    T0: AstBuilder<T = Minus>,
    T1: AstBuilder<T = Value>,
{
    pub fn fill(self, minus: T0, value: T1) -> Self {
        Self {
            minus: Some(minus),
            value: Some(value),
        }
    }
}
impl<T0, T1> AstBuilder for UnaryBuilder<T0, T1>
where
    T0: AstBuilder<T = Minus>,
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
            .chain(self.minus.map(|it| it.build_green(builder)).into_iter())
            .chain(self.value.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::Unary, children)
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
    pub fn op(&self) -> Option<Op> {
        self.0.children().filter_map(Op::new).next()
    }
    pub fn right(&self) -> Option<Value> {
        self.0.children().filter_map(Value::new).next()
    }
    pub fn build<T0, T1, T2>() -> BinaryBuilder<T0, T1, T2>
    where
        T0: AstBuilder<T = Value>,
        T1: AstBuilder<T = Op>,
        T2: AstBuilder<T = Value>,
    {
        Default::default()
    }
}
pub struct BinaryBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Value>,
    T1: AstBuilder<T = Op>,
    T2: AstBuilder<T = Value>,
{
    left: Option<T0>,
    op: Option<T1>,
    right: Option<T2>,
}
impl<T0, T1, T2> Default for BinaryBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Value>,
    T1: AstBuilder<T = Op>,
    T2: AstBuilder<T = Value>,
{
    fn default() -> Self {
        Self {
            left: Default::default(),
            op: Default::default(),
            right: Default::default(),
        }
    }
}
impl<T0, T1, T2> BinaryBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Value>,
    T1: AstBuilder<T = Op>,
    T2: AstBuilder<T = Value>,
{
    pub fn fill(self, left: T0, op: T1, right: T2) -> Self {
        Self {
            left: Some(left),
            op: Some(op),
            right: Some(right),
        }
    }
}
impl<T0, T1, T2> AstBuilder for BinaryBuilder<T0, T1, T2>
where
    T0: AstBuilder<T = Value>,
    T1: AstBuilder<T = Op>,
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
            .chain(self.op.map(|it| it.build_green(builder)).into_iter())
            .chain(self.right.map(|it| it.build_green(builder)).into_iter())
            .collect();
        builder.node(Nodes::Binary, children)
    }
}
