//! Hand-written JCL parser -- line-oriented, like the BMS parser.
//!
//! Parses JCL source text into `JclSource` AST. Handles:
//! - JOB card with keyword parameters
//! - EXEC PGM= and EXEC proc steps
//! - DD statements (DSN, DISP, DCB, SYSOUT, DUMMY, inline data, concatenation)
//! - IF/THEN/ELSE/ENDIF conditional processing
//! - INCLUDE, SET, JCLLIB
//! - PROC/PEND procedure definitions
//! - Continuation lines (trailing comma or non-blank col 72)
//! - Comment statements (//* ...)

use std::collections::HashMap;
use std::path::Path;

use crate::ast::{
    CondOperator, CondParam, CondTest, DatasetDd, DdKind, DdStatement, DispAction,
    DispStatus, Disposition, ExecType, IfCondition, IfStatement, IfTest, JclJob, JclProc,
    JclSource, JclStep, JobBodyItem, JobParams, PathDd, StepParams, SysoutDd,
};
use crate::error::{JclError, Result};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Parse JCL from a file path.
pub fn parse_jcl_file(path: &Path) -> Result<JclSource> {
    let source = std::fs::read_to_string(path)?;
    parse_jcl(&source)
}

/// Parse JCL source text into a typed AST.
pub fn parse_jcl(source: &str) -> Result<JclSource> {
    let lines = preprocess(source);
    let mut parser = JclParser::new(&lines);
    parser.parse()
}

// ---------------------------------------------------------------------------
// Preprocessing
// ---------------------------------------------------------------------------

/// A preprocessed logical line with its original line number.
#[derive(Debug)]
struct LogicalLine {
    /// 1-based line number of the first physical line.
    line_num: usize,
    /// The joined content (after continuation processing).
    content: String,
}

/// Preprocess JCL: join continuation lines.
///
/// JCL continuation: if a non-comment line ends with a comma (possibly
/// followed by a comment), the next line's content (after //) is appended.
fn preprocess(source: &str) -> Vec<LogicalLine> {
    let mut logical_lines = Vec::new();
    let mut current: Option<(usize, String)> = None;

    for (idx, raw_line) in source.lines().enumerate() {
        let line_num = idx + 1;

        // Blank lines end any continuation
        if raw_line.trim().is_empty() {
            if let Some((start, content)) = current.take() {
                logical_lines.push(LogicalLine { line_num: start, content });
            }
            continue;
        }

        // JCL lines start with // (except delimiter /* and JES2 /*)
        // Comment lines: //*
        if raw_line.starts_with("//*") {
            // Flush current continuation
            if let Some((start, content)) = current.take() {
                logical_lines.push(LogicalLine { line_num: start, content });
            }
            logical_lines.push(LogicalLine {
                line_num,
                content: raw_line.to_string(),
            });
            continue;
        }

        if let Some(payload) = raw_line.strip_prefix("//") {
            if let Some((_start, ref mut content)) = current {
                // Continuation: append this line's content
                let trimmed = payload.trim_start();
                content.push(' ');
                content.push_str(trimmed);

                // Check if this line also continues
                if !ends_with_continuation(content) {
                    let finished = current.take().unwrap();
                    logical_lines.push(LogicalLine { line_num: finished.0, content: finished.1 });
                }
            } else {
                // New statement
                let line_content = format!("//{payload}");
                if ends_with_continuation(&line_content) {
                    current = Some((line_num, line_content));
                } else {
                    logical_lines.push(LogicalLine { line_num, content: line_content });
                }
            }
        } else if raw_line.starts_with("/*") {
            // JES2 control or delimiter
            if let Some((start, content)) = current.take() {
                logical_lines.push(LogicalLine { line_num: start, content });
            }
            logical_lines.push(LogicalLine { line_num, content: raw_line.to_string() });
        } else {
            // Inline data or other content -- pass through
            if let Some((start, content)) = current.take() {
                logical_lines.push(LogicalLine { line_num: start, content });
            }
            logical_lines.push(LogicalLine { line_num, content: raw_line.to_string() });
        }
    }

    // Flush remaining
    if let Some((start, content)) = current.take() {
        logical_lines.push(LogicalLine { line_num: start, content });
    }

    logical_lines
}

/// Check if a JCL line ends with a continuation marker.
/// Continuation is indicated by a trailing comma (possibly before a comment).
fn ends_with_continuation(line: &str) -> bool {
    // Strip inline comment if present (content after space-space or in cols 72+)
    let effective = strip_inline_comment(line);
    effective.trim_end().ends_with(',')
}

/// Strip an inline comment from a JCL line.
/// In practice, comments start after column 72 or after double-space in parameters.
fn strip_inline_comment(line: &str) -> &str {
    // Simple heuristic: if line is longer than 71 chars, content after col 71 is comment
    if line.len() > 71 {
        &line[..71]
    } else {
        line
    }
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

struct JclParser<'a> {
    lines: &'a [LogicalLine],
    pos: usize,
}

impl<'a> JclParser<'a> {
    fn new(lines: &'a [LogicalLine]) -> Self {
        Self { lines, pos: 0 }
    }

    fn parse(&mut self) -> Result<JclSource> {
        // Skip leading JES2 control statements and comments
        self.skip_jes2_and_comments();

        // Detect if this is a PROC or a JOB
        if self.peek_is_proc() {
            let proc = self.parse_proc()?;
            Ok(JclSource::Proc(proc))
        } else {
            let job = self.parse_job()?;
            Ok(JclSource::Job(job))
        }
    }

    fn at_end(&self) -> bool {
        self.pos >= self.lines.len()
    }

    fn current(&self) -> Option<&LogicalLine> {
        self.lines.get(self.pos)
    }

    fn advance(&mut self) -> Option<&LogicalLine> {
        let line = self.lines.get(self.pos);
        if line.is_some() {
            self.pos += 1;
        }
        line
    }

    fn current_line_num(&self) -> usize {
        self.current().map_or(0, |l| l.line_num)
    }

    // -- Peeking helpers --

