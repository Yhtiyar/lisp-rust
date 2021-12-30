use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Node>),
    Map(HashMap<String, Value>),
    Function { args: Vec<String>, body: Vec<Node> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Atom(Value),
    FunctionCall(String, Vec<Node>),
    Program(Vec<Node>),
    Variable(String),
    EOF,
}
