use pest::Parser;

use crate::{
    calc::parser::{CalcParser, Rule, construct_instructions, print_tree},
    stackmachine::eval::Evaluator,
};

mod calc;
mod stackmachine;

pub fn main() {
    let input = "a = 123.45\nb = 67.89\nc = a b + 2 *";

    let pairs = CalcParser::parse(Rule::expression, input).unwrap();

    println!("Parsed Pairs:");
    for pair in pairs.clone() {
        print_tree(pair, 0);
    }

    let mut instructions = Vec::new();
    for pair in pairs {
        construct_instructions(pair, &mut instructions).unwrap();
    }

    println!("Constructed Instructions:");
    for instruction in instructions.clone() {
        println!("  {:?}", instruction);
    }

    let mut evaluator = Evaluator::new();
    evaluator.evaluation(instructions).unwrap();
    println!("Final Environment: {:?}", evaluator.get_env());
}
