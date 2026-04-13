use crate::error::{IntelError, IntelResult};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Identifiers and literals
    Identifier(String),
    StringLit(String),
    NumberLit(f64),

    // Punctuation
    Semicolon,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Comma,
    Newline,

    // Comparison operators
    Eq,       // =
    NotEq,    // !=
    Gt,       // >
    Lt,       // <
    Gte,      // >=
    Lte,      // <=
    Tilde,    // ~
    DoubleTilde, // ~~

    // Logical operators
    And,
    Or,
    Not,

    // Special keyword operators
    In,  // both a modifier and a filter op
    Has,

    // Node types
    Programs,
    Paragraphs,
    Fields,
    Copybooks,
    Files,
    Tables,
    Rules,

    // Traversal verbs
    Calling,
    CalledBy,
    Performing,
    PerformedBy,
    Reading,
    ReadBy,
    Writing,
    WrittenBy,
    Using,
    UsedBy,
    Accessing,
    AccessedBy,
    Containing,
    Within,

    // Domain verbs
    Trace,
    Rank,
    Similar,
    FindDead,
    DiscoverProcesses,
    EstimateCost,
    Deps,
    Impact,
    Compare,
    Save,
    Run,

    // Modifiers
    By,
    Top,
    With,
    Depth,
    Level,
    Order,
    Scope,
    Threshold,
    As,

    // Special
    Each,

    Eof,
}

impl Token {
    pub fn is_node_type(&self) -> bool {
        matches!(
            self,
            Token::Programs
                | Token::Paragraphs
                | Token::Fields
                | Token::Copybooks
                | Token::Files
                | Token::Tables
                | Token::Rules
        )
    }

    pub fn is_traversal_verb(&self) -> bool {
        matches!(
            self,
            Token::Calling
                | Token::CalledBy
                | Token::Performing
                | Token::PerformedBy
                | Token::Reading
                | Token::ReadBy
                | Token::Writing
                | Token::WrittenBy
                | Token::Using
                | Token::UsedBy
                | Token::Accessing
                | Token::AccessedBy
                | Token::Containing
                | Token::Within
        )
    }

    pub fn is_domain_verb(&self) -> bool {
        matches!(
            self,
            Token::Trace
                | Token::Rank
                | Token::Similar
                | Token::FindDead
                | Token::DiscoverProcesses
                | Token::EstimateCost
                | Token::Deps
                | Token::Impact
                | Token::Compare
                | Token::Save
                | Token::Run
        )
    }

