#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Normal,
    Keyword,
    Type,
    String,
    Number,
    Comment,
}

pub struct SyntaxDef {
    pub name: String,
    pub extensions: Vec<String>,
    pub keywords: Vec<String>,
    pub types: Vec<String>,
    pub singleline_comment: String,
}

pub struct Highlighter {
    hldb: Vec<SyntaxDef>,
}

impl Highlighter {
    pub fn new() -> Self {
        let rust_syntax = SyntaxDef {
            name: "Rust".to_string(),
            extensions: vec!["rs".to_string()],
            keywords: vec!["fn", "let", "mut", "if", "else", "match", "loop", "for", "in", "break", "continue", "return", "mod", "use", "pub", "struct", "enum", "impl", "trait", "type", "where", "Self", "self"]
                .into_iter().map(|s| s.to_string()).collect(),
            types: vec!["i32", "u32", "i64", "u64", "f32", "f64", "bool", "char", "str", "String", "Vec", "Option", "Result"]
                .into_iter().map(|s| s.to_string()).collect(),
            singleline_comment: "//".to_string(),
        };
        
        Self {
            hldb: vec![rust_syntax],
        }
    }

    pub fn highlight(&self, content: &str) -> Vec<(String, TokenType)> {
        let mut tokens = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let syntax = &self.hldb[0]; // Default to Rust

        for line in lines {
            self.tokenize_line(line, syntax, &mut tokens);
            tokens.push(("\n".to_string(), TokenType::Normal));
        }

        tokens
    }

    fn tokenize_line(&self, line: &str, syntax: &SyntaxDef, tokens: &mut Vec<(String, TokenType)>) {
        let mut current_word = String::new();
        let mut in_string = false;
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            // Handle comments
            if !in_string && c == syntax.singleline_comment.chars().next().unwrap() {
                if let Some(&next_c) = chars.peek() {
                    if next_c == syntax.singleline_comment.chars().nth(1).unwrap_or(' ') {
                        if !current_word.is_empty() {
                            self.push_word(&current_word, syntax, tokens);
                            current_word.clear();
                        }
                        let mut comment = String::from(c);
                        comment.extend(chars);
                        tokens.push((comment, TokenType::Comment));
                        return;
                    }
                }
            }

            if c == '"' {
                if in_string {
                    current_word.push(c);
                    tokens.push((current_word.clone(), TokenType::String));
                    current_word.clear();
                    in_string = false;
                } else {
                    if !current_word.is_empty() {
                        self.push_word(&current_word, syntax, tokens);
                        current_word.clear();
                    }
                    in_string = true;
                    current_word.push(c);
                }
                continue;
            }

            if in_string {
                current_word.push(c);
                continue;
            }

            if c.is_alphanumeric() || c == '_' {
                current_word.push(c);
            } else {
                if !current_word.is_empty() {
                    self.push_word(&current_word, syntax, tokens);
                    current_word.clear();
                }
                tokens.push((c.to_string(), TokenType::Normal));
            }
        }

        if !current_word.is_empty() {
            if in_string {
                tokens.push((current_word, TokenType::String));
            } else {
                self.push_word(&current_word, syntax, tokens);
            }
        }
    }

    fn push_word(&self, word: &str, syntax: &SyntaxDef, tokens: &mut Vec<(String, TokenType)>) {
        if syntax.keywords.contains(&word.to_string()) {
            tokens.push((word.to_string(), TokenType::Keyword));
        } else if syntax.types.contains(&word.to_string()) {
            tokens.push((word.to_string(), TokenType::Type));
        } else if word.chars().all(|c| c.is_digit(10)) {
            tokens.push((word.to_string(), TokenType::Number));
        } else {
            tokens.push((word.to_string(), TokenType::Normal));
        }
    }

    pub fn get_color(token_type: TokenType) -> (u8, u8, u8) {
        match token_type {
            TokenType::Keyword => (255, 120, 100), // Salmon Red
            TokenType::Type => (100, 200, 255),    // Sky Blue
            TokenType::String => (150, 255, 150),  // Light Green
            TokenType::Number => (255, 200, 100),  // Orange
            TokenType::Comment => (120, 120, 120), // Gray
            TokenType::Normal => (255, 255, 255),  // White
        }
    }
}
