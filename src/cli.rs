use clap::{Parser, Subcommand, error};

use std::path::Path;
use std::process::Command;

use crate::macros::*;

/// Trait for execute a command
pub trait Execute {
    /// Dynamically execute a command based on Self
    fn execute(&self);
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Installs the classes within `./tex`
    Install {
        /// asdsds
        #[arg(short='i', long="input")]
        class_dir: String
    },

    /// Uninstalls the user-defined classes
    Uninstall,


    /// Updates the classes if there's a newer version o the repository
    Update,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Program {
    #[command(subcommand)]
    command: Commands,
}

impl Program {
    /// Returns the private field `command`.
    pub fn command(&self) -> Commands {
        self.command.clone()
    }
}

impl Execute for Commands {
    fn execute(&self) {
        match self {
            Self::Install{class_dir} => self._install(class_dir),
            Self::Uninstall => self._uninstall(),
            Self::Update => self._update(),
        }
    }
}

impl Commands {
    /// Installs the classes defined within `.tex`
    fn _install(&self, class_dir: &String) {
        let c = Command::new("kpsewhich")
            .arg("-var-value=TEXMFHOME")
            .output();

        let output = match c {
            Ok(v) => v,
            Err(e) => {
                error!(e);
                std::process::exit(1);
            }
        };

        let result = match String::from_utf8(output.stdout) {
            Ok(v) => v,
            Err(e) => {
                error!(e); 
                std::process::exit(1);
            }
        };
        let texmf_home = result.trim();
        info!(format!("Using {texmf_home} as TEXMFHOME"));

        // Add class base path
        let classes = Path::new(texmf_home).join("tex").join("latex");

        let dirs = match std::fs::read_dir(class_dir) {
            Ok(v) => v,
            Err(e) => {
                error!(e);
                std::process::exit(1);
            }
        };

        // Create directories recursivelly based on the classes within `./tex`
        info!("Installing classes...");
        let mut install_count = 0;
        for file in std::fs::read_dir(class_dir)
            .unwrap()
            .filter(|x| x.as_ref().unwrap().path().extension().unwrap() == "cls")
            .map(|x| x.unwrap().path()) {
            // Creating file
            let dirname = classes.join(file.file_stem().unwrap());
            std::fs::create_dir_all(&dirname).unwrap();

            // Copying file
            let dst = dirname.join(file.file_name().unwrap());
            std::fs::copy(&file, &dst);
            info!(format!("\t {:?} -> {:?}", file.file_name().unwrap(), dst));
            install_count += 1;
        }
        
        if install_count == 0 {
            warning!("No '.cls' were found in './tex'.");
        } else {
            let msg = format!("{} were installed successfully at {:?}", install_count, classes.canonicalize().unwrap());
            info!(msg);
        }
    }

    /// Uninstalls the user-specified classes
    fn _uninstall(&self) {
        todo!();
    }

    /// Checks the main branch if there's a newer version o the repository, and update the classes
    fn _update(&self) {
        todo!();
    }
}