    fn skip_jes2_and_comments(&mut self) {
        while let Some(line) = self.current() {
            let content = line.content.trim();
            if content.starts_with("//*") || content.starts_with("/*") {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn peek_is_proc(&self) -> bool {
        for line in &self.lines[self.pos..] {
            let upper = line.content.to_uppercase();
            let trimmed = upper.trim();
            if trimmed.starts_with("//*") || trimmed.starts_with("/*") {
                continue;
            }
            if let Some(after_slashes) = trimmed.strip_prefix("//") {
                let payload = after_slashes.trim();
                // PROC keyword without JOB coming first
                if payload.starts_with("PROC ") || payload.eq_ignore_ascii_case("PROC") {
                    return true;
                }
                // Named: NAME PROC ...
                let tokens: Vec<&str> = payload.split_whitespace().collect();
                if tokens.len() >= 2 && tokens[1].eq_ignore_ascii_case("PROC") {
                    return true;
                }
                // If we see JOB, it's not a proc
                if tokens.len() >= 2 && tokens[1].eq_ignore_ascii_case("JOB") {
                    return false;
                }
            }
            break;
        }
        false
    }

    // -- JOB parsing --

    fn parse_job(&mut self) -> Result<JclJob> {
        self.skip_jes2_and_comments();

        // Parse JOB card
        let _line = self.current().ok_or(JclError::MissingJob)?;
        let (name, params, accounting, programmer) = self.parse_job_card()?;

        let mut job = JclJob {
            name,
            accounting,
            programmer,
            params,
            jcllib: Vec::new(),
            joblib: Vec::new(),
            body: Vec::new(),
            comments: Vec::new(),
        };

        // Parse body
        while !self.at_end() {
            let line = self.current().unwrap();
            let content = &line.content;

            if let Some(comment_text) = content.strip_prefix("//*") {
                let comment = comment_text.trim().to_string();
                job.comments.push(comment.clone());
                job.body.push(JobBodyItem::Comment(comment));
                self.pos += 1;
                continue;
            }

            if content.starts_with("/*") || content.trim().is_empty() {
                self.pos += 1;
                continue;
            }

            if !content.starts_with("//") {
                // Inline data or non-JCL -- skip
                self.pos += 1;
                continue;
            }

            let payload = content[2..].trim().to_string();
            let upper = payload.to_uppercase();

            // JCLLIB
            if contains_keyword(&upper, "JCLLIB") {
                if let Some(order) = extract_parm_value(&payload, "ORDER") {
                    job.jcllib = parse_paren_list(&order);
                }
                self.pos += 1;
                continue;
            }

            // JOBLIB DD
            if starts_with_dd_name(&upper, "JOBLIB") {
                let dd = self.parse_dd_statement()?;
                job.joblib.push(dd);
                continue;
            }

            // IF
            if contains_keyword(&upper, "IF") && upper.contains("THEN") {
                let if_stmt = parse_if_statement(&payload);
                job.body.push(JobBodyItem::If(if_stmt));
                self.pos += 1;
                continue;
            }

            // ELSE
            if is_keyword_statement(&upper, "ELSE") {
                job.body.push(JobBodyItem::Else);
                self.pos += 1;
                continue;
            }

            // ENDIF
            if is_keyword_statement(&upper, "ENDIF") {
                job.body.push(JobBodyItem::EndIf);
                self.pos += 1;
                continue;
            }

            // INCLUDE
            if contains_keyword(&upper, "INCLUDE") {
                if let Some(member) = extract_parm_value(&payload, "MEMBER") {
                    job.body.push(JobBodyItem::Include(member));
                }
                self.pos += 1;
                continue;
            }

            // SET
            if contains_keyword(&upper, "SET") {
                if let Some((sym, val)) = parse_set_statement(&payload) {
                    job.body.push(JobBodyItem::Set { symbol: sym, value: val });
                }
                self.pos += 1;
                continue;
            }

            // EXEC -- start of a step
            if contains_keyword(&upper, "EXEC") {
                let step = self.parse_step()?;
                job.body.push(JobBodyItem::Step(step));
                continue;
            }

            // Null statement (//) or unrecognized -- skip
            self.pos += 1;
        }

        Ok(job)
    }

    fn parse_job_card(&mut self) -> Result<(String, JobParams, Option<String>, Option<String>)> {
        let line = self.advance().ok_or(JclError::MissingJob)?;
        let content = &line.content;

        if !content.starts_with("//") {
            return Err(JclError::MissingJob);
        }

        let payload = content[2..].trim();
        let tokens: Vec<&str> = payload.split_whitespace().collect();

        if tokens.len() < 2 || !tokens[1].to_uppercase().starts_with("JOB") {
            return Err(JclError::MissingJob);
        }

        let name = tokens[0].to_string();
        // Find where JOB keyword ends and rest begins
        let rest = find_after_keyword(payload, "JOB");

        // Parse positional params (accounting, programmer name) and keyword params
        let (accounting, programmer, keyword_str) = parse_job_positionals(rest);
        let params = parse_job_params(&keyword_str);

        Ok((name, params, accounting, programmer))
    }

    // -- STEP parsing --

    fn parse_step(&mut self) -> Result<JclStep> {
        let line_num = self.current_line_num();
        let line = self.advance().ok_or(JclError::MissingExec { line: line_num })?;
        let content = &line.content;
        let payload = content[2..].trim();

        let (name, exec, params) = parse_exec_statement(payload)?;

        let mut step = JclStep {
            name,
            exec,
            params,
            dd_statements: Vec::new(),
            comments: Vec::new(),
        };

        // Parse DD statements until we hit the next EXEC, JOB, IF, ELSE, ENDIF, or end
        while !self.at_end() {
            let line = self.current().unwrap();
            let content = &line.content;

            if let Some(comment_text) = content.strip_prefix("//*") {
                step.comments.push(comment_text.trim().to_string());
                self.pos += 1;
                continue;
            }

            if content.starts_with("/*") {
                self.pos += 1;
                continue;
            }

            if !content.starts_with("//") {
                // Could be inline data -- check if we're expecting it
                self.pos += 1;
                continue;
            }

            let payload = content[2..].trim();
            let upper = payload.to_uppercase();

            // Check if this is the start of a new step or control statement
            if contains_keyword(&upper, "EXEC")
                || contains_keyword(&upper, "IF") && upper.contains("THEN")
                || is_keyword_statement(&upper, "ELSE")
                || is_keyword_statement(&upper, "ENDIF")
                || contains_keyword(&upper, "INCLUDE")
                || contains_keyword(&upper, "SET")
                || payload.is_empty()
            {
                break;
            }

            // DD statement
            if contains_keyword(&upper, "DD") || upper.contains(" DD ") || upper.contains(" DD,") {
                let dd = self.parse_dd_statement()?;
                step.dd_statements.push(dd);
            } else {
                // OUTPUT, CNTL, or other -- skip
                self.pos += 1;
            }
        }

        Ok(step)
    }

    // -- DD parsing --

    fn parse_dd_statement(&mut self) -> Result<DdStatement> {
        let line_num = self.current_line_num();
        let line = self.advance().ok_or(JclError::Syntax {
            line: line_num,
            message: "expected DD statement".to_string(),
        })?;
        let payload = line.content[2..].trim();
        let line_num = line.line_num;

        // Parse: [stepname.]ddname DD params...
        let (dd_name, step_qual, dd_rest) = split_dd_name_and_params(payload);

        let kind = parse_dd_kind(&dd_rest, line_num);

        let mut dd = DdStatement {
            name: dd_name,
            step_qualifier: step_qual,
            kind,
            concatenations: Vec::new(),
        };

        // Check for concatenation: next line starts with // DD (no name field)
        while !self.at_end() {
            let next = self.current().unwrap();
            let next_content = next.content.trim().to_string();
            let next_line_num = next.line_num;
            if let Some(after_slashes) = next_content.strip_prefix("//") {
                let next_payload = after_slashes.trim();
                let next_upper = next_payload.to_uppercase();
                // Concatenation: starts with DD (no name)
                if next_upper.starts_with("DD ") || next_upper == "DD" {
                    self.pos += 1;
                    let concat_rest = if next_payload.len() > 3 { &next_payload[3..] } else { "" };
                    let concat_kind = parse_dd_kind(concat_rest, next_line_num);
                    dd.concatenations.push(concat_kind);
                    continue;
                }
            }
            break;
        }

        Ok(dd)
    }

    // -- PROC parsing --

    fn parse_proc(&mut self) -> Result<JclProc> {
        self.skip_jes2_and_comments();

        let line_num = self.current_line_num();
        let line = self.advance().ok_or(JclError::Syntax {
            line: line_num,
            message: "expected PROC statement".to_string(),
        })?;

        let payload = line.content[2..].trim();
        let tokens: Vec<&str> = payload.split_whitespace().collect();

        let (name, symbols_str) = if tokens.first().is_some_and(|t| t.eq_ignore_ascii_case("PROC")) {
            (None, tokens.get(1..).map(|s| s.join(" ")).unwrap_or_default())
        } else if tokens.get(1).is_some_and(|t| t.eq_ignore_ascii_case("PROC")) {
            (Some(tokens[0].to_string()), tokens.get(2..).map(|s| s.join(" ")).unwrap_or_default())
        } else {
            (None, String::new())
        };

        let symbols = parse_symbolic_params(&symbols_str);

        let mut proc = JclProc {
            name,
            symbols,
            body: Vec::new(),
        };

        // Parse body until PEND or end
        while !self.at_end() {
            let line = self.current().unwrap();
            let content = &line.content;
            let upper = content.to_uppercase();

            if upper.contains("PEND") {
                self.pos += 1;
                break;
            }

            if let Some(comment_text) = content.strip_prefix("//*") {
                proc.body.push(JobBodyItem::Comment(comment_text.trim().to_string()));
                self.pos += 1;
                continue;
            }

            if !content.starts_with("//") {
                self.pos += 1;
                continue;
            }

            let payload_upper = content[2..].trim().to_uppercase();

            if contains_keyword(&payload_upper, "EXEC") {
                let step = self.parse_step()?;
                proc.body.push(JobBodyItem::Step(step));
            } else {
                self.pos += 1;
            }
        }

        Ok(proc)
    }
}

// ---------------------------------------------------------------------------
// Statement-level parsing helpers
// ---------------------------------------------------------------------------

/// Parse the JOB card's positional parameters (accounting, programmer name).
fn parse_job_positionals(rest: &str) -> (Option<String>, Option<String>, String) {
    let rest = rest.trim();
    if rest.is_empty() {
        return (None, None, String::new());
    }

    // JOB positionals are complex: accounting can be in parens, programmer name in quotes.
    // Simplified: split by comma, first = accounting, second (if quoted) = programmer.
    let mut accounting = None;
    let mut programmer = None;
    let mut keyword_start = 0;

    let chars: Vec<char> = rest.chars().collect();
    let mut i = 0;
    let mut paren_depth = 0;
    let mut in_quote = false;

    // Skip accounting info
    while i < chars.len() {
        let ch = chars[i];
        if in_quote {
            if ch == '\'' {
                if i + 1 < chars.len() && chars[i + 1] == '\'' {
                    i += 2;
                    continue;
                }
                in_quote = false;
            }
            i += 1;
            continue;
        }
        match ch {
            '\'' => { in_quote = true; i += 1; }
            '(' => { paren_depth += 1; i += 1; }
            ')' => { paren_depth -= 1; i += 1; }
            ',' if paren_depth == 0 => {
                let acct = rest[..i].trim();
                if !acct.is_empty() {
                    accounting = Some(acct.to_string());
                }
                i += 1;
                break;
            }
            ' ' if paren_depth == 0 => {
                // Space without comma -- rest is keyword params
                let acct = rest[..i].trim();
                if !acct.is_empty() {
                    accounting = Some(acct.to_string());
                }
                keyword_start = i;
                return (accounting, programmer, rest[keyword_start..].trim().to_string());
            }
            _ => { i += 1; }
        }
    }

    // After first comma, check for programmer name
    if i < chars.len() {
        let remaining = &rest[i..];
        let trimmed = remaining.trim_start();
        if let Some(after_quote) = trimmed.strip_prefix('\'') {
            // Quoted programmer name
            if let Some(end_quote) = after_quote.find('\'') {
                let prog_name = &after_quote[..end_quote];
                programmer = Some(prog_name.to_string());
                keyword_start = i + (remaining.len() - trimmed.len()) + end_quote + 2;
                // Skip trailing comma if present
                if keyword_start < rest.len() && rest.as_bytes().get(keyword_start) == Some(&b',') {
                    keyword_start += 1;
                }
            }
        } else {
            // Unquoted: next token up to comma or space
            let end = trimmed.find([',', ' ']).unwrap_or(trimmed.len());
            let prog = &trimmed[..end];
            if !prog.is_empty() && !prog.contains('=') {
                programmer = Some(prog.to_string());
                keyword_start = i + (remaining.len() - trimmed.len()) + end;
                if keyword_start < rest.len() && rest.as_bytes().get(keyword_start) == Some(&b',') {
                    keyword_start += 1;
                }
            } else {
                keyword_start = i;
            }
        }
    }

    let keyword_str = if keyword_start < rest.len() {
        rest[keyword_start..].trim().to_string()
    } else {
        String::new()
    };

    (accounting, programmer, keyword_str)
}

/// Parse JOB keyword parameters from a comma-separated string.
fn parse_job_params(text: &str) -> JobParams {
    let mut params = JobParams::default();
    let opts = parse_keyword_params(text);

    for (key, value) in &opts {
        match key.to_uppercase().as_str() {
            "CLASS" => params.class = Some(value.clone()),
            "MSGCLASS" => params.msgclass = Some(value.clone()),
            "MSGLEVEL" => params.msglevel = Some(parse_msglevel(value)),
            "NOTIFY" => params.notify = Some(value.clone()),
            "COND" => params.cond = parse_cond_param(value),
            "REGION" => params.region = Some(value.clone()),
            "TIME" => params.time = Some(value.clone()),
            "PRTY" => params.priority = value.parse().ok(),
            "TYPRUN" => params.typrun = Some(value.clone()),
            "RESTART" => params.restart = Some(value.clone()),
            "SCHENV" => params.schenv = Some(value.clone()),
            _ => { params.raw.insert(key.clone(), value.clone()); }
        }
    }

    params
}

/// Parse EXEC statement: [stepname] EXEC PGM=name|procname,PARM=...,COND=...
fn parse_exec_statement(payload: &str) -> Result<(Option<String>, ExecType, StepParams)> {
    let tokens: Vec<&str> = payload.split_whitespace().collect();

    let (step_name, _exec_idx) = if tokens.first().is_some_and(|t| t.eq_ignore_ascii_case("EXEC")) {
        (None, 0)
    } else if tokens.get(1).is_some_and(|t| t.eq_ignore_ascii_case("EXEC")) {
        (Some(tokens[0].to_string()), 1)
    } else {
        return Err(JclError::Syntax {
            line: 0,
            message: format!("expected EXEC statement, got: {payload}"),
        });
    };

    let rest = find_after_keyword(payload, "EXEC");
    let rest = rest.trim();

    // Parse the EXEC parameters
    let opts = parse_keyword_params(rest);

    let exec = if let Some(pgm) = opts.get("PGM") {
        ExecType::Pgm(pgm.clone())
    } else {
        // First non-keyword token is the proc name
        let first_token = rest.split(',').next().unwrap_or("").trim();
        let proc_name = if let Some(stripped) = first_token.strip_prefix("PROC=") {
            stripped.trim().to_string()
        } else if !first_token.contains('=') {
            first_token.to_string()
        } else {
            String::new()
        };
        let overrides: HashMap<String, String> = opts.iter()
            .filter(|(k, _)| {
                let ku = k.to_uppercase();
                !matches!(ku.as_str(), "PGM" | "PROC" | "PARM" | "COND" | "REGION"
                    | "TIME" | "ACCT" | "ADDRSPC" | "DYNAMNBR" | "MEMLIMIT" | "PARMDD")
            })
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        ExecType::Proc { name: proc_name, overrides }
    };

    let mut params = StepParams::default();
    for (key, value) in &opts {
        match key.to_uppercase().as_str() {
            "PARM" => params.parm = Some(value.clone()),
            "COND" => params.cond = parse_cond_param(value),
            "REGION" => params.region = Some(value.clone()),
            "TIME" => params.time = Some(value.clone()),
            "ACCT" => params.acct = Some(value.clone()),
            "ADDRSPC" => params.addrspc = Some(value.clone()),
            "DYNAMNBR" => params.dynamnbr = Some(value.clone()),
            "MEMLIMIT" => params.memlimit = Some(value.clone()),
            "PARMDD" => params.parmdd = Some(value.clone()),
            "PGM" | "PROC" => {} // Already handled
            _ => { params.raw.insert(key.clone(), value.clone()); }
        }
    }

    Ok((step_name, exec, params))
}

/// Split DD statement line into (dd_name, step_qualifier, rest_params).
fn split_dd_name_and_params(payload: &str) -> (String, Option<String>, String) {
    let tokens: Vec<&str> = payload.split_whitespace().collect();

    if tokens.is_empty() {
        return (String::new(), None, String::new());
    }

    // Find the DD keyword
    if tokens[0].eq_ignore_ascii_case("DD") {
        // Concatenation (no name)
        let rest = find_after_keyword(payload, "DD");
        return (String::new(), None, rest.to_string());
    }

    if tokens.get(1).is_some_and(|t| t.eq_ignore_ascii_case("DD")) {
        let name_part = tokens[0];
        // Handle step.ddname qualification
        let (dd_name, step_qual) = if name_part.contains('.') {
            let parts: Vec<&str> = name_part.splitn(2, '.').collect();
            (parts[1].to_string(), Some(parts[0].to_string()))
        } else {
            (name_part.to_string(), None)
        };
        let rest = find_after_keyword(payload, "DD");
        return (dd_name, step_qual, rest.to_string());
    }

    // Might be "NAME DD,PARAMS" without space -- shouldn't normally happen
    (tokens[0].to_string(), None, tokens.get(1..).map(|s| s.join(" ")).unwrap_or_default())
}

/// Parse DD parameters into a DdKind.
fn parse_dd_kind(params_str: &str, _line_num: usize) -> DdKind {
    let params_str = params_str.trim();
    let upper = params_str.to_uppercase();

    // DUMMY
    if upper.starts_with("DUMMY") || upper == "DUMMY" {
        return DdKind::Dummy;
    }

    // DYNAM
    if upper.starts_with("DYNAM") {
        return DdKind::Dynam;
    }

    // Inline data: * or DATA
    if params_str.starts_with('*') {
        let dlm = extract_parm_value(params_str, "DLM");
        return DdKind::InlineData { data: Vec::new(), delimiter: dlm };
    }
    if upper.starts_with("DATA") {
        let dlm = extract_parm_value(params_str, "DLM");
        return DdKind::InlineData { data: Vec::new(), delimiter: dlm };
    }

    // SYSOUT
    if let Some(class) = extract_parm_value(params_str, "SYSOUT") {
        let opts = parse_keyword_params(params_str);
        return DdKind::Sysout(SysoutDd {
            class,
            dest: opts.get("DEST").cloned(),
            copies: opts.get("COPIES").and_then(|v| v.parse().ok()),
            raw: opts,
        });
    }

    // DDNAME= reference
    if let Some(ref_name) = extract_parm_value(params_str, "DDNAME") {
        return DdKind::DdRef(ref_name);
    }

    // PATH= (HFS)
    if let Some(path) = extract_parm_value(params_str, "PATH") {
        let opts = parse_keyword_params(params_str);
        return DdKind::Path(PathDd {
            path,
            pathdisp: opts.get("PATHDISP").cloned(),
            pathmode: opts.get("PATHMODE").cloned(),
            pathopts: opts.get("PATHOPTS").cloned(),
        });
    }

    // Dataset DD (DSN= or default)
    let opts = parse_keyword_params(params_str);
    let mut ds = DatasetDd::default();

    if let Some(dsn_raw) = opts.get("DSN").or_else(|| opts.get("DSNAME")) {
        // Parse DSN=name(member)
        if let Some(paren_start) = dsn_raw.find('(') {
            ds.dsn = Some(dsn_raw[..paren_start].to_string());
            let member = dsn_raw[paren_start + 1..].trim_end_matches(')');
            ds.member = Some(member.to_string());
        } else {
            ds.dsn = Some(dsn_raw.clone());
        }
    }

    if let Some(disp_str) = opts.get("DISP") {
        ds.disp = parse_disp(disp_str);
    }

    // DCB parameters: may be DCB=(RECFM=FB,LRECL=80,...) or standalone RECFM=FB
    // If DCB= is present, expand its sub-parameters
    if let Some(dcb_val) = opts.get("DCB") {
        let dcb_inner = dcb_val.trim_matches(|c| c == '(' || c == ')');
        let dcb_sub = parse_keyword_params(dcb_inner);
        ds.dcb.recfm = dcb_sub.get("RECFM").cloned();
        ds.dcb.lrecl = dcb_sub.get("LRECL").and_then(|v| v.parse().ok());
        ds.dcb.blksize = dcb_sub.get("BLKSIZE").and_then(|v| v.parse().ok());
        ds.dcb.dsorg = dcb_sub.get("DSORG").cloned();
    }
    // Also check for standalone DCB params (modern JCL allows RECFM= without DCB=)
    if ds.dcb.recfm.is_none() { ds.dcb.recfm = opts.get("RECFM").cloned(); }
    if ds.dcb.lrecl.is_none() { ds.dcb.lrecl = opts.get("LRECL").and_then(|v| v.parse().ok()); }
    if ds.dcb.blksize.is_none() { ds.dcb.blksize = opts.get("BLKSIZE").and_then(|v| v.parse().ok()); }
    if ds.dcb.dsorg.is_none() { ds.dcb.dsorg = opts.get("DSORG").cloned(); }

    ds.space = opts.get("SPACE").cloned();
    ds.unit = opts.get("UNIT").cloned();
    ds.volume = opts.get("VOL").or_else(|| opts.get("VOLUME")).cloned();
    ds.label = opts.get("LABEL").cloned();
    ds.dataclas = opts.get("DATACLAS").cloned();
    ds.storclas = opts.get("STORCLAS").cloned();
    ds.mgmtclas = opts.get("MGMTCLAS").cloned();
    ds.recorg = opts.get("RECORG").cloned();
    ds.keyoff = opts.get("KEYOFF").cloned();
    ds.keylen = opts.get("KEYLEN").cloned();
    ds.like = opts.get("LIKE").cloned();
    ds.refdd = opts.get("REFDD").cloned();
    ds.expdt = opts.get("EXPDT").cloned();
    ds.retpd = opts.get("RETPD").cloned();
    ds.dsntype = opts.get("DSNTYPE").cloned();

    // Store remaining raw params
    for (k, v) in &opts {
        let ku = k.to_uppercase();
        if !matches!(ku.as_str(), "DSN" | "DSNAME" | "DISP" | "RECFM" | "LRECL"
            | "BLKSIZE" | "DSORG" | "SPACE" | "UNIT" | "VOL" | "VOLUME"
            | "LABEL" | "DATACLAS" | "STORCLAS" | "MGMTCLAS" | "RECORG"
            | "KEYOFF" | "KEYLEN" | "LIKE" | "REFDD" | "EXPDT" | "RETPD" | "DSNTYPE")
        {
            ds.raw.insert(k.clone(), v.clone());
        }
    }

    DdKind::Dataset(ds)
}

// ---------------------------------------------------------------------------
// Parameter parsing utilities
// ---------------------------------------------------------------------------

/// Parse comma-separated KEY=VALUE parameters, respecting parens and quotes.
fn parse_keyword_params(text: &str) -> HashMap<String, String> {
    let mut opts = HashMap::new();
    let text = text.trim();
    if text.is_empty() {
        return opts;
    }

    let mut key = String::new();
    let mut value = String::new();
    let mut in_key = true;
    let mut paren_depth: i32 = 0;
    let mut in_quote = false;

    for ch in text.chars() {
        if in_quote {
            value.push(ch);
            if ch == '\'' {
                in_quote = false;
            }
            continue;
        }

        match ch {
            '\'' => {
                in_quote = true;
                value.push(ch);
            }
            '(' if !in_key => {
                paren_depth += 1;
                value.push(ch);
            }
            ')' if !in_key => {
                paren_depth -= 1;
                value.push(ch);
            }
            '=' if in_key && paren_depth == 0 => {
                in_key = false;
            }
            ',' if paren_depth == 0 && !in_quote => {
                flush_param(&mut opts, &mut key, &mut value);
                in_key = true;
            }
            ' ' | '\t' if paren_depth == 0 && in_key && key.is_empty() => {
                // Skip leading whitespace
            }
            ' ' | '\t' if paren_depth == 0 && !in_quote && value.is_empty() && in_key => {
                // Token without = (positional or flag). Flush as flag.
                if !key.trim().is_empty() {
                    opts.insert(key.trim().to_uppercase(), String::new());
                    key.clear();
                }
            }
            _ => {
                if in_key {
                    key.push(ch);
                } else {
                    value.push(ch);
                }
            }
        }
    }

    flush_param(&mut opts, &mut key, &mut value);
    opts
}

fn flush_param(opts: &mut HashMap<String, String>, key: &mut String, value: &mut String) {
    let k = key.trim().to_uppercase();
    let v = value.trim().to_string();
    if !k.is_empty() {
        opts.insert(k, v);
    }
    key.clear();
    value.clear();
}

/// Parse DISP=(status,normal,abnormal).
fn parse_disp(s: &str) -> Option<Disposition> {
    let inner = s.trim_matches(|c| c == '(' || c == ')');
    let parts: Vec<&str> = inner.split(',').map(str::trim).collect();

    let status = match parts.first()?.to_uppercase().as_str() {
        "NEW" => DispStatus::New,
        "OLD" => DispStatus::Old,
        "SHR" => DispStatus::Shr,
        "MOD" => DispStatus::Mod,
        _ => return None,
    };

    let normal = parts.get(1).and_then(|s| parse_disp_action(s));
    let abnormal = parts.get(2).and_then(|s| parse_disp_action(s));

    Some(Disposition { status, normal, abnormal })
}

fn parse_disp_action(s: &str) -> Option<DispAction> {
    match s.trim().to_uppercase().as_str() {
        "DELETE" => Some(DispAction::Delete),
        "KEEP" => Some(DispAction::Keep),
        "PASS" => Some(DispAction::Pass),
        "CATLG" => Some(DispAction::Catlg),
        "UNCATLG" => Some(DispAction::Uncatlg),
        _ => None,
    }
}

/// Parse COND= parameter value.
///
/// Formats:
/// - COND=EVEN / COND=ONLY
/// - COND=(4,LT) -- single test
/// - COND=(4,LT,STEP1) -- single test with step
/// - COND=((4,LT,STEP1),(8,EQ,STEP2)) -- multiple tests
fn parse_cond_param(s: &str) -> Option<CondParam> {
    let upper = s.trim().to_uppercase();
    if upper == "EVEN" {
        return Some(CondParam::Even);
    }
    if upper == "ONLY" {
        return Some(CondParam::Only);
    }

    let trimmed = s.trim();
    // Strip exactly one layer of outer parens
    let inner = if trimmed.starts_with('(') && trimmed.ends_with(')') {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    };

    let mut tests = Vec::new();

    // Detect single vs multiple: if inner starts with '(' it's multiple tests
    if inner.trim_start().starts_with('(') {
        // Multiple: (code,op,step),(code,op,step)
        for part in split_paren_groups(inner) {
            if let Some(test) = parse_single_cond_test(&part) {
                tests.push(test);
            }
        }
    } else {
        // Single: code,op[,step]
        if let Some(test) = parse_single_cond_test(inner) {
            tests.push(test);
        }
    }

    if tests.is_empty() {
        None
    } else {
        Some(CondParam::Tests(tests))
    }
}

/// Parse a single COND test: "4,LT,STEP1" or "4,LT"
fn parse_single_cond_test(s: &str) -> Option<CondTest> {
    let clean = s.trim().trim_matches(|c| c == '(' || c == ')');
    let items: Vec<&str> = clean.split(',').map(str::trim).collect();
    if items.len() >= 2 {
        let code: u16 = items[0].parse().ok()?;
        let operator = parse_cond_operator(items[1])?;
        let step = items.get(2).filter(|s| !s.is_empty()).map(std::string::ToString::to_string);
        Some(CondTest { code, operator, step })
    } else {
        None
    }
}

/// Split "(a,b,c),(d,e,f)" into ["a,b,c", "d,e,f"]
fn split_paren_groups(s: &str) -> Vec<String> {
    let mut groups = Vec::new();
    let mut current = String::new();
    let mut depth = 0;

    for ch in s.chars() {
        match ch {
            '(' => {
                depth += 1;
                if depth > 1 { current.push(ch); }
            }
            ')' => {
                depth -= 1;
                if depth > 0 {
                    current.push(ch);
                } else {
                    let trimmed = current.trim().to_string();
                    if !trimmed.is_empty() {
                        groups.push(trimmed);
                    }
                    current.clear();
                }
            }
            ',' if depth == 0 => {
                // separator between groups, skip
            }
            _ => current.push(ch),
        }
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        groups.push(trimmed);
    }

    groups
}

fn parse_cond_operator(s: &str) -> Option<CondOperator> {
    match s.trim().to_uppercase().as_str() {
        "GT" => Some(CondOperator::Gt),
        "GE" => Some(CondOperator::Ge),
        "EQ" => Some(CondOperator::Eq),
        "LT" => Some(CondOperator::Lt),
        "LE" => Some(CondOperator::Le),
        "NE" => Some(CondOperator::Ne),
        _ => None,
    }
}

/// Parse MSGLEVEL=(statements,messages).
fn parse_msglevel(s: &str) -> (u8, u8) {
    let inner = s.trim_matches(|c| c == '(' || c == ')');
    let parts: Vec<&str> = inner.split(',').collect();
    let a = parts.first().and_then(|p| p.trim().parse().ok()).unwrap_or(1);
    let b = parts.get(1).and_then(|p| p.trim().parse().ok()).unwrap_or(1);
    (a, b)
}

/// Parse IF statement condition (simplified).
fn parse_if_statement(payload: &str) -> IfStatement {
    // Extract between IF and THEN
    let upper = payload.to_uppercase();
    let if_pos = upper.find("IF").unwrap_or(0);
    let then_pos = upper.find("THEN").unwrap_or(upper.len());
    let cond_text = &payload[if_pos + 2..then_pos].trim();

    let condition = parse_if_condition(cond_text);
    IfStatement { condition }
}

/// Parse IF condition expression (simplified -- handles common patterns).
fn parse_if_condition(text: &str) -> IfCondition {
    let text = text.trim().trim_matches(|c| c == '(' || c == ')').trim();
    let upper = text.to_uppercase();

    // Check for NOT
    if upper.starts_with("NOT ") || upper.starts_with("NOT(") {
        let rest = text[3..].trim().trim_start_matches('(').trim_end_matches(')');
        return IfCondition::Not(Box::new(parse_if_condition(rest)));
    }

    // Check for AND/OR (simplified: split on first occurrence)
    // This is a simplified parser -- complex nested conditions may need more work
    if let Some(pos) = find_logical_operator(&upper, " AND ") {
        let left = &text[..pos];
        let right = &text[pos + 5..];
        return IfCondition::And(
            Box::new(parse_if_condition(left)),
            Box::new(parse_if_condition(right)),
        );
    }
    if let Some(pos) = find_logical_operator(&upper, " OR ") {
        let left = &text[..pos];
        let right = &text[pos + 4..];
        return IfCondition::Or(
            Box::new(parse_if_condition(left)),
            Box::new(parse_if_condition(right)),
        );
    }

    // Single test: [step.]RC op value, [step.]ABEND, [step.]RUN
    let test = parse_single_if_test(text);
    IfCondition::Test(test)
}

fn find_logical_operator(upper: &str, op: &str) -> Option<usize> {
    let mut paren_depth = 0;
    let bytes = upper.as_bytes();
    let op_bytes = op.as_bytes();

    for i in 0..bytes.len().saturating_sub(op_bytes.len()) {
        match bytes[i] {
            b'(' => paren_depth += 1,
            b')' => paren_depth -= 1,
            _ => {}
        }
        if paren_depth == 0 && &bytes[i..i + op_bytes.len()] == op_bytes {
            return Some(i);
        }
    }
    None
}

/// Parse a single IF test: [step.]RC/ABEND/ABENDCC/RUN op value
fn parse_single_if_test(text: &str) -> IfTest {
    let upper = text.trim().to_uppercase();
    let parts: Vec<&str> = upper.split_whitespace().collect();

    if parts.is_empty() {
        return IfTest::Rc { step: None, operator: CondOperator::Eq, value: 0 };
    }

    // Parse step.keyword or just keyword
    let (step, keyword) = if parts[0].contains('.') {
        let split: Vec<&str> = parts[0].splitn(2, '.').collect();
        (Some(split[0].to_string()), split[1].to_string())
    } else {
        (None, parts[0].to_string())
    };

    match keyword.as_str() {
        "ABEND" => {
            let value = if parts.len() >= 3 {
                parts[2].eq_ignore_ascii_case("TRUE")
            } else {
                true
            };
            IfTest::Abend { step, value }
        }
        "RUN" => {
            let value = if parts.len() >= 3 {
                parts[2].eq_ignore_ascii_case("TRUE")
            } else {
                true
            };
            IfTest::Run { step, value }
        }
        "ABENDCC" => {
            let operator = parts.get(1).and_then(|s| parse_cond_operator(s)).unwrap_or(CondOperator::Eq);
            let value = parts.get(2).unwrap_or(&"0").to_string();
            IfTest::AbendCc { step, operator, value }
        }
        _ => {
            // Default: RC comparison
            let operator = parts.get(1).and_then(|s| parse_cond_operator(s)).unwrap_or(CondOperator::Eq);
            let value = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
            IfTest::Rc { step, operator, value }
        }
    }
}

/// Parse SET symbol=value statement.
fn parse_set_statement(payload: &str) -> Option<(String, String)> {
    // Payload: [name] SET symbol=value
    let upper = payload.to_uppercase();
    let set_pos = upper.find("SET")?;
    let rest = payload[set_pos + 3..].trim();
    let eq_pos = rest.find('=')?;
    let symbol = rest[..eq_pos].trim().to_string();
    let value = rest[eq_pos + 1..].trim().to_string();
    Some((symbol, value))
}

/// Parse PROC symbolic parameters: SYM1=DEFAULT1,SYM2=,...
fn parse_symbolic_params(text: &str) -> HashMap<String, Option<String>> {
    let mut symbols = HashMap::new();
    for part in text.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some(eq_pos) = part.find('=') {
            let name = part[..eq_pos].trim().to_string();
            let value = part[eq_pos + 1..].trim();
            symbols.insert(name, if value.is_empty() { None } else { Some(value.to_string()) });
        }
    }
    symbols
}

/// Parse a parenthesized list: (A,B,C) -> vec!["A", "B", "C"]
fn parse_paren_list(s: &str) -> Vec<String> {
    let inner = s.trim_matches(|c| c == '(' || c == ')');
    inner.split(',')
        .map(|p| p.trim().trim_matches('\'').to_string())
        .filter(|p| !p.is_empty())
        .collect()
}

// ---------------------------------------------------------------------------
// Token helpers
// ---------------------------------------------------------------------------

/// Check if a JCL line (uppercase payload after //) contains a keyword at a word boundary.
fn contains_keyword(upper: &str, keyword: &str) -> bool {
    // Check: starts with keyword, or second token is keyword
    let tokens: Vec<&str> = upper.split_whitespace().collect();
    tokens.iter().any(|t| t.eq_ignore_ascii_case(keyword))
}

/// Check if line is just //[name] KEYWORD (no params).
fn is_keyword_statement(upper: &str, keyword: &str) -> bool {
    let tokens: Vec<&str> = upper.split_whitespace().collect();
    match tokens.len() {
        1 => tokens[0].eq_ignore_ascii_case(keyword),
        2 => tokens[0].eq_ignore_ascii_case(keyword) || tokens[1].eq_ignore_ascii_case(keyword),
        _ => false,
    }
}

/// Check if line starts with a specific DD name.
fn starts_with_dd_name(upper: &str, name: &str) -> bool {
    let tokens: Vec<&str> = upper.split_whitespace().collect();
    tokens.first().is_some_and(|t| t.eq_ignore_ascii_case(name))
        && tokens.get(1).is_some_and(|t| t.eq_ignore_ascii_case("DD"))
}

/// Extract a parameter value from a params string: KEY=value
fn extract_parm_value(text: &str, key: &str) -> Option<String> {
    let opts = parse_keyword_params(text);
    opts.get(&key.to_uppercase()).cloned()
}

/// Find the text after a keyword in a JCL payload.
/// E.g., `find_after_keyword("MYJOB   JOB (ACCT),'NAME',CLASS=A", "JOB")` -> `"(ACCT),'NAME',CLASS=A"`
fn find_after_keyword<'a>(payload: &'a str, keyword: &str) -> &'a str {
    let upper = payload.to_uppercase();
    // Find keyword as a whole word (preceded by whitespace or start)
    let keyword_upper = keyword.to_uppercase();
    let mut search_from = 0;
    while let Some(pos) = upper[search_from..].find(&keyword_upper) {
        let abs_pos = search_from + pos;
        let end_pos = abs_pos + keyword_upper.len();
        // Check word boundary: preceded by whitespace or start, followed by whitespace/comma/= or end
        let before_ok = abs_pos == 0 || payload.as_bytes()[abs_pos - 1].is_ascii_whitespace();
        let after_ok = end_pos >= payload.len()
            || payload.as_bytes()[end_pos].is_ascii_whitespace()
            || payload.as_bytes()[end_pos] == b','
            || payload.as_bytes()[end_pos] == b'=';
        if before_ok && after_ok {
            return payload[end_pos..].trim_start();
        }
        search_from = abs_pos + 1;
    }
    ""
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_job() {
        let jcl = r#"
//MYJOB   JOB (ACCT),'JOHN DOE',CLASS=A,MSGCLASS=X,MSGLEVEL=(1,1)
//STEP1   EXEC PGM=IEFBR14
//DD1     DD DSN=MY.DATASET,DISP=SHR
//STEP2   EXEC PGM=SORT
//SORTIN  DD DSN=INPUT.FILE,DISP=SHR
//SORTOUT DD DSN=OUTPUT.FILE,DISP=(NEW,CATLG,DELETE),
//           DCB=(RECFM=FB,LRECL=80,BLKSIZE=27920),
//           SPACE=(CYL,(10,5),RLSE),UNIT=SYSDA
//SYSIN   DD *
//
"#;
        let result = parse_jcl(jcl).unwrap();
        let job = match result {
            JclSource::Job(j) => j,
            _ => panic!("expected Job"),
        };

        assert_eq!(job.name, "MYJOB");
        assert_eq!(job.params.class.as_deref(), Some("A"));
        assert_eq!(job.params.msgclass.as_deref(), Some("X"));
        assert_eq!(job.params.msglevel, Some((1, 1)));

        let steps = job.steps();
        assert_eq!(steps.len(), 2);

        // Step 1: IEFBR14
        assert_eq!(steps[0].name.as_deref(), Some("STEP1"));
        assert!(steps[0].is_noop());
        assert_eq!(steps[0].dd_statements.len(), 1);
        assert_eq!(steps[0].dd_statements[0].dsn(), Some("MY.DATASET"));

        // Step 2: SORT
        assert_eq!(steps[1].name.as_deref(), Some("STEP2"));
        assert!(steps[1].is_sort());
        assert_eq!(steps[1].dd_statements.len(), 3);

        let sortout = steps[1].dd("SORTOUT").unwrap();
        assert_eq!(sortout.dsn(), Some("OUTPUT.FILE"));
        assert_eq!(sortout.disp_status(), Some(DispStatus::New));
        if let DdKind::Dataset(ds) = &sortout.kind {
            assert_eq!(ds.dcb.recfm.as_deref(), Some("FB"));
            assert_eq!(ds.dcb.lrecl, Some(80));
            assert_eq!(ds.dcb.blksize, Some(27920));
        } else {
            panic!("expected Dataset DD");
        }

        let sysin = steps[1].dd("SYSIN").unwrap();
        assert!(sysin.is_inline());
    }

