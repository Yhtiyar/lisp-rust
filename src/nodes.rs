use super::interpretator::Scope;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Node>),
    Map(HashMap<String, Value>),
    Function(Function),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Native(NativeFunction),
    UserDefined(Vec<String>, Vec<Node>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NativeFunction {
    pub name: String,
    pub args: Vec<String>,
    pub func: fn(Vec<Value>) -> Result<Value, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Atom(Value),
    FunctionCall(String, Vec<Node>),
    Program(Vec<Node>),
    Variable(String),
    EOF,
}

impl Node {
    pub fn evaluate(&self, scope: &Scope) -> Result<Value, String> {
        match self {
            Node::Atom(v) => Ok(v.clone()),
            Node::FunctionCall(name, args) => {
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(arg.evaluate(scope)?);
                }
                let func = scope.get(&name).unwrap();
                match func {
                    Value::Function(f) => {
                        unimplemented!()
                    }
                    _ => Err(format!("{} is not a function", name)),
                }
            }
            Node::Program(nodes) => {
                let mut result = Value::Null;
                for node in nodes {
                    result = node.evaluate(scope)?;
                }
                Ok(result)
            }
            Node::Variable(name) => Ok(scope.get(name).unwrap().clone()),
            Node::EOF => Ok(Value::Null),
        }
    }
}

impl Value {
    pub fn evaluate(&self, scope: &Scope) -> Value {
        match self {
            Value::Number(n) => Value::Number(*n),
            Value::String(s) => Value::String(s.clone()),
            Value::Boolean(b) => Value::Boolean(*b),
            Value::List(l) => Value::List(
                l.iter()
                    .map(|n| Node::Atom(n.evaluate(scope).unwrap()))
                    .collect(),
            ),
            Value::Map(m) => Value::Map(
                m.iter()
                    .map(|(k, v)| (k.clone(), v.evaluate(scope)))
                    .collect(),
            ),
            Value::Function(f) => match f {
                Function::Native(nf) => {
                    unimplemented!()
                }
                Function::UserDefined(args, body) => {
                    let mut new_scope = Scope::new(Some(scope.clone()));
                    unimplemented!()
                }
            },
            Value::Null => Value::Null,
        }
    }
}
