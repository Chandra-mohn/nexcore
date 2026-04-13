//! Simple arithmetic expression evaluator for cost formulas.
//!
//! Supports: `+`, `-`, `*`, `/`, parentheses, numeric literals,
//! variable references (`loc`, `complexity`), dotted config references
//! (`productivity.qa_ratio`), and cross-formula references (`manual.rewrite`).

use std::collections::HashMap;

/// A resolved set of variable bindings for expression evaluation.
pub type Bindings = HashMap<String, f64>;

/// Evaluate a formula string against a set of variable bindings.
///
/// Returns the computed f64 result, or an error message.
pub fn eval(formula: &str, bindings: &Bindings) -> Result<f64, String> {
    let tokens = tokenize(formula)?;
    let mut parser = ExprParser::new(&tokens, bindings);
    let result = parser.parse_expr()?;
    if parser.pos < parser.tokens.len() {
        return Err(format!(
            "unexpected token at position {}: {:?}",
            parser.pos, parser.tokens[parser.pos]
        ));
    }
    Ok(result)
}

// --- Tokenizer ---

#[derive(Debug, Clone, PartialEq)]
enum ExprToken {
    Number(f64),
    Ident(String),     // variable name or dotted reference
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

fn tokenize(input: &str) -> Result<Vec<ExprToken>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' | '\n' | '\r' => i += 1,
            '+' => { tokens.push(ExprToken::Plus); i += 1; }
            '-' => {
                // Disambiguate unary minus from subtraction:
                // If preceded by nothing, (, or an operator -> unary minus on the number
                let is_unary = tokens.is_empty()
                    || matches!(
                        tokens.last(),
                        Some(ExprToken::Plus | ExprToken::Minus | ExprToken::Star | ExprToken::Slash | ExprToken::LParen)
                    );
                if is_unary && i + 1 < chars.len() && (chars[i + 1].is_ascii_digit() || chars[i + 1] == '.') {
                    // Parse negative number
                    let start = i;
                    i += 1;
                    while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                        i += 1;
                    }
                    let num_str: String = chars[start..i].iter().collect();
                    let val = num_str.parse::<f64>().map_err(|e| format!("bad number '{num_str}': {e}"))?;
                    tokens.push(ExprToken::Number(val));
                } else {
                    tokens.push(ExprToken::Minus);
                    i += 1;
                }
            }
            '*' => { tokens.push(ExprToken::Star); i += 1; }
            '/' => { tokens.push(ExprToken::Slash); i += 1; }
            '(' => { tokens.push(ExprToken::LParen); i += 1; }
            ')' => { tokens.push(ExprToken::RParen); i += 1; }
            c if c.is_ascii_digit() || c == '.' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                let val = num_str.parse::<f64>().map_err(|e| format!("bad number '{num_str}': {e}"))?;
                tokens.push(ExprToken::Number(val));
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_' || chars[i] == '.') {
                    i += 1;
                }
                let ident: String = chars[start..i].iter().collect();
                tokens.push(ExprToken::Ident(ident));
            }
            c => return Err(format!("unexpected character '{c}' at position {i}")),
        }
    }

    Ok(tokens)
}

// --- Recursive descent parser ---

struct ExprParser<'a> {
    tokens: &'a [ExprToken],
    pos: usize,
    bindings: &'a Bindings,
}

impl<'a> ExprParser<'a> {
    fn new(tokens: &'a [ExprToken], bindings: &'a Bindings) -> Self {
        Self { tokens, pos: 0, bindings }
    }

    fn peek(&self) -> Option<&ExprToken> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&ExprToken> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    /// expr = term (('+' | '-') term)*
    fn parse_expr(&mut self) -> Result<f64, String> {
        let mut left = self.parse_term()?;
        while let Some(op) = self.peek() {
            match op {
                ExprToken::Plus => { self.advance(); left += self.parse_term()?; }
                ExprToken::Minus => { self.advance(); left -= self.parse_term()?; }
                _ => break,
            }
        }
        Ok(left)
    }

    /// term = factor (('*' | '/') factor)*
    fn parse_term(&mut self) -> Result<f64, String> {
        let mut left = self.parse_factor()?;
        while let Some(op) = self.peek() {
            match op {
                ExprToken::Star => { self.advance(); left *= self.parse_factor()?; }
                ExprToken::Slash => {
                    self.advance();
                    let right = self.parse_factor()?;
                    if right == 0.0 {
                        return Err("division by zero".to_owned());
                    }
                    left /= right;
                }
                _ => break,
            }
        }
        Ok(left)
    }

