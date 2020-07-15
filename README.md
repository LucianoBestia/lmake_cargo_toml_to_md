[comment]: # (lmake_md_to_doc_comments segment start A)

# lmake_cargo_toml_to_md

[comment]: # (lmake_cargo_toml_to_md start)

***version: 0.5.4  date: 2020-07-15 authors: Luciano Bestia***  
**Includes cargo.toml data into md.**

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-108-green.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-81-blue.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-13-purple.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/lmake_cargo_toml_to_md/)

[comment]: # (lmake_lines_of_code end)

[![crates.io](https://meritbadge.herokuapp.com/lmake_cargo_toml_to_md)](https://crates.io/crates/lmake_cargo_toml_to_md) [![Documentation](https://docs.rs/lmake_cargo_toml_to_md/badge.svg)](https://docs.rs/lmake_cargo_toml_to_md/) [![crev reviews](
https://web.crev.dev/rust-reviews/badge/crev_count/lmake_cargo_toml_to_md.svg
)](https://web.crev.dev/rust-reviews/crate/lmake_cargo_toml_to_md/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/lmake_cargo_toml_to_md/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/reader_for_microxml/blob/master/LICENSE)

Includes data from cargo.toml to md files.  
To avoid out of sync data like version, authors and description.  
The `lmake_cargo_toml_to_md` binary must be executed in the project root folder where is the cargo.toml file.  
It works only for single projects and not for workspaces.  

## include cargo.toml data into md file

In the md file write these markers:  

```markdown
1 [comment]: # (lmake_cargo_toml_to_md start)
2 [comment]: # (lmake_cargo_toml_to_md end)
```

lmake_cargo_toml_to_md deletes the old lines between the markers  
and includes the date and the cargo.toml data:  
version, authors, description.  

## Tasks in Makefile.toml  

I use `cargo make` to script the repetitive commands sequences.  
<https://github.com/sagiegurari/cargo-make>  
In `Makefile.toml` add a task like this:  

```toml
[tasks.doc]
description = "create docs from doc comments"
clear = true
script = [
    "lmake_cargo_toml_to_md",
    "lmake_md_to_doc_comments",
    "cargo doc --no-deps --document-private-items",
    "\\rsync -avz --delete-after target/doc/*  docs/",
]
```

[comment]: # (lmake_md_to_doc_comments segment end A)

[comment]: # (lmake_md_to_doc_comments segment start B)

## install

Install from crates.io:  
`cargo install lmake_cargo_toml_to_md`  
Then you can use it in every rust project folder.  
No arguments needed to execute the util.  

## Development

Documentation:  
<https://lucianobestia.github.io/lmake_cargo_toml_to_md/>  
List of prepared make tasks for development: build, run, doc, publish,...  
`clear; cargo make`  

[comment]: # (lmake_md_to_doc_comments segment end B)

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  
