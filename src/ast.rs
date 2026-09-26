#[derive(Debug, Clone)]
pub enum Statement {
    TypedAssignment {
        name: String,
        type_name: String,
        value: String,
    },
    TypedArrayAssignment {
        name: String,
        values: Vec<String>,
    },
    PlainAssignment {
        name: String,
        value: String,
    },
    TryCatch {
        try_body: Vec<String>,
        catch_body: Vec<String>,
    },
    IfElse {
        left: String,
        operator: String,
        right: String,
        then_body: Vec<String>,
        else_body: Option<Vec<String>>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<String>,
    },
    Comment(String),
    Raw(String),
    Blank,
}