    /// factor = Number | Ident | '(' expr ')'
    fn parse_factor(&mut self) -> Result<f64, String> {
        match self.peek() {
            Some(ExprToken::Number(_)) => {
                if let Some(ExprToken::Number(val)) = self.advance().cloned() {
                    Ok(val)
                } else {
                    Err("expected number".to_owned())
                }
            }
            Some(ExprToken::Ident(_)) => {
                if let Some(ExprToken::Ident(name)) = self.advance().cloned() {
                    self.bindings.get(&name).copied().ok_or_else(|| {
                        format!("undefined variable '{name}'")
                    })
                } else {
                    Err("expected identifier".to_owned())
                }
            }
            Some(ExprToken::LParen) => {
                self.advance(); // consume '('
                let val = self.parse_expr()?;
                match self.peek() {
                    Some(ExprToken::RParen) => { self.advance(); Ok(val) }
                    _ => Err("expected ')'".to_owned()),
                }
            }
            Some(tok) => Err(format!("unexpected token: {tok:?}")),
            None => Err("unexpected end of expression".to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(vars: &[(&str, f64)]) -> Bindings {
        vars.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    #[test]
    fn simple_arithmetic() {
        assert_eq!(eval("2 + 3", &b(&[])).unwrap(), 5.0);
        assert_eq!(eval("10 - 4", &b(&[])).unwrap(), 6.0);
        assert_eq!(eval("3 * 4", &b(&[])).unwrap(), 12.0);
        assert_eq!(eval("15 / 3", &b(&[])).unwrap(), 5.0);
    }

    #[test]
    fn operator_precedence() {
        assert_eq!(eval("2 + 3 * 4", &b(&[])).unwrap(), 14.0);
        assert_eq!(eval("(2 + 3) * 4", &b(&[])).unwrap(), 20.0);
    }

    #[test]
    fn variable_references() {
        let bindings = b(&[("loc", 1000.0), ("complexity", 5.0)]);
        assert_eq!(eval("loc / 100", &bindings).unwrap(), 10.0);
        assert_eq!(eval("loc / 100 * complexity", &bindings).unwrap(), 50.0);
    }

    #[test]
    fn dotted_references() {
        let bindings = b(&[
            ("productivity.qa_ratio", 1.5),
            ("rewrite", 10.0),
        ]);
        assert_eq!(eval("rewrite * productivity.qa_ratio", &bindings).unwrap(), 15.0);
    }

    #[test]
    fn cross_formula_references() {
        let bindings = b(&[
            ("manual.qa_testing", 30.0),
            ("nex_reduction.qa_reduction", 0.6),
        ]);
        let result = eval("manual.qa_testing * (1 - nex_reduction.qa_reduction)", &bindings).unwrap();
        assert!((result - 12.0).abs() < 0.001);
    }

    #[test]
    fn nested_parens() {
        assert_eq!(eval("((2 + 3) * (4 + 1))", &b(&[])).unwrap(), 25.0);
    }

    #[test]
    fn negative_number() {
        assert_eq!(eval("-3 + 5", &b(&[])).unwrap(), 2.0);
    }

    #[test]
    fn division_by_zero() {
        let err = eval("10 / 0", &b(&[])).unwrap_err();
        assert!(err.contains("division by zero"));
    }

    #[test]
    fn undefined_variable() {
        let err = eval("foo + 1", &b(&[])).unwrap_err();
        assert!(err.contains("undefined variable 'foo'"));
    }

    #[test]
    fn zero_literal() {
        assert_eq!(eval("0", &b(&[])).unwrap(), 0.0);
    }

    #[test]
    fn decimal_literals() {
        assert_eq!(eval("3.14 * 2", &b(&[])).unwrap(), 6.28);
    }

    #[test]
    fn complex_formula() {
        let bindings = b(&[
            ("loc", 2000.0),
            ("code_multiplier", 2.5),
            ("productivity.manual_loc_per_day", 100.0),
        ]);
        let result = eval("loc / productivity.manual_loc_per_day * code_multiplier", &bindings).unwrap();
        assert_eq!(result, 50.0);
    }
}
