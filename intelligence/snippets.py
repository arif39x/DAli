SNIPPETS = {
    "mod": "mod $1 {\n    $0\n}",
    "fn": "fn $1($2) -> $3 {\n    $0\n}",
    "struct": "struct $1 {\n    $0\n}",
    "impl": "impl $1 {\n    $0\n}",
    "match": "match $1 {\n    $2 => $3,\n    _ => $0,\n}",
    "enum": "enum $1 {\n    $0\n}",
    "if": "if $1 {\n    $0\n}",
    "for": "for $1 in $2 {\n    $0\n}",
    "while": "while $1 {\n    $0\n}",
    "let": "let $1 = $2;",
    "pub": "pub $1",
    "use": "use $1;",
    "crate": "crate::$1",
    "super": "super::$1",
    "self": "self::$1",
}

def expand_snippet(trigger):
    return SNIPPETS.get(trigger)

def parse_placeholders(expanded):

    return expanded

if __name__ == "__main__":
    print(expand_snippet("mod"))
