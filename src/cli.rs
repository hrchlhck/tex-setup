use clap::{Parser, Subcommand};
use std::process::Command;
use std::{io::Write, path::Path};

use crate::macros::*;
use crate::gist::{GistCommit, get_info};
use reqwest::blocking::ClientBuilder;

/// Trait for execute a command
pub trait Execute {
    /// Dynamically execute a command based on Self
    fn execute(&self);
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Installs the classes listed in file
    Install {
        #[arg(short = 'i', long = "input")]
        class_file: String,

        #[arg(short = 't', long = "token")]
        token: String,
    },

    /// Uninstalls the user-defined classes
    Uninstall,

    /// Checks for updates
    CheckUpdate,
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
            Self::Install { class_file, token } => self._install(class_file, token),
            Self::Uninstall => self._uninstall(),
            Self::CheckUpdate => self._check_update(),
        }
    }
}

impl Commands {
    /// Installs the classes defined within `.tex`
    fn _install(&self, class_file: &String, token: &str) {
        let texmf_home = get_texmf_home();
        info!(format!("Using {texmf_home} as TEXMFHOME"));

        // Add class base path
        let classes = Path::new(&texmf_home).join("tex").join("latex");

        match std::fs::exists(class_file) {
            Ok(_) => {}
            Err(e) => {
                error!(e);
                std::process::exit(1);
            }
        };

        let user_home = std::env::var("HOME").unwrap();
        let (dirname, filename) = get_env();
        let file_path = Path::new(&user_home).join(&dirname);
        info!(format!("Created configuration file at {:?}", file_path));

        match std::fs::create_dir(&file_path) {
            Ok(_) => {}
            Err(_) => {
                warning!(format!("{:?} already exists", file_path));
            }
        }

        // Create directories recursivelly based on the classes within `./tex`
        info!("Installing classes...");
        let mut install_count = 0;

        let buf = std::fs::read_to_string(class_file).unwrap();
        let mut version_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path.join(filename))
            .unwrap();
        

        for (i, line) in buf.split("\n").enumerate() {
            let url_parts: Vec<&str> = line.trim().split("/").collect();
            let gist_id = url_parts[url_parts.len() - 1 as usize].trim();
            let info = get_info(gist_id, token);
            let file = info.files.iter().nth(i).unwrap();
            let version = info.history.iter().nth(i).unwrap().version.clone();
            info!(format!("\t {} -> {}", file.0, version));
            version_file
                .write(format!("{};{version};{gist_id}", file.0).as_bytes())
                .unwrap();

            let dir_name = Path::new(&file.0).file_stem().unwrap();
            let class_path = classes.join(dir_name);
            match std::fs::create_dir(&class_path) {
                Ok(_) => {},
                Err(_) => {
                    warning!("File already exists. Updating with the newest version");
                }
            }

            let mut class_file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(class_path.join(file.0)).unwrap();

            class_file.write(file.1.content.as_bytes()).unwrap();

            install_count += 1;
        }

        if install_count == 0 {
            warning!("No '.cls' were found in './tex'.");
        } else {
            let msg = format!(
                "{} were installed successfully at {:?}",
                install_count,
                classes.canonicalize().unwrap()
            );
            info!(msg);
        }
    }

    /// Uninstalls the user-specified classes
    fn _uninstall(&self) {
        let texmf_home = get_texmf_home();

        info!(format!("Using {texmf_home} as TEXMFHOME"));

        // Add class base path
        let classes = Path::new(&texmf_home).join("tex").join("latex");

        for entry in std::fs::read_dir(classes).unwrap().map(|x| x.unwrap()) {
            if entry.path().is_dir() {
                std::fs::remove_dir_all(&entry.path()).unwrap();
                info!(format!("Removed {:?}", entry.path().as_os_str()));
            }
        }

        let (dirname, _) = get_env();
        let home = std::env::var("HOME").unwrap();
        let home_path = Path::new(&home).join(dirname);
        std::fs::remove_dir_all(&home_path).unwrap();
        info!(format!("Removed configuration files at {home_path:?}"));

        info!("tex-setup uninstalled successfully");
    }

    /// checks updates
    fn _check_update(&self) {
        let client = ClientBuilder::new()
            .user_agent("TeX-Setup/0.1.0")
            .build()
            .unwrap();

        
        let user_home = std::env::var("HOME").unwrap();
        let (dirname, filename) = get_env();
        let path = Path::new(&user_home).join(&dirname).join(&filename);
        let file_contents = std::fs::read_to_string(path).unwrap();
        
        for gist in file_contents.split("\n") {
            let gist_id = gist.split(";").last().unwrap();
            let msg = format!("Checking updates for {gist_id}...");
            info!(msg);
            let result = client
                .get(format!("https://api.github.com/gists/{gist_id}"))
                .send()
                .unwrap();
    
            let gist: GistCommit = result.json().unwrap();
    
            let mut requires_update = false;
            for line in file_contents.split("\n") {
                let line: Vec<&str> = line.split(";").collect();
                let (filename, version) = (line[0], line[1]);
    
                if !gist.history[0].version.eq(version) {
                    warning!(format!("{filename} requires update!"));
                    requires_update = true;
                }
            }

            if requires_update {
                info!("To update re-run the tool using the install option!");
            } else {
                info!("Everything is up to date!");
            }
        }
    }
}

fn get_env() -> (String, String) {
    let dirname = env!("CONFIGURATION_DIRNAME").trim_matches(|c| c == '"' || c == '\'');
    let filename = env!("CONFIGURATION_FILENAME").trim_matches(|c| c == '"' || c == '\'');

    (dirname.to_string(), filename.to_string())
}

fn get_texmf_home() -> String{
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

   result.trim().to_string()
}