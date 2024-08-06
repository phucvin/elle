use std::fmt;

use crate::compiler::enums::Type;
use crate::lexer::colors::*;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Use,
    Public,
    Function,
    Type,
    Identifier,
    IntegerLiteral,
    LongLiteral,
    FloatingPoint,
    FloatLiteral,
    CharLiteral,
    StringLiteral,
    TrueLiteral,
    FalseLiteral,
    ExactLiteral,
    Comment,
    Colon,
    AtMark,
    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftBlockBrace,
    RightBlockBrace,
    Comma,
    Not,
    Equal,
    AddEqual,
    SubtractEqual,
    MultiplyEqual,
    DivideEqual,
    ModulusEqual,
    BitwiseXorEqual,
    BitwiseOrEqual,
    BitwiseAndEqual,
    AddOne,
    SubtractOne,
    // Exponent,
    RightArrow,
    LeftArrow,
    Semicolon,
    If,
    Else,
    For,
    While,
    Return,
    Question,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    EqualTo,
    NotEqualTo,
    And,
    BitwiseAnd,
    BitwiseXor,
    Or,
    BitwiseOr,
    None,
    Constant,
    Store,
    Break,
    Continue,
    To,
    Ellipsis,
    Variadic,
    Dot,
    Yield,
    Step,
    Deref,
    Defer,
    Size,
    Unary,
    ArrayLength,
    External,
    Address,
    Define, // Used for both structs and enums
    ShiftRight,
    ShiftLeft,
    ShiftRightEqual,
    ShiftLeftEqual,
    Global,
    Local,
}

impl TokenKind {
    pub fn highest_precedence() -> i8 {
        // Self::Exponent.precedence()
        Self::Multiply.precedence()
    }

    pub fn precedence(&self) -> i8 {
        match self {
            // Self::Exponent => 9,
            Self::Multiply | Self::Divide | Self::Modulus => 8,
            Self::Add | Self::Subtract => 7,
            Self::ShiftLeft | Self::ShiftRight => 6,
            Self::LessThan | Self::LessThanEqual | Self::GreaterThan | Self::GreaterThanEqual => 5,
            Self::EqualTo | Self::NotEqualTo => 4,
            Self::And | Self::BitwiseAnd => 3,
            Self::BitwiseXor => 2,
            Self::Or | Self::BitwiseOr => 1,
            _ => 0,
        }
    }

