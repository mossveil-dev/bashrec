// Scans a raw shell line and wraps any unquoted `$variable` or `${variable}`
// references in double quotes. Already-quoted variables are left untouched.
pub fn auto_quote(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut result = String::new();
    let mut in_double_quotes = false;
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c == '"' {
            in_double_quotes = !in_double_quotes;
            result.push(c);
            i += 1;
            continue;
        }

        if c == '$' && !in_double_quotes {
            let (var_ref, next_i) = extract_variable(&chars, i);
            if let Some(var_ref) = var_ref {
                result.push('"');
                result.push_str(&var_ref);
                result.push('"');
                i = next_i;
                continue;
            }
        }

        result.push(c);
        i += 1;
    }

    result
}

// Extracts a `$name` or `${name}` reference starting at index `start`
// (where chars[start] == '$'). Returns the matched text and the index
// right after it, or None if it's not a valid variable reference.
fn extract_variable(chars: &[char], start: usize) -> (Option<String>, usize) {
    let mut i = start + 1;

    if i < chars.len() && chars[i] == '{' {
        // ${name} form
        let mut j = i + 1;
        while j < chars.len() && chars[j] != '}' {
            j += 1;
        }
        if j < chars.len() {
            let text: String = chars[start..=j].iter().collect();
            return (Some(text), j + 1);
        }
        return (None, start + 1);
    }

    // $name form: consume identifier characters
    let name_start = i;
    while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
        i += 1;
    }

    if i == name_start {
        // just a lone "$" with nothing valid after it
        return (None, start + 1);
    }

    let text: String = chars[start..i].iter().collect();
    (Some(text), i)
}