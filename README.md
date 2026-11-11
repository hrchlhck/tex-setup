# tex-setup

> Automate the installation and management of custom LaTeX classes (`.cls`) on macOS from GitHub Gists.

`tex-setup` simplifies syncing your personal LaTeX document classes to your local `TEXMFHOME` directory. It tracks Gist revisions so you can keep your local template library updated across environments. 

---

## Features

- **Gist Syncing**: Batch-install LaTeX classes directly from GitHub Gist URLs.
- **Revision Tracking**: Easily check if local classes match their remote Gist versions.
- **Clean Uninstallation**: Wipe installed classes and configuration files with a single command.
- **Automatic Path Resolution**: Directs installations to `~/Library/texmf/tex/latex`.

---

## Prerequisites

- **macOS** with a TeX distribution (e.g., MacTeX, BasicTeX).
- **Rust / Cargo** toolchain.
- **GitHub Personal Access Token (PAT)** with read access to Gists.

---

## Installation

Install directly via Cargo:

```sh
cargo install --locked --git https://github.com/hrchlhck/tex-setup.git
```

## Input File Format
Create a text file (e.g., `classes.list`) containing one GitHub Gist URL per line:
```
https://gist.github.com/username/gist_id_1
https://gist.github.com/username/gist_id_2
```

# Usage 
## 1. Install Classes

Sync classes listed in your input file using your GitHub Personal Access Token:
```sh
tex-setup install -i classes.list -t <YOUR_GITHUB_PERSONAL_ACCESS_TOKEN>
```

Example output:
```log
INFO: Using /Users/user/Library/texmf as TEXMFHOME
INFO: Created configuration file at "/Users/user/.hrck_texsetup"
INFO: Installing classes...
INFO:   atividades.cls -> 40ad6593e1652f7e7945fbfe7826ca8bb3b8dd75
INFO: 1 installed successfully at "/Users/user/Library/texmf/tex/latex"
```

## 2. Check for Updates
Compare installed local classes against the latest commit hashes on GitHub Gist:

```sh
tex-setup check-update
```

Example output:
```log
INFO: Checking updates for 8e8208ed24c493baedb88cca568d1c18...
INFO: Everything is up to date!
```

## 3. Remove all configuration files and LaTeX classes managed by `tex-setup`:
```sh
tex-setup uninstall
```

Example output:
```log
INFO: Using /Users/user/Library/texmf as TEXMFHOME
INFO: Removed "/Users/user/Library/texmf/tex/latex/myclass"
INFO: Removed configuration files at "/Users/user/.hrck_texsetup"
INFO: tex-setup uninstalled successfully
```

# Contribution and feedback
All feedback are welcome. Feel free to file bugs, requests for documentation and any other feedback to the [issue tracker](https://github.com/hrchlhck/tex-setup/issues).
