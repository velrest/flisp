use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(short, long)]
    pub start_path: std::path::PathBuf,
}
