use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LexerState {
    Normal,
    InMultiLineComment,
}

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
    pub multiline_start: Option<String>,
    pub multiline_end: Option<String>,
}

pub struct Highlighter {
    pub(crate) hldb: Vec<SyntaxDef>,
    line_cache: HashMap<(String, LexerState), (Vec<(String, TokenType)>, LexerState)>,
}

impl Highlighter {
    pub fn new() -> Self {
        let rust_syntax = SyntaxDef {
            name: "Rust".to_string(),
            extensions: vec!["rs".to_string()],
            keywords: vec![
                "fn", "let", "mut", "if", "else", "match", "loop", "for", "in", "break",
                "continue", "return", "mod", "use", "pub", "struct", "enum", "impl", "trait",
                "type", "where", "Self", "self",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
            types: vec![
                "i32", "u32", "i64", "u64", "f32", "f64", "bool", "char", "str", "String", "Vec",
                "Option", "Result",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
            singleline_comment: "//".to_string(),
            multiline_start: Some("/*".to_string()),
            multiline_end: Some("*/".to_string()),
        };

        let python_syntax = SyntaxDef {
            name: "Python".to_string(),
            extensions: vec!["py".to_string()],
            keywords: vec![
                "def", "class", "if", "elif", "else", "for", "while", "return", "import", "from",
                "as", "with", "try", "except", "finally", "raise", "lambda", "yield", "pass",
                "break", "continue",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
            types: vec![
                "True", "False", "None", "int", "float", "str", "list", "dict", "set", "tuple",
                "bool",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
            singleline_comment: "#".to_string(),
            multiline_start: None,
            multiline_end: None,
        };

        Self {
            hldb: vec![rust_syntax, python_syntax],
            line_cache: HashMap::new(),
        }
    }

    pub fn highlight(&mut self, content: &str, extension: &str) -> Vec<(String, TokenType)> {
        let mut tokens = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let syntax = self
            .hldb
            .iter()
            .find(|s| s.extensions.contains(&extension.to_string()))
            .unwrap_or(&self.hldb[0]);

        let mut current_state = LexerState::Normal;

        for line in lines {
            let cache_key = (line.to_string(), current_state);
            if let Some((cached_tokens, next_state)) = self.line_cache.get(&cache_key) {
                tokens.extend(cached_tokens.clone());
                current_state = *next_state;
            } else {
                let mut line_tokens = Vec::new();
                let next_state = self.tokenize_line(line, syntax, &mut line_tokens, current_state);
                self.line_cache
                    .insert(cache_key, (line_tokens.clone(), next_state));
                tokens.extend(line_tokens);
                current_state = next_state;
            }
            tokens.push(("\n".to_string(), TokenType::Normal));
        }

        tokens
    }

    fn tokenize_line(
        &self,
        line: &str,
        syntax: &SyntaxDef,
        tokens: &mut Vec<(String, TokenType)>,
        start_state: LexerState,
    ) -> LexerState {
        let mut current_word = String::new();
        let mut in_string = false;
        let mut current_state = start_state;
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            if current_state == LexerState::InMultiLineComment {
                current_word.push(c);
                if let (Some(ref end), Some(&next_c)) = (&syntax.multiline_end, chars.peek()) {
                    if c == end.chars().next().unwrap() && next_c == end.chars().nth(1).unwrap() {
                        chars.next();
                        current_word.push(next_c);
                        tokens.push((current_word.clone(), TokenType::Comment));
                        current_word.clear();
                        current_state = LexerState::Normal;
                    }
                }
                continue;
            }

            // Handle start of multi-line comment
            if !in_string && current_state == LexerState::Normal {
                if let Some(ref start) = syntax.multiline_start {
                    if c == start.chars().next().unwrap() {
                        if let Some(&next_c) = chars.peek() {
                            if next_c == start.chars().nth(1).unwrap() {
                                chars.next();
                                if !current_word.is_empty() {
                                    self.push_word(&current_word, syntax, tokens);
                                    current_word.clear();
                                }
                                current_word.push(c);
                                current_word.push(next_c);
                                current_state = LexerState::InMultiLineComment;
                                continue;
                            }
                        }
                    }
                }
            }

            // Handle single-line comments
            if !in_string
                && current_state == LexerState::Normal
                && c == syntax.singleline_comment.chars().next().unwrap()
            {
                if let Some(&next_c) = chars.peek() {
                    if next_c == syntax.singleline_comment.chars().nth(1).unwrap_or(' ') {
                        if !current_word.is_empty() {
                            self.push_word(&current_word, syntax, tokens);
                            current_word.clear();
                        }
                        let mut comment = String::from(c);
                        comment.extend(chars);
                        tokens.push((comment, TokenType::Comment));
                        return LexerState::Normal;
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
                LexerState::Normal // String doesn't span lines in this simple lexer
            } else if current_state == LexerState::InMultiLineComment {
                tokens.push((current_word, TokenType::Comment));
                LexerState::InMultiLineComment
            } else {
                self.push_word(&current_word, syntax, tokens);
                LexerState::Normal
            }
        } else {
            current_state
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
        (255, 255, 255)
    }
}
