pub enum Token {
    Let,
    If,
    Loop,
    Fun,

    Identifier,

    RCurly,
    LCurly,

    RParen,
    LParen,

    Number,
    Text,

    Equal,
    IsEqual,
    IsGreater,
    IsGreatEqual,
    IsLesser,
    IsLesserEqual
}
