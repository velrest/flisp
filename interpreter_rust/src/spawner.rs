use crate::expression_parser::{RunContext, evaluate_expression};
use std::{io, process, thread, time};

type SpawnCallback = fn() -> io::Result<()>;

pub fn spawn<'a>(
    context: RunContext,
    path: std::path::PathBuf,
    file_name: Option<String>,
) -> io::Result<()> {
    let file_name = match file_name {
        Some(name) => name,
        _ => "main".to_owned(),
    };
    // Process approach
    // // Command variant
    // let output = process::Command::new(
    //     "/home/susumo/projects/flisp/interpreter_rust/target/debug/interpreter_rust",
    // )
    // .arg("-s")
    // .arg(path)
    // .spawn()
    // .expect("failed");
    // println!("{output:?}");
    // thread::sleep(time::Duration::from_millis(1000));
    std::thread::scope(|scope| {
        // Thread variant
        let thread_config = thread::Builder::new().name(file_name);
        let handler = thread_config.spawn_scoped(scope, move || {
            thread::sleep(time::Duration::from_millis(1000));
            evaluate_expression(context)
        });
        let a = handler.expect("failed to spawn").join().unwrap();
        println!("hanler {a:?}");
    });
    Ok(())
}
