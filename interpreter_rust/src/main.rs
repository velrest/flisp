use clap::Parser;

mod cli;
mod spawner;

mod expression_parser;
use expression_parser::evaluate_expression;

use crate::expression_parser::RunContext;

fn main() {
    let cli::Arguments {
        start_path,
        use_processes,
    } = cli::Arguments::parse();

    let file_name = match start_path.file_name() {
        Some(name) => match name.to_str() {
            Some(str) => str,
            _ => panic!("Initial filename could not be converted to string for {start_path:?}!"),
        },
        _ => panic!("Initial filename could not be read for {start_path:?}!"),
    };

    let main_context = RunContext {
        path: start_path.clone(),
        file_name,
        parent_context: None,
    };

    evaluate_expression(main_context, use_processes).unwrap();
}
