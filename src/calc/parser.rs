use pest::iterators::Pair;
use pest_derive::Parser;

use crate::stackmachine::ops::{Op, OpType};

#[derive(Parser)]
#[grammar = "calc.pest"]
pub struct CalcParser;

pub fn print_tree(pair: pest::iterators::Pair<Rule>, indent: usize) {
    let rule = pair.as_rule();
    let span = pair.as_span();
    let text = pair.as_str();

    println!("{:indent$}Rule: {:?}", "", rule, indent = indent);

    println!("{:indent$}Span: {:?}", "", span, indent = indent);

    println!("{:indent$}Text: {:?}", "", text, indent = indent);

    for inner_pair in pair.into_inner() {
        print_tree(inner_pair, indent + 2);
    }
}

pub fn construct_instructions(pair: Pair<Rule>, instructions: &mut Vec<Op>) -> Result<(), String> {
    match pair.as_rule() {
        Rule::expression => {
            for inner_pair in pair.into_inner() {
                construct_instructions(inner_pair, instructions)?;
            }
        }
        Rule::WHITESPACE => {
            return Err("Unexpected whitespace in instruction construction".into());
        }
        Rule::alpha => {
            return Err("Unexpected alpha in instruction construction".into());
        }
        Rule::digit => {
            return Err("Unexpected digit in instruction construction".into());
        }
        Rule::IDENT => {
            instructions.push(Op {
                op_type: OpType::Load,
                val: None,
                var: Some(pair.as_str().to_string()),
            });
        }
        Rule::NUMBER => {
            let value: f64 = pair
                .as_str()
                .parse()
                .map_err(|e| format!("Failed to parse number: {}", e))?;
            instructions.push(Op {
                op_type: OpType::Const,
                val: Some(crate::stackmachine::ops::ValueType(value)),
                var: None,
            });
        }
        Rule::PLUS => {
            instructions.push(Op {
                op_type: OpType::Add,
                val: None,
                var: None,
            });
        }
        Rule::MINUS => {
            instructions.push(Op {
                op_type: OpType::Sub,
                val: None,
                var: None,
            });
        }
        Rule::MUL => {
            instructions.push(Op {
                op_type: OpType::Mul,
                val: None,
                var: None,
            });
        }
        Rule::DIV => {
            instructions.push(Op {
                op_type: OpType::Div,
                val: None,
                var: None,
            });
        }
        Rule::symbol => {
            for inner_pair in pair.into_inner() {
                construct_instructions(inner_pair, instructions)?
            }
        }
        Rule::atom => {
            for inner_pair in pair.into_inner() {
                construct_instructions(inner_pair, instructions)?
            }
        }
        Rule::term => {
            for inner_pair in pair.into_inner() {
                construct_instructions(inner_pair, instructions)?
            }
        }
        Rule::expression_item => {
            let mut iter = pair.into_inner();
            let opt_var = iter.next();
            for inner_pair in iter {
                construct_instructions(inner_pair, instructions)?;
            }
            let var = opt_var.ok_or_else(|| "Expected variable assignment in expression_item")?;
            if var.as_rule() == Rule::IDENT {
                instructions.push(Op {
                    op_type: OpType::Store,
                    val: None,
                    var: var.as_str().to_string().into(),
                });
            } else {
                return Err("Expected variable assignment in expression_item".into());
            }
        }
    }

    Ok(())
}
