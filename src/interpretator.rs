use super::{nodes::*, parser::*};
use std::collections::HashMap;
use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum InterpretatorError {
    CastError(String),
    EvaluationError(String),
}
impl error::Error for InterpretatorError {}

impl fmt::Display for InterpretatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpretatorError::CastError(s) => write!(f, "CastError: {}", s),
            InterpretatorError::EvaluationError(s) => write!(f, "EvaluationError: {}", s),
        }
    }
}

pub trait Cast {
    fn cast_to_number(&self) -> Result<f64, InterpretatorError>;
    fn cast_to_string(&self) -> Result<String, InterpretatorError>;
    fn cast_to_bool(&self) -> Result<bool, InterpretatorError>;
}

impl Cast for Value {
    fn cast_to_number(&self) -> Result<f64, InterpretatorError> {
        match self {
            Value::Number(n) => Ok(*n),
            Value::Boolean(b) => Ok((*b as i64) as f64),
            Value::String(s) => match s.parse::<f64>() {
                Ok(n) => Ok(n),
                Err(_) => Err(InterpretatorError::CastError(format!(
                    "failed to cast {} to number",
                    s
                ))),
            },
            Value::List(_) => Err(InterpretatorError::CastError(
                "Cannot cast list to number".to_string(),
            )),
            Value::Map(_) => Err(InterpretatorError::CastError(
                "Cannot cast map to number".to_string(),
            )),
            Value::Function(_) => Err(InterpretatorError::CastError(
                "Cannot cast function to number".to_string(),
            )),
            Value::Null => Err(InterpretatorError::CastError(
                "Cannot cast null to number".to_string(),
            )),
        }
    }

    fn cast_to_bool(&self) -> Result<bool, InterpretatorError> {
        match self {
            Value::Number(n) => Ok(*n != 0.0),
            Value::Boolean(b) => Ok(*b),
            Value::String(s) => match s.parse::<bool>() {
                Ok(n) => Ok(n),
                Err(_) => Err(InterpretatorError::CastError(format!(
                    "failed to cast {} to bool",
                    s
                ))),
            },
            Value::List(_) => Err(InterpretatorError::CastError(
                "Cannot cast list to bool".to_string(),
            )),
            Value::Map(_) => Err(InterpretatorError::CastError(
                "Cannot cast map to bool".to_string(),
            )),
            Value::Function(_) => Err(InterpretatorError::CastError(
                "Cannot cast function to bool".to_string(),
            )),
            Value::Null => Err(InterpretatorError::CastError(
                "Cannot cast null to bool".to_string(),
            )),
        }
    }

    fn cast_to_string(&self) -> Result<String, InterpretatorError> {
        match self {
            Value::Number(n) => Ok(n.to_string()),
            Value::Boolean(b) => Ok(b.to_string()),
            Value::String(s) => Ok(s.clone()),
            Value::List(_) => Err(InterpretatorError::CastError(
                "Cannot cast list to string".to_string(),
            )),
            Value::Map(_) => Err(InterpretatorError::CastError(
                "Cannot cast map to string".to_string(),
            )),
            Value::Function(_) => Err(InterpretatorError::CastError(
                "Cannot cast function to string".to_string(),
            )),
            Value::Null => Err(InterpretatorError::CastError(
                "Cannot cast null to string".to_string(),
            )),
        }
    }
}
#[derive(Debug)]
pub struct Scope<'a> {
    pub variables: HashMap<String, Value>,
    pub parent: Option<Box<&'a Scope<'a>>>,
}

impl<'a> Scope<'a> {
    pub fn new(parent: Option<&'a Scope>) -> Scope<'a> {
        let parent_scope = match parent {
            Some(s) => Some(Box::new(s)),
            None => None,
        };
        Scope {
            variables: HashMap::new(),
            parent: parent_scope,
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        match self.variables.get(name) {
            Some(v) => Some(v),
            None => match &self.parent {
                Some(p) => p.get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
}

pub struct Interpretator<'a> {
    pub global_scope: Scope<'a>,
}

impl<'a> Interpretator<'a> {
    pub fn new(global_scope: Option<Scope<'a>>) -> Interpretator<'a> {
        let global_scope = match global_scope {
            Some(s) => s,
            None => Scope::new(None),
        };
        Interpretator {
            global_scope: global_scope,
        }
    }

    pub fn run(&mut self, source: String) -> Result<Value, Box<dyn error::Error>> {
        let mut parser = Parser::from_source(source)?;
        let program = parser.parseProgram()?;

        match program.evaluate(&self.global_scope) {
            Ok(v) => Ok(v),
            Err(e) => Err(Box::new(InterpretatorError::EvaluationError(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpretator_scope() {
        let mut scope = Scope::new(None);
        scope.set("a".to_string(), Value::Number(1.0));
        assert_eq!(scope.get("a"), Some(&Value::Number(1.0)));
        assert_eq!(scope.get("b"), None);
    }

    #[test]
    fn test_interpretator_scope_parent() {
        let mut parent = Scope::new(None);
        parent.set("a".to_string(), Value::Number(1.0));
        let scope = Scope::new(Some(&parent));
        assert_eq!(scope.get("a"), Some(&Value::Number(1.0)));
        assert_eq!(scope.get("b"), None);
    }

    #[test]
    fn test_interpretator_scope_parent_parent() {
        let mut parent = Scope::new(None);
        parent.set("a".to_string(), Value::Number(1.0));
        let mut parent2 = Scope::new(Some(&parent));
        parent2.set("b".to_string(), Value::Number(2.0));
        let scope = Scope::new(Some(&parent2));
        assert_eq!(scope.get("a"), Some(&Value::Number(1.0)));
        assert_eq!(scope.get("b"), Some(&Value::Number(2.0)));
        assert_eq!(scope.get("c"), None);
    }

    #[test]
    fn test_interpretator_redefining_parent_scope_variable() {
        let mut parent = Scope::new(None);
        parent.set("a".to_string(), Value::Number(1.0));
        let mut scope = Scope::new(Some(&parent));
        scope.set("a".to_string(), Value::Number(2.0));
        assert_eq!(scope.get("a"), Some(&Value::Number(2.0)));
        assert_eq!(parent.get("a"), Some(&Value::Number(1.0)));
    }

    #[test]
    fn test_interpretator_initialize_without_global_scope() {
        let mut interpretator = Interpretator::new(None);
        interpretator
            .global_scope
            .set("a".to_string(), Value::Number(1.0));
        assert_eq!(
            interpretator.global_scope.get("a").unwrap(),
            &Value::Number(1.0)
        );
    }
}
