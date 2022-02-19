use super::interpretator::Scope;
use std::collections::HashMap;
#[allow(dead_code)]
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
    UserDefined(UserDefinedFunction),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserDefinedFunction {
    pub args: Vec<String>,
    pub body: Vec<Node>,
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
                let func = match scope.get(&name) {
                    Some(v) => v,
                    None => return Err(format!("{} is not defined", name)),
                };

                match func {
                    Value::Function(f) => {
                        let mut new_scope = Scope::new(Some(scope));
                        let arg_names = match f {
                            Function::UserDefined(f) => f.args.clone(),
                            Function::Native(f) => f.args.clone(),
                        };

                        if args.len() != arg_names.len() {
                            return Err(format!(
                                "Function {} takes {} arguments, but {} were given",
                                name,
                                arg_names.len(),
                                args.len()
                            ));
                        }
                        let mut evaluated_args = vec![];
                        for (i, arg) in args.iter().enumerate() {
                            let arg_val = arg.evaluate(scope)?;
                            new_scope.set(arg_names[i].clone(), arg_val.clone());
                            evaluated_args.push(arg_val);
                        }

                        let result = match f {
                            Function::UserDefined(f) => {
                                let new_scope = Scope::new(Some(&new_scope));
                                let mut result = Ok(Value::Null);
                                for node in &f.body {
                                    result = node.evaluate(&new_scope);
                                }

                                return result;
                            }
                            Function::Native(f) => (f.func)(evaluated_args),
                        };
                        result
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
            Value::Function(f) => Value::Function(f.clone()),
            Value::Null => Value::Null,
        }
    }
}
