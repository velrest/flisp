use clap::{Parser, builder::OsStr};
use std::{fs, io, process, thread, time};

mod cli;

fn main() {
    let cli::Arguments {
        start_path,
        use_processes,
    } = cli::Arguments::parse();
    evaluate_expression(start_path, use_processes).unwrap();
}

fn spawn(
    path: std::path::PathBuf,
    use_processes: bool,
    file_name: Option<String>,
) -> io::Result<()> {
    let file_name = match file_name {
        Some(name) => name,
        _ => "main".to_owned(),
    };
    if use_processes {
        // Command variant
        let output = process::Command::new(
            "/home/susumo/projects/flisp/interpreter_rust/target/debug/interpreter_rust",
        )
        .arg("-s")
        .arg(path)
        .spawn()
        .expect("failed");
        println!("{output:?}");
        thread::sleep(time::Duration::from_millis(1000));
    } else {
        // Thread variant
        let thread_config = thread::Builder::new().name(file_name);
        let handler = thread_config.spawn(move || {
            thread::sleep(time::Duration::from_millis(1000));
            evaluate_expression(path, use_processes)
        });
        let a = handler?.join().unwrap();
        println!("hanler {a:?}");
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct ExpressionMismatch {
    path: std::path::PathBuf,
    file_name: OsStr,
}

type ExpressionResult<T> = std::result::Result<T, ExpressionMismatch>;

#[derive(Debug, Clone)]
enum Statement {
    WriteIO,
}

fn match_built_in(name: &str) -> Statement {
    if !name.contains(":") {
        println!("{name:?} does not include : in name.");
        panic!()
    }

    let collection = name.split(":").collect::<Vec<&str>>();
    let expression = collection[1];

    match expression {
        "write_io" => Statement::WriteIO,
        _ => panic!(),
    }
}

fn read_sorted_dir(path: std::path::PathBuf) -> Vec<std::io::Result<fs::DirEntry>> {
    let mut paths: Vec<_> = fs::read_dir(path).unwrap().collect();
    paths.sort_by_key(|dir| match dir {
        Ok(dir) => dir.file_name().into_string(),
        _ => Ok("Z".to_owned()),
    });
    paths
}

fn evaluate_expression(path: std::path::PathBuf, use_processes: bool) -> io::Result<()> {
    println!("Item  = {path:?}");
    let paths = read_sorted_dir(path);
    println!("{:?}", paths);
    for child in paths {
        match child {
            Ok(entry) => match entry.file_type() {
                Ok(file_type) => {
                    if file_type.is_file() {
                        println!("is file");
                        let built_in = match_built_in(&entry.file_name().into_string().unwrap());
                        println!("{built_in:?}")
                    } else if file_type.is_dir() {
                        println!("is dir");
                        println!("is dir {:?}", entry);
                        spawn(
                            entry.path(),
                            use_processes,
                            Some(entry.file_name().into_string().unwrap()),
                        );
                    } else if file_type.is_symlink() {
                        println!("is symlink");
                    } else {
                        println!("Unknown {:?}", entry.path());
                    }
                }
                Err(e) => {
                    println!("Error getting file type for {:?}: {}", entry.path(), e);
                }
            },
            _ => println!("Oops"),
        }
        // if child.is_dir?() {
        //     println!("dir")
        // } else if child.is_file() {
        //     println!("file")
        // } else if child.is_symlink() {
        //     println!("symlink")
        // }
    }
    Ok(())
    // for child in paths {
    //     if child.is_dir() {
    //     } else if child.is_file() {
    //         //compute
    //     } else if child.is_symlink() {

    //         //follow
    //     } else {
    //         process::exit(1);
    //     }
    // }
}
