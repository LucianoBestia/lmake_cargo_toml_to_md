// region: lmake_md_to_doc_comments include README.md A //!
//! # lmake_cargo_toml_to_md
//!
//! ***version: 0.5.5  date: 2020-08-22 authors: Luciano Bestia***  
//! **Includes Cargo.toml data into md.**
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-108-green.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-119-blue.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-13-purple.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)
//!
//! [![crates.io](https://meritbadge.herokuapp.com/lmake_cargo_toml_to_md)](https://crates.io/crates/lmake_cargo_toml_to_md) [![Documentation](https://docs.rs/lmake_cargo_toml_to_md/badge.svg)](https://docs.rs/lmake_cargo_toml_to_md/) [![crev reviews](
//! https://web.crev.dev/rust-reviews/badge/crev_count/lmake_cargo_toml_to_md.svg
//! )](https://web.crev.dev/rust-reviews/crate/lmake_cargo_toml_to_md/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/lmake_cargo_toml_to_md/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/blob/master/LICENSE)
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
//!     "lmake_md_to_doc_comments",
//!     "cargo doc --no-deps --document-private-items",
//!     "\\rsync -avz --delete-after target/doc/*  docs/",
//! ]
//! ```
//!
// endregion: lmake_md_to_doc_comments include README.md A //!

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};
use chrono::Datelike;
use chrono::Utc;
use glob::glob;
use lazy_static::lazy_static;
use regex::*;
use std::fs;
use unwrap::unwrap;

lazy_static! {
    static ref REGEX_REMOVE_EMAIL: Regex = unwrap!(Regex::new(r#"( <.+?>)"#));
}
lazy_static! {
    static ref REGEX_VERSION: Regex = unwrap!(Regex::new(r#"(?m)^version = "(.+?)"$"#));
}

lazy_static! {
    static ref REGEX_AUTHORS: Regex = unwrap!(Regex::new(r#"(?m)^authors = \["(.+?)"\]$"#));
}
lazy_static! {
    static ref REGEX_REPOSITORY: Regex = unwrap!(Regex::new(r#"(?m)^repository = "(.+?)"$"#));
}
lazy_static! {
    static ref REGEX_DESCRIPTION: Regex = unwrap!(Regex::new(r#"(?m)^description = "(.+?)"$"#));
}
lazy_static! {
    static ref REGEX_MD_START: Regex =
        Regex::new(r#"(?m)^\[comment\]: # \(lmake_cargo_toml_to_md start\)$"#).unwrap();
}
lazy_static! {
    static ref REGEX_MD_END: Regex =
        Regex::new(r#"(?m)^\[comment\]: # \(lmake_cargo_toml_to_md end\)$"#).unwrap();
}

pub fn cargo_toml_to_md() {
    let cargo_toml_text = unwrap!(fs::read_to_string("Cargo.toml"));
    let cap = unwrap!(REGEX_VERSION.captures(&cargo_toml_text));
    let version = cap.get(1).unwrap().as_str();
    let cap = unwrap!(REGEX_AUTHORS.captures(&cargo_toml_text));
    let authors = cap.get(1).unwrap().as_str();
    let authors = REGEX_REMOVE_EMAIL.replace_all(authors, "");
    let cap = unwrap!(REGEX_REPOSITORY.captures(&cargo_toml_text));
    let repository = cap.get(1).unwrap().as_str();
    let cap = unwrap!(REGEX_DESCRIPTION.captures(&cargo_toml_text));
    let description = cap.get(1).unwrap().as_str();

    let new_text = format!(
        "\n**{}**  \n***[repo]({}); version: {}  date: {} authors: {}***  \n\n",
        description,
        repository,
        version,
        &utc_now(),
        authors,
    );
    println!("new text: '{}'", Green.paint(&new_text));

    for filename_result in unwrap!(glob("*.md")) {
        let filename_pathbuff = unwrap!(filename_result);
        let md_filename = unwrap!(filename_pathbuff.to_str());
        let mut md_text_content = unwrap!(fs::read_to_string(md_filename));

        if let Some(cap) = REGEX_MD_START.captures(&md_text_content) {
            let pos_start = unwrap!(cap.get(0)).end() + 1;
            if let Some(cap) = REGEX_MD_END.captures(&md_text_content) {
                let pos_end = unwrap!(cap.get(0)).start();
                md_text_content.replace_range(pos_start..pos_end, &new_text);
                println!("write md file: {}", Yellow.paint(md_filename));
                unwrap!(fs::write(md_filename, md_text_content));
            }
        }
    }
}
/// utc now
fn utc_now() -> String {
    let now = Utc::now();
    format!(
        "{:04}-{:02}-{:02}",
        now.year(),
        now.month() as i32,
        now.day() as i32,
    )
}
