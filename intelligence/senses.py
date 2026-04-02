def calculate_indentation(buffer_context: str) -> int:

#  the buffer context the last few lines) to determine the indentation level for the next line.

    lines = buffer_context.splitlines()
    if not lines:
        return 0

    last_line = lines[-1].rstrip()

    current_indent = len(last_line) - len(last_line.lstrip())   # count leading spaces of the last line

    if last_line.endswith(':') or last_line.endswith('{'):
        return current_indent + 4

    # Handle closing braces
    if last_line.strip() == '}':
        return max(0, current_indent - 4)

    return current_indent

def get_status_message():
    return "Ready"
