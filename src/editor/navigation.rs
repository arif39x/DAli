use super::state::Editor;

impl Editor {
    pub(crate) fn get_tree_view(&self, dir: &str, depth: usize) -> Vec<String> {
        let mut result = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let name = entry.file_name().to_string_lossy().to_string();
                let path = entry.path();
                let indent = "  ".repeat(depth);
                let meta = std::fs::metadata(&path).ok();
                let suffix = if meta.map(|m| m.is_dir()).unwrap_or(false) { "/" } else { "" };
                result.push(format!("{}{}{}", indent, name, suffix));
                if path.is_dir() && depth < 2 {
                    result.extend(self.get_tree_view(&path.to_string_lossy(), depth + 1));
                }
            }
        }
        result
    }
}
