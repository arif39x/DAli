import ast
import re

class IndentVisitor(ast.NodeVisitor):
    def __init__(self, target_line):
        self.target_line = target_line
        self.indent_level = 0
        self.current_depth = 0

    def visit(self, node):
        if hasattr(node, 'lineno'):
            if node.lineno <= self.target_line:
                self.indent_level = max(self.indent_level, self.current_depth)
        
        self.current_depth += 1
        super().visit(node)
        self.current_depth -= 1

def calculate_indentation(buffer_context, target_line):
    """
    Calculates the correct indentation for a new line based on AST depth.
    If AST parsing fails, it falls back to a regex-based approach.
    """
    try:
        tree = ast.parse(buffer_context)
        visitor = IndentVisitor(target_line)
        visitor.visit(tree)
        # Each depth level is usually 4 spaces
        return visitor.indent_level * 4
    except SyntaxError:
        # Fallback: Regex-based logic
        lines = buffer_context.splitlines()
        if not lines:
            return 0
        
        last_10_lines = lines[-10:]
        indent = 0
        for line in reversed(last_10_lines):
            stripped = line.strip()
            if stripped.endswith(':') or stripped.endswith('{') or stripped.endswith('('):
                # Found an opening block, calculate its indentation
                match = re.match(r'^(\s*)', line)
                if match:
                    current_indent = len(match.group(1).replace('\t', '    '))
                    return current_indent + 4
        
        # Default to the same indentation as the last non-empty line
        for line in reversed(lines):
            if line.strip():
                match = re.match(r'^(\s*)', line)
                if match:
                    return len(match.group(1).replace('\t', '    '))
        
        return 0

if __name__ == "__main__":
    code = "def foo():\n    if True:\n        "
    print(calculate_indentation(code, 2))
