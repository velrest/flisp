use clap::Parser;
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
        thread::sleep(time::Duration::from_millis(10000));
    } else {
        // Thread variant
        let thread_config = thread::Builder::new().name(file_name);
        let handler = thread_config.spawn(move || {
            thread::sleep(time::Duration::from_millis(10000));
            evaluate_expression(path, use_processes)
        });
        let a = handler?.join().unwrap();
        println!("hanler {a:?}");
    }
    Ok(())
}

fn evaluate_expression(path: std::path::PathBuf, use_processes: bool) -> io::Result<()> {
    println!("Item  = {path:?}");
    let paths = fs::read_dir(&path)?;
    println!("{:?}", paths);
    for child in paths {
        match child {
            Ok(entry) => match entry.file_type() {
                Ok(file_type) => {
                    if file_type.is_file() {
                        println!("is file");
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
