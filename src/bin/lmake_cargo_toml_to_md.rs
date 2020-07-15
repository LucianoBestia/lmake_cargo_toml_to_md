// region: lmake_md_to_doc_comments include README.md A //!
//! # lmake_cargo_toml_to_md
//!
//! ***version: 0.5.4  date: 2020-07-15 authors: Luciano Bestia***  
//! **Includes cargo.toml data into md.**
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-108-green.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-81-blue.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-13-purple.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)
//!
//! Includes data from cargo.toml to md files.  
//! To avoid out of sync data like version, authors and description.  
//! The `lmake_cargo_toml_to_md` binary must be executed in the project root folder where is the cargo.toml file.  
//! It works only for single projects and not for workspaces.  
//!
//! ## include cargo.toml data into md file
//!
//! In the md file write these markers:  
//!
//! ```markdown
//! 1 [comment]: # (lmake_cargo_toml_to_md start)
//! 2 [comment]: # (lmake_cargo_toml_to_md end)
//! ```
//!
//! lmake_cargo_toml_to_md deletes the old lines between the markers  
//! and includes the date and the cargo.toml data:  
//! version, authors, description.  
//!
//! ## Tasks in Makefile.toml  
//!
//! I use `cargo make` to script the repetitive commands sequences.  
//! <https://github.com/sagiegurari/cargo-make>  
//! In `Makefile.toml` add a task like this:  
//!
//! ```toml
//! [tasks.doc]
//! description = "create docs from doc comments"
//! clear = true
//! script = [
//!     "lmake_cargo_toml_to_md",
//!     "cargo doc --no-deps --document-private-items",
//!     "\\rsync -avz --delete-after target/doc/*  docs/",
//! ]
//! ```
//!
// endregion: lmake_md_to_doc_comments include README.md A //!

// region: lmake_md_to_doc_comments include README.md B //!
//! ## install
//!
//! Install from crates.io:  
//! `cargo install lmake_cargo_toml_to_md`  
//! Then you can use it in every rust project folder.  
//! No arguments needed to execute the util.  
//!
//! ## Development
//!
//! Documentation:  
//! <https://lucianobestia.github.io/lmake_cargo_toml_to_md/>  
//! List of prepared make tasks for development: build, run, doc, publish,...  
//! `clear; cargo make`  
//!
// endregion: lmake_md_to_doc_comments include README.md B //!

use lmake_cargo_toml_to_md::*;

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};
//use ansi_term::Style;
use clap::App;
use std::env;

/// The program starts here. No arguments.
fn main() {
    // this function is different for Windows and for Linux.
    // Look at the code of this function (2 variations).
    enable_ansi_support();

    // define the CLI input line parameters using the clap library
    let _matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();
    println!("---- {} start ----", Green.paint(env!("CARGO_PKG_NAME")));
    cargo_toml_to_md();
    println!("---- {} end ----", Green.paint(env!("CARGO_PKG_NAME")));
}

// region: different function code for Linux and Windows
#[cfg(target_family = "windows")]
/// only on windows "enable ansi support" must be called
pub fn enable_ansi_support() {
    let _enabled = ansi_term::enable_ansi_support();
}

#[cfg(target_family = "unix")]
//on Linux "enable ansi support" must not be called
pub fn enable_ansi_support() {
    // do nothing
}
