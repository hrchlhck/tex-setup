#![allow(unused)]

mod cli;
mod macros;

use clap::Parser;

use crate::cli::{
    Commands::{Install, Uninstall},
    Execute,
};

fn main() {
    let args = cli::Program::parse();

    args.command().execute();
}
