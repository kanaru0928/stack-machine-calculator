use pest_derive::Parser;

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
