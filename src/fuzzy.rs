// Removed unused import

pub fn subsequence_match(query: &str, target: &str) -> bool {
    let mut query_chars = query.chars();
    let mut current_query_char = query_chars.next();

    if current_query_char.is_none() {
        return true;
    }

    for target_char in target.chars() {
        if let Some(q_char) = current_query_char {
            if q_char.to_lowercase().next() == target_char.to_lowercase().next() {
                current_query_char = query_chars.next();
                if current_query_char.is_none() {
                    return true;
                }
            }
        }
    }

    false
}

pub struct FuzzySearch {
    files: Vec<String>,
}

impl FuzzySearch {
    pub fn new(files: Vec<String>) -> Self {
        Self { files }
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<(String, usize)> {
        let mut matches: Vec<(String, usize)> = self.files
            .iter()
            .filter(|f| subsequence_match(query, f))
            .map(|f| (f.clone(), f.len().saturating_sub(query.len())))
            .collect();

        // Sort by distance (lower is better)
        matches.sort_by_key(|&(_, dist)| dist);
        matches.into_iter().take(limit).collect()
    }
}