    #[test]
    fn parse_job_with_conditions() {
        let jcl = r#"
//CONDJOB JOB ,'TEST',CLASS=A
//STEP1   EXEC PGM=PROG1
//SYSOUT  DD SYSOUT=*
// IF STEP1.RC LE 4 THEN
//STEP2   EXEC PGM=PROG2,COND=(4,LT,STEP1)
//SYSOUT  DD SYSOUT=*
// ELSE
//STEP3   EXEC PGM=ERRHNDLR
//SYSOUT  DD SYSOUT=A
// ENDIF
//
"#;
        let result = parse_jcl(jcl).unwrap();
        let job = match result {
            JclSource::Job(j) => j,
            _ => panic!("expected Job"),
        };

        assert_eq!(job.name, "CONDJOB");

        // Should have: Step, If, Step, Else, Step, EndIf
        let mut step_count = 0;
        let mut if_count = 0;
        let mut else_count = 0;
        let mut endif_count = 0;
        for item in &job.body {
            match item {
                JobBodyItem::Step(_) => step_count += 1,
                JobBodyItem::If(_) => if_count += 1,
                JobBodyItem::Else => else_count += 1,
                JobBodyItem::EndIf => endif_count += 1,
                _ => {}
            }
        }
        assert_eq!(step_count, 3);
        assert_eq!(if_count, 1);
        assert_eq!(else_count, 1);
        assert_eq!(endif_count, 1);
    }

