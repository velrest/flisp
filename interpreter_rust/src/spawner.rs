use std::{io, process, thread, time};

type SpawnCallback = fn() -> io::Result<()>;

pub fn spawn<T>(
    path: std::path::PathBuf,
    use_processes: bool,
    file_name: Option<String>,
    func: SpawnCallback,
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
        let handler = thread_config.spawn(func);
        let a = handler?.join().unwrap();
        println!("hanler {a:?}");
    }
    Ok(())
}
