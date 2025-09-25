use clap::Parser;
use std::fs;
use std::process;

mod cli;

fn main() {
    let cli::Arguments { start_path } = cli::Arguments::parse();
    evaluate_expression(start_path);
}

fn evaluate_expression(path: std::path::PathBuf) {
    println!("Item  = {path:?}");
    let paths = match fs::read_dir(&path) {
        Ok(paths) => paths,
        Err(error) => {
            println!("{path:?}  = {error:?}");
            std::fs::ReadDir {}
        }
    };
    for child in paths {
        if child.is_dir() {
        } else if child.is_file() {
            //compute
        } else if child.is_symlink() {

            //follow
        } else {
            process::exit(1);
        }
    }
}
