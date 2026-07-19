use clap::Parser;
use yy::{cli, work_folder};

fn main() {
    let cli = cli::Cli::parse();

    match cli::run(&work_folder(), cli) {
        Ok(output) => print!("{output}"),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}
