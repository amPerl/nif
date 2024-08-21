#[derive(Clone, Default)]
pub struct RustWriter {
    spaces: usize,
    output: String,
}

impl RustWriter {
    pub fn indent(&mut self) {
        self.spaces += 4;
    }

    pub fn dedent(&mut self) {
        self.spaces -= 4;
    }

    pub fn write_indent(&mut self) {
        self.output.push_str(&" ".repeat(self.spaces));
    }

    pub fn writeln(&mut self, line: &str) {
        self.write_indent();
        self.output.push_str(line);
        self.output.push('\n');
    }

    pub fn write_comment(&mut self, comment: &str) {
        for line in comment.lines() {
            self.writeln(&format!("// {}", line.trim_start()));
        }
    }

    pub fn write_doc_comment(&mut self, comment: &str) {
        for line in comment.lines() {
            self.writeln(&format!("/// {}", line.trim_start()));
        }
    }

    pub fn write_block_comment(&mut self, comment: &str) {
        self.writeln("/*");
        for line in comment.lines() {
            self.writeln(&format!(" * {}", line));
        }
        self.writeln(" */");
    }

    pub fn finish(self) -> String {
        self.output
    }
}
