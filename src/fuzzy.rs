use std::cmp;

pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let v1: Vec<char> = s1.chars().collect();
    let v2: Vec<char> = s2.chars().collect();
    let len1 = v1.len();
    let len2 = v2.len();

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if v1[i - 1] == v2[j - 1] { 0 } else { 1 };
            matrix[i][j] = cmp::min(
                matrix[i - 1][j] + 1, // Deletion
                cmp::min(
                    matrix[i][j - 1] + 1, // Insertion
                    matrix[i - 1][j - 1] + cost, // Substitution
                ),
            );
        }
    }
    matrix[len1][len2]
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
            .map(|f| (f.clone(), levenshtein_distance(query, f)))
            .collect();

        // Sort by distance (lower is better)
        matches.sort_by_key(|&(_, dist)| dist);
        matches.into_iter().take(limit).collect()
    }
}