    #[test]
    fn parse_exec_proc() {
        let jcl = r#"
//PROCJOB JOB ,'TEST',CLASS=A
//STEP1   EXEC MYPROC,PARM='HELLO'
//STEP1.SYSIN DD DSN=MY.INPUT,DISP=SHR
//
"#;
        let result = parse_jcl(jcl).unwrap();
        let job = match result {
            JclSource::Job(j) => j,
            _ => panic!("expected Job"),
        };

        let steps = job.steps();
        assert_eq!(steps.len(), 1);

        assert_eq!(steps[0].proc_name(), Some("MYPROC"));
        assert_eq!(steps[0].params.parm.as_deref(), Some("'HELLO'"));
    }

    #[test]
    fn parse_sysout_and_dummy() {
        let jcl = r#"
//TESTJOB JOB ,'TEST'
//STEP1   EXEC PGM=MYPROG
//SYSOUT  DD SYSOUT=*
//SYSPRINT DD SYSOUT=A
//NULLDD  DD DUMMY
//
"#;
        let result = parse_jcl(jcl).unwrap();
        let job = match result {
            JclSource::Job(j) => j,
            _ => panic!("expected Job"),
        };

        let step = &job.steps()[0];
        assert!(step.dd("SYSOUT").unwrap().is_sysout());
        assert!(step.dd("SYSPRINT").unwrap().is_sysout());
        assert!(step.dd("NULLDD").unwrap().is_dummy());
    }