    pub fn is_modifier(&self) -> bool {
        matches!(
            self,
            Token::By
                | Token::Top
                | Token::In
                | Token::With
                | Token::Depth
                | Token::Level
                | Token::Order
                | Token::Scope
                | Token::Threshold
                | Token::As
        )
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(mut self) -> IntelResult<Vec<Token>> {
        while self.pos < self.input.len() {
            self.skip_spaces();

            if self.pos >= self.input.len() {
                break;
            }

            let ch = self.current();

            match ch {
                // Comment: skip to end of line (including the trailing newline)
                '-' if self.peek_next() == Some('-') => {
                    while self.pos < self.input.len() && self.current() != '\n' {
                        self.pos += 1;
                    }
                    if self.pos < self.input.len() {
                        self.pos += 1; // skip the newline after comment
                    }
                }

                // Continuation line marker (..)
                '.' if self.peek_next() == Some('.') => {
                    self.pos += 2;
                    // Just skip it -- purely visual aid
                }

                '\n' => {
                    // Collapse multiple newlines into one
                    if self.tokens.last() != Some(&Token::Newline) {
                        self.tokens.push(Token::Newline);
                    }
                    self.pos += 1;
                }

                ';' => {
                    self.tokens.push(Token::Semicolon);
                    self.pos += 1;
                }
                '[' => {
                    self.tokens.push(Token::LeftBracket);
                    self.pos += 1;
                }
                ']' => {
                    self.tokens.push(Token::RightBracket);
                    self.pos += 1;
                }
                '(' => {
                    self.tokens.push(Token::LeftParen);
                    self.pos += 1;
                }
                ')' => {
                    self.tokens.push(Token::RightParen);
                    self.pos += 1;
                }
                ',' => {
                    self.tokens.push(Token::Comma);
                    self.pos += 1;
                }

                // Operators
                '=' => {
                    self.tokens.push(Token::Eq);
                    self.pos += 1;
                }
                '!' if self.peek_next() == Some('=') => {
                    self.tokens.push(Token::NotEq);
                    self.pos += 2;
                }
                '>' if self.peek_next() == Some('=') => {
                    self.tokens.push(Token::Gte);
                    self.pos += 2;
                }
                '<' if self.peek_next() == Some('=') => {
                    self.tokens.push(Token::Lte);
                    self.pos += 2;
                }
                '>' => {
                    self.tokens.push(Token::Gt);
                    self.pos += 1;
                }
                '<' => {
                    self.tokens.push(Token::Lt);
                    self.pos += 1;
                }
                '~' if self.peek_next() == Some('~') => {
                    self.tokens.push(Token::DoubleTilde);
                    self.pos += 2;
                }
                '~' => {
                    self.tokens.push(Token::Tilde);
                    self.pos += 1;
                }

                // String literal
                '\'' => {
                    self.pos += 1;
                    let start = self.pos;
                    while self.pos < self.input.len() && self.current() != '\'' {
                        self.pos += 1;
                    }
                    if self.pos >= self.input.len() {
                        return Err(IntelError::ParseError {
                            reason: "unterminated string literal".to_owned(),
                        });
                    }
                    let s = self.input[start..self.pos].to_owned();
                    self.tokens.push(Token::StringLit(s));
                    self.pos += 1; // skip closing quote
                }

                // Number
                c if c.is_ascii_digit() => {
                    let start = self.pos;
                    while self.pos < self.input.len()
                        && (self.current().is_ascii_digit() || self.current() == '.')
                    {
                        self.pos += 1;
                    }
                    let num_str = &self.input[start..self.pos];
                    let num: f64 = num_str.parse().map_err(|_| IntelError::ParseError {
                        reason: format!("invalid number: '{num_str}'"),
                    })?;
                    self.tokens.push(Token::NumberLit(num));
                }

                // Keyword or identifier
                c if c.is_ascii_alphanumeric() || c == '_' => {
                    let word = self.read_word();
                    let token = classify_word(&word);
                    self.tokens.push(token);
                }

                c => {
                    return Err(IntelError::ParseError {
                        reason: format!("unexpected character: '{c}'"),
                    });
                }
            }
        }

        // Remove trailing newlines
        while self.tokens.last() == Some(&Token::Newline) {
            self.tokens.pop();
        }

        self.tokens.push(Token::Eof);
        Ok(self.tokens)
    }

    fn current(&self) -> char {
        self.input.as_bytes()[self.pos] as char
    }

    fn peek_next(&self) -> Option<char> {
        if self.pos + 1 < self.input.len() {
            Some(self.input.as_bytes()[self.pos + 1] as char)
        } else {
            None
        }
    }

    fn skip_spaces(&mut self) {
        while self.pos < self.input.len() {
            let ch = self.current();
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn read_word(&mut self) -> String {
        let start = self.pos;
        while self.pos < self.input.len() {
            let ch = self.current();
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '*' {
                self.pos += 1;
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_owned()
    }
}

fn classify_word(word: &str) -> Token {
    match word.to_lowercase().as_str() {
        // Node types
        "programs" => Token::Programs,
        "paragraphs" => Token::Paragraphs,
        "fields" => Token::Fields,
        "copybooks" => Token::Copybooks,
        "files" => Token::Files,
        "tables" => Token::Tables,
        "rules" => Token::Rules,

        // Traversal verbs
        "calling" => Token::Calling,
        "called-by" => Token::CalledBy,
        "performing" => Token::Performing,
        "performed-by" => Token::PerformedBy,
        "reading" => Token::Reading,
        "read-by" => Token::ReadBy,
        "writing" => Token::Writing,
        "written-by" => Token::WrittenBy,
        "using" => Token::Using,
        "used-by" => Token::UsedBy,
        "accessing" => Token::Accessing,
        "accessed-by" => Token::AccessedBy,
        "containing" => Token::Containing,
        "within" => Token::Within,

        // Domain verbs
        "trace" => Token::Trace,
        "rank" => Token::Rank,
        "similar" => Token::Similar,
        "find-dead" => Token::FindDead,
        "discover-processes" => Token::DiscoverProcesses,
        "estimate-cost" => Token::EstimateCost,
        "deps" => Token::Deps,
        "impact" => Token::Impact,
        "compare" => Token::Compare,
        "save" => Token::Save,
        "run" => Token::Run,

        // Modifiers
        "by" => Token::By,
        "top" => Token::Top,
        "in" => Token::In,
        "with" => Token::With,
        "depth" => Token::Depth,
        "level" => Token::Level,
        "order" => Token::Order,
        "scope" => Token::Scope,
        "threshold" => Token::Threshold,
        "as" => Token::As,

        // Logical
        "and" => Token::And,
        "or" => Token::Or,
        "not" => Token::Not,

        // Filter operators
        "has" => Token::Has,

        // Special
        "each" => Token::Each,

        // Everything else is an identifier
        _ => Token::Identifier(word.to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Vec<Token> {
        Lexer::new(input).tokenize().unwrap()
    }

    #[test]
    fn simple_traverse() {
        let tokens = lex("programs calling CLRG0100;");
        assert_eq!(
            tokens,
            vec![
                Token::Programs,
                Token::Calling,
                Token::Identifier("CLRG0100".to_owned()),
                Token::Semicolon,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn domain_verb_with_modifiers() {
        let tokens = lex("rank programs by complexity top 20;");
        assert_eq!(
            tokens,
            vec![
                Token::Rank,
                Token::Programs,
                Token::By,
                Token::Identifier("complexity".to_owned()),
                Token::Top,
                Token::NumberLit(20.0),
                Token::Semicolon,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn list_syntax() {
        let tokens = lex("[CLRG0100, VALUTIL]");
        assert_eq!(
            tokens,
            vec![
                Token::LeftBracket,
                Token::Identifier("CLRG0100".to_owned()),
                Token::Comma,
                Token::Identifier("VALUTIL".to_owned()),
                Token::RightBracket,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn filter_syntax() {
        let tokens = lex("(complexity > 4.0 and type = 'batch')");
        assert_eq!(
            tokens,
            vec![
                Token::LeftParen,
                Token::Identifier("complexity".to_owned()),
                Token::Gt,
                Token::NumberLit(4.0),
                Token::And,
                Token::Identifier("type".to_owned()),
                Token::Eq,
                Token::StringLit("batch".to_owned()),
                Token::RightParen,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn multiline_with_comments() {
        let tokens = lex(
            "-- find clearing programs\nprograms calling CLRG0100\nrank by complexity;",
        );
        assert_eq!(
            tokens,
            vec![
                Token::Programs,
                Token::Calling,
                Token::Identifier("CLRG0100".to_owned()),
                Token::Newline,
                Token::Rank,
                Token::By,
                Token::Identifier("complexity".to_owned()),
                Token::Semicolon,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn each_keyword() {
        let tokens = lex("programs using each;");
        assert_eq!(
            tokens,
            vec![Token::Programs, Token::Using, Token::Each, Token::Semicolon, Token::Eof]
        );
    }

    #[test]
    fn find_dead_hyphenated() {
        let tokens = lex("find-dead level paragraph;");
        assert_eq!(
            tokens,
            vec![
                Token::FindDead,
                Token::Level,
                Token::Identifier("paragraph".to_owned()),
                Token::Semicolon,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn operators() {
        let tokens = lex("(name ~ 'CLRG*')");
        assert_eq!(
            tokens,
            vec![
                Token::LeftParen,
                Token::Identifier("name".to_owned()),
                Token::Tilde,
                Token::StringLit("CLRG*".to_owned()),
                Token::RightParen,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn double_tilde_regex() {
        let tokens = lex("(name ~~ 'CL[A-Z]+')");
        assert_eq!(
            tokens,
            vec![
                Token::LeftParen,
                Token::Identifier("name".to_owned()),
                Token::DoubleTilde,
                Token::StringLit("CL[A-Z]+".to_owned()),
                Token::RightParen,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn continuation_dots_ignored() {
        let tokens = lex("programs calling CLRG0100\n  .. rank by risk;");
        assert_eq!(
            tokens,
            vec![
                Token::Programs,
                Token::Calling,
                Token::Identifier("CLRG0100".to_owned()),
                Token::Newline,
                Token::Rank,
                Token::By,
                Token::Identifier("risk".to_owned()),
                Token::Semicolon,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn unterminated_string_error() {
        let result = Lexer::new("(name = 'oops)").tokenize();
        assert!(result.is_err());
    }
}
