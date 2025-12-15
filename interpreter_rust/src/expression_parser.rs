use clap::builder::OsStr;
use std::{fs, io, thread, time};

use crate::spawner::spawn;

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

#[derive(Debug, Clone)]
pub struct RunContext<'a> {
    pub path: std::path::PathBuf,
    pub file_name: &'a str,
    pub parent_context: Option<Box<RunContext<'a>>>,
}

pub fn evaluate_expression(context: RunContext, use_processes: bool) -> io::Result<()> {
    let path = context.path;
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
                        let name = entry.file_name().into_string().unwrap();
                        if name == "_mem" {
                            println!("Ignoring _mem");
                            continue;
                        }
                        println!("is dir");
                        println!("is dir {:?}", entry);
                        spawn(entry.path(), use_processes, Some(name), move || {
                            thread::sleep(time::Duration::from_millis(1000));
                            evaluate_expression(context.clone(), use_processes)
                        });
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

fn match_built_in(name: &str) -> Statement {
    // I think we only want to allow : as a sorting feature but not enforce it.
    // if !name.contains(":") {
    //     println!("{name:?} does not include : in name.");
    //     panic!()
    // }

    let expression = if name.contains(":") {
        let collection = name.split(":").collect::<Vec<&str>>();
        collection[1]
    } else {
        name
    };

    match expression {
        "write_io" => Statement::WriteIO,
        "mem" => Statement::WriteIO,
        _ => panic!(),
    }
}

fn eval(context: RunContext, statement: Statement) -> RunContext {
    let built_in = match_built_in(context.file_name);
    println!("{built_in:?}");
    context
}

fn read_sorted_dir(path: std::path::PathBuf) -> Vec<std::io::Result<fs::DirEntry>> {
    let mut paths: Vec<_> = fs::read_dir(path).unwrap().collect();
    paths.sort_by_key(|dir| match dir {
        Ok(dir) => dir.file_name().into_string(),
        _ => Ok("Z".to_owned()),
    });
    paths
}
