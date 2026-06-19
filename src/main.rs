use std::{env, fs};

use pest::Parser;

use crate::{
    calc::{parser::{CalcParser, Rule, print_tree}, trans::construct_instructions},
    stackmachine::eval::Evaluator,
};

mod calc;
mod stackmachine;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");

    let pairs = CalcParser::parse(Rule::expression, &input).unwrap();

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
