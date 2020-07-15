// region: lmake_md_to_doc_comments include README.md A //!
// endregion: lmake_md_to_doc_comments include README.md A //!

// region: lmake_md_to_doc_comments include README.md B //!
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
