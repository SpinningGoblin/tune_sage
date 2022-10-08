#[derive(Clone)]
pub enum QueryOperator {
    And,
    Or,
}

impl Default for QueryOperator {
    fn default() -> Self {
        Self::And
    }
}
