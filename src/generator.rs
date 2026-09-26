use crate::ast::Statement;

pub fn generate(statements: &[Statement]) -> String {
    let mut output = String::new();

    for stmt in statements {
        match stmt {
            Statement::TypedAssignment { name, type_name, value } => {
                output.push_str(&format!("{}=\"{}\"\n", name, value));

                if let Some(check) = type_check_snippet(name, type_name) {
                    output.push_str(&check);
                    output.push('\n');
                }
            }
            Statement::TypedArrayAssignment { name, values } => {
                let quoted_values: Vec<String> = values.iter().map(|v| format!("\"{}\"", v)).collect();
                output.push_str(&format!("{}=({})\n", name, quoted_values.join(" ")));
            }
            Statement::PlainAssignment { name, value } => {
                output.push_str(&format!("{}=\"{}\"\n", name, value));
            }
            Statement::TryCatch { try_body, catch_body } => {
                output.push_str("if ! {\n");
                for line in try_body {
                    output.push_str(&format!("  {}\n", crate::quoting::auto_quote(line)));
                }
                output.push_str("}; then\n");
                for line in catch_body {
                    output.push_str(&format!("  {}\n", line));
                }
                output.push_str("fi\n");
            }
            Statement::IfElse { left, operator, right, then_body, else_body } => {
                let bash_op = match operator.as_str() {
                    ">" => "-gt",
                    "<" => "-lt",
                    ">=" => "-ge",
                    "<=" => "-le",
                    "==" => "-eq",
                    "!=" => "-ne",
                    _ => "-eq",
                };

                output.push_str(&format!("if [[ \"${}\" {} \"{}\" ]]; then\n", left, bash_op, right));
                for line in then_body {
                    output.push_str(&format!("  {}\n", crate::quoting::auto_quote(line)));
                }

                if let Some(else_lines) = else_body {
                    output.push_str("else\n");
                    for line in else_lines {
                        output.push_str(&format!("  {}\n", crate::quoting::auto_quote(line)));
                    }
                }

                output.push_str("fi\n");
            }
            Statement::Function { name, params, body } => {
                output.push_str(&format!("{}() {{\n", name));

                for (idx, param) in params.iter().enumerate() {
                    output.push_str(&format!("  local {}=\"${}\"\n", param, idx + 1));
                }

                for line in body {
                    output.push_str(&format!("  {}\n", crate::quoting::auto_quote(line)));
                }

                output.push_str("}\n");
            }
            Statement::Comment(text) => {
                output.push_str(text);
                output.push('\n');
            }
            Statement::Raw(text) => {
                output.push_str(&crate::quoting::auto_quote(text));
                output.push('\n');
            }
            Statement::Blank => {
                output.push('\n');
            }
        }
    }

    output
}

fn type_check_snippet(name: &str, type_name: &str) -> Option<String> {
    match type_name {
        "number" => Some(format!(
            "if ! [[ \"${name}\" =~ ^-?[0-9]+$ ]]; then echo \"TypeError: {name} must be a number\" >&2; exit 1; fi"
        )),
        "boolean" => Some(format!(
            "if [[ \"${name}\" != \"true\" && \"${name}\" != \"false\" ]]; then echo \"TypeError: {name} must be true or false\" >&2; exit 1; fi"
        )),
        "string" => None,
        _ => None,
    }
}