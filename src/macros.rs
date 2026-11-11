#[doc(hidden)]
pub use colored::Colorize as _colorize;

macro_rules! error {
    ($x:expr) => {
        eprintln!("{}: {}", "ERROR".red().bold(), $x);
    };
}

macro_rules! info {
    ($x:expr) => {
        eprintln!("{}: {}", "INFO".green().bold(), $x);
    };
}

macro_rules! warning {
    ($x:expr) => {
        println!("{}: {}", "WARN".blue().bold(), $x);
    };
}

pub(crate) use error;
pub(crate) use info;
pub(crate) use warning;
