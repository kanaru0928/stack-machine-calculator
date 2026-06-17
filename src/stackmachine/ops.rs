#[derive(Debug, Clone)]
pub struct ValueType(pub f64);

#[derive(Debug, Clone, PartialEq)]
pub enum OpType {
    Const,
    Load,
    Store,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub struct Op {
    pub op_type: OpType,
    pub val: Option<ValueType>,
    pub var: Option<String>,
}
