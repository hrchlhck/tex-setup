mod cli;
mod macros;
mod gist;

use clap::Parser;

use crate::cli::Execute;

fn main() {
    let args = cli::Program::parse();

    let cmd = args.command();

    cmd.execute();
}
