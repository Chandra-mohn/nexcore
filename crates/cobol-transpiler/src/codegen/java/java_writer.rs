//! Java source code writer with indentation management.
//!
//! Provides a simple buffered writer that tracks indentation level
//! for generating well-formatted Java code. Mirrors RustWriter exactly.

/// Writer for generating formatted Java source code.
#[derive(Debug)]
pub struct JavaWriter {
    /// Accumulated output lines.
    buffer: String,
    /// Current indentation level.
    indent: usize,
    /// String used for one level of indentation.
    indent_str: String,
}

impl JavaWriter {
    /// Create a new writer with 4-space indentation.
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(4096),
            indent: 0,
            indent_str: "    ".to_string(),
        }
    }

    /// Write a line with current indentation.
    pub fn line(&mut self, text: &str) {
        if text.is_empty() {
            self.buffer.push('\n');
        } else {
            for _ in 0..self.indent {
                self.buffer.push_str(&self.indent_str);
            }
            self.buffer.push_str(text);
            self.buffer.push('\n');
        }
    }

    /// Write text without a newline (with indentation).
    pub fn write(&mut self, text: &str) {
        for _ in 0..self.indent {
            self.buffer.push_str(&self.indent_str);
        }
        self.buffer.push_str(text);
    }

    /// Write raw text without indentation or newline.
    pub fn raw(&mut self, text: &str) {
        self.buffer.push_str(text);
    }

    /// Write a blank line.
    pub fn blank_line(&mut self) {
        self.buffer.push('\n');
    }

    /// Increase indentation level.
    pub fn indent(&mut self) {
        self.indent += 1;
    }

    /// Decrease indentation level.
    pub fn dedent(&mut self) {
        self.indent = self.indent.saturating_sub(1);
    }

    /// Write a line and increase indentation (for opening blocks).
    pub fn open_block(&mut self, text: &str) {
        self.line(text);
        self.indent();
    }

    /// Decrease indentation and write a line (for closing blocks).
    pub fn close_block(&mut self, text: &str) {
        self.dedent();
        self.line(text);
    }

    /// Get the accumulated output.
    pub fn finish(self) -> String {
        self.buffer
    }

    /// Get a reference to the current buffer.
    pub fn as_str(&self) -> &str {
        &self.buffer
    }
}

impl Default for JavaWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_output() {
        let mut w = JavaWriter::new();
        w.line("public class Foo {");
        w.indent();
        w.line("int x = 0;");
        w.dedent();
        w.line("}");
        let output = w.finish();
        assert!(output.contains("    int x = 0;"));
    }

    #[test]
    fn open_close_block() {
        let mut w = JavaWriter::new();
        w.open_block("if (true) {");
        w.line("doSomething();");
        w.close_block("}");
        let output = w.finish();
        assert!(output.contains("if (true) {"));
        assert!(output.contains("    doSomething();"));
        assert!(output.ends_with("}\n"));
    }

    #[test]
    fn blank_lines() {
        let mut w = JavaWriter::new();
        w.line("line1");
        w.blank_line();
        w.line("line2");
        let output = w.finish();
        assert_eq!(output, "line1\n\nline2\n");
    }

    #[test]
    fn nested_blocks() {
        let mut w = JavaWriter::new();
        w.open_block("public class Foo {");
        w.open_block("public void bar() {");
        w.line("return;");
        w.close_block("}");
        w.close_block("}");
        let output = w.finish();
        assert!(output.contains("        return;"));
    }
}