    #[test]
    fn parse_disp_values() {
        assert_eq!(parse_disp("SHR").unwrap().status, DispStatus::Shr);
        assert_eq!(parse_disp("(NEW,CATLG,DELETE)").unwrap().status, DispStatus::New);
        assert_eq!(parse_disp("(NEW,CATLG,DELETE)").unwrap().normal, Some(DispAction::Catlg));
        assert_eq!(parse_disp("(NEW,CATLG,DELETE)").unwrap().abnormal, Some(DispAction::Delete));
        assert_eq!(parse_disp("(OLD,KEEP)").unwrap().status, DispStatus::Old);
        assert_eq!(parse_disp("(MOD,CATLG)").unwrap().status, DispStatus::Mod);
    }

    #[test]
    fn parse_cond_tests() {
        let cond = parse_cond_param("(4,LT,STEP1)").unwrap();
        if let CondParam::Tests(tests) = cond {
            assert_eq!(tests.len(), 1);
            assert_eq!(tests[0].code, 4);
            assert_eq!(tests[0].operator, CondOperator::Lt);
            assert_eq!(tests[0].step.as_deref(), Some("STEP1"));
        } else {
            panic!("expected Tests");
        }

        assert!(matches!(parse_cond_param("EVEN"), Some(CondParam::Even)));
        assert!(matches!(parse_cond_param("ONLY"), Some(CondParam::Only)));
    }

