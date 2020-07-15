[comment]: # (lmake_md_to_doc_comments segment start A)

# lmake_cargo_toml_to_md

[comment]: # (lmake_cargo_toml_to_md start)

***version: 0.5.4  date: 2020-07-15 authors: Luciano Bestia***  
**Includes cargo.toml data into md.**

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)

[comment]: # (lmake_lines_of_code end)

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
    "cargo doc --no-deps --document-private-items",
    "\\rsync -avz --delete-after target/doc/*  docs/",
]
```

[comment]: # (lmake_md_to_doc_comments segment end A)

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

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  
