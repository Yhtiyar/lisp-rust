use std::collections::HashMap;

pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Function { args: Vec<String>, body: Vec<Node> },
}
pub enum Node {
    Atom(Value),
    FunctionCall(String, Vec<Node>),
}
