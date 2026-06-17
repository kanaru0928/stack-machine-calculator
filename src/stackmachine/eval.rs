use std::collections::HashMap;

use crate::stackmachine::ops::{Op, OpType, ValueType};

#[derive(Debug, Clone)]
pub struct Evalator {
    stack: Vec<ValueType>,
    env: HashMap<String, ValueType>,
}

impl Evalator {
    pub fn new() -> Self {
        Evalator {
            stack: Vec::new(),
            env: HashMap::new(),
        }
    }

    fn pop_stack(&mut self) -> Result<ValueType, String> {
        self.stack
            .pop()
            .ok_or_else(|| "Stack underflow".to_string())
    }

    fn eval_const(&mut self, value: ValueType) -> Result<(), String> {
        self.stack.push(value);
        Ok(())
    }

    fn eval_load(&mut self, var: String) -> Result<(), String> {
        let val = self
            .env
            .get(&var)
            .ok_or_else(|| format!("Variable {} not found", var))?;
        self.stack.push(val.clone());
        Ok(())
    }

    fn eval_store(&mut self, var: String) -> Result<(), String> {
        let val = self
            .stack
            .pop()
            .ok_or_else(|| "Stack underflow on store operation".to_string())?;
        self.env.insert(var, val);
        Ok(())
    }

    fn eval_add(&mut self) -> Result<(), String> {
        let right = self.pop_stack()?;
        let left = self.pop_stack()?;
        let result = ValueType(left.0 + right.0);

        self.stack.push(result);
        Ok(())
    }

    fn eval_sub(&mut self) -> Result<(), String> {
        let right = self.pop_stack()?;
        let left = self.pop_stack()?;
        let result = ValueType(left.0 - right.0);

        self.stack.push(result);
        Ok(())
    }

    fn eval_mul(&mut self) -> Result<(), String> {
        let right = self.pop_stack()?;
        let left = self.pop_stack()?;
        let result = ValueType(left.0 * right.0);

        self.stack.push(result);
        Ok(())
    }

    fn eval_div(&mut self) -> Result<(), String> {
        let right = self.pop_stack()?;
        let left = self.pop_stack()?;
        if right.0 == 0.0 {
            return Err("Division by zero".to_string());
        }
        let result = ValueType(left.0 / right.0);

        self.stack.push(result);
        Ok(())
    }

