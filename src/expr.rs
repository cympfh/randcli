#[derive(Debug, Clone, PartialEq)]
pub struct Term(pub String, pub Vec<f64>);

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub code: Vec<Term>,
}