    #[test]
    fn parse_proc_definition() {
        let jcl = r#"
//MYPROC  PROC INDSN=,OUTDSN=,RC=0
//STEP1   EXEC PGM=MYPROG,PARM=&RC
//INPUT   DD DSN=&INDSN,DISP=SHR
//OUTPUT  DD DSN=&OUTDSN,DISP=(NEW,CATLG)
// PEND
"#;
        let result = parse_jcl(jcl).unwrap();
        let proc = match result {
            JclSource::Proc(p) => p,
            _ => panic!("expected Proc"),
        };

        assert_eq!(proc.name.as_deref(), Some("MYPROC"));
        assert_eq!(proc.symbols.len(), 3);
        assert_eq!(proc.symbols.get("INDSN"), Some(&None));
        assert_eq!(proc.symbols.get("RC"), Some(&Some("0".to_string())));

        let steps: Vec<_> = proc.body.iter().filter_map(|i| {
            if let JobBodyItem::Step(s) = i { Some(s) } else { None }
        }).collect();
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].program(), Some("MYPROG"));
    }

    #[test]
    fn step_utility_detection() {
        let step = JclStep {
            name: Some("S1".to_string()),
            exec: ExecType::Pgm("SORT".to_string()),
            params: StepParams::default(),
            dd_statements: Vec::new(),
            comments: Vec::new(),
        };
        assert!(step.is_sort());
        assert!(!step.is_idcams());
        assert!(!step.is_noop());
        assert!(!step.is_db2());

        let step2 = JclStep {
            name: Some("S2".to_string()),
            exec: ExecType::Pgm("IKJEFT01".to_string()),
            params: StepParams::default(),
            dd_statements: Vec::new(),
            comments: Vec::new(),
        };
        assert!(step2.is_db2());
    }

    #[test]
    fn system_dd_detection() {
        let dd = DdStatement {
            name: "SYSIN".to_string(),
            step_qualifier: None,
            kind: DdKind::Dummy,
            concatenations: Vec::new(),
        };
        assert!(dd.is_system_dd());

        let dd2 = DdStatement {
            name: "MYDATA".to_string(),
            step_qualifier: None,
            kind: DdKind::Dummy,
            concatenations: Vec::new(),
        };
        assert!(!dd2.is_system_dd());
    }

    #[test]
    fn keyword_params_with_dcb_parens() {
        let opts = parse_keyword_params("DSN=MY.DS,DISP=(NEW,CATLG),DCB=(RECFM=FB,LRECL=80)");
        assert_eq!(opts.get("DSN").unwrap(), "MY.DS");
        assert_eq!(opts.get("DISP").unwrap(), "(NEW,CATLG)");
        // DCB with parens is tricky -- the individual params should be extractable
        assert!(opts.contains_key("DCB") || opts.contains_key("RECFM"));
    }
}