    pub fn evaluation(&mut self, instructions: Vec<Op>) -> Result<(), String> {
        for inst in instructions {
            match inst.op_type {
                OpType::Const => {
                    let val = inst
                        .val
                        .ok_or_else(|| "Const operation requires a value".to_string())?;
                    self.eval_const(val)
                        .map_err(|e| format!("Error occured while const operation: {}", e))?;
                }
                OpType::Load => {
                    let var = inst
                        .var
                        .ok_or_else(|| "Load operation requires a variable name".to_string())?;
                    self.eval_load(var)
                        .map_err(|e| format!("Error occured while load operation: {}", e))?;
                }
                OpType::Store => {
                    let var = inst
                        .var
                        .ok_or_else(|| "Store operation requires a variable name".to_string())?;
                    self.eval_store(var)
                        .map_err(|e| format!("Error occured while store operation: {}", e))?;
                }
                OpType::Add => {
                    self.eval_add()
                        .map_err(|e| format!("Error occured while add operation: {}", e))?;
                }
                OpType::Sub => {
                    self.eval_sub()
                        .map_err(|e| format!("Error occured while sub operation: {}", e))?;
                }
                OpType::Mul => {
                    self.eval_mul()
                        .map_err(|e| format!("Error occured while mul operation: {}", e))?;
                }
                OpType::Div => {
                    self.eval_div()
                        .map_err(|e| format!("Error occured while div operation: {}", e))?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const() {
        let mut evalator = Evalator::new();
        let instructions = vec![Op {
            op_type: OpType::Const,
            val: Some(ValueType(42.0)),
            var: None,
        }];
        evalator.evaluation(instructions).unwrap();
        assert_eq!(evalator.stack.len(), 1);
        assert_eq!(evalator.stack[0].0, 42.0);
    }

    #[test]
    fn test_invalid_const() {
        let mut evalator = Evalator::new();
        let instructions = vec![Op {
            op_type: OpType::Const,
            val: None,
            var: None,
        }];
        let result = evalator.evaluation(instructions);
        assert!(result.is_err());
    }

    #[test]
    fn test_store() {
        let mut evalator = Evalator::new();
        let instructions = vec![
            Op {
                op_type: OpType::Const,
                val: Some(ValueType(42.0)),
                var: None,
            },
            Op {
                op_type: OpType::Store,
                val: None,
                var: Some("x".to_string()),
            },
        ];
        evalator.evaluation(instructions).unwrap();
        assert_eq!(evalator.stack.len(), 0);
        assert_eq!(evalator.env.get("x").unwrap().0, 42.0);
    }

    #[test]
    fn test_invalid_store() {
        let mut evalator = Evalator::new();
        let instructions = vec![Op {
            op_type: OpType::Store,
            val: None,
            var: Some("x".to_string()),
        }];
        let result = evalator.evaluation(instructions);
        assert!(result.is_err());
    }

    #[test]
    fn test_load() {
        let mut evalator = Evalator::new();
        evalator.env.insert("x".to_string(), ValueType(42.0));
        let instructions = vec![Op {
            op_type: OpType::Load,
            val: None,
            var: Some("x".to_string()),
        }];
        evalator.evaluation(instructions).unwrap();
        assert_eq!(evalator.stack.len(), 1);
        assert_eq!(evalator.stack[0].0, 42.0);
    }

    #[test]
    fn test_invalid_load() {
        let mut evalator = Evalator::new();
        let instructions = vec![Op {
            op_type: OpType::Load,
            val: None,
            var: Some("x".to_string()),
        }];
        let result = evalator.evaluation(instructions);
        assert!(result.is_err());
    }

    #[test]
    fn test_add() {
        let mut evalator = Evalator::new();
        let instructions = vec![
            Op {
                op_type: OpType::Const,
                val: Some(ValueType(10.0)),
                var: None,
            },
            Op {
                op_type: OpType::Const,
                val: Some(ValueType(32.0)),
                var: None,
            },
            Op {
                op_type: OpType::Add,
                val: None,
                var: None,
            },
        ];
        evalator.evaluation(instructions).unwrap();
        assert_eq!(evalator.stack.len(), 1);
        assert_eq!(evalator.stack[0].0, 42.0);
    }

    #[test]
    fn test_sub() {
        let mut evalator = Evalator::new();
        let instructions = vec![
            Op {
                op_type: OpType::Const,
                val: Some(ValueType(42.0)),
                var: None,
            },
            Op {
                op_type: OpType::Const,
                val: Some(ValueType(10.0)),
                var: None,
            },
            Op {
                op_type: OpType::Sub,
                val: None,
                var: None,
            },
        ];
        evalator.evaluation(instructions).unwrap();
        assert_eq!(evalator.stack.len(), 1);
        assert_eq!(evalator.stack[0].0, 32.0);
    }

    #[test]
    fn test_mul() {
        let mut evalator = Evalator::new();
        let instructions = vec![
            Op {
                op_type: OpType::Const,
                val: Some(ValueType(6.0)),
                var: None,
            },
            Op {
                op_type: OpType::Const,
                val: Some(ValueType(7.0)),
                var: None,
            },
            Op {
                op_type: OpType::Mul,
                val: None,
                var: None,
            },
        ];
        evalator.evaluation(instructions).unwrap();
        assert_eq!(evalator.stack.len(), 1);
        assert_eq!(evalator.stack[0].0, 42.0);
    }

    #[test]
    fn test_div() {
        let mut evalator = Evalator::new();
        let instructions = vec![
            Op {
                op_type: OpType::Const,
                val: Some(ValueType(84.0)),
                var: None,
            },
            Op {
                op_type: OpType::Const,
                val: Some(ValueType(2.0)),
                var: None,
            },
            Op {
                op_type: OpType::Div,
                val: None,
                var: None,
            },
        ];
        evalator.evaluation(instructions).unwrap();
        assert_eq!(evalator.stack.len(), 1);
        assert_eq!(evalator.stack[0].0, 42.0);
    }

    #[test]
    fn test_div_by_zero() {
        let mut evalator = Evalator::new();
        let instructions = vec![
            Op {
                op_type: OpType::Const,
                val: Some(ValueType(42.0)),
                var: None,
            },
            Op {
                op_type: OpType::Const,
                val: Some(ValueType(0.0)),
                var: None,
            },
            Op {
                op_type: OpType::Div,
                val: None,
                var: None,
            },
        ];
        let result = evalator.evaluation(instructions);
        assert!(result.is_err());
    }
}
