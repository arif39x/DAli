use std::collections::HashMap;

pub struct TextOps {
    snippets: HashMap<String, String>,
}

impl TextOps {
    pub fn new() -> Self {
        let mut snippets = HashMap::new();
        snippets.insert("mod".to_string(), "mod $1 {\n    $0\n}".to_string());
        snippets.insert("fn".to_string(), "fn $1($2) -> $3 {\n    $0\n}".to_string());
        snippets.insert("struct".to_string(), "struct $1 {\n    $0\n}".to_string());
        snippets.insert("impl".to_string(), "impl $1 {\n    $0\n}".to_string());
        snippets.insert(
            "match".to_string(),
            "match $1 {\n    $2 => $3,\n    _ => $0,\n}".to_string(),
        );
        snippets.insert("enum".to_string(), "enum $1 {\n    $0\n}".to_string());
        snippets.insert("if".to_string(), "if $1 {\n    $0\n}".to_string());
        snippets.insert("for".to_string(), "for $1 in $2 {\n    $0\n}".to_string());
        snippets.insert("while".to_string(), "while $1 {\n    $0\n}".to_string());
        snippets.insert("let".to_string(), "let $1 = $2;".to_string());
        snippets.insert("pub".to_string(), "pub $1".to_string());
        snippets.insert("use".to_string(), "use $1;".to_string());

        Self { snippets }
    }

    pub fn calculate_indent(&self, content: &str, row: usize) -> usize {
        let lines: Vec<&str> = content.lines().collect();
        if row == 0 || row > lines.len() {
            return 0;
        }

        let prev_line = lines[row - 1];
        let trimmed = prev_line.trim();

        let mut indent = prev_line.chars().take_while(|c| c.is_whitespace()).count();

        if trimmed.ends_with('{') || trimmed.ends_with(':') || trimmed.ends_with('(') {
            indent += 4;
        }

        indent
    }

    pub fn expand_snippet(&self, trigger: &str) -> Option<&str> {
        self.snippets.get(trigger).map(|s| s.as_str())
    }
}
