use std::collections::HashMap;
use lazy_static::lazy_static;

type TokenType = &'static str;

struct Token {
    r#type: TokenType,
    literal: String,
}

const ILLEGAL: TokenType = "ILLEGAL";
const EOF: &str = "EOF";

const IDENT: &str = "IDENT";
const INT: &str = "INT";
const STRING: &str = "STRING";

const ASSIGN: &str = "=";
const PLUS: &str = "+";
const MINUS: &str = "-";
const BANG: &str = "!";
const ASTERISK: &str = "*";
const SLASH: &str = "/";

const LT: &str = "<";
const GT: &str = ">";

const COMMA: &str = ",";
const SEMICOLON: &str = ";";
const COLON: &str = ":";

const LPAREN: &str = "(";
const RPAREN: &str = ")";
const LBRACE: &str = "{";
const RBRACE: &str = "}";
const LBRACKET: &str = "[";
const RBRACKET: &str = "]";

const EQ: &str = "==";
const NOT_EQ: &str = "!=";

const FUNCTION: &str = "FUNCTION";
const LET: &str = "LET";
const TRUE: &str = "TRUE";
const FALSE: &str = "FALSE";
const IF: &str = "IF";
const ELSE: &str = "ELSE";
const RETURN: &str = "RETURN";
const MACRO: &str = "MACRO";

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
        ("fn", FUNCTION),
        ("let", LET),
        ("true", TRUE),
        ("false", FALSE),
        ("if", IF),
        ("else", ELSE),
        ("return", RETURN),
        ("macro", MACRO),
    ]);
}

pub fn look_up_ident(ident: &str) -> TokenType {
    match KEYWORDS.get(ident) {
        Some(keyword) => keyword,
        None => IDENT,
    }
}
