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

pub fn evaluate_expression(context: RunContext) -> io::Result<()> {
    let path = &context.path;
    println!("Item  = {path:?}");
    let paths = read_sorted_dir(path);
    for child in paths {
        match child {
            Ok(entry) => match entry.file_type() {
                Ok(file_type) => {
                    if file_type.is_file() {
                        let built_in = match_built_in(&entry.file_name().into_string().unwrap());
                        println!("{built_in:?}");
                        eval(context.clone(), built_in);
                    } else if file_type.is_dir() {
                        let name = entry.file_name().into_string().unwrap();
                        if name == "_mem" {
                            println!("Ignoring _mem");
                            continue;
                        }
                        let new_context = RunContext {
                            path: entry.path().clone(),
                            file_name: &name,
                            parent_context: Some(Box::new(context.clone())),
                        };

                        // Replace with src/spawner::spawn
                        evaluate_expression(new_context);
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
    }
    Ok(())
}

fn match_built_in(name: &str) -> Statement {
    let expression = if name.contains(":") {
        let collection = name.split(":").collect::<Vec<&str>>();
        collection[1]
    } else {
        name
    };

    match expression {
        "write_io" => Statement::WriteIO,
        _ => panic!(),
    }
}

fn eval(context: RunContext, statement: Statement) -> RunContext {
    match (statement) {
        Statement::WriteIO => println!("Whoop WriteIO reached!!"),
    }
    context
}

fn read_sorted_dir(path: &std::path::PathBuf) -> Vec<std::io::Result<fs::DirEntry>> {
    let mut paths: Vec<_> = fs::read_dir(path).unwrap().collect();
    paths.sort_by_key(|dir| match dir {
        Ok(dir) => dir.file_name().into_string(),
        _ => Ok("Z".to_owned()),
    });
    paths
}

// pub fn evaluate_expression(context: &RunContext, use_processes: bool) -> io::Result<()> {
//     let path = &context.path;
//     println!("Item  = {path:?}");
//     let paths = read_sorted_dir(path);
//     println!("{:?}", paths);
//     for child in paths {
//         match child {
//             Ok(entry) => match entry.file_type() {
//                 Ok(file_type) => {
//                     if file_type.is_file() {
//                         println!("is file");
//                         let built_in = match_built_in(&entry.file_name().into_string().unwrap());
//                         println!("{built_in:?}")
//                     } else if file_type.is_dir() {
//                         let name = entry.file_name().into_string().unwrap();
//                         if name == "_mem" {
//                             println!("Ignoring _mem");
//                             continue;
//                         }
//                         println!("is dir");
//                         println!("is dir {:?}", entry);
//                         spawn_closure(&context, entry.path(), use_processes, Some(name));
//                     } else if file_type.is_symlink() {
//                         println!("is symlink");
//                     } else {
//                         println!("Unknown {:?}", entry.path());
//                     }
//                 }
//                 Err(e) => {
//                     println!("Error getting file type for {:?}: {}", entry.path(), e);
//                 }
//             },
//             _ => println!("Oops"),
//         }
//         // if child.is_dir?() {
//         //     println!("dir")
//         // } else if child.is_file() {
//         //     println!("file")
//         // } else if child.is_symlink() {
//         //     println!("symlink")
//         // }
//     }
//     Ok(())
//     // for child in paths {
//     //     if child.is_dir() {
//     //     } else if child.is_file() {
//     //         //compute
//     //     } else if child.is_symlink() {

//     //         //follow
//     //     } else {
//     //         process::exit(1);
//     //     }
//     // }
// }