    pub fn is_arithmetic(&self) -> bool {
        match self.to_owned() {
            Self::Multiply
            // | Self::Exponent
            | Self::Divide
            | Self::Modulus
            | Self::Add
            | Self::Subtract
            | Self::LessThan
            | Self::LessThanEqual
            | Self::GreaterThan
            | Self::GreaterThanEqual
            | Self::EqualTo
            | Self::NotEqualTo
            | Self::And
            | Self::Or
            | Self::BitwiseXor
            | Self::BitwiseOr
            | Self::BitwiseAnd
            | Self::ShiftLeft
            | Self::ShiftRight => true,
            _ => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        match self.to_owned() {
            Self::StringLiteral
            | Self::IntegerLiteral
            | Self::CharLiteral
            | Self::FloatLiteral
            | Self::LongLiteral
            | Self::ExactLiteral
            | Self::TrueLiteral
            | Self::FalseLiteral
            | Self::Break
            | Self::Continue
            | Self::FloatingPoint => true,
            _ => false,
        }
    }

    pub fn is_declarative(&self) -> bool {
        match self.to_owned() {
            Self::AddEqual
            | Self::SubtractEqual
            | Self::MultiplyEqual
            | Self::DivideEqual
            | Self::ModulusEqual
            | Self::BitwiseXorEqual
            | Self::BitwiseOrEqual
            | Self::BitwiseAndEqual
            | Self::ShiftLeftEqual
            | Self::ShiftRightEqual
            | Self::AddOne
            | Self::SubtractOne => true,
            _ => false,
        }
    }

    pub fn is_comparative(&self) -> bool {
        match self {
            Self::GreaterThan
            | Self::GreaterThanEqual
            | Self::LessThan
            | Self::LessThanEqual
            | Self::EqualTo
            | Self::NotEqualTo => true,
            _ => false,
        }
    }

    pub fn is_unary_context(&self) -> bool {
        match self {
            Self::LeftParenthesis
            | Self::LeftCurlyBrace
            | Self::LeftBlockBrace
            | Self::Comma
            | Self::Colon
            | Self::Not
            | Self::Semicolon
            | Self::Return
            | Self::While
            | Self::For
            | Self::If
            | Self::Equal => true,
            other if other.is_declarative() => true,
            other if other.is_arithmetic() => true,
            _ => false,
        }
    }

    pub fn to_non_declarative(&self) -> TokenKind {
        match self {
            Self::AddEqual => TokenKind::Add,
            Self::SubtractEqual => TokenKind::Subtract,
            Self::MultiplyEqual => TokenKind::Multiply,
            Self::DivideEqual => TokenKind::Divide,
            Self::ModulusEqual => TokenKind::Modulus,
            Self::BitwiseXorEqual => TokenKind::BitwiseXor,
            Self::BitwiseAndEqual => TokenKind::BitwiseAnd,
            Self::BitwiseOrEqual => TokenKind::BitwiseOr,
            Self::ShiftLeftEqual => TokenKind::ShiftLeft,
            Self::ShiftRightEqual => TokenKind::ShiftRight,
            other => panic!("Invalid identifier operation {:?}", other),
        }
    }

    pub fn is_one_operator(&self) -> bool {
        match self.to_owned() {
            Self::AddOne | Self::SubtractOne => true,
            _ => false,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Debug, Clone)]
pub enum ValueKind {
    String(String),
    Number(i128),
    Character(char),
    Nil,
}

impl ValueKind {
    pub fn to_type_string(&self) -> Option<Type> {
        match self.clone() {
            ValueKind::String(val) => match val.as_str() {
                "string" => Some(Type::Pointer(Box::new(Type::Char))),
                "fun" => Some(Type::Byte), // Cannot be the same as fn
                "i8" => Some(Type::Byte),
                "u8" => Some(Type::UnsignedByte),
                "i16" => Some(Type::Halfword),
                "u16" => Some(Type::UnsignedHalfword),
                "i32" => Some(Type::Word),
                "u32" => Some(Type::UnsignedWord),
                "i64" => Some(Type::Long),
                "u64" => Some(Type::UnsignedLong),
                "f32" => Some(Type::Single),
                "f64" => Some(Type::Double),
                "char" => Some(Type::Word),
                "bool" => Some(Type::Byte),
                // Arbitrary because it will be turned into `long` anyway when used as void*`
                "void" => Some(Type::Void),
                "nil" => None,
                other => Some(Type::Struct(other.to_string())),
            },
            _ => None,
        }
    }

    pub fn is_base_type(&self) -> bool {
        self.to_type_string().is_some()
            && match self.to_type_string().unwrap() {
                Type::Struct(_) => false,
                _ => true,
            }
    }

    pub fn get_string_inner(&self) -> Option<String> {
        match self.clone() {
            Self::String(val) => Some(val),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Location {
    pub file: String,
    pub row: usize,
    pub column: usize,
    pub ctx: String,
    pub length: usize,
}

impl Location {
    pub fn display(&self, is_warning: bool) -> String {
        return format!(
            "{BOLD}{UNDERLINE}{GREEN}{}{RESET}:{UNDERLINE}{fmt}{}{RESET}:{UNDERLINE}{YELLOW}{}{RESET}",
            self.file,
            self.row + 1,
            self.column + 1,
            fmt = if is_warning { YELLOW } else { RED }
        );
    }

    pub fn display_plain(&self) -> String {
        return format!("{}:{}:{}", self.file, self.row + 1, self.column + 1);
    }

    pub fn display_pretty(&self, message: String, is_warning: bool) -> String {
        let upper = format!(
            "{fmt}{}{RESET}[{}]{fmt}{}{RESET}",
            "-".repeat(20),
            self.display(is_warning),
            "-".repeat(20),
            fmt = if is_warning { YELLOW } else { RED }
        );
        let upper_plain = format!(
            "{}[{}]{}",
            "-".repeat(20),
            self.display_plain(),
            "-".repeat(20)
        );
        let padding = 4;
        let ident = self.column - (self.ctx.len() - self.ctx.trim_start().len());

        let left = if ident >= self.length {
            ident - self.length
        } else {
            ident
        };

        let string = self.ctx.trim_start().split_at(left);
        let lhs = string.0;
        let warning_part = string.1.get(0..self.length).unwrap();
        let rhs = string.1.get(self.length..).unwrap();

        return format!(
            "\n\n{}\n{}\n\n{}{}{BOLD}{fmt}{UNDERLINE}{}{RESET}{}\n{}{}{BOLD}{GREEN}^{}{RESET}\n{fmt}{}{RESET}\n\n",
            upper,
            message,
            " ".repeat(padding),
            lhs,
            warning_part,
            rhs,
            " ".repeat(padding),
            " ".repeat(left),
            "~".repeat(self.length - 1),
            "-".repeat(upper_plain.len()),
            fmt = if is_warning { YELLOW } else { RED }
        );
    }

    pub fn warning(&self, message: String) -> String {
        self.display_pretty(message, true)
    }

    pub fn error(&self, message: String) -> String {
        self.display_pretty(message, false)
    }

    pub fn default(file: String) -> Location {
        Location {
            file,
            row: 0,
            column: 0,
            ctx: "".to_string(),
            length: 0,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.row + 1, self.column + 1)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.row + 1, self.column + 1)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub location: Location,
    pub value: ValueKind,
}

#[derive(Debug, Clone)]
pub enum ParseResult {
    Float(f64),
    Int(i64),
}
