pub type Span = (usize, usize);

#[derive(Debug, PartialEq, Clone)]
pub enum OpenTagKind {
    Full,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Abstract,
    AndEqual,
    Array,
    ArrayCast,
    As,
    Asterisk,
    Attribute,
    BoolCast,
    BooleanAnd,
    NamespaceSeparator,
    BooleanOr,
    Break,
    Callable,
    Case,
    Catch,
    Class,
    ClassConstant,
    Clone,
    CloseTag,
    Coalesce,
    CoalesceEqual,
    Comment(String),
    Comma,
    ConcatEqual,
    Const,
    ConstantString(String),
    Continue,
    CurlyOpen,
    Decrement,
    Declare,
    Default,
    DirConstant,
    DivEqual,
    Do,
    DocComment,
    DoubleCast,
    DoubleColon,
    DoubleArrow,
    DoubleEquals,
    Echo,
    Ellipsis,
    Else,
    ElseIf,
    Empty,
    EndDeclare,
    EndFor,
    EndForeach,
    EndIf,
    EndSwitch,
    EndWhile,
    Enum,
    Eof,
    Equals,
    Extends,
    Final,
    Function,
    GreaterThan,
    GreaterThanEquals,
    Identifier(String),
    Dot,
    Null,
    True,
    False,
    Use,
    If,
    Implements,
    InlineHtml(String),
    Int(i64),
    Float(f64),
    LeftBrace,
    LeftBracket,
    LeftParen,
    LessThan,
    LessThanEquals,
    Minus,
    OpenTag(OpenTagKind),
    Percent,
    Plus,
    Pow,
    Private,
    Protected,
    Public,
    Return,
    RightBrace,
    RightBracket,
    RightParen,
    SemiColon,
    Slash,
    Static,
    TripleEquals,
    Var,
    Variable(String),
    FullyQualifiedIdentifier(String),
    QualifiedIdentifier(String),
    Colon,
    Caret,
    Question,
    Bang,
    And,
    BitAnd,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Default for Token {
    fn default() -> Self {
        Self { kind: TokenKind::Eof, span: (0, 0) }
    }
}