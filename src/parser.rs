use crate::ast::Statement;

pub fn parse(input: &str) -> Vec<Statement> {
    let lines: Vec<&str> = input.lines().collect();
    let mut statements = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();

        if trimmed.is_empty() {
            statements.push(Statement::Blank);
            i += 1;
            continue;
        }

        if trimmed.starts_with('#') {
            statements.push(Statement::Comment(trimmed.to_string()));
            i += 1;
            continue;
        }

        if trimmed.starts_with("if ") && trimmed.ends_with('{') {
            let (stmt, next_i) = parse_if_else(&lines, i);
            statements.push(stmt);
            i = next_i;
            continue;
        }

        if trimmed.starts_with("function ") && trimmed.ends_with('{') {
            let (stmt, next_i) = parse_function(&lines, i);
            statements.push(stmt);
            i = next_i;
            continue;
        }

        if trimmed == "try {" {
            let (stmt, next_i) = parse_try_catch(&lines, i);
            statements.push(stmt);
            i = next_i;
            continue;
        }

        if let Some(stmt) = parse_typed_array_assignment(trimmed) {
            statements.push(stmt);
            i += 1;
            continue;
        }

        if let Some(stmt) = parse_typed_assignment(trimmed) {
            statements.push(stmt);
            i += 1;
            continue;
        }

        if let Some(stmt) = parse_plain_assignment(trimmed) {
            statements.push(stmt);
            i += 1;
            continue;
        }

        statements.push(Statement::Raw(trimmed.to_string()));
        i += 1;
    }

    statements
}

// Parses a `try { ... } catch { ... }` block starting at index `start`
// (where lines[start] == "try {"). Returns the statement and the index
// of the line right after the closing `}`.
fn parse_try_catch(lines: &[&str], start: usize) -> (Statement, usize) {
    let mut i = start + 1;
    let mut try_body = Vec::new();

    while i < lines.len() && lines[i].trim() != "} catch {" {
        try_body.push(lines[i].trim().to_string());
        i += 1;
    }
    i += 1; // skip "} catch {"

    let mut catch_body = Vec::new();
    while i < lines.len() && lines[i].trim() != "}" {
        catch_body.push(lines[i].trim().to_string());
        i += 1;
    }
    i += 1; // skip closing "}"

    (Statement::TryCatch { try_body, catch_body }, i)
}

fn parse_typed_assignment(line: &str) -> Option<Statement> {
    let rest = line.strip_prefix("let ")?;
    let (left, right) = rest.split_once('=')?;
    let (name, type_part) = left.trim().split_once(':')?;

    Some(Statement::TypedAssignment {
        name: name.trim().to_string(),
        type_name: type_part.trim().to_string(),
        value: strip_quotes(right.trim()),
    })
}

fn parse_typed_array_assignment(line: &str) -> Option<Statement> {
    let rest = line.strip_prefix("let ")?;
    let (left, right) = rest.split_once('=')?;
    let (name, type_part) = left.trim().split_once(':')?;

    if type_part.trim() != "array" {
        return None;
    }

    let right = right.trim();
    let inner = right.strip_prefix('[')?.strip_suffix(']')?;

    let values: Vec<String> = inner
        .split(',')
        .map(|v| strip_quotes(v.trim()))
        .filter(|v| !v.is_empty())
        .collect();

    Some(Statement::TypedArrayAssignment {
        name: name.trim().to_string(),
        values,
    })
}

fn parse_plain_assignment(line: &str) -> Option<Statement> {
    let (left, right) = line.split_once('=')?;
    if left.contains(' ') || left.is_empty() {
        return None;
    }

    Some(Statement::PlainAssignment {
        name: left.to_string(),
        value: strip_quotes(right.trim()),
    })
}

fn parse_condition(cond: &str) -> Option<(String, String, String)> {
    let operators = ["==", "!=", ">=", "<=", ">", "<"];

    for op in operators {
        if let Some(pos) = cond.find(op) {
            let left = cond[..pos].trim().to_string();
            let right = cond[pos + op.len()..].trim().to_string();
            return Some((left, op.to_string(), right));
        }
    }

    None
}

#[allow(dead_code)]
fn bash_operator(op: &str) -> &str {
    match op {
        ">" => "-gt",
        "<" => "-lt",
        ">=" => "-ge",
        "<=" => "-le",
        "==" => "-eq",
        "!=" => "-ne",
        _ => op,
    }
}

fn parse_if_else(lines: &[&str], start: usize) -> (Statement, usize) {
    let header = lines[start].trim();
    // header looks like: "if x > 5 {"
    let cond_part = header
        .strip_prefix("if ")
        .unwrap_or(header)
        .trim_end_matches('{')
        .trim();

    let (left, operator, right) = parse_condition(cond_part)
        .unwrap_or((cond_part.to_string(), "==".to_string(), "".to_string()));

    let mut i = start + 1;
    let mut then_body = Vec::new();

    while i < lines.len() && lines[i].trim() != "} else {" && lines[i].trim() != "}" {
        then_body.push(lines[i].trim().to_string());
        i += 1;
    }

    let mut else_body = None;

    if i < lines.len() && lines[i].trim() == "} else {" {
        i += 1;
        let mut body = Vec::new();
        while i < lines.len() && lines[i].trim() != "}" {
            body.push(lines[i].trim().to_string());
            i += 1;
        }
        else_body = Some(body);
    }

    i += 1; // skip closing "}"

    (
        Statement::IfElse {
            left,
            operator,
            right,
            then_body,
            else_body,
        },
        i,
    )
}

fn parse_function(lines: &[&str], start: usize) -> (Statement, usize) {
    let header = lines[start].trim();
    // header looks like: "function greet(name, age) {"
    let rest = header.strip_prefix("function ").unwrap_or(header);
    let rest = rest.trim_end_matches('{').trim();

    let (name, params_part) = rest.split_once('(').unwrap_or((rest, ""));
    let params_str = params_part.trim_end_matches(')');

    let params: Vec<String> = params_str
        .split(',')
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect();

    let mut i = start + 1;
    let mut body = Vec::new();

    while i < lines.len() && lines[i].trim() != "}" {
        body.push(lines[i].trim().to_string());
        i += 1;
    }
    i += 1; // skip closing "}"

    (
        Statement::Function {
            name: name.trim().to_string(),
            params,
            body,
        },
        i,
    )
}

fn strip_quotes(value: &str) -> String {
    if (value.starts_with('"') && value.ends_with('"'))
        || (value.starts_with('\'') && value.ends_with('\''))
    {
        if value.len() >= 2 {
            return value[1..value.len() - 1].to_string();
        }
    }
    value.to_string()
}

// Removes a trailing `# comment` from a line, but only if the `#` is not
// inside a quoted string. Returns the code part and the comment part (if any).
#[allow(dead_code)]
fn strip_comment(line: &str) -> (String, Option<String>) {
    let chars: Vec<char> = line.chars().collect();
    let mut in_double_quotes = false;
    let mut in_single_quotes = false;

    for (i, &c) in chars.iter().enumerate() {
        match c {
            '"' if !in_single_quotes => in_double_quotes = !in_double_quotes,
            '\'' if !in_double_quotes => in_single_quotes = !in_single_quotes,
            '#' if !in_double_quotes && !in_single_quotes => {
                let code: String = chars[..i].iter().collect();
                let comment: String = chars[i..].iter().collect();
                return (code.trim_end().to_string(), Some(comment));
            }
            _ => {}
        }
    }

    (line.to_string(), None)
}